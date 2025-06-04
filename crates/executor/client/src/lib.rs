use alloy_consensus::{EthereumTxEnvelope, TxEip4844};
use alloy_primitives::B256;
use reth_chainspec::MAINNET;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::Block;
use reth_stateless::{
    validation::{stateless_validation, StatelessValidationError},
    ExecutionWitness, StatelessInput,
};
use serde::{Deserialize, Serialize};

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// Input for the client executor, containing block data and execution witness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientExecutorInput {
    /// The block to execute
    pub current_block: Block<EthereumTxEnvelope<TxEip4844>>,
    /// The execution witness containing state proofs and preimages
    pub witness: ExecutionWitness,
}

pub mod io {
    pub use super::ClientExecutorInput;
}

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor;

impl ClientExecutor {
    /// Execute a block and fully verify the STF (state transition function).
    /// Instead of passing in the entire state, we only pass in the state roots along with merkle proofs
    /// for the storage slots that were modified and accessed.
    pub fn execute(&self, input: ClientExecutorInput) -> eyre::Result<B256> {
        let stateless_input = StatelessInput { block: input.current_block, witness: input.witness };

        let chain_spec = MAINNET.clone();
        let config = EthEvmConfig::new(chain_spec.clone());
        let block_hash = stateless_validation(
            stateless_input.block,
            stateless_input.witness,
            chain_spec,
            config,
        );
        match block_hash {
            Ok(block_hash) => Ok(block_hash),
            Err(StatelessValidationError::StatelessExecutionFailed(inner_msg)) => {
                Err(eyre::eyre!("Stateless execution failed with details: {}", inner_msg))
            }
            Err(e) => Err(eyre::eyre!("Stateless validation error: {:?}", e)),
        }
    }
}
