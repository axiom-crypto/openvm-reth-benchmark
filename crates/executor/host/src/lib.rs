mod preimage;

use std::collections::BTreeSet;

use alloy_consensus::{proofs::calculate_receipt_root, TxEnvelope, TxReceipt};
use alloy_primitives::Bloom;
use alloy_provider::{network::Ethereum, Provider};
use eyre::{eyre, Ok};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::from_proof::transition_proofs_to_tries;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use preimage::PreimageDictionary;
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::Block;
use reth_primitives_traits::block::Block as _;
use reth_trie::AccountProof;
use revm::database::CacheDB;
use revm_primitives::{Address, B256};

/// An executor that fetches data from a [Provider] to execute blocks in the [ClientExecutor].
#[derive(Debug, Clone)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
}

impl<P: Provider<Ethereum> + Clone + std::fmt::Debug> HostExecutor<P> {
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P) -> Self {
        Self { provider }
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
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;
        let previous_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        // Setup the spec for the block executor.
        tracing::info!("setting up the spec for the block executor");
        let spec = MAINNET.clone();

        // Setup the database for the block executor.
        tracing::info!("setting up the database for the block executor");
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);
        let cache_db = CacheDB::new(&rpc_db);

        // Execute the block and fetch all the necessary data along the way.
        tracing::info!(
            "executing the block and with rpc db: block_number={}, transaction_count={}",
            block_number,
            current_block.body.transactions.len()
        );

        let block = current_block.clone().try_into_recovered()?;

        tracing::info!("validate_block_consensus");
        let consensus = EthBeaconConsensus::new(spec.clone());
        consensus.validate_header(block.sealed_header())?;
        consensus.validate_block_pre_execution(&block)?;

        let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec.clone()), cache_db);

        let executor_output = block_executor.execute(&block)?;

        // Pre-compute receipts root and logs bloom to avoid duplicate computation in validation.
        tracing::info!("computing receipts root and logs bloom");
        let receipts_with_bloom =
            executor_output.receipts.iter().map(TxReceipt::with_bloom_ref).collect::<Vec<_>>();
        let receipts_root = calculate_receipt_root(&receipts_with_bloom);
        let logs_bloom =
            receipts_with_bloom.iter().fold(Bloom::ZERO, |bloom, r| bloom | r.bloom_ref());

        // Validate the block post execution.
        tracing::info!("validating the block post execution");
        validate_block_post_execution(
            &block,
            &spec,
            &executor_output.receipts,
            &executor_output.requests,
            Some((receipts_root, logs_bloom)),
        )?;

        // Convert the output to an execution outcome.
        let executor_outcome = ExecutionOutcome::new(
            executor_output.state,
            vec![executor_output.result.receipts],
            current_block.header.number,
            vec![executor_output.result.requests],
        );

        let state_requests = rpc_db.get_state_requests();

        // For every account we touched, fetch the storage proofs for all the slots we touched.
        tracing::info!("fetching storage proofs");
        let mut before_proofs = revm_primitives::HashMap::default();
        let mut after_proofs = revm_primitives::HashMap::default();

        for (address, used_keys) in state_requests.iter() {
            let modified_keys = executor_outcome
                .state()
                .state
                .get(address)
                .map(|account| {
                    account.storage.keys().map(|key| B256::from(*key)).collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();

            let keys = used_keys
                .iter()
                .map(|key| B256::from(*key))
                .chain(modified_keys.clone().into_iter())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            let storage_proof = self
                .provider
                .get_proof(*address, keys.clone())
                .block_id((block_number - 1).into())
                .await?;
            let before_proof = eip1186_proof_to_account_proof(storage_proof);
            before_proofs.insert(before_proof.address, before_proof);

            let storage_proof = self
                .provider
                .get_proof(*address, modified_keys)
                .block_id((block_number).into())
                .await?;
            let after_proof = eip1186_proof_to_account_proof(storage_proof);
            after_proofs.insert(after_proof.address, after_proof);
        }

        // Build preimage dictionary from state requests and bundle state for orphan resolution.
        let dict = PreimageDictionary::build(&state_requests, executor_outcome.state());

        // Resolution loop: build tries, attempt update, resolve orphans if needed.
        const MAX_ORPHAN_ITERATIONS: usize = 10;
        let mut iterations = 0;
        let state = loop {
            let state = transition_proofs_to_tries(
                previous_block.state_root,
                &before_proofs,
                &after_proofs,
            )?;

            let mut state_clone = state.clone();
            match state_clone.update_from_bundle_state(executor_outcome.state()) {
                std::result::Result::Ok(()) => break state,
                Err(openvm_mpt::Error::NodeNotResolved(digest)) => {
                    iterations += 1;
                    if iterations > MAX_ORPHAN_ITERATIONS {
                        return Err(eyre!(
                            "exceeded max orphan resolution iterations ({})",
                            MAX_ORPHAN_ITERATIONS
                        ));
                    }

                    let location = state
                        .find_orphan_location(&digest)
                        .ok_or_else(|| eyre!("cannot locate orphan digest {digest:#} in trie"))?;

                    if let Some(hashed_addr) = location.hashed_address {
                        // Storage trie orphan
                        let address = dict.find_account_by_hash(&hashed_addr).ok_or_else(|| {
                            eyre!("cannot find address for hashed address {hashed_addr:#}")
                        })?;
                        let slot = dict
                            .find_storage_key_by_prefix(&address, &location.nibble_prefix)
                            .ok_or_else(|| {
                                eyre!(
                                    "cannot find storage key matching nibble prefix for \
                                     address {address} (cold sibling)"
                                )
                            })?;

                        tracing::info!(
                            "resolving storage orphan (iteration {}): address={}, slot={:#}",
                            iterations,
                            address,
                            slot
                        );

                        let slot_b256 = B256::from(slot);
                        fetch_and_merge_proofs(
                            &self.provider,
                            address,
                            vec![slot_b256],
                            block_number,
                            &mut before_proofs,
                            &mut after_proofs,
                        )
                        .await?;
                    } else {
                        // State trie orphan
                        let address = dict
                            .find_account_key_by_prefix(&location.nibble_prefix)
                            .ok_or_else(|| {
                                eyre!(
                                    "cannot find account matching nibble prefix (cold sibling)"
                                )
                            })?;

                        tracing::info!(
                            "resolving state trie orphan (iteration {}): address={}",
                            iterations,
                            address
                        );

                        fetch_and_merge_proofs(
                            &self.provider,
                            address,
                            vec![],
                            block_number,
                            &mut before_proofs,
                            &mut after_proofs,
                        )
                        .await?;
                    }
                }
                Err(e) => return Err(e.into()),
            }
        };

        // Skip state root verification for now.
        // It works with Alchemy but for some reason not with Quicknode.
        // It is checked on the client (guest) side and works with all providers.

        // Derive the block header.
        //
        // Note: the receipts root and gas used are verified by `validate_block_post_execution`.
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

        // Log the result.
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
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}

/// Fetch before and after proofs for the given address/keys and merge them into the proof maps.
async fn fetch_and_merge_proofs(
    provider: &(impl Provider<Ethereum> + Clone),
    address: Address,
    keys: Vec<B256>,
    block_number: u64,
    before_proofs: &mut revm_primitives::HashMap<Address, AccountProof>,
    after_proofs: &mut revm_primitives::HashMap<Address, AccountProof>,
) -> eyre::Result<()> {
    let before = provider
        .get_proof(address, keys.clone())
        .block_id((block_number - 1).into())
        .await?;
    let before = eip1186_proof_to_account_proof(before);
    merge_account_proof(before_proofs, before);

    let after = provider.get_proof(address, keys).block_id(block_number.into()).await?;
    let after = eip1186_proof_to_account_proof(after);
    merge_account_proof(after_proofs, after);

    eyre::Ok(())
}

/// Merge a new account proof into the proof map.
/// If the address already exists, merge storage proofs (deduplicate by key).
/// If the address is new, insert the whole proof.
fn merge_account_proof(
    proofs: &mut revm_primitives::HashMap<Address, AccountProof>,
    new_proof: AccountProof,
) {
    if let Some(existing) = proofs.get_mut(&new_proof.address) {
        let existing_keys: BTreeSet<_> =
            existing.storage_proofs.iter().map(|sp| sp.key).collect();
        for sp in new_proof.storage_proofs {
            if !existing_keys.contains(&sp.key) {
                existing.storage_proofs.push(sp);
            }
        }
    } else {
        proofs.insert(new_proof.address, new_proof);
    }
}
