use std::collections::BTreeSet;

use revm_primitives::HashMap;

use alloy_consensus::{TxEnvelope, TxReceipt};
use alloy_primitives::{Address, Bloom};
use alloy_provider::{network::Ethereum, Provider};
use eyre::{eyre, Ok};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::from_proof::{
    detect_orphans_in_proofs, transition_proofs_to_tries_with_deletions, StateOrphan,
    StorageDeletions, StorageOrphan,
};
use openvm_primitives::{account_proof::eip1186_proof_to_account_proof, AccountProof};
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::Block;
use reth_primitives_traits::block::Block as _;
use revm::database::CacheDB;
use revm_primitives::B256;

mod lookup;
mod rpc;

pub use lookup::PreimageLookup;

/// Configuration for orphan resolution.
#[derive(Debug, Clone)]
pub struct OrphanResolutionConfig {
    /// Preimage lookup table for finding storage keys with specific hash prefixes.
    /// If None, only debug_storageRangeAt will be used (slower but always works).
    pub lookup: Option<PreimageLookup>,
    /// Maximum number of retry iterations for orphan resolution.
    pub max_retries: usize,
}

impl Default for OrphanResolutionConfig {
    fn default() -> Self {
        Self { lookup: None, max_retries: 10 }
    }
}

impl OrphanResolutionConfig {
    /// Creates a config with a precomputed lookup table.
    pub fn with_lookup(prefix_nibbles: u8) -> Self {
        Self { lookup: Some(PreimageLookup::new(prefix_nibbles)), max_retries: 10 }
    }
}

/// An executor that fetches data from a [Provider] to execute blocks in the [ClientExecutor].
#[derive(Debug, Clone)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
    /// Configuration for orphan resolution.
    pub orphan_config: OrphanResolutionConfig,
}

impl<P: Provider<Ethereum> + Clone + std::fmt::Debug> HostExecutor<P> {
    /// Create a new [`HostExecutor`] with a specific [Provider].
    pub fn new(provider: P) -> Self {
        Self { provider, orphan_config: OrphanResolutionConfig::default() }
    }

    /// Create a new [`HostExecutor`] with orphan resolution config.
    pub fn with_orphan_config(provider: P, orphan_config: OrphanResolutionConfig) -> Self {
        Self { provider, orphan_config }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<ClientExecutorInput> {
        // Fetch the current block and the previous block from the provider.
        tracing::info!("fetching the current block and the previous block");
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {block_number}"))?;
        let previous_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {block_number}"))?;

        // Setup the spec for the block executor.
        tracing::info!("setting up the spec for the block executor");
        let spec = MAINNET.clone();

        // Setup the database for the block executor.
        tracing::info!("setting up the database for the block executor");
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);
        let cache_db = CacheDB::new(&rpc_db);

        // Execute the block and fetch all the necessary data along the way.
        tracing::info!(
            "executing the block and with rpc db: block_number={block_number}, transaction_count={}",
            current_block.body.transactions.len()
        );

        let block = current_block.clone().try_into_recovered()?;

        tracing::info!("validate_block_consensus");
        let consensus = EthBeaconConsensus::new(spec.clone());
        consensus.validate_header(block.sealed_header())?;
        consensus.validate_block_pre_execution(&block)?;

        let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec.clone()), cache_db);

        let executor_output = block_executor.execute(&block)?;

        // Validate the block post execution.
        tracing::info!("validating the block post execution");
        validate_block_post_execution(
            &block,
            &spec,
            &executor_output.receipts,
            &executor_output.requests,
        )?;

        // Accumulate the logs bloom.
        tracing::info!("accumulating the logs bloom");
        let mut logs_bloom = Bloom::default();
        executor_output.receipts.iter().for_each(|r| {
            logs_bloom.accrue_bloom(&r.bloom());
        });

        // Convert the output to an execution outcome.
        let executor_outcome = ExecutionOutcome::new(
            executor_output.state,
            vec![executor_output.result.receipts],
            current_block.header.number,
            vec![executor_output.result.requests],
        );

        let state_requests = rpc_db.get_state_requests();

        // Collect storage deletions (slots that go from non-zero to zero) and account deletions
        let mut storage_deletions: HashMap<Address, StorageDeletions> = HashMap::default();
        let mut deleted_accounts: Vec<Address> = Vec::new();

        for (address, _) in state_requests.iter() {
            if let Some(account) = executor_outcome.state().state.get(address) {
                // Check if account is deleted (info becomes None)
                if account.info.is_none() {
                    tracing::debug!(%address, "detected account deletion");
                    deleted_accounts.push(*address);
                }

                // Check for storage deletions
                let deleted_keys: Vec<B256> = account
                    .storage
                    .iter()
                    .filter(|(_, slot)| {
                        // Original was non-zero, new value is zero
                        !slot.original_value().is_zero() && slot.present_value().is_zero()
                    })
                    .map(|(key, _)| B256::from(key.to_be_bytes::<32>()))
                    .collect();

                if !deleted_keys.is_empty() {
                    tracing::debug!(%address, count = deleted_keys.len(), "detected storage deletions");
                    storage_deletions.insert(*address, StorageDeletions { deleted_keys });
                }
            }
        }

        // For every account we touched, fetch the storage proofs for all the slots we touched.
        tracing::info!("fetching storage proofs");
        let mut before_storage_proofs: HashMap<Address, AccountProof> = HashMap::default();
        let mut after_storage_proofs: HashMap<Address, AccountProof> = HashMap::default();

        for (address, used_keys) in state_requests.iter() {
            let modified_keys: Vec<B256> = executor_outcome
                .state()
                .state
                .get(address)
                .map(|account| {
                    account
                        .storage
                        .keys()
                        .map(|key| B256::from(key.to_be_bytes::<32>()))
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
                .into_iter()
                .collect();

            let keys: Vec<B256> = used_keys
                .iter()
                .map(|key| B256::from(key.to_be_bytes::<32>()))
                .chain(modified_keys.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();

            let storage_proof = self
                .provider
                .get_proof(*address, keys.clone())
                .block_id((block_number - 1).into())
                .await?;
            before_storage_proofs.insert(*address, eip1186_proof_to_account_proof(storage_proof));

            let storage_proof = self
                .provider
                .get_proof(*address, modified_keys)
                .block_id(block_number.into())
                .await?;
            after_storage_proofs.insert(*address, eip1186_proof_to_account_proof(storage_proof));
        }

        // Build tries with orphan detection and resolution
        let state = self
            .build_tries_with_orphan_resolution(
                previous_block.state_root,
                previous_block.hash_slow(),
                block_number - 1,
                &mut before_storage_proofs,
                &mut after_storage_proofs,
                &storage_deletions,
                &deleted_accounts,
            )
            .await?;

        // Derive the block header.
        let mut header = current_block.header.clone();
        header.parent_hash = previous_block.hash_slow();
        header.ommers_hash = current_block.body.calculate_ommers_root();
        header.state_root = current_block.state_root;
        header.transactions_root = current_block.transactions_root;
        header.receipts_root = current_block.header.receipts_root;
        header.withdrawals_root = current_block.body.calculate_withdrawals_root();
        header.logs_bloom = logs_bloom;
        header.requests_hash = current_block.requests_hash;

        // Assert the derived header is correct.
        assert_eq!(header.hash_slow(), current_block.header.hash_slow(), "header mismatch");

        tracing::info!(
            "successfully executed block: block_number={}, block_hash={}, state_root={}",
            current_block.header.number,
            header.hash_slow(),
            current_block.state_root
        );

        // Fetch the parent headers needed to constrain the BLOCKHASH opcode.
        let oldest_ancestor = *rpc_db.oldest_ancestor.borrow();
        let mut ancestor_headers = vec![];
        tracing::info!("fetching {} ancestor headers", block_number - oldest_ancestor);
        for height in (oldest_ancestor..=(block_number - 1)).rev() {
            let block = self.provider.get_block_by_number(height.into()).await?.unwrap();
            ancestor_headers.push(block.header.into());
        }

        let state_bytes = state.encode_to_state_bytes();

        // Create the client input.
        let client_input = ClientExecutorInput {
            current_block,
            ancestor_headers,
            parent_state_bytes: state_bytes,
            bytecodes: rpc_db.get_bytecodes(),
        };
        tracing::info!("successfully generated client input");

        Ok(client_input)
    }

    /// Builds tries from proofs with orphan resolution.
    ///
    /// If orphans are detected, fetches additional proofs and retries.
    #[allow(clippy::too_many_arguments)]
    async fn build_tries_with_orphan_resolution(
        &self,
        state_root: B256,
        block_hash: B256,
        block_number: u64,
        before_proofs: &mut HashMap<Address, AccountProof>,
        after_proofs: &mut HashMap<Address, AccountProof>,
        storage_deletions: &HashMap<Address, StorageDeletions>,
        deleted_accounts: &[Address],
    ) -> eyre::Result<openvm_mpt::EthereumState> {
        // Use detect_orphans_in_proofs in the retry loop to avoid leaking memory
        for iteration in 0..self.orphan_config.max_retries {
            let orphans = detect_orphans_in_proofs(
                state_root,
                before_proofs,
                after_proofs,
                storage_deletions,
                deleted_accounts,
            )?;

            if orphans.is_empty() {
                // No orphans - build the final state (this allocates the bump)
                let result = transition_proofs_to_tries_with_deletions(
                    state_root,
                    before_proofs,
                    after_proofs,
                    storage_deletions,
                    deleted_accounts,
                )?;
                return Ok(result.state);
            }

            tracing::info!(
                iteration,
                storage_orphan_count = orphans.storage_orphans.len(),
                state_orphan_count = orphans.state_orphans.len(),
                "detected unresolvable orphans, fetching additional proofs"
            );

            // Resolve storage orphans by fetching additional proofs
            // Group orphans by address to batch proof fetching
            let mut orphans_by_address: HashMap<Address, Vec<B256>> = HashMap::default();
            for orphan in &orphans.storage_orphans {
                let storage_key = self.resolve_storage_orphan_prefix(block_hash, orphan).await?;

                tracing::debug!(
                    address = %orphan.address,
                    prefix = ?orphan.prefix,
                    %storage_key,
                    "resolved storage orphan prefix to storage key"
                );

                orphans_by_address.entry(orphan.address).or_default().push(storage_key);
            }

            // Fetch additional storage proofs and merge them
            for (address, extra_keys) in orphans_by_address {
                // Fetch current keys plus new keys
                let account_proof = before_proofs.get(&address).unwrap();
                let mut all_keys: Vec<B256> =
                    account_proof.storage_proofs.iter().map(|p| p.key).collect();
                all_keys.extend(extra_keys);

                // Re-fetch the full proof with all keys
                let proof = self
                    .provider
                    .get_proof(address, all_keys)
                    .block_id(block_number.into())
                    .await?;

                // Replace the account proof with the new one
                before_proofs.insert(address, eip1186_proof_to_account_proof(proof));
            }

            // Resolve state orphans by fetching additional account proofs
            for orphan in &orphans.state_orphans {
                let address = self.resolve_state_orphan_prefix(block_hash, orphan).await?;

                tracing::debug!(
                    prefix = ?orphan.prefix,
                    %address,
                    "resolved state orphan prefix to address"
                );

                // Fetch proof for this address if we don't already have it
                if let std::collections::hash_map::Entry::Vacant(e) = before_proofs.entry(address) {
                    let proof = self
                        .provider
                        .get_proof(address, vec![])
                        .block_id(block_number.into())
                        .await?;
                    e.insert(eip1186_proof_to_account_proof(proof));

                    // Also fetch after proof
                    let after_proof = self
                        .provider
                        .get_proof(address, vec![])
                        .block_id((block_number + 1).into())
                        .await?;
                    after_proofs.insert(address, eip1186_proof_to_account_proof(after_proof));
                }
            }
        }

        Err(eyre!(
            "failed to resolve all orphans after {} iterations",
            self.orphan_config.max_retries
        ))
    }

    /// Resolves a storage orphan prefix to a storage key.
    ///
    /// First tries the preimage lookup table, then falls back to debug_storageRangeAt.
    async fn resolve_storage_orphan_prefix(
        &self,
        block_hash: B256,
        orphan: &StorageOrphan,
    ) -> eyre::Result<B256> {
        // Try preimage lookup first
        if let Some(ref lookup) = self.orphan_config.lookup {
            if let Some(key) = lookup.find(&orphan.prefix) {
                return Ok(key);
            }
            tracing::debug!(
                prefix_len = orphan.prefix.len(),
                "prefix too long for lookup table, using debug_storageRangeAt"
            );
        }

        // Fall back to debug_storageRangeAt
        rpc::get_next_storage_key(&self.provider, block_hash, orphan.address, &orphan.prefix).await
    }

    /// Resolves a state orphan prefix to an account address.
    ///
    /// Uses debug_accountRange to find an account whose keccak256 hash starts with the prefix.
    async fn resolve_state_orphan_prefix(
        &self,
        block_hash: B256,
        orphan: &StateOrphan,
    ) -> eyre::Result<Address> {
        rpc::get_next_account(&self.provider, block_hash, &orphan.prefix).await
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}
