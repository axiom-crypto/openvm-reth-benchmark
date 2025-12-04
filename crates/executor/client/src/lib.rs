/// Chain-specific execution configuration.
pub mod chain;
pub mod error;
/// Client program input data types.
pub mod io;

use std::{marker::PhantomData, sync::Arc};

use alloy_consensus::TxReceipt;
use alloy_primitives::Bloom;
use openvm_primitives::chain_spec::{dev, mainnet};
use reth_chainspec::ChainSpec;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_execution_types::ExecutionOutcome;
use reth_primitives::Header;
use reth_revm::db::CacheDB;

use crate::{
    error::ClientExecutionError,
    io::{ClientExecutorInput, ClientExecutorInputWithState},
};

pub use chain::{ChainExecutorConfig, EthereumConfig, LighterConfig};

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor<C: ChainExecutorConfig = EthereumConfig> {
    _phantom: PhantomData<C>,
}

/// EVM chain variants that implement different execution/validation rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainVariant {
    Mainnet,
    Dev,
}

impl ClientExecutor<EthereumConfig> {
    /// Execute an Ethereum block with the specified chain variant.
    ///
    /// This is a convenience method for Ethereum execution.
    pub fn execute_ethereum(
        &self,
        chain_variant: ChainVariant,
        pre_input: ClientExecutorInput,
    ) -> Result<Header, ClientExecutionError> {
        let spec = Arc::new(match chain_variant {
            ChainVariant::Mainnet => mainnet(),
            ChainVariant::Dev => dev(),
        });
        self.execute_with_spec(spec, pre_input)
    }
}

impl<C: ChainExecutorConfig> ClientExecutor<C> {
    /// Creates a new client executor.
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }

    /// Execute a block with the given chain specification.
    pub fn execute_with_spec(
        &self,
        spec: Arc<ChainSpec>,
        pre_input: ClientExecutorInput,
    ) -> Result<Header, ClientExecutionError> {
        let mut input = ClientExecutorInputWithState::build(pre_input)?;

        // Run pre-execution hook (e.g., install OpenVM crypto)
        C::pre_execution_hook()?;

        // Initialize the witnessed database with verified storage proofs.
        let witness_db = input.witness_db()?;
        let cache_db = CacheDB::new(&witness_db);

        // Recover senders from signatures
        let current_block = C::recover_block(input.input.current_block.clone())?;

        // Validate the block pre-execution (includes header validation)
        C::validate_block_pre_execution(&current_block, &spec)?;

        // Execute the block
        let evm_config = C::evm_config(spec.clone());
        let block_executor = BasicBlockExecutor::new(evm_config, cache_db);
        let executor_output = block_executor.execute(&current_block)?;

        // Validate the block post-execution
        C::validate_block_post_execution(&current_block, &spec, &executor_output.result)?;

        // Accumulate the logs bloom.
        let mut logs_bloom = Bloom::default();
        executor_output.receipts.iter().for_each(|r| {
            logs_bloom.accrue_bloom(&r.bloom());
        });

        // Convert the output to an execution outcome.
        let executor_outcome = ExecutionOutcome::new(
            executor_output.state,
            vec![executor_output.result.receipts],
            input.input.current_block.header.number,
            vec![executor_output.result.requests],
        );

        drop(witness_db);

        // Verify the state root.
        let state_root = {
            input.state.update_from_bundle_state(&executor_outcome.bundle)?;
            input.state.state_trie.hash()
        };

        let expected_state_root = C::expected_state_root(&input.input.current_block);
        if state_root != expected_state_root {
            return Err(ClientExecutionError::StateRootMismatch {
                actual: state_root,
                expected: expected_state_root,
            });
        }

        // Derive the block header.
        //
        // Note: the receipts root and gas used are verified by `validate_block_post_execution`.
        let mut header = input.input.current_block.header.clone();
        header.parent_hash = input.parent_header().hash_slow();
        header.ommers_hash = input.input.current_block.body.calculate_ommers_root();
        header.state_root = expected_state_root;
        header.transactions_root = input.input.current_block.transactions_root;
        header.receipts_root = input.input.current_block.header.receipts_root;
        header.withdrawals_root = input.input.current_block.body.calculate_withdrawals_root();
        header.logs_bloom = logs_bloom;
        header.requests_hash = input.input.current_block.requests_hash;

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_executor_default() {
        let executor: ClientExecutor = ClientExecutor::new();
        // Just verify it compiles and can be created
        let _ = executor;
    }

    #[test]
    fn test_client_executor_ethereum() {
        let executor: ClientExecutor<EthereumConfig> = ClientExecutor::new();
        let _ = executor;
    }
}
