use std::iter::once;

use eyre::Result;
use itertools::Itertools;
use openvm_mpt::EthereumState;
use openvm_witness_db::WitnessDb;
use reth_primitives::{Block, Header, RecoveredBlock, TransactionSigned, B256, U256};
use reth_stateless::ExecutionWitness;
use reth_trie::TrieAccount;
use revm_primitives::{keccak256, AccountInfo, Address, Bytecode, HashMap};
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};

/// The input for the client to execute a block and fully verify the STF (state transition
/// function).
///
/// Instead of passing in the entire state, we only pass in the state roots along with merkle proofs
/// for the storage slots that were modified and accessed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientExecutorInput {
    /// The current block (which will be executed inside the client).
    pub current_block: RecoveredBlock<Block<TransactionSigned>>,
    pub witness: ExecutionWitness,
}
