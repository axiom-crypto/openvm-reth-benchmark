use alloy_primitives::map::HashMap;
use alloy_primitives::{map::B256Map, Bytes};
use eyre::Result;
use mpt::{proofs_to_tries, transition_proofs_to_tries, MptNode, MptNodeReference};
use reth_errors::ProviderError;
use reth_stateless::validation::StatelessValidationError;
use reth_stateless::{ExecutionWitness, StatelessTrie};
use reth_trie::{AccountProof, TrieAccount};
use reth_trie_common::HashedPostState as RethHashedPostState;
use revm::primitives::{keccak256, Address, B256, U256};
use revm_bytecode::Bytecode;
use serde::{Deserialize, Serialize};
use state::HashedPostState;
use std::cell::RefCell;

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;
pub mod state;

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
    #[serde(skip)]
    pub rlp_by_digest: RefCell<Option<B256Map<Bytes>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StorageTries(pub RefCell<HashMap<B256, MptNode>>);

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

    /// Mutates state based on diffs provided in [`HashedPostState`].
    pub fn update(&mut self, post_state: &HashedPostState) {
        for (hashed_address, account) in post_state.accounts.iter() {
            let hashed_address_bytes = hashed_address.as_slice();

            match account {
                Some(account) => {
                    // Ensure a storage trie exists for this account
                    let storage_root = {
                        let mut storage_tries = self.storage_tries.0.borrow_mut();
                        let storage_trie =
                            storage_tries.entry(*hashed_address).or_insert_with(MptNode::default);

                        if let Some(state_storage) = post_state.storages.get(hashed_address) {
                            if state_storage.wiped {
                                storage_trie.clear();
                            }

                            // Apply storage modifications
                            for (key, value) in state_storage.storage.iter() {
                                let key = key.as_slice();
                                if value.is_zero() {
                                    storage_trie.delete(key).unwrap();
                                } else {
                                    storage_trie.insert_rlp(key, *value).unwrap();
                                }
                            }
                        }

                        storage_trie.hash()
                    };

                    // Update/insert the account after storage updates
                    let state_account = TrieAccount {
                        nonce: account.nonce,
                        balance: account.balance,
                        storage_root,
                        code_hash: account.get_bytecode_hash(),
                    };
                    self.state_trie.insert_rlp(hashed_address_bytes, state_account).unwrap();
                }
                None => {
                    // Account destroyed: remove from state trie and drop cached storage trie
                    self.state_trie.delete(hashed_address_bytes).unwrap();
                    self.storage_tries.0.borrow_mut().remove(hashed_address);
                }
            }
        }
    }

    /// Computes the state root.
    pub fn state_root(&self) -> B256 {
        self.state_trie.hash()
    }
}

/// Implementation of StatelessTrie for EthereumState
///
/// This implementation adapts the existing EthereumState MPT structure to work with
/// reth's StatelessTrie interface. Key features:
///
/// - Converts ExecutionWitness RLP data into MPT structures
/// - Provides account and storage lookups compatible with reth's expectations
/// - Handles state root calculation with proper type conversion
/// - Supports lazy loading of storage tries (TODO: full implementation)
///
/// Note: This is a bridge implementation that may need hasher unification
/// to work seamlessly with reth's type system.
impl StatelessTrie for EthereumState {
    /// Initialize the stateless trie using the `ExecutionWitness`.
    fn new(
        witness: &ExecutionWitness,
        pre_state_root: B256,
    ) -> Result<(Self, B256Map<Bytecode>), StatelessValidationError> {
        // Hash all the RLP nodes once
        let rlp_by_digest: B256Map<_> =
            witness.state.iter().map(|rlp| (keccak256(rlp), rlp.clone())).collect();

        // Create state trie from the witness RLP data
        let state_trie = if rlp_by_digest.is_empty() {
            mpt::node_from_digest(pre_state_root)
        } else {
            // Parse the RLP nodes and build a complete trie
            let mut node_store = HashMap::default();
            for (digest, rlp_data) in &rlp_by_digest {
                match MptNode::decode(rlp_data) {
                    Ok(node) => {
                        node_store.insert(MptNodeReference::Digest(*digest), node);
                    }
                    Err(_) => continue, // Skip invalid RLP data
                }
            }

            // Start with root digest and resolve all nodes
            let root_node = mpt::node_from_digest(pre_state_root);
            mpt::resolve_nodes(&root_node, &node_store)
        };

        // Initialize empty storage tries - will be populated lazily
        let storage_tries = StorageTries::default();

        // Hash all the supplied bytecode
        let bytecode = witness
            .codes
            .iter()
            .map(|code| (keccak256(code), Bytecode::new_raw(code.clone())))
            .collect();

        let state = EthereumState {
            state_trie,
            storage_tries,
            rlp_by_digest: RefCell::new(Some(rlp_by_digest)),
        };

        Ok((state, bytecode))
    }

    /// Returns the `TrieAccount` that corresponds to the `Address`.
    fn account(&self, address: Address) -> Result<Option<TrieAccount>, ProviderError> {
        let hashed_address = keccak256(address);

        // Get account from state trie using the hashed address
        match self.state_trie.get_rlp::<TrieAccount>(&hashed_address.as_slice()) {
            Ok(account_opt) => {
                if let Some(account) = &account_opt {
                    // Lazily create storage trie for this account if needed
                    // This mimics the zeth implementation's lazy loading behavior
                    if let Some(ref rlp_by_digest) = *self.rlp_by_digest.borrow() {
                        let mut storage_tries = self.storage_tries.0.borrow_mut();

                        if !storage_tries.contains_key(&hashed_address) {
                            // Create storage trie from witness data using account's storage_root
                            let storage_root = account.storage_root;

                            // Parse RLP nodes to build storage node store
                            let mut node_store = HashMap::default();
                            for (digest, rlp_data) in rlp_by_digest {
                                if let Ok(node) = MptNode::decode(rlp_data) {
                                    node_store.insert(MptNodeReference::Digest(*digest), node);
                                }
                            }

                            // Build storage trie from the storage root
                            let storage_trie =
                                if storage_root == mpt::EMPTY_ROOT || storage_root == B256::ZERO {
                                    MptNode::default()
                                } else {
                                    let root_node = mpt::node_from_digest(storage_root);
                                    mpt::resolve_nodes(&root_node, &node_store)
                                };

                            storage_tries.insert(hashed_address, storage_trie);
                        }
                    }
                }
                Ok(account_opt)
            }
            Err(e) => Err(ProviderError::other(e)),
        }
    }

    /// Returns the storage slot value that corresponds to the given (address, slot) tuple.
    ///
    /// This method assumes that account() has been called first to lazily load the storage trie.
    /// If no storage trie exists for the account, returns U256::ZERO.
    fn storage(&self, address: Address, slot: U256) -> Result<U256, ProviderError> {
        let hashed_address = keccak256(address);
        let hashed_slot = keccak256(B256::from(slot));

        // Get storage trie for this account
        // Note: In the zeth implementation, this assumes account() was called first
        // to ensure the storage trie exists
        let storage_tries = self.storage_tries.0.borrow();
        if let Some(storage_trie) = storage_tries.get(&hashed_address) {
            match storage_trie.get_rlp::<U256>(&hashed_slot.as_slice()) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => Ok(U256::ZERO),
                Err(e) => Err(ProviderError::other(e)),
            }
        } else {
            // Storage trie not found - return zero (empty storage)
            Ok(U256::ZERO)
        }
    }

    /// Computes the new state root from the HashedPostState.
    fn calculate_state_root(
        &mut self,
        state: RethHashedPostState,
    ) -> Result<B256, StatelessValidationError> {
        // Convert reth's HashedPostState to our local type
        // Note: This will need hasher unification to work properly
        let local_state = HashedPostState {
            accounts: state.accounts.into_iter().collect(),
            storages: state
                .storages
                .into_iter()
                .map(|(k, v)| {
                    // Convert reth's HashedStorage to our local HashedStorage
                    let local_storage = state::HashedStorage {
                        wiped: v.wiped,
                        storage: v.storage.into_iter().collect(),
                    };
                    (k, local_storage)
                })
                .collect(),
        };

        // Use existing update method which already handles HashedPostState
        self.update(&local_state);
        Ok(self.state_root())
    }
}
