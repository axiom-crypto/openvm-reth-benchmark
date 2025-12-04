//! Chain-specific execution configuration.
//!
//! This module provides the [`ChainExecutorConfig`] trait that allows the executor
//! to support multiple chains (Ethereum mainnet, LighterEVM, etc.) with different
//! execution rules, validation logic, and witness requirements.

use std::{fmt::Debug, sync::Arc};

use alloy_consensus::Header;
use alloy_primitives::B256;
use reth_chainspec::ChainSpec;
use reth_consensus::{Consensus, HeaderValidator};
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_evm::ConfigureEvm;
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::BlockExecutionResult;
use reth_primitives::{Block, Receipt};
use reth_primitives_traits::{block::Block as BlockTrait, NodePrimitives, RecoveredBlock};

use crate::error::ClientExecutionError;

/// Configuration trait for chain-specific execution behavior.
pub trait ChainExecutorConfig: Debug + Clone + Default + Send + Sync + 'static {
    /// Additional witness data needed beyond standard EVM state.
    type ExtraWitness: Debug + Clone + Default + Send + Sync;

    /// The EVM configuration type.
    type EvmConfig: ConfigureEvm<
            Primitives: NodePrimitives<Block = Block, BlockHeader = Header, Receipt = Receipt>,
        > + Clone;

    /// Creates the EVM configuration for this chain.
    fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig;

    /// Returns the chain ID.
    fn chain_id(chain_spec: &ChainSpec) -> u64 {
        chain_spec.chain.id()
    }

    /// Validates and recovers the block (recovers senders from signatures).
    fn recover_block(block: Block) -> Result<RecoveredBlock<Block>, ClientExecutionError> {
        block
            .try_into_recovered()
            .map_err(|e| ClientExecutionError::BlockSenderRecoveryError(e.into()))
    }

    /// Validates the block before execution (including header validation).
    /// Default: no validation (for ZK-proved chains).
    fn validate_block_pre_execution(
        _block: &RecoveredBlock<Block>,
        _chain_spec: &ChainSpec,
    ) -> Result<(), ClientExecutionError> {
        Ok(())
    }

    /// Validates the block after execution. Default: no validation.
    fn validate_block_post_execution(
        _block: &RecoveredBlock<Block>,
        _chain_spec: &ChainSpec,
        _result: &BlockExecutionResult<Receipt>,
    ) -> Result<(), ClientExecutionError> {
        Ok(())
    }

    /// Returns the expected state root from the block.
    fn expected_state_root(block: &Block) -> B256 {
        block.state_root
    }

    /// Hook called before execution starts.
    fn pre_execution_hook() -> Result<(), ClientExecutionError> {
        Ok(())
    }
}

/// Ethereum mainnet configuration.
#[derive(Debug, Clone, Default)]
pub struct EthereumConfig;

impl ChainExecutorConfig for EthereumConfig {
    type ExtraWitness = ();
    type EvmConfig = EthEvmConfig;

    fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig {
        EthEvmConfig::new(chain_spec)
    }

    fn validate_block_pre_execution(
        block: &RecoveredBlock<Block>,
        chain_spec: &ChainSpec,
    ) -> Result<(), ClientExecutionError> {
        let consensus = EthBeaconConsensus::new(Arc::new(chain_spec.clone()));
        // Validate header
        consensus
            .validate_header(block.sealed_header())
            .map_err(ClientExecutionError::InvalidHeader)?;
        // Validate block pre-execution
        consensus
            .validate_block_pre_execution(block)
            .map_err(ClientExecutionError::InvalidBlockPreExecution)
    }

    fn validate_block_post_execution(
        block: &RecoveredBlock<Block>,
        chain_spec: &ChainSpec,
        result: &BlockExecutionResult<Receipt>,
    ) -> Result<(), ClientExecutionError> {
        reth_ethereum_consensus::validate_block_post_execution(
            block,
            chain_spec,
            &result.receipts,
            &result.requests,
        )
        .map_err(ClientExecutionError::InvalidBlockPostExecution)
    }

    fn pre_execution_hook() -> Result<(), ClientExecutionError> {
        #[cfg(feature = "openvm")]
        {
            openvm_revm_crypto::install_openvm_crypto()
                .map_err(|e| ClientExecutionError::Other(e.to_string()))?;
        }
        Ok(())
    }
}

/// Placeholder configuration for LighterEVM chains.
///
/// This struct demonstrates how to implement `ChainExecutorConfig` for custom chains.
/// LighterEVM has specific requirements:
///
/// - **Extra Witness**: Needs `CoreStateWitness` for READ_CORE precompile access
/// - **Custom Validation**: ZK-proved, so no consensus validation needed
/// - **State Root**: May be stored in header extra data field
/// - **Custom Transactions**: Supports Priority TX (0x7E) with native token minting
///
/// # Example Implementation (to be completed in lighter-reth)
///
/// ```ignore
/// use lighter_reth::core::CoreStateWitness;
/// use lighter_reth::evm::LighterEvmConfig;
///
/// impl ChainExecutorConfig for LighterConfig {
///     type ExtraWitness = CoreStateWitness;
///     type EvmConfig = LighterEvmConfig;
///
///     fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig {
///         LighterEvmConfig::new(chain_spec)
///     }
///
///     // No validation needed - ZK proves correctness
///     fn validate_block_pre_execution(_block: &RecoveredBlock<Block>, _chain_spec: &ChainSpec)
///         -> Result<(), ClientExecutionError> { Ok(()) }
///
///     fn validate_block_post_execution(_block: &RecoveredBlock<Block>, _chain_spec: &ChainSpec,
///         _result: &BlockExecutionResult<Receipt>) -> Result<(), ClientExecutionError> { Ok(()) }
///
///     // State root stored in header extra data
///     fn expected_state_root(block: &Block) -> B256 {
///         B256::from_slice(&block.header.extra_data[..32])
///     }
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct LighterConfig;

// NOTE: LighterConfig implementation will be completed in lighter-reth crate
// once LighterEvmConfig and CoreStateWitness are available.
// For now, this serves as documentation and a marker type.

#[cfg(test)]
mod tests {
    use super::*;
    use openvm_primitives::chain_spec::mainnet;

    #[test]
    fn test_ethereum_config_creation() {
        let chain_spec = Arc::new(mainnet());
        let _evm_config = EthereumConfig::evm_config(chain_spec.clone());
        assert_eq!(EthereumConfig::chain_id(&chain_spec), 1);
    }

    #[test]
    fn test_extra_witness_is_unit_for_ethereum() {
        let witness: <EthereumConfig as ChainExecutorConfig>::ExtraWitness = Default::default();
        assert_eq!(std::mem::size_of_val(&witness), 0);
    }

    #[test]
    fn test_pre_execution_hook_succeeds() {
        assert!(EthereumConfig::pre_execution_hook().is_ok());
    }

    #[test]
    fn test_default_validation_passes() {
        #[derive(Debug, Clone, Default)]
        struct NoValidationConfig;

        impl ChainExecutorConfig for NoValidationConfig {
            type ExtraWitness = ();
            type EvmConfig = EthEvmConfig;
            fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig {
                EthEvmConfig::new(chain_spec)
            }
        }

        // Default validation should pass (does nothing for ZK chains)
        let block = Block::default();
        let recovered = NoValidationConfig::recover_block(block).unwrap();
        let chain_spec = mainnet();
        assert!(NoValidationConfig::validate_block_pre_execution(&recovered, &chain_spec).is_ok());
    }

    #[test]
    fn test_recover_block_empty() {
        let block = Block::default();
        assert!(EthereumConfig::recover_block(block).is_ok());
    }

    #[test]
    fn test_expected_state_root_from_block() {
        // EthereumConfig reads state root directly from block.state_root
        let block: Block = Block::default();
        assert_eq!(EthereumConfig::expected_state_root(&block), block.state_root);
    }

    #[test]
    fn test_custom_config_with_extra_witness() {
        // Test that a config with non-unit ExtraWitness can be defined
        #[derive(Debug, Clone, Default)]
        struct CustomWitness {
            data: Vec<u8>,
        }

        #[derive(Debug, Clone, Default)]
        struct CustomConfig;

        impl ChainExecutorConfig for CustomConfig {
            type ExtraWitness = CustomWitness;
            type EvmConfig = EthEvmConfig;
            fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig {
                EthEvmConfig::new(chain_spec)
            }
        }

        let witness: <CustomConfig as ChainExecutorConfig>::ExtraWitness = CustomWitness {
            data: vec![1, 2, 3],
        };
        assert_eq!(witness.data.len(), 3);
    }

    #[test]
    fn test_custom_state_root_override() {
        // Test a config that always returns a fixed state root
        #[derive(Debug, Clone, Default)]
        struct FixedStateRootConfig;

        impl ChainExecutorConfig for FixedStateRootConfig {
            type ExtraWitness = ();
            type EvmConfig = EthEvmConfig;

            fn evm_config(chain_spec: Arc<ChainSpec>) -> Self::EvmConfig {
                EthEvmConfig::new(chain_spec)
            }

            fn expected_state_root(_block: &Block) -> B256 {
                // Custom chains might derive state root differently
                B256::repeat_byte(0xFF)
            }
        }

        let block: Block = Block::default();
        assert_eq!(
            FixedStateRootConfig::expected_state_root(&block),
            B256::repeat_byte(0xFF)
        );
    }
}
