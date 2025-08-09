use alloy_consensus::TxEnvelope;
use alloy_provider::{ext::DebugApi, network::Ethereum, Provider};
use alloy_rpc_types_debug::ExecutionWitness as RpcExecutionWitness;
use eyre::{eyre, Ok};
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_primitives::{Block, RecoveredBlock};
use reth_stateless::StatelessInput;

/// An executor that fetches data from a [Provider] to execute blocks in the [ClientExecutor].
#[derive(Debug, Clone)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
}

impl<P: Provider<Ethereum> + DebugApi + Clone> HostExecutor<P> {
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<StatelessInput> {
        // Fetch the current block.
        tracing::info!("fetching current block {}", block_number);
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        // Fetch execution witness via debug API.
        tracing::info!("fetching execution witness via debug API for block {}", block_number);
        let RpcExecutionWitness { state, codes, headers, .. } =
            self.provider.debug_execution_witness(block_number.into()).await?;

        let execution_witness = reth_stateless::ExecutionWitness {
            state,
            codes,
            keys: vec![], // not needed
            headers,
        };

        tracing::info!("successfully generated stateless input via debug witness");

        Ok(StatelessInput { block: current_block, witness: execution_witness })
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
