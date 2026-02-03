use reth_evm::block::BlockExecutionError;
use reth_revm::primitives::alloy_primitives::BlockNumber;
use reth_storage_errors::provider::ProviderError;

#[derive(thiserror::Error, Debug)]
pub enum WitnessError {
    #[error("parent block not found for block number {0}")]
    ParentBlockNotFound(BlockNumber),

    #[error("provider error: {0}")]
    Provider(#[from] ProviderError),

    #[error("block execution error: {0}")]
    BlockExecution(#[from] BlockExecutionError),

    #[error("RLP decoding error: {0}")]
    Rlp(#[from] alloy_rlp::Error),

    #[error("MPT error: {0}")]
    Mpt(#[from] openvm_mpt::Error),
}

pub type WitnessResult<T> = Result<T, WitnessError>;
