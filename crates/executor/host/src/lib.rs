use std::collections::BTreeSet;

use alloy_consensus::{TxEnvelope, TxReceipt};
use alloy_primitives::Bloom;
use alloy_provider::{network::Ethereum, Provider};
use eyre::{eyre, Ok};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::{state::HashedPostState, EthereumState};
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
use revm::database::CacheDB;
use revm_primitives::{HashMap as RevmHashMap, B256};

/// Execute a block and return the execution outcome and state requests
pub fn execute_block_with_rpc_db<P: Provider<Ethereum> + Clone>(
    current_block: &Block,
    rpc_db: &RpcDb<P>,
) -> eyre::Result<(
    ExecutionOutcome,
    RevmHashMap<revm_primitives::Address, Vec<revm_primitives::U256>>,
)> {
    // Setup the spec for the block executor.
    tracing::info!("setting up the spec for the block executor");
    let spec = MAINNET.clone();

    // Setup the database for the block executor.
    tracing::info!("setting up the database for the block executor");
    let cache_db = CacheDB::new(rpc_db);

    // Execute the block and fetch all the necessary data along the way.
    tracing::info!(
        "executing the block: block_number={}, transaction_count={}",
        current_block.header.number,
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

    // Convert the output to an execution outcome.
    let executor_outcome = ExecutionOutcome::new(
        executor_output.state,
        vec![executor_output.result.receipts],
        current_block.header.number,
        vec![executor_output.result.requests],
    );

    // Get state requests and convert to revm HashMap
    let std_state_requests = rpc_db.get_state_requests();
    let mut revm_state_requests = RevmHashMap::default();
    for (k, v) in std_state_requests {
        revm_state_requests.insert(k, v);
    }

    Ok((executor_outcome, revm_state_requests))
}

/// Fetch storage proofs for the given addresses and keys
pub async fn fetch_storage_proofs<P: Provider<Ethereum> + Clone>(
    provider: &P,
    state_requests: &RevmHashMap<revm_primitives::Address, Vec<revm_primitives::U256>>,
    executor_outcome: &ExecutionOutcome,
    block_number: u64,
) -> eyre::Result<(Vec<reth_trie::AccountProof>, Vec<reth_trie::AccountProof>)> {
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

        let before_proof =
            provider.get_proof(*address, keys.clone()).block_id((block_number - 1).into()).await?;
        before_storage_proofs.push(eip1186_proof_to_account_proof(before_proof));

        let after_proof =
            provider.get_proof(*address, modified_keys).block_id(block_number.into()).await?;
        after_storage_proofs.push(eip1186_proof_to_account_proof(after_proof));
    }

    Ok((before_storage_proofs, after_storage_proofs))
}

/// Verify state root matches expected
pub fn verify_state_root(
    state: &EthereumState,
    executor_outcome: &ExecutionOutcome,
    expected_state_root: revm_primitives::B256,
) -> eyre::Result<()> {
    // Verify the state root.
    tracing::info!("verifying the state root");
    let state_root = {
        let mut mutated_state = state.clone();
        let post_state = HashedPostState::from_bundle_state(&executor_outcome.bundle.state);
        mutated_state.update(&post_state);
        mutated_state.state_root()
    };
    if state_root != expected_state_root {
        eyre::bail!("mismatched state root");
    }
    Ok(())
}

/// Fetch ancestor headers for BLOCKHASH opcode
pub async fn fetch_ancestor_headers<P: Provider<Ethereum> + Clone>(
    provider: &P,
    oldest_ancestor: u64,
    block_number: u64,
) -> eyre::Result<Vec<reth_primitives::Header>> {
    let mut ancestor_headers = vec![];
    tracing::info!("fetching {} ancestor headers", block_number - oldest_ancestor);
    for height in (oldest_ancestor..=(block_number - 1)).rev() {
        let block = provider.get_block_by_number(height.into()).await?.unwrap();
        ancestor_headers.push(block.header.into());
    }
    Ok(ancestor_headers)
}

/// Build the final ClientExecutorInput
pub fn build_client_executor_input(
    current_block: Block,
    ancestor_headers: Vec<reth_primitives::Header>,
    parent_state: EthereumState,
    state_requests: RevmHashMap<revm_primitives::Address, Vec<revm_primitives::U256>>,
    bytecodes: Vec<revm::state::Bytecode>,
) -> ClientExecutorInput {
    ClientExecutorInput { current_block, ancestor_headers, parent_state, state_requests, bytecodes }
}

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

        // Create RPC database
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);

        // Execute the block and get execution outcome + state requests
        let (executor_outcome, state_requests) =
            execute_block_with_rpc_db(&current_block, &rpc_db)?;

        // Fetch storage proofs
        let (before_storage_proofs, after_storage_proofs) =
            fetch_storage_proofs(&self.provider, &state_requests, &executor_outcome, block_number)
                .await?;

        // Build EthereumState
        let state = EthereumState::from_transition_proofs(
            previous_block.state_root,
            &before_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
            &after_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        )?;

        // Verify state root
        verify_state_root(&state, &executor_outcome, current_block.state_root)?;

        // Derive and verify the block header.
        let mut header = current_block.header.clone();
        header.parent_hash = previous_block.hash_slow();
        header.ommers_hash = current_block.body.calculate_ommers_root();
        header.state_root = current_block.state_root;
        header.transactions_root = current_block.transactions_root;
        header.receipts_root = current_block.header.receipts_root;
        header.withdrawals_root = current_block.body.calculate_withdrawals_root();

        // Accumulate the logs bloom.
        tracing::info!("accumulating the logs bloom");
        let mut logs_bloom = Bloom::default();
        executor_outcome.receipts().iter().for_each(|receipts| {
            receipts.iter().for_each(|r| {
                logs_bloom.accrue_bloom(&r.bloom());
            });
        });
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

        // Fetch ancestor headers
        let oldest_ancestor = *rpc_db.oldest_ancestor.borrow();
        let ancestor_headers =
            fetch_ancestor_headers(&self.provider, oldest_ancestor, block_number).await?;

        // Get bytecodes
        let bytecodes = rpc_db.get_bytecodes();

        // Build final client input
        let client_input = build_client_executor_input(
            current_block,
            ancestor_headers,
            state,
            state_requests,
            bytecodes,
        );

        tracing::info!("successfully generated client input");
        Ok(client_input)
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}
