#[macro_use]
mod utils;

// #[cfg(target_os = "zkvm")]
// mod crypto;

use std::{fmt::Debug, sync::Arc};

use openvm_mpt::EthereumState;
use openvm_primitives::chain_spec::mainnet;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{stateless_validation_with_trie, StatelessInput};
use revm_primitives::B256;

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
            stateless_validation_with_trie::<EthereumState, _, _>(
                input.block,
                input.witness,
                chain_spec,
                config,
            )
        })?;

        Ok(block_hash)
    }
}
