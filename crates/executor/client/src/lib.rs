#[macro_use]
mod utils;

use std::fmt::Debug;

#[allow(unused_imports)]
pub use openvm_mpt;
use openvm_mpt::state::HashedPostState;
use openvm_primitives::chain_spec::mainnet;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
#[allow(unused_imports)]
pub use reth_primitives;
use reth_primitives::Header;
use reth_primitives_traits::block::Block as _;
use reth_revm::db::CacheDB;

/// Chain ID for Ethereum Mainnet.
pub const CHAIN_ID_ETH_MAINNET: u64 = 0x1;

/// An executor that executes a block inside a zkVM.
#[derive(Debug, Clone, Default)]
pub struct ClientExecutor;

impl ClientExecutor {
    pub fn execute(&self, input: StatelessInput) -> eyre::Result<B256> {
        let chain_spec = Arc::new(mainnet());
        let config = EthEvmConfig::new(chain_spec.clone());
        let block_hash = profile!("stateless validation", {
            stateless_validation(input.block, input.witness, chain_spec, config)
        })?;

        Ok(block_hash)
    }
}
