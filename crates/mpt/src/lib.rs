use eyre::Result;
use mpt::{proofs_to_tries, MptNode};
use mpt2::ArenaBasedMptNode;
use reth_trie::{AccountProof, TrieAccount};
use revm::primitives::{Address, Bytes, HashMap, B256};
use serde::{Deserialize, Serialize};
use state::HashedPostState;

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;
pub mod mpt2;
pub mod state;
pub mod utils;

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
}

/// Ethereum state trie and account storage tries using arena-based MPT nodes for better
/// performance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthereumState2 {
    pub state_trie: ArenaBasedMptNode,
    pub storage_tries: StorageTries2,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub HashMap<B256, MptNode>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StorageTries2(pub HashMap<B256, ArenaBasedMptNode>);

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

    /// Builds Ethereum state tries from relevant proofs from a given state.
    pub fn from_proofs(state_root: B256, proofs: &HashMap<Address, AccountProof>) -> Result<Self> {
        proofs_to_tries(state_root, proofs).map_err(|err| eyre::eyre!("{}", err))
    }

    /// Mutates state based on diffs provided in [`HashedPostState`].
    pub fn update(&mut self, post_state: &HashedPostState) {
        for (hashed_address, account) in post_state.accounts.iter() {
            let hashed_address = hashed_address.as_slice();

            match account {
                Some(account) => {
                    let state_storage = &post_state.storages.get(hashed_address).unwrap();
                    let storage_root = {
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
                    };

                    let state_account = TrieAccount {
                        nonce: account.nonce,
                        balance: account.balance,
                        storage_root,
                        code_hash: account.get_bytecode_hash(),
                    };
                    self.state_trie.insert_rlp(hashed_address, state_account).unwrap();
                }
                _ => {
                    self.state_trie.delete(hashed_address).unwrap();
                }
            }
        }
    }

    /// Computes the state root.
    pub fn state_root(&self) -> B256 {
        self.state_trie.hash()
    }
}

impl EthereumState2 {
    /// Builds Ethereum state tries from relevant proofs before and after a state transition
    /// using the arena-based MPT implementation for better performance.
    pub fn from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        mpt2::transition_proofs_to_tries_arena(state_root, parent_proofs, proofs)
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

    /// Mutates state based on diffs provided in [`HashedPostState`].
    pub fn update(&mut self, post_state: &HashedPostState) {
        for (hashed_address, account) in post_state.accounts.iter() {
            let hashed_address = hashed_address.as_slice();

            match account {
                Some(account) => {
                    let state_storage = &post_state.storages.get(hashed_address).unwrap();
                    let storage_root = {
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
                    };

                    let state_account = TrieAccount {
                        nonce: account.nonce,
                        balance: account.balance,
                        storage_root,
                        code_hash: account.get_bytecode_hash(),
                    };
                    self.state_trie.insert_rlp(hashed_address, state_account).unwrap();
                }
                _ => {
                    self.state_trie.delete(hashed_address).unwrap();
                }
            }
        }
    }

    /// Computes the state root.
    pub fn state_root(&self) -> B256 {
        self.state_trie.hash()
    }
}

// Custom serde implementations for compact RLP-based serialization
impl Serialize for StorageTries2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as Vec<(B256, Vec<u8>)> where Vec<u8> is the RLP blob
        let storage_blobs: Vec<(B256, Vec<u8>)> =
            self.0.iter().map(|(addr, trie)| (*addr, trie.to_full_rlp())).collect();
        storage_blobs.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StorageTries2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let storage_blobs: Vec<(B256, Vec<u8>)> = Vec::deserialize(deserializer)?;
        let mut storage_tries = HashMap::default();

        for (addr, rlp_blob) in storage_blobs {
            let trie =
                ArenaBasedMptNode::decode_from_rlp(&rlp_blob).map_err(serde::de::Error::custom)?;
            storage_tries.insert(addr, trie);
        }

        Ok(StorageTries2(storage_tries))
    }
}

impl Serialize for EthereumState2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as (state_trie_blob, storage_tries)
        let state_blob = self.state_trie.to_full_rlp();
        (serde_bytes::Bytes::new(&state_blob), &self.storage_tries).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EthereumState2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (state_blob, storage_tries): (Vec<u8>, StorageTries2) =
            Deserialize::deserialize(deserializer)?;
        let state_trie =
            ArenaBasedMptNode::decode_from_rlp(&state_blob).map_err(serde::de::Error::custom)?;

        Ok(EthereumState2 { state_trie, storage_tries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_transition_proofs_2_empty() {
        // Test that from_transition_proofs works with empty proofs for EthereumState2
        let state_root = B256::ZERO;
        let parent_proofs = HashMap::default();
        let proofs = HashMap::default();

        let result = EthereumState2::from_transition_proofs(state_root, &parent_proofs, &proofs);
        assert!(result.is_ok());

        let ethereum_state = result.unwrap();
        assert!(ethereum_state.storage_tries.0.is_empty());
    }
}
