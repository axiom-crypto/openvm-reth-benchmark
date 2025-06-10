use std::collections::BTreeSet;

use alloy_consensus::TxEnvelope;
use alloy_primitives::Bytes;
use alloy_provider::{network::Ethereum, Provider};
use alloy_rlp::Encodable;
use alloy_rpc_types_debug::ExecutionWitness;
use eyre::{eyre, Ok, OptionExt};
use openvm_mpt::EthereumState;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::validate_block_post_execution;
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::Block;
use reth_primitives::RecoveredBlock;
use reth_primitives_traits::block::Block as _;
use reth_revm::db::BundleState;
use reth_stateless::StatelessInput;
use revm::database::CacheDB;
use revm_primitives::Address;
use revm_primitives::HashMap;
use revm_primitives::B256;
use revm_primitives::U256;

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

        let executor_output = block_executor.execute(&block)?;

        // Validate the block post execution.
        tracing::info!("validating the block post execution");
        validate_block_post_execution(
            &block,
            &spec,
            &executor_output.receipts,
            &executor_output.requests,
        )?;

        let (state, keys) = self
            .build_mpt(
                rpc_db.get_state_requests(),
                executor_output.state,
                previous_block.state_root,
                block_number,
            )
            .await?;

        let lowest_block_number = *rpc_db.oldest_ancestor.borrow();

        let execution_witness = ExecutionWitness {
            state: state.all_rlp_nodes().into_iter().collect(),
            codes: rpc_db.get_bytecodes().iter().map(|b| b.original_bytes()).collect(),
            keys,
            headers: self
                .build_headers_for_witness_range(lowest_block_number, block_number)
                .await?,
        };

        tracing::info!("successfully generated client input");

        let stateless_input = StatelessInput { block: current_block, witness: execution_witness };

        Ok(stateless_input)
    }

    /// Build headers for witness verification based on lowest_block_number (like reth)
    async fn build_headers_for_witness_range(
        &self,
        from: u64,
        to: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        let mut headers = Vec::new();
        for block_num in from..to {
            let block = self
                .provider
                .get_block_by_number(block_num.into())
                .await?
                .ok_or_eyre("couldn't fetch block in header range")?;

            let header = block.header;

            let mut header_bytes = Vec::new();
            header.encode(&mut header_bytes);
            headers.push(header_bytes.into());
        }

        tracing::info!("added {} RLP-encoded headers for witness verification", headers.len());
        Ok(headers)
    }

    async fn build_mpt(
        &self,
        state_requests: HashMap<Address, Vec<U256>>,
        state: BundleState,
        previous_block_state_root: B256,
        block_number: u64,
    ) -> eyre::Result<(EthereumState, Vec<Bytes>)> {
        // For every account we touched, fetch the storage proofs for all the slots we touched.
        tracing::info!("fetching storage proofs");
        let mut before_storage_proofs = Vec::new();
        let mut after_storage_proofs = Vec::new();

        let mut keys_bytes: Vec<Bytes> = vec![];

        for (address, used_keys) in state_requests.iter() {
            let modified_keys = state
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

            keys_bytes.extend(keys.into_iter().map(|key| key.into()));
        }

        let state = EthereumState::from_transition_proofs(
            previous_block_state_root,
            &before_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
            &after_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        )?;

        Ok((state, keys_bytes))
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
