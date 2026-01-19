use bumpalo::Bump;
use reth_trie::TrieAccount;
use revm::database::BundleState;
use revm_primitives::{keccak256, map::DefaultHashBuilder, HashMap, B256};

use crate::{Error, Mpt};

/// Serialized Ethereum state.
///
/// Each trie is serialized with:
/// - `num_nodes`: Number of nodes in the pre-state trie (for validation)
/// - `final_num_nodes`: Number of nodes after block execution (for capacity pre-allocation)
/// - `bytes`: The serialized trie data
///
/// Using `final_num_nodes` for capacity allocation eliminates the need for growth factors
/// and prevents reallocation during state updates.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EthereumStateBytes {
    /// (num_nodes, final_num_nodes, bytes) for the state trie
    pub state_trie: (usize, usize, bytes::Bytes),
    /// Vec of (address_hash, num_nodes, final_num_nodes, bytes) for storage tries
    pub storage_tries: Vec<(B256, usize, usize, bytes::Bytes)>,
}

#[derive(Debug, Clone)]
pub struct EthereumState {
    pub state_trie: Mpt<'static>,
    pub storage_tries: HashMap<B256, Mpt<'static>>,
    pub bump: &'static Bump,
}

impl EthereumState {
    pub fn new() -> Self {
        let bump = Box::leak(Box::new(Bump::new()));
        Self {
            state_trie: Mpt::new(bump),
            storage_tries: HashMap::with_capacity_and_hasher(1, DefaultHashBuilder::default()),
            bump,
        }
    }

    pub fn from_tries(
        state_trie: Mpt<'static>,
        storage_tries: impl IntoIterator<Item = (B256, Mpt<'static>)>,
    ) -> Self {
        Self {
            state_trie,
            storage_tries: storage_tries.into_iter().collect(),
            bump: Box::leak(Box::new(Bump::new())),
        }
    }

    pub fn update_from_bundle_state(&mut self, bundle_state: &BundleState) -> Result<(), Error> {
        for (address, account) in &bundle_state.state {
            let hashed_address = keccak256(address);

            if let Some(info) = &account.info {
                let storage_trie =
                    self.storage_tries.entry(hashed_address).or_insert(Mpt::new(self.bump));

                if account.status.was_destroyed() {
                    *storage_trie = Mpt::new(self.bump);
                }

                for (slot, value) in &account.storage {
                    let hashed_slot = keccak256(slot.to_be_bytes::<32>());
                    if value.present_value.is_zero() {
                        storage_trie.delete(hashed_slot.as_slice())?;
                    } else {
                        storage_trie.insert_rlp(hashed_slot.as_slice(), value.present_value)?;
                    }
                }
                let storage_root = storage_trie.hash();
                let state_account = TrieAccount {
                    nonce: info.nonce,
                    balance: info.balance,
                    storage_root,
                    code_hash: info.code_hash,
                };
                self.state_trie.insert_rlp(hashed_address.as_slice(), state_account)?;
            } else {
                self.state_trie.delete(hashed_address.as_slice()).unwrap();
                self.storage_tries.remove(&hashed_address);
            }
        }

        Ok(())
    }

    /// Encodes the state to bytes, using the provided post-execution state to determine
    /// final node counts for optimal capacity pre-allocation.
    ///
    /// The `final_state` should be a clone of this state after applying `update_from_bundle_state`.
    /// This allows the guest to pre-allocate exact capacity, avoiding the 1.5x growth factor.
    #[cfg(feature = "host")]
    pub fn encode_to_state_bytes_with_final(
        &self,
        final_state: &EthereumState,
    ) -> EthereumStateBytes {
        let state_num_nodes = self.state_trie.num_nodes();
        let final_state_num_nodes = final_state.state_trie.num_nodes();
        let state_bytes = bytes::Bytes::from(self.state_trie.encode_trie());

        let mut storage_bytes: Vec<_> = self
            .storage_tries
            .iter()
            .map(|(addr, trie)| {
                // Use 0 for destroyed tries - triggers fallback 1.5x growth factor
                let final_num_nodes =
                    final_state.storage_tries.get(addr).map(|t| t.num_nodes()).unwrap_or(0);
                (*addr, trie.num_nodes(), final_num_nodes, bytes::Bytes::from(trie.encode_trie()))
            })
            .collect();
        storage_bytes.sort_by_key(|(addr, _, _, _)| *addr);

        EthereumStateBytes {
            state_trie: (state_num_nodes, final_state_num_nodes, state_bytes),
            storage_tries: storage_bytes,
        }
    }

    /// Encodes the state to bytes without final node counts (uses pre-state counts as fallback).
    /// Prefer `encode_to_state_bytes_with_final` when post-execution state is available.
    #[cfg(feature = "host")]
    pub fn encode_to_state_bytes(&self) -> EthereumStateBytes {
        self.encode_to_state_bytes_with_final(self)
    }
}

impl Default for EthereumState {
    fn default() -> Self {
        Self::new()
    }
}
