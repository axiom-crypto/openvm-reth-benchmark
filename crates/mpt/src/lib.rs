use eyre::Result;
use mpt::{proofs_to_tries, transition_proofs_to_tries, MptNode};
use reth_trie::HashedPostState;
use reth_trie::{AccountProof, TrieAccount};
use revm::primitives::{Address, HashMap, B256};
use serde::{Deserialize, Serialize};

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;

/// Module for a zero-copy, flat MPT representation.
pub mod flat;

use flat::FlatTrieOwned;

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
}

/// Flat, zero-copy version of EthereumState for fast deserialization.
/// This is what gets sent to the guest to avoid the expensive pointer-heavy deserialization.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FlatEthereumState {
    pub state_trie: FlatTrieOwned,
    pub storage_tries: FlatStorageTries,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub HashMap<B256, MptNode>);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlatStorageTries(pub HashMap<B256, FlatTrieOwned>);

impl EthereumState {
    /// Builds Ethereum state tries from relevant proofs before and after a state transition.
    pub fn from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        transition_proofs_to_tries(state_root, parent_proofs, proofs)
            .map_err(|err| eyre::eyre!("{}", err))
    }

    /// Builds Ethereum state tries from relevant proofs from a given state.
    pub fn from_proofs(state_root: B256, proofs: &HashMap<Address, AccountProof>) -> Result<Self> {
        proofs_to_tries(state_root, proofs).map_err(|err| eyre::eyre!("{}", err))
    }

    /// Converts this EthereumState into a flat representation for fast serialization.
    /// This is intended to be run on the host.
    pub fn to_flat(&self) -> FlatEthereumState {
        let state_trie = self.state_trie.to_flat_owned();

        let storage_tries = FlatStorageTries(
            self.storage_tries.0.iter().map(|(k, v)| (*k, v.to_flat_owned())).collect(),
        );

        FlatEthereumState { state_trie, storage_tries }
    }

    /// Mutates state based on diffs provided in [`HashedPostState`].
    pub fn update(&mut self, post_state: &HashedPostState) {
        for (hashed_address, account) in post_state.accounts.iter() {
            match account {
                Some(account) => {
                    let storage_root = if let Some(state_storage) =
                        post_state.storages.get(hashed_address)
                    {
                        // Account has storage updates
                        let storage_trie = self.storage_tries.0.get_mut(hashed_address).unwrap();

                        if state_storage.wiped {
                            storage_trie.clear();
                        }

                        for (key, value) in state_storage.storage.iter() {
                            let key = key.as_slice();
                            if value.is_zero() {
                                storage_trie.delete(key).unwrap();
                            } else {
                                storage_trie.insert_rlp(key, *value).unwrap();
                            }
                        }

                        storage_trie.hash()
                    } else {
                        // Account has no storage updates, use existing storage root
                        self.storage_tries
                            .0
                            .get(hashed_address)
                            .map(|trie| trie.hash())
                            .unwrap_or(reth_trie::EMPTY_ROOT_HASH)
                    };

                    let state_account = TrieAccount {
                        nonce: account.nonce,
                        balance: account.balance,
                        storage_root,
                        code_hash: account.get_bytecode_hash(),
                    };
                    self.state_trie.insert_rlp(hashed_address.as_slice(), state_account).unwrap();
                }
                _ => {
                    self.state_trie.delete(hashed_address.as_slice()).unwrap();
                }
            }
        }
    }

    /// Computes the state root.
    pub fn state_root(&self) -> B256 {
        self.state_trie.hash()
    }
}

impl FlatEthereumState {
    /// Converts this flat state back into the mutable EthereumState.
    /// This is the "inflation" step that we delay until mutation is needed.
    pub fn to_mpt_state(&self) -> EthereumState {
        // Rebuild the state trie from the flat representation
        let state_trie = self.rebuild_mpt_from_flat(&self.state_trie);

        // Rebuild all storage tries
        let storage_tries = StorageTries(
            self.storage_tries
                .0
                .iter()
                .map(|(k, flat_trie)| (*k, self.rebuild_mpt_from_flat(flat_trie)))
                .collect(),
        );

        EthereumState { state_trie, storage_tries }
    }

    /// Applies post-state changes and computes the new state root without mutation.
    /// This is the zero-copy alternative to inflate -> update -> hash.
    pub fn apply_post_state(&self, post_state: &HashedPostState) -> eyre::Result<B256> {
        use crate::flat::update::apply_post_state_to_flat;
        apply_post_state_to_flat(self, post_state)
    }

    /// Rebuilds an MptNode tree from a flat representation.
    /// This is the expensive operation we delay until mutation is actually needed.
    fn rebuild_mpt_from_flat(&self, flat_trie: &FlatTrieOwned) -> MptNode {
        if flat_trie.index.is_empty() {
            return MptNode::default();
        }

        // The root is the last node in our post-order traversal
        let root_idx = flat_trie.index.len() - 1;
        self.rebuild_node_from_flat(flat_trie, root_idx)
    }

    /// Recursively rebuilds a single node from the flat representation.
    fn rebuild_node_from_flat(&self, flat_trie: &FlatTrieOwned, node_idx: usize) -> MptNode {
        let node = &flat_trie.index[node_idx];
        let ref_slice = &flat_trie.blob
            [node.ref_offset as usize..(node.ref_offset + node.ref_len as u32) as usize];

        match node.kind {
            0 => MptNode::default(), // Null
            1 => {
                // Branch - reconstruct from reference
                let mut children: [Option<Box<MptNode>>; 16] = Default::default();
                let mask = node.data;
                let mut child_list_idx = 0;

                for i in 0..16 {
                    if (mask & (1u16 << i)) != 0 {
                        let child_flat_idx =
                            flat_trie.branch_children[node.child_idx as usize + child_list_idx];
                        children[i] = Some(Box::new(
                            self.rebuild_node_from_flat(flat_trie, child_flat_idx as usize),
                        ));
                        child_list_idx += 1;
                    }
                }

                mpt::MptNodeData::Branch(children).into()
            }
            2 => {
                // Leaf - handle both inline RLP and digest references
                let prefix = if ref_slice.len() == 32 {
                    // Digest reference - get prefix from prefixes array
                    let prefix_idx = node.data as usize;
                    flat_trie.prefixes[prefix_idx].clone()
                } else {
                    // Inline RLP - parse to get prefix
                    let rlp = rlp::Rlp::new(ref_slice);
                    rlp.val_at(0).unwrap()
                };
                let value = flat_trie.leaf_values[node.child_idx as usize].clone();
                mpt::MptNodeData::Leaf(prefix, value).into()
            }
            3 => {
                // Extension - handle both inline RLP and digest references
                let prefix = if ref_slice.len() == 32 {
                    // Digest reference - get prefix from prefixes array
                    let prefix_idx = node.data as usize;
                    flat_trie.prefixes[prefix_idx].clone()
                } else {
                    // Inline RLP - parse to get prefix
                    let rlp = rlp::Rlp::new(ref_slice);
                    rlp.val_at(0).unwrap()
                };
                let child_node = self.rebuild_node_from_flat(flat_trie, node.child_idx as usize);
                mpt::MptNodeData::Extension(prefix, Box::new(child_node)).into()
            }
            4 => {
                // Digest - the reference contains the digest directly
                if ref_slice.len() == 32 {
                    mpt::MptNodeData::Digest(B256::from_slice(ref_slice)).into()
                } else {
                    // This shouldn't happen for digest nodes
                    mpt::MptNode::decode(ref_slice).unwrap()
                }
            }
            _ => unreachable!(),
        }
    }

    /// Computes the state root using the flat representation.
    pub fn state_root(&self) -> B256 {
        self.state_trie.view().hash()
    }
}
