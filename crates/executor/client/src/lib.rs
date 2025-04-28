/// Client program input data types.
pub mod io;
#[macro_use]
mod utils;

pub mod custom;

use std::{borrow::BorrowMut, fmt::Display};

use custom::CustomEvmConfig;
use eyre::eyre;
use io::ClientExecutorInput;
#[allow(unused_imports)]
pub use openvm_mpt;
use openvm_mpt::state::HashedPostState;
use openvm_primitives::chain_spec;
use reth_chainspec::ChainSpec;
use reth_errors::ProviderError;
use reth_ethereum_consensus::validate_block_post_execution as validate_block_post_execution_ethereum;
use reth_evm::execute::{BlockExecutionOutput, BlockExecutorProvider, Executor};
use reth_evm_ethereum::execute::EthExecutorProvider;
use reth_evm_optimism::OpExecutorProvider;
use reth_execution_types::ExecutionOutcome;
use reth_optimism_consensus::validate_block_post_execution as validate_block_post_execution_optimism;
#[allow(unused_imports)]
pub use reth_primitives;
use reth_primitives::{proofs, Block, BlockWithSenders, Bloom, Header, Receipt, Receipts, Request};
use reth_stateless::validation::stateless_validation;
use revm::{db::CacheDB, Database};
use revm_primitives::{address, B256, U256};

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor;

/// Implementation for Ethereum-specific execution/validation logic.
#[derive(Debug)]
pub struct EthereumVariant;

/// EVM chain variants that implement different execution/validation rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChainVariant {
    /// Ethereum networks.
    Ethereum,
}

impl ChainVariant {
    /// Returns the chain ID for the given variant.
    pub fn chain_id(&self) -> u64 {
        match self {
            ChainVariant::Ethereum => CHAIN_ID_ETH_MAINNET,
        }
    }

    pub fn chain_spec(&self) -> ChainSpec {
        match self {
            ChainVariant::Ethereum => chain_spec::mainnet(),
        }
    }
}

impl ClientExecutor {
    pub fn execute(&self, mut input: ClientExecutorInput) -> eyre::Result<B256>
    where
        V: Variant,
    {
        let chain_variant = ChainVariant::Ethereum;
        let chain_spec = Arc::new(chain_variant.chain_spec());

        let ClientExecutorInput { current_block, witness } = input;
        let block_hash = stateless_validation(current_block, witness, chain_spec)?;
        Ok(block_hash)
    }
}

impl Variant for EthereumVariant {
    fn spec() -> ChainSpec {
        openvm_primitives::chain_spec::mainnet()
    }

    fn execute<DB>(
        executor_block_input: &BlockWithSenders,
        executor_difficulty: U256,
        cache_db: DB,
    ) -> eyre::Result<BlockExecutionOutput<Receipt>>
    where
        DB: Database<Error: Into<ProviderError> + Display>,
    {
        Ok(EthExecutorProvider::new(
            Self::spec().into(),
            CustomEvmConfig::from_variant(ChainVariant::Ethereum),
        )
        .executor(cache_db)
        .execute((executor_block_input, executor_difficulty).into())?)
    }

    fn validate_block_post_execution(
        block: &BlockWithSenders,
        chain_spec: &ChainSpec,
        receipts: &[Receipt],
        requests: &[Request],
    ) -> eyre::Result<()> {
        Ok(validate_block_post_execution_ethereum(block, chain_spec, receipts, requests)?)
    }
}
