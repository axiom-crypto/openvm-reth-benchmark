use std::collections::BTreeSet;

use alloy_consensus::{TxEnvelope, TxReceipt};
use alloy_primitives::{Address, Bloom};
use alloy_provider::{network::Ethereum, Provider};
use eyre::{eyre, Ok};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::from_proof::transition_proofs_to_tries;
use openvm_mpt::StateUpdateResult;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
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
use revm_primitives::{keccak256, HashMap, B256};

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
    /// Uses the Zeth-style orphan resolution: builds state, applies bundle state
    /// with orphan tracking, and resolves any orphans found during the actual
    /// trie operations.
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

        // For every account we touched, fetch the storage proofs for all the slots we touched.
        tracing::info!("fetching storage proofs");
        let mut before_storage_proofs = Vec::new();
        let mut after_storage_proofs = Vec::new();

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
            before_storage_proofs.push(eip1186_proof_to_account_proof(storage_proof));

            let storage_proof = self
                .provider
                .get_proof(*address, modified_keys)
                .block_id((block_number).into())
                .await?;
            after_storage_proofs.push(eip1186_proof_to_account_proof(storage_proof));
        }

        // Build state tries with Zeth-style orphan resolution.
        // Instead of scanning for all unresolved digests upfront, we apply the bundle state
        // and only resolve orphans that actually block delete operations.
        let mut before_proofs: HashMap<Address, AccountProof> =
            before_storage_proofs.iter().map(|p| (p.address, p.clone())).collect();
        let mut after_proofs: HashMap<Address, AccountProof> =
            after_storage_proofs.iter().map(|p| (p.address, p.clone())).collect();

        // Orphan resolution loop: apply bundle state, resolve blocking orphans, repeat
        const MAX_ORPHAN_ITERATIONS: usize = 10;
        let mut iteration = 0;

        let state = loop {
            iteration += 1;
            if iteration > MAX_ORPHAN_ITERATIONS {
                return Err(eyre!(
                    "failed to resolve orphans after {} iterations",
                    MAX_ORPHAN_ITERATIONS
                ));
            }

            // Build the state tries from proofs
            let mut state = transition_proofs_to_tries(
                previous_block.state_root,
                &before_proofs,
                &after_proofs,
            )?;

            // Try to apply the bundle state and collect any orphans that block progress
            let bundle_state = executor_outcome.state();
            let orphan_result = state.update_from_bundle_state_with_orphans(bundle_state)?;

            let total_orphans = orphan_result.state_orphans.len()
                + orphan_result.storage_orphans.values().map(|v| v.len()).sum::<usize>();

            if total_orphans == 0 {
                // No orphans blocking - we're done!
                eprintln!("[orphan] no blocking orphans found (iteration {})", iteration);
                break state;
            }

            eprintln!(
                "[orphan] iteration {}: found {} blocking orphans ({} state, {} storage accounts)",
                iteration,
                total_orphans,
                orphan_result.state_orphans.len(),
                orphan_result.storage_orphans.len()
            );

            // Resolve the blocking orphans by fetching additional proofs
            let resolved = self
                .resolve_orphans_from_result(
                    &orphan_result,
                    block_number,
                    current_block.hash_slow(),
                    &state_requests,
                    &mut before_proofs,
                    &mut after_proofs,
                )
                .await?;

            if resolved == 0 {
                return Err(eyre!(
                    "failed to resolve any orphans - {} remain unresolvable",
                    total_orphans
                ));
            }

            eprintln!("[orphan] resolved {} orphans, rebuilding state...", resolved);
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

    /// Resolve orphans from a StateUpdateResult by fetching additional proofs.
    /// Returns the number of orphans successfully resolved.
    async fn resolve_orphans_from_result(
        &self,
        orphan_result: &StateUpdateResult,
        block_number: u64,
        block_hash: B256,
        state_requests: &HashMap<Address, Vec<revm_primitives::U256>>,
        before_proofs: &mut HashMap<Address, AccountProof>,
        after_proofs: &mut HashMap<Address, AccountProof>,
    ) -> eyre::Result<usize> {
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);
        // Match zeth: query pre-state for the given block hash with txIndex=0.
        let tx_index = 0usize;
        let mut resolved_count = 0;

        // Resolve storage orphans - collect all keys per address, then batch into single proofs
        for (hashed_address, orphans) in &orphan_result.storage_orphans {
            // Find the actual address from the hashed address
            let address = match state_requests
                .keys()
                .find(|addr| keccak256(addr) == *hashed_address)
            {
                Some(addr) => *addr,
                None => {
                    eprintln!("[orphan] SKIP: no address for hash {}", hashed_address);
                    continue;
                }
            };

            // Debug: show what proofs we have for this account
            if let Some(before_proof) = before_proofs.get(&address) {
                eprintln!(
                    "[orphan-debug] address {} has {} before storage proofs",
                    address,
                    before_proof.storage_proofs.len()
                );
                for sp in &before_proof.storage_proofs {
                    let hashed_key = keccak256(sp.key.0);
                    eprintln!(
                        "[orphan-debug]   before key={}, hashed={}, value={}",
                        sp.key, hashed_key, sp.value
                    );
                }
            }
            if let Some(after_proof) = after_proofs.get(&address) {
                eprintln!(
                    "[orphan-debug] address {} has {} after storage proofs",
                    address,
                    after_proof.storage_proofs.len()
                );
                for sp in &after_proof.storage_proofs {
                    let hashed_key = keccak256(sp.key.0);
                    eprintln!(
                        "[orphan-debug]   after key={}, hashed={}, value={}",
                        sp.key, hashed_key, sp.value
                    );
                }
            }

            // Collect all discovered storage keys for this address across all orphans
            let mut all_storage_keys = Vec::new();

            for orphan in orphans {
                eprintln!(
                    "[orphan] resolving storage orphan for address {}: prefix={:?}, digest={}",
                    address, orphan.prefix, orphan.digest
                );

                // Use debug_storageRangeAt to find matching storage keys (may return multiple)
                let storage_keys = rpc_db
                    .get_storage_key_by_hash_prefix(block_hash, address, &orphan.prefix, tx_index)
                    .await?;

                eprintln!(
                    "[orphan] SUCCESS: found {} storage key(s) for prefix {:?}: {:?}",
                    storage_keys.len(),
                    orphan.prefix,
                    storage_keys
                );

                all_storage_keys.extend(storage_keys);
                resolved_count += 1;
            }

            if all_storage_keys.is_empty() {
                continue;
            }

            // Deduplicate keys
            all_storage_keys.sort();
            all_storage_keys.dedup();

            // Batch all discovered keys into a single eth_getProof call per address
            let before_proof = self
                .provider
                .get_proof(address, all_storage_keys.clone())
                .block_id((block_number - 1).into())
                .await?;

            if let Some(existing) = before_proofs.get_mut(&address) {
                existing
                    .storage_proofs
                    .extend(eip1186_proof_to_account_proof(before_proof).storage_proofs);
            }

            let after_proof = self
                .provider
                .get_proof(address, all_storage_keys)
                .block_id(block_number.into())
                .await?;

            if let Some(existing) = after_proofs.get_mut(&address) {
                existing
                    .storage_proofs
                    .extend(eip1186_proof_to_account_proof(after_proof).storage_proofs);
            }
        }

        // Resolve state trie orphans
        for orphan in &orphan_result.state_orphans {
            eprintln!(
                "[orphan] resolving state trie orphan: prefix={:?}, digest={}",
                orphan.prefix, orphan.digest
            );

            let address = match rpc_db
                .get_account_address_by_hash_prefix(block_hash, &orphan.prefix, tx_index)
                .await?
            {
                Some(addr) => addr,
                None => {
                    tracing::warn!(
                        "could not find account for state trie orphan prefix {:?}, digest={} - skipping",
                        orphan.prefix,
                        orphan.digest
                    );
                    continue;
                }
            };

            tracing::info!("found address {} for state trie orphan", address);

            // Fetch account proofs
            let before_proof = self
                .provider
                .get_proof(address, vec![])
                .block_id((block_number - 1).into())
                .await?;

            before_proofs
                .entry(address)
                .or_insert_with(|| eip1186_proof_to_account_proof(before_proof));

            let after_proof = self
                .provider
                .get_proof(address, vec![])
                .block_id(block_number.into())
                .await?;

            after_proofs
                .entry(address)
                .or_insert_with(|| eip1186_proof_to_account_proof(after_proof));

            resolved_count += 1;
        }

        Ok(resolved_count)
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}
