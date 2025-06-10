use eyre::Result;
use mpt::MptNode;
use reth_trie::AccountProof;
use revm::primitives::{Address, Bytes, HashMap, B256};
use serde::{Deserialize, Serialize};

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;
pub mod mpt2;
pub mod utils;

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub HashMap<B256, MptNode>);

impl EthereumState {
    /// Builds Ethereum state tries from relevant proofs before and after a state transition.
    pub fn from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        mpt::transition_proofs_to_tries(state_root, parent_proofs, proofs)
            .map_err(|err| eyre::eyre!("{}", err))
    }

    /// Builds Ethereum state tries from relevant proofs before and after a state transition.
    pub fn from_transition_proofs_2(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        mpt2::transition_proofs_to_tries(state_root, parent_proofs, proofs)
            .map_err(|err| eyre::eyre!("{}", err))
    }

    /// Extracts all RLP-encoded trie nodes from the state and storage tries.
    ///
    /// This collects all nodes from both the state trie and all storage tries,
    /// avoiding duplicates by using a HashMap keyed by node hash.
    pub fn all_rlp_nodes(&self) -> Vec<Bytes> {
        let mut nodes = HashMap::default();

        // Collect nodes from the state trie
        self.state_trie.rlp_nodes(&mut nodes);

        // Collect nodes from all storage tries
        for storage_trie in self.storage_tries.0.values() {
            storage_trie.rlp_nodes(&mut nodes);
        }

        // Convert to Vec<Bytes>
        nodes.into_values().map(Bytes::from).collect()
    }
}
