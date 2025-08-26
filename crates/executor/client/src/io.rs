use std::iter::once;

use bumpalo::Bump;
use eyre::{bail, OptionExt, Result};
use itertools::Itertools;
use openvm_witness_db::WitnessDb;
use reth_primitives::{Block, Header, TransactionSigned};
use reth_trie::TrieAccount;
use revm::state::{AccountInfo, Bytecode};
use revm_primitives::{keccak256, map::DefaultHashBuilder, Address, HashMap, B256, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// The input for the client to execute a block and fully verify the STF (state transition
/// function).
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Network state as of the parent block.
    pub parent_state_bytes: mptnew::EthereumStateBytes,
    /// Requests to account state and storage slots.
    pub state_requests: HashMap<Address, Vec<U256>>,
    /// Account bytecodes.
    pub bytecodes: Vec<Bytecode>,
}

#[derive(Debug, Clone)]
pub struct ClientExecutorInputWithState {
    pub input: &'static ClientExecutorInput,
    pub state: mptnew::EthereumState,
}

impl ClientExecutorInputWithState {
    /// Parses `input.parent_state_bytes` into `EthereumState` and verifies state and storage roots.
    pub fn build(input: ClientExecutorInput) -> Result<Self> {
        let input = Box::leak(Box::new(input));
        let bump = Box::leak(Box::new(Bump::with_capacity(1000 * 1000)));

        let state = {
            let (state_num_nodes, state_bytes) = &input.parent_state_bytes.state_trie;

            let state_trie =
                mptnew::MptTrie::decode_trie(bump, &mut state_bytes.as_ref(), *state_num_nodes)?;
            if state_trie.hash() != input.ancestor_headers[0].state_root {
                bail!("state root mismatch");
            }

            let mut storage_tries = HashMap::with_capacity_and_hasher(
                input.parent_state_bytes.storage_tries.len(),
                DefaultHashBuilder::default(),
            );
            for (hashed_address, num_nodes, storage_trie_bytes) in
                &input.parent_state_bytes.storage_tries
            {
                let account_in_trie =
                    state_trie.get_rlp::<TrieAccount>(hashed_address.as_slice()).unwrap();
                let expected_storage_root =
                    account_in_trie.map_or(reth_trie::EMPTY_ROOT_HASH, |a| a.storage_root);

                let storage_trie = mptnew::MptTrie::decode_trie(
                    bump,
                    &mut storage_trie_bytes.as_ref(),
                    *num_nodes,
                )?;
                if storage_trie.hash() != expected_storage_root {
                    bail!("storage root mismatch");
                }

                storage_tries.insert(*hashed_address, storage_trie);
            }
            mptnew::EthereumState { state_trie, storage_tries, bump }
        };

        Ok(Self { input, state })
    }
}

impl ClientExecutorInputWithState {
    /// Gets the immediate parent block's header.
    #[inline(always)]
    pub fn parent_header(&self) -> &Header {
        &self.input.ancestor_headers[0]
    }

    /// Creates a [`WitnessDb`].
    pub fn witness_db(&self) -> Result<WitnessDb> {
        <Self as WitnessInput>::witness_db(self)
    }
}

impl WitnessInput for ClientExecutorInputWithState {
    #[inline(always)]
    fn state(&self) -> &mptnew::EthereumState {
        &self.state
    }

    #[inline(always)]
    fn state_anchor(&self) -> B256 {
        self.parent_header().state_root
    }

    #[inline(always)]
    fn state_requests(&self) -> impl Iterator<Item = (&Address, &Vec<U256>)> {
        self.input.state_requests.iter()
    }

    #[inline(always)]
    fn bytecodes(&self) -> impl Iterator<Item = &Bytecode> {
        self.input.bytecodes.iter()
    }

    #[inline(always)]
    fn headers(&self) -> impl Iterator<Item = &Header> {
        once(&self.input.current_block.header).chain(self.input.ancestor_headers.iter())
    }
}

/// A trait for constructing [`WitnessDb`].
pub trait WitnessInput {
    /// Gets a reference to the state from which account info and storage slots are loaded.
    fn state(&self) -> &mptnew::EthereumState;

    /// Gets the state trie root hash that the state referenced by
    /// [state()](trait.WitnessInput#tymethod.state) must conform to.
    fn state_anchor(&self) -> B256;

    /// Gets an iterator over address state requests. For each request, the account info and storage
    /// slots are loaded from the relevant tries in the state returned by
    /// [state()](trait.WitnessInput#tymethod.state).
    fn state_requests(&self) -> impl Iterator<Item = (&Address, &Vec<U256>)>;

    /// Gets an iterator over account bytecodes.
    fn bytecodes(&self) -> impl Iterator<Item = &Bytecode>;

    /// Gets an iterator over references to a consecutive, reverse-chronological block headers
    /// starting from the current block header.
    fn headers(&self) -> impl Iterator<Item = &Header>;

    /// Creates a [`WitnessDb`] from a [`WitnessInput`] implementation. To do so, it verifies the
    /// state root, ancestor headers and account bytecodes, and constructs the account and
    /// storage values by reading against state tries.
    ///
    /// NOTE: For some unknown reasons, calling this trait method directly from outside of the type
    /// implementing this trait causes a zkVM run to cost over 5M cycles more. To avoid this, define
    /// a method inside the type that calls this trait method instead.
    #[inline(always)]
    fn witness_db(&self) -> Result<WitnessDb> {
        let state = self.state();

        let bytecodes_by_hash =
            self.bytecodes().map(|code| (code.hash_slow(), code)).collect::<HashMap<_, _>>();

        let state_requests_iter = self.state_requests();
        let (lower, _) = state_requests_iter.size_hint();
        let mut accounts = HashMap::with_capacity_and_hasher(lower, DefaultHashBuilder::default());
        let mut storage = HashMap::with_capacity_and_hasher(lower, DefaultHashBuilder::default());

        for (&address, slots) in state_requests_iter {
            let hashed_address = keccak256(address);

            let account_in_trie =
                state.state_trie.get_rlp::<TrieAccount>(hashed_address.as_slice())?;

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
                                .ok_or_eyre("missing bytecode")?)
                            // Cloning here is fine as `Bytes` is cheap to clone.
                            .to_owned(),
                        ),
                    },
                    _ => Default::default(),
                },
            );

            if !slots.is_empty() {
                let mut address_storage =
                    HashMap::with_capacity_and_hasher(slots.len(), DefaultHashBuilder::default());

                let storage_trie = state
                    .storage_tries
                    .get(&hashed_address)
                    .ok_or_eyre("parent state does not contain storage trie")?;

                for &slot in slots {
                    let slot_value = storage_trie
                        .get_rlp::<U256>(keccak256(slot.to_be_bytes::<32>()).as_slice())?
                        .unwrap_or_default();
                    address_storage.insert(slot, slot_value);
                }

                storage.insert(address, address_storage);
            }
        }

        // Verify and build block hashes
        let headers_iter = self.headers();
        let (lower, _) = headers_iter.size_hint();
        let mut block_hashes: HashMap<u64, B256, _> =
            HashMap::with_capacity_and_hasher(lower, DefaultHashBuilder::default());
        for (child_header, parent_header) in headers_iter.tuple_windows() {
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
