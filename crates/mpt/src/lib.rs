use eyre::Result;
use reth_trie::{AccountProof, HashedPostState, TrieAccount};
use revm::primitives::{Address, HashMap, B256};
use serde::{Deserialize, Serialize};

use alloy_primitives::map::B256Set;
use alloy_rlp::{Decodable, Encodable};
use risc0_ethereum_trie::{orphan, CachedTrie};
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Zeth-style MPT node wrapper around risc0_ethereum_trie::CachedTrie
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MptNode<T: Decodable + Encodable> {
    inner: CachedTrie,
    phantom_data: PhantomData<T>,
}

impl<T: Decodable + Encodable> Default for MptNode<T> {
    fn default() -> Self {
        Self { inner: CachedTrie::default(), phantom_data: PhantomData }
    }
}

impl<T: Decodable + Encodable> MptNode<T> {
    pub fn get_rlp(&self, key: impl AsRef<[u8]>) -> alloy_rlp::Result<Option<T>> {
        match self.inner.get(key) {
            None => Ok(None),
            Some(mut bytes) => Ok(Some(T::decode(&mut bytes)?)),
        }
    }

    pub fn insert_rlp<K, V>(&mut self, key: K, value: V)
    where
        K: AsRef<[u8]>,
        V: Borrow<T>,
    {
        self.inner.insert(key, alloy_rlp::encode(value.borrow()));
    }

    /// Tries to resolve the potential removal orphan corresponding to `key` from the given
    /// post-removal proof. If the orphan cannot be resolved from the proof alone, the `key`
    /// corresponding to the unresolved path is added to `unresolvable`.
    pub fn resolve_orphan<K, N>(
        &mut self,
        key: K,
        post_state_proof: impl IntoIterator<Item = N>,
        unresolvable: &mut B256Set,
    ) -> anyhow::Result<()>
    where
        K: AsRef<[u8]>,
        N: AsRef<[u8]>,
    {
        match self.inner.resolve_orphan(key, post_state_proof) {
            Ok(_) => {}
            Err(orphan::Error::Unresolvable(prefix)) => {
                // convert the unresolvable prefix nibbles into a B256 key with zero padding
                let key = B256::right_padding_from(&prefix.pack());
                unresolvable.insert(key);
            }
            Err(err) => return Err(err.into()),
        };

        Ok(())
    }

    #[inline]
    pub fn from_digest(digest: B256) -> Self {
        if digest == B256::ZERO {
            Self::default()
        } else {
            Self { inner: CachedTrie::from_digest(digest), phantom_data: PhantomData }
        }
    }

    #[inline]
    pub fn from_rlp<N: AsRef<[u8]>>(nodes: impl IntoIterator<Item = N>) -> alloy_rlp::Result<Self> {
        Ok(Self { inner: CachedTrie::from_rlp(nodes)?, phantom_data: PhantomData })
    }

    /// Extract all witness nodes (all trie node preimages) for reth-stateless
    /// For now, we'll need to build this functionality by tracking nodes during construction
    pub fn extract_witness_nodes(&self) -> Vec<alloy_primitives::Bytes> {
        // TODO: The risc0_ethereum_trie::CachedTrie doesn't expose all cached nodes directly
        // We need to implement witness collection during trie construction
        // For now, return empty - this will be populated when we implement proof construction
        vec![]
    }
}

impl<T: Decodable + Encodable> Deref for MptNode<T> {
    type Target = CachedTrie;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Decodable + Encodable> DerefMut for MptNode<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Ethereum state trie and account storage tries using risc0-ethereum-trie.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode<TrieAccount>,
    pub storage_tries: StorageTries,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub HashMap<B256, MptNode<revm::primitives::U256>>);

impl EthereumState {
    /// Builds Ethereum state tries from relevant proofs before and after a state transition.
    pub fn from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        // We need to implement this using risc0-ethereum-trie methods
        Self::build_from_transition_proofs(state_root, parent_proofs, proofs)
    }

    /// Extracts all trie node data for use with reth-stateless.
    /// This provides the complete "tissue" of trie nodes needed for stateless validation.
    pub fn extract_witness_data(&self) -> Result<Vec<alloy_primitives::Bytes>> {
        let mut witness_nodes = Vec::new();

        // Extract all nodes from the state trie
        witness_nodes.extend(self.state_trie.extract_witness_nodes());

        // Extract all nodes from storage tries
        for (_, storage_trie) in &self.storage_tries.0 {
            witness_nodes.extend(storage_trie.extract_witness_nodes());
        }

        Ok(witness_nodes)
    }

    /// Mutates state based on diffs provided in [`HashedPostState`].
    pub fn update(&mut self, post_state: &HashedPostState) {
        for (hashed_address, account) in post_state.accounts.iter() {
            let hashed_address_bytes = hashed_address.as_slice();

            match account {
                Some(account) => {
                    // Get or create storage trie for this account
                    let storage_trie = self
                        .storage_tries
                        .0
                        .entry(*hashed_address)
                        .or_insert_with(|| MptNode::<revm::primitives::U256>::default());

                    // Apply storage changes if they exist
                    if let Some(state_storage) = post_state.storages.get(hashed_address) {
                        if state_storage.wiped {
                            storage_trie.clear();
                        }

                        for (key, value) in state_storage.storage.iter() {
                            let key = key.as_slice();
                            if value.is_zero() {
                                storage_trie.remove(key);
                            } else {
                                storage_trie.insert_rlp(key, *value);
                            }
                        }
                    }

                    let storage_root = storage_trie.hash();

                    let state_account = TrieAccount {
                        nonce: account.nonce,
                        balance: account.balance,
                        storage_root,
                        code_hash: account.get_bytecode_hash(),
                    };
                    self.state_trie.insert_rlp(hashed_address_bytes, state_account);
                }
                _ => {
                    self.state_trie.remove(hashed_address_bytes);
                    // Also remove the storage trie for deleted accounts
                    self.storage_tries.0.remove(hashed_address);
                }
            }
        }
    }

    /// Computes the state root.
    pub fn state_root(&self) -> B256 {
        // TODO: use hash but it becomes mut? Maybe Rc or will be slow?
        self.state_trie.hash_slow()
    }

    // Helper methods for building tries from proofs
    fn build_from_transition_proofs(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<Self> {
        // Initialize state trie from the parent state root
        let mut state_trie = MptNode::<TrieAccount>::from_digest(state_root);
        let mut storage_tries = StorageTries::default();

        // First, build the state trie from initial (parent) account proofs
        for (address, parent_proof) in parent_proofs {
            let hashed_address = revm::primitives::keccak256(*address);

            // Hydrate state trie with parent account proof
            state_trie.hydrate_from_rlp(&parent_proof.proof)?;

            // Build storage trie from parent storage proofs
            let mut storage_trie =
                MptNode::<revm::primitives::U256>::from_digest(parent_proof.storage_root);

            // Hydrate storage trie with all storage proof nodes
            for storage_proof in &parent_proof.storage_proofs {
                storage_trie.hydrate_from_rlp(&storage_proof.proof)?;
            }

            storage_tries.0.insert(hashed_address, storage_trie);
        }

        // Then, extend tries with current (post-state) proofs
        for (address, current_proof) in proofs {
            let hashed_address = revm::primitives::keccak256(*address);

            // Extend state trie with current account proof
            state_trie.hydrate_from_rlp(&current_proof.proof)?;

            // Handle storage trie for this address
            let storage_trie = storage_tries.0.entry(hashed_address).or_insert_with(|| {
                MptNode::<revm::primitives::U256>::from_digest(current_proof.storage_root)
            });

            // Extend storage trie with current storage proofs
            for storage_proof in &current_proof.storage_proofs {
                storage_trie.hydrate_from_rlp(&storage_proof.proof)?;
            }
        }

        // Ensure we have storage tries for all addresses that appear in either proof set
        let all_addresses: std::collections::HashSet<Address> =
            parent_proofs.keys().chain(proofs.keys()).cloned().collect();

        for address in all_addresses {
            let hashed_address = revm::primitives::keccak256(address);

            if !storage_tries.0.contains_key(&hashed_address) {
                // Create empty storage trie for addresses without explicit storage proofs
                let storage_root = parent_proofs
                    .get(&address)
                    .or_else(|| proofs.get(&address))
                    .map(|proof| proof.storage_root)
                    .unwrap_or(B256::ZERO);

                let storage_trie = if storage_root != B256::ZERO {
                    MptNode::<revm::primitives::U256>::from_digest(storage_root)
                } else {
                    MptNode::<revm::primitives::U256>::default()
                };

                storage_tries.0.insert(hashed_address, storage_trie);
            }
        }

        Ok(EthereumState { state_trie, storage_tries })
    }
}
