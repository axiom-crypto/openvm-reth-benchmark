use bumpalo::Bump;
use reth_trie::TrieAccount;
use revm::database::BundleState;
use revm_primitives::{keccak256, map::DefaultHashBuilder, HashMap, B256};

use crate::{Error, MptTrie};

/// Serialized Ethereum state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EthereumStateBytes {
    pub state_trie: (usize, bytes::Bytes),
    pub storage_tries: Vec<(B256, usize, bytes::Bytes)>,
}

#[derive(Debug, Clone)]
pub struct EthereumState {
    pub state_trie: MptTrie<'static>,
    pub storage_tries: HashMap<B256, MptTrie<'static>>,
    pub bump: &'static Bump,
}

impl EthereumState {
    pub fn new() -> Self {
        let bump = Box::leak(Box::new(Bump::new()));
        Self {
            state_trie: MptTrie::new(bump),
            storage_tries: HashMap::with_capacity_and_hasher(1, DefaultHashBuilder::default()),
            bump,
        }
    }

    pub fn update_from_bundle_state(&mut self, bundle_state: &BundleState) -> Result<(), Error> {
        for (address, account) in &bundle_state.state {
            let hashed_address = keccak256(address);

            if let Some(info) = &account.info {
                let storage_trie =
                    self.storage_tries.entry(hashed_address).or_insert(MptTrie::new(self.bump));

                if account.status.was_destroyed() {
                    *storage_trie = MptTrie::new(self.bump);
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
}

impl Default for EthereumState {
    fn default() -> Self {
        Self::new()
    }
}
