use crate::word_bytes::OptimizedBytes;
use eyre::Result;
use mpt::{proofs_to_tries, MptNode};
use mpt2::ArenaBasedMptNode;
use reth_revm::db::BundleState;
use reth_trie::{AccountProof, TrieAccount};
use revm::primitives::{Address, HashMap, B256};
use revm_primitives::keccak256;
use serde::{Deserialize, Serialize};
use state::HashedPostState;

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;
pub mod mpt2;
pub mod state;
pub mod utils;
pub mod word_bytes;

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
}

/// Ethereum state trie and account storage tries using arena-based MPT nodes for better
/// performance.
#[derive(Debug, Clone)]
pub struct EthereumState2 {
    pub state_trie: ArenaBasedMptNode<'static>,
    pub storage_tries: StorageTries2,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub HashMap<B256, MptNode>);

#[derive(Debug, Clone, Default)]
pub struct StorageTries2(pub HashMap<B256, ArenaBasedMptNode<'static>>);

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
    #[cfg(feature = "build_mpt")]
    pub fn from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        use crate::mpt2::build_mpt::transition_proofs_to_tries_arena;
        transition_proofs_to_tries_arena(state_root, parent_proofs, proofs)
            .map_err(|err| eyre::eyre!("{}", err))
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

    pub fn update_from_bundle_state(&mut self, bundle_state: &BundleState) {
        // A single insertion can split a leaf into an extension and a branch with two leaves,
        // adding up to 3 new nodes. A deletion can also cause node modifications.
        // We use a pessimistic multiplier of 4 to be safe.
        const MPT_NODE_MULTIPLIER: usize = 4;

        // 1. Reserve capacity for the state trie.
        let num_changed_accounts = bundle_state.state.len();
        self.state_trie.reserve(num_changed_accounts * MPT_NODE_MULTIPLIER);

        // Create a reusable buffer for RLP encoding to reduce allocations.
        let mut rlp_buf = Vec::with_capacity(128);

        // 2. Perform the updates, reserving for storage tries just-in-time.
        for (address, account) in &bundle_state.state {
            let hashed_address = keccak256(address);

            if let Some(info) = &account.info {
                // 1. Update storage trie and get the new storage root
                let storage_trie = self.storage_tries.0.entry(hashed_address).or_default();
                storage_trie.reserve(account.storage.len() * MPT_NODE_MULTIPLIER);

                if account.status.was_destroyed() {
                    storage_trie.clear();
                }

                for (slot, value) in &account.storage {
                    let hashed_slot = keccak256(B256::from(*slot));
                    if value.present_value.is_zero() {
                        storage_trie.delete(hashed_slot.as_slice()).unwrap();
                    } else {
                        storage_trie
                            .insert_rlp_with_buf(
                                hashed_slot.as_slice(),
                                &value.present_value,
                                &mut rlp_buf,
                            )
                            .unwrap();
                    }
                }
                let storage_root = storage_trie.hash();

                // 2. Create TrieAccount and insert into state trie
                let state_account = TrieAccount {
                    nonce: info.nonce,
                    balance: info.balance,
                    storage_root,
                    code_hash: info.code_hash,
                };
                self.state_trie
                    .insert_rlp_with_buf(hashed_address.as_slice(), state_account, &mut rlp_buf)
                    .unwrap();
            } else {
                // account.info is None, which means it was destroyed.
                self.state_trie.delete(hashed_address.as_slice()).unwrap();
                self.storage_tries.0.remove(&hashed_address);
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
            // We need to leak the memory to get a 'static lifetime for serde compatibility
            let leaked_bytes: &'static [u8] = Box::leak(rlp_blob.into_boxed_slice());
            let trie = ArenaBasedMptNode::decode_from_rlp(leaked_bytes)
                .map_err(serde::de::Error::custom)?;
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
        let state_blob = OptimizedBytes(self.state_trie.to_full_rlp());
        (state_blob, &self.storage_tries).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EthereumState2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (state_blob, storage_tries): (OptimizedBytes, StorageTries2) =
            Deserialize::deserialize(deserializer)?;
        // We need to leak the memory to get a 'static lifetime for serde compatibility
        let leaked_bytes: &'static [u8] = Box::leak(state_blob.0.into_boxed_slice());
        let state_trie =
            ArenaBasedMptNode::decode_from_rlp(leaked_bytes).map_err(serde::de::Error::custom)?;

        Ok(EthereumState2 { state_trie, storage_tries })
    }
}
