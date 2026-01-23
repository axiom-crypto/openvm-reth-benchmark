pub mod error;
/// Client program input data types.
pub mod io;

use std::{fmt::Debug, sync::Arc};

use alloy_consensus::TxReceipt;
use alloy_primitives::Bloom;
use openvm_primitives::chain_spec::{dev, mainnet};
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::Header;
use reth_primitives_traits::block::Block as _;
use reth_revm::db::CacheDB;

use crate::{
    error::ClientExecutionError,
    io::{ClientExecutorInput, ClientExecutorInputWithState},
};

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor;

/// EVM chain variants that implement different execution/validation rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainVariant {
    Mainnet,
    Dev,
}

impl ClientExecutor {
    pub fn execute(
        &self,
        chain_variant: ChainVariant,
        pre_input: ClientExecutorInput,
    ) -> Result<Header, ClientExecutionError> {
        let mut input = ClientExecutorInputWithState::build(pre_input)?;

        // Install OpenVM crypto optimizations
        #[cfg(feature = "openvm")]
        {
            println!("Installing OpenVM crypto optimizations");
            openvm_revm_crypto::install_openvm_crypto()
                .expect("failed to install OpenVM crypto provider");
        }

        // Initialize the witnessed database with verified storage proofs.
        let witness_db = input.witness_db()?;
        let cache_db = CacheDB::new(&witness_db);

        // Execute the block.
        let spec = Arc::new(match chain_variant {
            ChainVariant::Mainnet => mainnet(),
            ChainVariant::Dev => dev(),
        });
        // Recover senders
        let current_block = input
            .input
            .current_block
            .clone()
            .try_into_recovered()
            .map_err(|err| ClientExecutionError::BlockSenderRecoveryError(err.into()))?;

        // validate the block pre-execution
        {
            let consensus = EthBeaconConsensus::new(spec.clone());

            consensus
                .validate_header(current_block.sealed_header())
                .map_err(ClientExecutionError::InvalidHeader)?;

            consensus
                .validate_block_pre_execution(&current_block)
                .map_err(ClientExecutionError::InvalidBlockPreExecution)?;
        };

        let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec.clone()), cache_db);
        let executor_output = block_executor.execute(&current_block)?;

        // Validate the block post execution.
        validate_block_post_execution(
            &current_block,
            &spec,
            &executor_output.receipts,
            &executor_output.requests,
            None,
        )
        .map_err(ClientExecutionError::InvalidBlockPostExecution)?;

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

        if state_root != input.input.current_block.state_root {
            return Err(ClientExecutionError::StateRootMismatch {
                actual: state_root,
                expected: input.input.current_block.state_root,
            });
        }

        // Derive the block header.
        //
        // Note: the receipts root and gas used are verified by `validate_block_post_execution`.
        let mut header = input.input.current_block.header.clone();
        header.parent_hash = input.parent_header().hash_slow();
        header.ommers_hash = input.input.current_block.body.calculate_ommers_root();
        header.state_root = input.input.current_block.state_root;
        header.transactions_root = input.input.current_block.transactions_root;
        header.receipts_root = input.input.current_block.header.receipts_root;
        header.withdrawals_root = input.input.current_block.body.calculate_withdrawals_root();
        header.logs_bloom = logs_bloom;
        header.requests_hash = input.input.current_block.requests_hash;

        Ok(header)
    }
}
