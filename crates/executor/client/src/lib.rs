use alloy_primitives::B256;
use reth_chainspec::MAINNET;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::TransactionSigned;
use reth_stateless::{validation::stateless_validation, ExecutionWitness};
use serde::{Deserialize, Serialize};

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// The input for the client to execute a block and fully verify the STF (state transition
/// function).
///
/// Instead of passing in the entire state, we only pass in the state roots along with merkle proofs
/// for the storage slots that were modified and accessed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientExecutorInput {
    /// The current block (which will be executed inside the client).
    pub current_block: alloy_consensus::Block<TransactionSigned>,
    pub witness: ExecutionWitness,
}

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor;

impl ClientExecutor {
    pub fn execute(&self, input: ClientExecutorInput) -> eyre::Result<B256> {
        let chain_spec = MAINNET.clone();
        let ClientExecutorInput { current_block, witness } = input;
        let config = EthEvmConfig::new(chain_spec.clone());
        let block_hash = stateless_validation(current_block, witness, chain_spec, config)?;
        Ok(block_hash)
    }
}
