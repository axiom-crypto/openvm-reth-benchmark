use std::iter::once;

use alloy_rlp::Decodable as AlloyDecodable;
use eyre::Result;
use itertools::Itertools;
use openvm_mpt::FlatEthereumState;
use openvm_witness_db::WitnessDb;
use reth_primitives::{Block, Header, TransactionSigned};
use reth_trie::TrieAccount;
use revm::state::{AccountInfo, Bytecode};
use revm_primitives::{keccak256, Address, HashMap, B256, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// The input for the client to execute a block and fully verify the STF (state transition
/// function).
///
/// Instead of passing in the entire state, we only pass in the state roots along with merkle proofs
/// for the storage slots that were modified and accessed.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientExecutorInput {
    /// The current block (which will be executed inside the client).
    #[serde_as(
        as = "reth_primitives_traits::serde_bincode_compat::Block<'_, TransactionSigned, Header>"
    )]
    pub current_block: Block<TransactionSigned, Header>,
    /// The previous block headers starting from the most recent. There must be at least one header
    /// to provide the parent state root.
    #[serde_as(as = "Vec<alloy_consensus::serde_bincode_compat::Header>")]
    pub ancestor_headers: Vec<Header>,
    /// Network state as of the parent block, serialized with rkyv for zero-copy deserialization.
    /// This contains FlatEthereumState data in rkyv format for maximum performance.
    pub rkyv_parent_state_bytes: Vec<u8>,
    /// Requests to account state and storage slots.
    pub state_requests: HashMap<Address, Vec<U256>>,
    /// Account bytecodes.
    pub bytecodes: Vec<Bytecode>,
}

impl ClientExecutorInput {
    /// Gets the immediate parent block's header.
    #[inline(always)]
    pub fn parent_header(&self) -> &Header {
        &self.ancestor_headers[0]
    }

    /// Access the parent state with zero-copy rkyv deserialization.
    /// This is the preferred method for performance.
    pub fn access_parent_state(&self) -> Result<&rkyv::Archived<FlatEthereumState>> {
        FlatEthereumState::access_rkyv_bytes(&self.rkyv_parent_state_bytes)
            .map_err(|e| eyre::eyre!("Failed to access parent state: {}", e))
    }

    /// Get parent state as owned FlatEthereumState (defeats zero-copy purpose).
    /// Only use this for compatibility when you need an owned state.
    pub fn get_parent_state_owned(&self) -> Result<FlatEthereumState> {
        FlatEthereumState::from_rkyv_bytes_owned(&self.rkyv_parent_state_bytes)
            .map_err(|e| eyre::eyre!("Failed to deserialize parent state: {}", e))
    }

    /// Creates a [`WitnessDb`] from the rkyv serialized flat state representation.
    pub fn witness_db(&self) -> Result<WitnessDb> {
        // Use zero-copy access to the archived state
        let _archived_state = self.access_parent_state()?;

        // For now, we need to work with the archived data directly
        // In a full implementation, you'd implement operations directly on archived data
        // but for compatibility, let's deserialize for now
        let flat_state = self.get_parent_state_owned()?;
        let flat_state_trie = flat_state.state_trie.view();

        if self.parent_header().state_root != flat_state_trie.hash() {
            eyre::bail!("parent state root mismatch");
        }

        let bytecodes_by_hash =
            self.bytecodes.iter().map(|code| (code.hash_slow(), code)).collect::<HashMap<_, _>>();

        let mut accounts = HashMap::default();
        let mut storage = HashMap::default();
        for (&address, slots) in self.state_requests.iter() {
            let hashed_address = keccak256(address);
            let hashed_address = hashed_address.as_slice();

            let account_in_trie: Option<TrieAccount> = flat_state_trie
                .get(hashed_address)
                .map_err(|e| eyre::eyre!("failed to get account from flat trie: {}", e))?
                .map(|val| AlloyDecodable::decode(&mut val.as_slice()))
                .transpose()?;

            accounts.insert(
                address,
                match account_in_trie {
                    Some(account_in_trie) => AccountInfo {
                        balance: account_in_trie.balance,
                        nonce: account_in_trie.nonce,
                        code_hash: account_in_trie.code_hash,
                        code: Some(
                            (*bytecodes_by_hash
                                .get(&account_in_trie.code_hash)
                                .ok_or_else(|| eyre::eyre!("missing bytecode"))?)
                            .to_owned(),
                        ),
                    },
                    _ => Default::default(),
                },
            );

            if !slots.is_empty() {
                let mut address_storage = HashMap::default();

                if let Some(storage_trie_view) = flat_state
                    .storage_tries
                    .get(&B256::from_slice(hashed_address))
                    .map(|t| t.view())
                {
                    for &slot in slots {
                        let slot_bytes = keccak256(slot.to_be_bytes::<32>());
                        let slot_value: U256 = storage_trie_view
                            .get(slot_bytes.as_slice())
                            .map_err(|e| {
                                eyre::eyre!("failed to get storage from flat trie: {}", e)
                            })?
                            .map(|val| AlloyDecodable::decode(&mut val.as_slice()))
                            .transpose()?
                            .unwrap_or_default();
                        address_storage.insert(slot, slot_value);
                    }
                }

                storage.insert(address, address_storage);
            }
        }

        // Verify and build block hashes
        let mut block_hashes: HashMap<u64, B256, _> = HashMap::default();
        for (child_header, parent_header) in
            once(&self.current_block.header).chain(self.ancestor_headers.iter()).tuple_windows()
        {
            if parent_header.number != child_header.number - 1 {
                eyre::bail!("non-consecutive blocks");
            }

            if parent_header.hash_slow() != child_header.parent_hash {
                eyre::bail!("parent hash mismatch");
            }

            block_hashes.insert(parent_header.number, child_header.parent_hash);
        }

        Ok(WitnessDb { accounts, storage, block_hashes })
    }
}
