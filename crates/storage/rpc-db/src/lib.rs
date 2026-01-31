use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

use alloy_provider::{network::Ethereum, Provider};
use alloy_rpc_types::BlockId;
use reth_revm::{
    primitives::{keccak256, Address, HashMap, B256, U256},
    state::{AccountInfo, Bytecode},
    DatabaseRef,
};
use reth_storage_errors::{db::DatabaseError, provider::ProviderError};
use serde::Deserialize;

/// A database that fetches data from a [Provider].
#[derive(Debug, Clone)]
pub struct RpcDb<P> {
    /// The provider which fetches data.
    pub provider: P,
    /// The block to fetch data from.
    pub block: BlockId,
    /// The cached accounts.
    pub accounts: RefCell<HashMap<Address, AccountInfo>>,
    /// The cached storage values.
    pub storage: RefCell<HashMap<Address, HashMap<U256, U256>>>,
    /// The oldest block whose header/hash has been requested.
    pub oldest_ancestor: RefCell<u64>,
}

/// Errors that can occur when interacting with the [RpcDb].
#[derive(Debug, Clone, thiserror::Error)]
pub enum RpcDbError {
    #[error("failed to fetch data: {0}")]
    RpcError(String),
    #[error("failed to find block")]
    BlockNotFound,
    #[error("failed to find trie node preimage")]
    PreimageNotFound,
}

/// Response from debug_storageRangeAt RPC method.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRangeResult {
    pub storage: HashMap<B256, StorageEntry>,
    pub next_key: Option<B256>,
}

/// Individual storage entry from debug_storageRangeAt.
#[derive(Debug, Deserialize)]
pub struct StorageEntry {
    pub key: Option<B256>,
    pub value: U256,
}

/// Response from debug_accountRange RPC method.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRangeResult {
    /// Map from address to account info. The key is the actual address, not hashed.
    pub accounts: HashMap<Address, AccountRangeEntry>,
    /// Next key for pagination (base64 encoded, we don't need to parse it).
    #[serde(default)]
    pub next: Option<serde_json::Value>,
}

/// Individual account entry from debug_accountRange.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRangeEntry {
    /// Balance as a decimal string or hex.
    #[serde(deserialize_with = "deserialize_balance")]
    pub balance: U256,
    pub nonce: u64,
    pub root: B256,
    pub code_hash: B256,
}

/// Deserialize balance which can be either decimal string or hex string.
fn deserialize_balance<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    if s.starts_with("0x") {
        U256::from_str_radix(&s[2..], 16).map_err(serde::de::Error::custom)
    } else {
        U256::from_str_radix(&s, 10).map_err(serde::de::Error::custom)
    }
}

/// Convert nibble prefix to B256 hash (right-padded with zeros).
/// nibbles: [0xa, 0xb, 0xc] → 0xabc00000...
pub fn nibbles_to_hash(nibbles: &[u8]) -> B256 {
    let mut bytes = [0u8; 32];
    for (i, chunk) in nibbles.chunks(2).enumerate() {
        if i >= 32 {
            break;
        }
        bytes[i] = (chunk[0] << 4) | chunk.get(1).copied().unwrap_or(0);
    }
    B256::from(bytes)
}

/// Check if a hash starts with the given nibble prefix.
pub fn hash_matches_prefix(hash: &B256, nibbles: &[u8]) -> bool {
    let bytes = hash.as_slice();
    nibbles.iter().enumerate().all(|(i, &nib)| {
        let actual = if i % 2 == 0 { bytes[i / 2] >> 4 } else { bytes[i / 2] & 0x0f };
        actual == nib
    })
}

impl<P: Provider<Ethereum> + Clone> RpcDb<P> {
    /// Create a new [`RpcDb`].
    pub fn new(provider: P, block: u64) -> Self {
        RpcDb {
            provider,
            block: block.into(),
            accounts: RefCell::new(HashMap::default()),
            storage: RefCell::new(HashMap::default()),
            oldest_ancestor: RefCell::new(block),
        }
    }

    /// Fetch the [AccountInfo] for an [Address].
    pub async fn fetch_account_info(&self, address: Address) -> Result<AccountInfo, RpcDbError> {
        tracing::info!("fetching account info for address: {}", address);

        // Fetch the proof for the account.
        let proof = self
            .provider
            .get_proof(address, vec![])
            .block_id(self.block)
            .await
            .map_err(|e| RpcDbError::RpcError(e.to_string()))?;

        // Fetch the code of the account.
        let code = self
            .provider
            .get_code_at(address)
            .block_id(self.block)
            .await
            .map_err(|e| RpcDbError::RpcError(e.to_string()))?;

        // Construct the account info & write it to the log.
        let bytecode = Bytecode::new_raw(code);
        let account_info = AccountInfo {
            nonce: proof.nonce,
            balance: proof.balance,
            code_hash: bytecode.hash_slow(),
            code: Some(bytecode.clone()),
            account_id: None,
        };

        // Record the account info to the state.
        self.accounts.borrow_mut().insert(address, account_info.clone());

        Ok(account_info)
    }

    /// Fetch the storage value at an [Address] and [U256] index.
    pub async fn fetch_storage_at(
        &self,
        address: Address,
        index: U256,
    ) -> Result<U256, RpcDbError> {
        tracing::info!("fetching storage value at address: {}, index: {}", address, index);

        // Fetch the storage value.
        let value = self
            .provider
            .get_storage_at(address, index)
            .block_id(self.block)
            .await
            .map_err(|e| RpcDbError::RpcError(e.to_string()))?;

        // Record the storage value to the state.
        let mut storage_values = self.storage.borrow_mut();
        let entry = storage_values.entry(address).or_default();
        entry.insert(index, value);

        Ok(value)
    }

    /// Fetch the block hash for a block number.
    pub async fn fetch_block_hash(&self, number: u64) -> Result<B256, RpcDbError> {
        tracing::info!("fetching block hash for block number: {}", number);

        // Fetch the block.
        let block = self
            .provider
            .get_block_by_number(number.into())
            .await
            .map_err(|e| RpcDbError::RpcError(e.to_string()))?;

        // Record the block hash to the state.
        let block = block.ok_or(RpcDbError::BlockNotFound)?;
        let hash = block.header.hash;

        let mut oldest_ancestor = self.oldest_ancestor.borrow_mut();
        *oldest_ancestor = number.min(*oldest_ancestor);

        Ok(hash)
    }

    /// Gets all the state keys used. The client uses this to read the actual state data from tries.
    pub fn get_state_requests(&self) -> HashMap<Address, Vec<U256>> {
        let accounts = self.accounts.borrow();
        let storage = self.storage.borrow();

        accounts
            .keys()
            .chain(storage.keys())
            .map(|&address| {
                let storage_keys_for_address: BTreeSet<U256> = storage
                    .get(&address)
                    .map(|storage_map| storage_map.keys().cloned().collect())
                    .unwrap_or_default();

                (address, storage_keys_for_address.into_iter().collect())
            })
            .collect()
    }

    /// Gets all account bytecodes.
    pub fn get_bytecodes(&self) -> Vec<Bytecode> {
        let accounts = self.accounts.borrow();

        accounts
            .values()
            .flat_map(|account| account.code.clone())
            .map(|code| (code.hash_slow(), code))
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>()
    }

    /// Find a storage key whose keccak256 hash starts with the given nibble prefix.
    /// Uses debug_storageRangeAt RPC method.
    ///
    /// This is used for orphan resolution when `eth_getProof` doesn't include
    /// all necessary nodes for MPT branch collapse operations.
    ///
    /// `tx_index` should be 0 to query the pre-state for the given block hash,
    /// which matches the zeth debug_storageRangeAt usage.
    pub async fn get_storage_key_by_hash_prefix(
        &self,
        block_hash: B256,
        address: Address,
        prefix_nibbles: &[u8],
        tx_index: usize,
    ) -> Result<B256, RpcDbError> {
        tracing::info!(
            "fetching storage key by hash prefix: address={}, prefix={:?}, tx_index={}",
            address,
            prefix_nibbles,
            tx_index
        );

        // Convert nibbles to start key (right-padded with zeros), matching zeth.
        let start_key = nibbles_to_hash(prefix_nibbles);
        eprintln!(
            "[orphan] debug_storageRangeAt: start_key={} (from nibbles {:?}), txIndex={}",
            start_key, prefix_nibbles, tx_index
        );

        // Call debug_storageRangeAt with limit=1, like zeth.
        let result: StorageRangeResult = self
            .provider
            .raw_request(
                Cow::Borrowed("debug_storageRangeAt"),
                (block_hash, tx_index, address, start_key, 1u64),
            )
            .await
            .map_err(|e| RpcDbError::RpcError(e.to_string()))?;

        let (hashed_key, entry) = result
            .storage
            .into_iter()
            .next()
            .ok_or(RpcDbError::PreimageNotFound)?;
        let storage_key = entry.key.ok_or(RpcDbError::PreimageNotFound)?;

        let storage_hash = keccak256(storage_key);
        if !hash_matches_prefix(&storage_hash, prefix_nibbles) {
            return Err(RpcDbError::RpcError(format!(
                "Invalid debug_storageRangeAt response: prefix={:?}, hashed_key={}, key={}, key_hash={}",
                prefix_nibbles, hashed_key, storage_key, storage_hash
            )));
        }

        Ok(storage_key)
    }

    /// Find an account address whose keccak256 hash starts with the given nibble prefix.
    ///
    /// This is challenging because debug_accountRange (on most providers) iterates by
    /// address order, not by hash order. We try debug_accountRange but it may not find
    /// the account efficiently.
    ///
    /// Note: `tx_index` is accepted for API consistency but is not used because
    /// debug_accountRange doesn't support txIndex like debug_storageRangeAt does.
    /// It queries the state at the given block hash.
    #[allow(unused_variables)]
    pub async fn get_account_address_by_hash_prefix(
        &self,
        block_hash: B256,
        prefix_nibbles: &[u8],
        tx_index: usize,
    ) -> Result<Option<Address>, RpcDbError> {
        tracing::debug!("fetching account address by hash prefix: prefix={:?}", prefix_nibbles);

        // Try debug_accountRange - on geth it might iterate by hash, on Quicknode it iterates by address
        let target_hash = nibbles_to_hash(prefix_nibbles);

        // Try a few pages of results in case we get lucky
        let mut start = target_hash;
        for page in 0..5 {
            let result = match self
                .provider
                .raw_request::<_, AccountRangeResult>(
                    Cow::Borrowed("debug_accountRange"),
                    (block_hash, start, 256u64, true, true, false),
                )
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    tracing::debug!("debug_accountRange failed: {}", e);
                    break;
                }
            };

            for (address, _entry) in &result.accounts {
                let address_hash = keccak256(address);
                if hash_matches_prefix(&address_hash, prefix_nibbles) {
                    tracing::info!(
                        "found via debug_accountRange (page {}): {} with hash {}",
                        page,
                        address,
                        address_hash
                    );
                    return Ok(Some(*address));
                }
            }

            // Move to next page if available
            match result.next {
                Some(_) if !result.accounts.is_empty() => {
                    // Use the last address as the next start point
                    if let Some(last_addr) = result.accounts.keys().max() {
                        start = keccak256(last_addr);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        tracing::debug!(
            "could not find account with hash prefix {:?} via debug_accountRange",
            prefix_nibbles
        );
        Ok(None)
    }
}

impl<P: Provider<Ethereum> + Clone> DatabaseRef for RpcDb<P> {
    type Error = ProviderError;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let handle = tokio::runtime::Handle::try_current().map_err(|_| {
            ProviderError::Database(DatabaseError::Other("no tokio runtime found".to_string()))
        })?;
        let result =
            tokio::task::block_in_place(|| handle.block_on(self.fetch_account_info(address)));
        let account_info =
            result.map_err(|e| ProviderError::Database(DatabaseError::Other(e.to_string())))?;

        if account_info.eq(&AccountInfo::default()) {
            return Ok(None);
        }

        Ok(Some(account_info))
    }

    fn code_by_hash_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unimplemented!()
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let handle = tokio::runtime::Handle::try_current().map_err(|_| {
            ProviderError::Database(DatabaseError::Other("no tokio runtime found".to_string()))
        })?;
        let result =
            tokio::task::block_in_place(|| handle.block_on(self.fetch_storage_at(address, index)));
        let value =
            result.map_err(|e| ProviderError::Database(DatabaseError::Other(e.to_string())))?;
        Ok(value)
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        let handle = tokio::runtime::Handle::try_current().map_err(|_| {
            ProviderError::Database(DatabaseError::Other("no tokio runtime found".to_string()))
        })?;
        let result = tokio::task::block_in_place(|| handle.block_on(self.fetch_block_hash(number)));
        let value =
            result.map_err(|e| ProviderError::Database(DatabaseError::Other(e.to_string())))?;
        Ok(value)
    }
}
