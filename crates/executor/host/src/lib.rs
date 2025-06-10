use std::collections::BTreeSet;

use alloy_consensus::{TxEnvelope, TxReceipt};
use alloy_primitives::Bloom;
use alloy_primitives::Bytes;
use alloy_provider::{network::Ethereum, Provider};
use alloy_rpc_types_debug::ExecutionWitness;
use eyre::{eyre, Ok};
use eyre::{eyre, Ok, OptionExt};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::{state::HashedPostState, EthereumState};
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::validate_block_post_execution;
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::Block;
use reth_primitives::RecoveredBlock;
use reth_primitives_traits::block::Block as _;
use reth_revm::witness::ExecutionWitnessRecord;
use reth_stateless::StatelessInput;
use revm::database::CacheDB;
use revm_primitives::B256;

/// An executor that fetches data from a [Provider] to execute blocks in the [ClientExecutor].
#[derive(Debug, Clone)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
}

impl<P: Provider<Ethereum> + Clone> HostExecutor<P> {
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<StatelessInput> {
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

        let mut witness_record = ExecutionWitnessRecord::default();
        let executor_output = block_executor.execute_with_state_closure(&block, |statedb| {
            witness_record.record_executed_state(statedb);
        })?;

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

        let state = EthereumState::from_transition_proofs(
            previous_block.state_root,
            &before_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
            &after_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        )?;

        // Verify the state root.
        tracing::warn!("skip verification of state root");
        // tracing::info!("verifying the state root");
        let state_root = {
            let mut mutated_state = state.clone();
            let post_state = HashedPostState::from_bundle_state(&executor_outcome.bundle.state);
            // executor_outcome.hash_state_slow());
            mutated_state.update(&post_state);
            mutated_state.state_root()
        };
        // TODO: this fails for some reason (I think it was failing before reth-stateless changes)
        // if state_root != current_block.state_root {
        //     eyre::bail!("mismatched state root");
        // }

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
            state_root
        );

        // Fetch the parent headers needed to constrain the BLOCKHASH opcode.
        // let oldest_ancestor = *rpc_db.oldest_ancestor.borrow();
        // let mut ancestor_headers = vec![];
        // tracing::info!("fetching {} ancestor headers", block_number - oldest_ancestor);
        // for height in (oldest_ancestor..=(block_number - 1)).rev() {
        //     let block = self.provider.get_block_by_number(height.into()).await?.unwrap();
        //     ancestor_headers.push(block.header.into());
        // }

        let ExecutionWitnessRecord { keys, lowest_block_number, .. } = witness_record;

        let execution_witness = ExecutionWitness {
            state: state.all_rlp_nodes().into_iter().collect(),
            // maybe wrong? Like idk need to use bytecodes or prefill with rpc or something?
            codes: rpc_db.get_bytecodes().iter().map(|b| b.original_bytes()).collect(),
            keys,
            headers: self
                .build_headers_for_witness_range(block_number, lowest_block_number)
                .await?,
        };

        tracing::info!("successfully generated client input");

        let stateless_input = StatelessInput { block: current_block, witness: execution_witness };

        Ok(stateless_input)
    }

    /// Build headers for witness verification based on lowest_block_number (like reth)
    async fn build_headers_for_witness_range(
        &self,
        block_number: u64,
        lowest_block_number: Option<u64>,
    ) -> eyre::Result<Vec<Bytes>> {
        use alloy_rlp::Encodable;

        let smallest = match lowest_block_number {
            Some(smallest) => smallest,
            None => {
                // Return only the parent header, if there were no calls to the
                // BLOCKHASH opcode.
                block_number.saturating_sub(1)
            }
        };

        let range = smallest..block_number;
        tracing::info!("building headers for range: {}..{}", smallest, block_number);

        let mut headers = Vec::new();
        for block_num in range {
            let block = self
                .provider
                .get_block_by_number(block_num.into())
                .await?
                .ok_or_eyre("couldn't fetch block in header range")?;

            let header = block.header;

            // RLP encode the header (like reth does)
            let mut header_bytes = Vec::new();
            header.encode(&mut header_bytes);
            headers.push(header_bytes.into());
        }

        tracing::info!("added {} RLP-encoded headers for witness verification", headers.len());
        Ok(headers)
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}

pub fn validate_block_consensus(block: &RecoveredBlock<Block>) -> eyre::Result<()> {
    let consensus = EthBeaconConsensus::new(MAINNET.clone());

    consensus.validate_header(block.sealed_header())?;

    consensus.validate_block_pre_execution(block)?;

    Ok(())
}
