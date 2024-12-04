use bincode::serde::Compat;
use eyre::Result;
use reth_trie::{AccountProof, HashedPostState, TrieAccount};
use revm::primitives::{Address, HashMap, B256};

/// Module containing MPT code adapted from `zeth`.
pub mod mpt;
use mpt::{proofs_to_tries, transition_proofs_to_tries, MptNode};

/// Ethereum state trie and account storage tries.
#[derive(Debug, Clone, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct EthereumState {
    pub state_trie: MptNode,
    pub storage_tries: StorageTries,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StorageTries(pub HashMap<B256, MptNode>);

impl bincode::Encode for StorageTries {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&(self.0.len() as u64), encoder)?;
        dbg!(self.0.len() as u64);
        for (k, v) in self.0.iter() {
            bincode::Encode::encode(&Compat(*k), encoder)?;
            bincode::Encode::encode(v, encoder)?;
        }
        Ok(())
    }
}

impl bincode::Decode for StorageTries {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let len_u64: u64 = bincode::Decode::decode(decoder)?;
        println!("{}", len_u64);
        let len: usize = len_u64
            .try_into()
            .map_err(|_| bincode::error::DecodeError::OutsideUsizeRange(len_u64))?;
        decoder.claim_container_read::<(Compat<B256>, MptNode)>(len)?;

        let hash_builder: _ = Default::default();
        let mut map = HashMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(Compat<B256>, MptNode)>());

            let k = Compat::<B256>::decode(decoder)?;
            let v = bincode::Decode::decode(decoder)?;
            map.insert(k.0, v);
        }
        Ok(Self(map))
    }
}
impl<'de> bincode::BorrowDecode<'de> for StorageTries {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let len_u64: u64 = bincode::Decode::decode(decoder)?;
        dbg!(len_u64);
        let len: usize = len_u64
            .try_into()
            .map_err(|_| bincode::error::DecodeError::OutsideUsizeRange(len_u64))?;
        decoder.claim_container_read::<(Compat<B256>, MptNode)>(len)?;

        let hash_builder: _ = Default::default();
        let mut map = HashMap::with_capacity_and_hasher(len, hash_builder);
        for _ in 0..len {
            // See the documentation on `unclaim_bytes_read` as to why we're doing this here
            decoder.unclaim_bytes_read(core::mem::size_of::<(Compat<B256>, MptNode)>());

            let k = Compat::<B256>::borrow_decode(decoder)?;
            let v = bincode::BorrowDecode::borrow_decode(decoder)?;
            map.insert(k.0, v);
        }
        Ok(Self(map))
    }
}

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
                None => {
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
