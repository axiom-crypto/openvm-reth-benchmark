use std::sync::Arc;

use alloy_consensus::TxEnvelope;
use alloy_provider::{network::Ethereum, Provider};
use alloy_rpc_types::BlockNumberOrTag;
use eyre::{eyre, Ok, OptionExt};
use openvm_rpc_proxy::{execution_witness, PreimageLookup};
use openvm_stateless_executor::io::StatelessExecutorInput;
use openvm_stateless_witness::{generate_block_input_from_witness, BlockExecutionWitness};
use reth_chainspec::MAINNET;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::Block;

/// An executor that fetches data from a [Provider] to execute blocks in the [StatelessExecutor].
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    provider: P,
    evm_config: Arc<EthEvmConfig>,
    lookup: Arc<PreimageLookup>,
}

impl<P> HostExecutor<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P, preimage_cache_nibbles: u8) -> Self {
        let evm_config = Arc::new(EthEvmConfig::ethereum(MAINNET.clone()));
        // Initialize the preimage lookup table
        let lookup = Arc::new(PreimageLookup::new(preimage_cache_nibbles));
        Self { provider, evm_config, lookup }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<StatelessExecutorInput> {
        let block_id = BlockNumberOrTag::Number(block_number);
        let execution_witness =
            execution_witness(self.evm_config.clone(), &self.provider, block_id, &self.lookup)
                .await?;
        // TODO(refactor): have execution_witness return BlockExecutionWitness
        let parent_block_number = block_number - 1;
        let parent_block = self
            .provider
            .get_block_by_number(parent_block_number.into())
            .await?
            .ok_or_eyre("parent block not found")?;
        let current_block = self
            .provider
            .get_block_by_number(block_id)
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        let input = generate_block_input_from_witness(BlockExecutionWitness {
            execution_witness,
            parent_state_root: parent_block.header.state_root,
            current_block,
        })?;
        tracing::info!("successfully generated client input");

        Ok(input)
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}
