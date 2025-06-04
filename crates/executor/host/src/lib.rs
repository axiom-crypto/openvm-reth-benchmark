use std::{collections::BTreeSet, marker::PhantomData};

use alloy_consensus::{BlockBody, EthereumTxEnvelope, TxReceipt};
use alloy_primitives::Bloom;
use alloy_primitives::Bytes;
use alloy_provider::{
    bindings::IMulticall3::getCurrentBlockGasLimitCall,
    network::{eip2718::Encodable2718, AnyNetwork, AnyRpcBlock, BlockResponse},
    Provider,
};
use alloy_transport::Transport;
use eyre::{eyre, Ok};
use openvm_client_executor::ClientExecutorInput;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_consensus::FullConsensus;
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_evm::{execute::Executor, ConfigureEvm};
use reth_evm_ethereum::execute::EthExecutorProvider;
use reth_execution_types::{BlockExecutionResult, ExecutionOutcome};
use reth_primitives::{Block, RecoveredBlock};
use reth_primitives_traits::proofs;
use reth_trie::HashedPostState;
use revm::database::CacheDB;
use revm_primitives::B256;

/// /// An executor that fetches data from a [Provider] to generate [ExecutionWitness] for a block.
#[derive(Debug, Clone)]
pub struct HostExecutor<T: Transport + Clone, P: Provider<AnyNetwork> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
    /// A phantom type to make the struct generic over the transport.
    pub phantom: PhantomData<T>,
}
impl<T: Transport + Clone, P: Provider<AnyNetwork> + Clone> HostExecutor<T, P> {
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P) -> Self {
        Self { provider, phantom: PhantomData }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<ClientExecutorInput> {
        // For now only MAINNET is supported.
        let spec = MAINNET.clone();

        // Fetch the current block and the previous block from the provider.
        tracing::info!("fetching the current block and the previous block");
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;
        let previous_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .full()
            .await?
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        // Setup the database for the block executor.
        tracing::info!("setting up the database for the block executor");
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);
        let cache_db = CacheDB::new(&rpc_db);

        // Execute the block and fetch all the necessary data along the way.
        tracing::info!(
            "executing the block and with rpc db: block_number={block_number}, transaction_count={transaction_count}",
            transaction_count = current_block.transactions().len()
        );

        let executor_block_input = recover_rpc_block(current_block);

        let executor_output = EthExecutorProvider::new(spec.into())
            .executor(cache_db)
            .execute(&executor_block_input)?;

        // Validate the block post execution.
        tracing::info!("validating the block post execution");
        let consensus = EthBeaconConsensus::new(spec.into());

        let block_execution_result = BlockExecutionResult {
            receipts: executor_output.receipts,
            requests: executor_output.requests,
            gas_used: executor_output.gas_used,
        };

        consensus.validate_block_post_execution(&executor_block_input, &block_execution_result)?;

        // === Old code === Prob not needed anymore
        // Accumulate the logs bloom.
        // tracing::info!("accumulating the logs bloom");
        // let mut logs_bloom = Bloom::default();
        // executor_output.receipts.iter().for_each(|r| {
        //     logs_bloom.accrue_bloom(&r.bloom());
        // });

        // // Convert the output to an execution outcome.
        // let executor_outcome = ExecutionOutcome::new(
        //     executor_output.state,
        //     vec![executor_output.receipts],
        //     current_block.header.number,
        //     vec![executor_output.requests.into()],
        // );

        // let state_requests = rpc_db.get_state_requests();

        // For every account we touched, fetch the storage proofs for all the slots we touched.
        // tracing::info!("fetching storage proofs");
        // let mut before_storage_proofs = Vec::new();
        // let mut after_storage_proofs = Vec::new();

        // for (address, used_keys) in state_requests.iter() {
        //     // Get modified storage keys from the executor outcome
        //     let modified_keys: Vec<B256> = executor_outcome
        //         .state()
        //         .state
        //         .get(address)
        //         .map(|account| {
        //             account.storage.keys().map(|key| B256::from(*key)).collect::<BTreeSet<_>>()
        //         })
        //         .unwrap_or_default()
        //         .into_iter()
        //         .collect();

        //     // Combine used and modified keys, removing duplicates
        //     let keys: Vec<B256> = used_keys
        //         .iter()
        //         .map(|key| B256::from(*key))
        //         .chain(modified_keys.iter().cloned())
        //         .collect::<BTreeSet<_>>()
        //         .into_iter()
        //         .collect();

        //     // Get storage proofs for previous block
        //     let before_proof = self
        //         .provider
        //         .get_proof(*address, keys.clone())
        //         .block_id((block_number - 1).into())
        //         .await?;
        //     before_storage_proofs.push(eip1186_proof_to_account_proof(before_proof));

        //     // Get storage proofs for current block
        //     let after_proof = self
        //         .provider
        //         .get_proof(*address, modified_keys)
        //         .block_id(block_number.into())
        //         .await?;
        //     after_storage_proofs.push(eip1186_proof_to_account_proof(after_proof));
        // }

        // let state = EthereumState::from_transition_proofs(
        //     previous_block.state_root,
        //     &before_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        //     &after_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        // )?;

        // // Verify the state root.
        // tracing::info!("verifying the state root");
        // let state_root = {
        //     let mut mutated_state = state.clone();
        //     let post_state = HashedPostState::from_bundle_state(&executor_outcome.bundle.state);
        //     // executor_outcome.hash_state_slow());
        //     mutated_state.update(&post_state);
        //     mutated_state.state_root()
        // };
        // if state_root != current_block.state_root {
        //     eyre::bail!("mismatched state root");
        // }

        // // Derive the block header.
        // //
        // // Note: the receipts root and gas used are verified by `validate_block_post_execution`.
        // let mut header = current_block.header.clone();
        // header.parent_hash = previous_block.hash_slow();
        // header.ommers_hash = proofs::calculate_ommers_root(&current_block.ommers);
        // header.state_root = current_block.state_root;
        // header.transactions_root = proofs::calculate_transaction_root(&current_block.body);
        // header.receipts_root = current_block.header.receipts_root;
        // header.withdrawals_root = current_block
        //     .withdrawals
        //     .clone()
        //     .map(|w| proofs::calculate_withdrawals_root(w.into_inner().as_slice()));
        // header.logs_bloom = logs_bloom;
        // header.requests_root =
        //     current_block.requests.as_ref().map(|r| proofs::calculate_requests_root(&r.0));

        // // Assert the derived header is correct.
        // assert_eq!(header.hash_slow(), current_block.header.hash_slow(), "header mismatch");

        // // Log the result.
        // tracing::info!(
        //     "successfully executed block: block_number={}, block_hash={}, state_root={}",
        //     current_block.header.number,
        //     header.hash_slow(),
        //     state_root
        // );

        // // Fetch the parent headers needed to constrain the BLOCKHASH opcode.
        // let oldest_ancestor = *rpc_db.oldest_ancestor.borrow();
        // let mut ancestor_headers = vec![];
        // tracing::info!("fetching {} ancestor headers", block_number - oldest_ancestor);
        // for height in (oldest_ancestor..=(block_number - 1)).rev() {
        //     let block = self.provider.get_block_by_number(height.into()).await?.unwrap();
        //     ancestor_headers.push(block.inner.header.try_into()?);
        // }

        let mut witness

        // Create the client input.
        let client_input = ClientExecutorInput {
            current_block: V::pre_process_block(&current_block),
            witness: todo!(),
        };
        tracing::info!("successfully generated client input");

        Ok(client_input)
    }
}

fn recover_rpc_block(
    block: AnyRpcBlock,
) -> RecoveredBlock<Block<EthereumTxEnvelope<alloy_consensus::TxEip4844>>> {
    let block = block.clone();
    let current_header =
        block.inner.header.inner.clone().try_into_header().expect("failed to convert header");

    let current_transactions: Vec<EthereumTxEnvelope<alloy_consensus::TxEip4844>> = block
        .inner
        .transactions
        .clone()
        .map(|t| {
            let envelope = t.as_envelope().expect("only Ethereum transactions are supported");
            let pooled = envelope.clone().try_into_pooled().expect("failed to convert to pooled");
            pooled.into()
        })
        .into_transactions_vec();

    let current_body = BlockBody {
        transactions: current_transactions,
        // TODO: can be restored from current_block.uncles but it's not needed?
        ommers: vec![],
        withdrawals: block.withdrawals.clone(),
    };

    let current_sealed_block: alloy_consensus::Block<
        EthereumTxEnvelope<alloy_consensus::TxEip4844>,
    > = Block::new(current_header, current_body);
    let recovered_block =
        RecoveredBlock::try_recover(current_sealed_block).expect("failed to recover block");

    recovered_block
}
