//! Debug RPC methods for fetching storage and account range data.

use alloy_primitives::{keccak256, Address, B256};
use alloy_provider::Provider;
use eyre::{ensure, eyre};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::lookup::pack_nibbles;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageRangeQueryResponse {
    storage: HashMap<B256, StorageRangeQueryResponseEntry>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    next_key: Option<B256>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageRangeQueryResponseEntry {
    /// The actual storage key (preimage of the hash).
    key: Option<B256>,
    /// The storage value.
    value: alloy_primitives::U256,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountRangeQueryResponse {
    accounts: HashMap<B256, AccountRangeQueryResponseEntry>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    next: Option<B256>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountRangeQueryResponseEntry {
    /// The actual account address (preimage of the hash).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    address: Option<Address>,
    balance: alloy_primitives::U256,
    nonce: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    code_hash: Option<B256>,
}

/// Fetches the next storage key for an address using `debug_storageRangeAt`.
///
/// This is used to find a storage key whose keccak256 hash starts with the given prefix,
/// which is needed to fetch additional proofs for orphan resolution.
///
/// # Arguments
/// * `tx_index` - The transaction index to query state at. Use the transaction count
///   of the block to get the final state after all transactions.
pub(crate) async fn get_next_storage_key<P: Provider>(
    provider: &P,
    block_hash: B256,
    tx_index: usize,
    address: Address,
    prefix: &[u8],
) -> eyre::Result<B256> {
    tracing::debug!(%address, ?prefix, tx_index, "debug_storageRangeAt");

    const STORAGE_RANGE_PAGE_SIZE: u64 = 64;
    const STORAGE_RANGE_MAX_PAGES: usize = 64;

    // Pack nibbles into bytes, right-padded with zeros.
    let mut start_key = B256::right_padding_from(&pack_nibbles(prefix));
    let mut pages = 0usize;

    loop {
        let params = (block_hash, tx_index, address, start_key, STORAGE_RANGE_PAGE_SIZE);

        let response: StorageRangeQueryResponse = provider
            .client()
            .request("debug_storageRangeAt", params)
            .await
            .map_err(|e| eyre!("debug_storageRangeAt failed: {e}"))?;

        if response.storage.is_empty() {
            return Err(eyre!("No storage slots returned from debug_storageRangeAt"));
        }

        if let Some((hashed_key, entry)) = response
            .storage
            .iter()
            .find(|(hash, _)| hash_has_prefix(hash, prefix))
        {
            let storage_key = entry
                .key
                .ok_or_else(|| eyre!("Preimage storage key is missing from debug response"))?;
            let computed_hash = keccak256(storage_key);
            if computed_hash != *hashed_key {
                return Err(eyre!(
                    "debug_storageRangeAt returned inconsistent preimage: expected hash {hashed_key}, got {computed_hash}"
                ));
            }
            return Ok(storage_key);
        }

        let next_key = response.next_key.ok_or_else(|| {
            eyre!(
                "No storage exists at prefix {:?} (range exhausted before match)",
                prefix
            )
        })?;

        if !hash_has_prefix(&next_key, prefix) {
            return Err(eyre!(
                "No storage exists at prefix {:?} (next key outside prefix range)",
                prefix
            ));
        }

        pages += 1;
        if pages >= STORAGE_RANGE_MAX_PAGES {
            return Err(eyre!(
                "debug_storageRangeAt pagination exceeded {} pages without a match",
                STORAGE_RANGE_MAX_PAGES
            ));
        }

        start_key = next_key;
    }
}

/// Fetches the next account address using `debug_accountRange`.
///
/// This is used to find an account whose keccak256(address) starts with the given prefix,
/// which is needed to fetch additional proofs for state trie orphan resolution.
///
/// Note: This requires the node to have preimage storage enabled. If the node doesn't
/// store address preimages, this will fail.
///
/// Note: Unlike `debug_storageRangeAt`, `debug_accountRange` doesn't have a txIndex
/// parameter - it queries state at the end of the block.
pub(crate) async fn get_next_account<P: Provider>(
    provider: &P,
    block_hash: B256,
    prefix: &[u8],
) -> eyre::Result<Address> {
    tracing::debug!(?prefix, "debug_accountRange");

    const ACCOUNT_RANGE_PAGE_SIZE: u64 = 64;
    const ACCOUNT_RANGE_MAX_PAGES: usize = 64;

    // Pack nibbles into bytes, right-padded with zeros.
    let mut start_key = B256::right_padding_from(&pack_nibbles(prefix));
    let mut pages = 0usize;

    loop {
        // Params: (blockNrOrHash, start, maxResults, nocode, nostorage, incompletes)
        let params = (block_hash, start_key, ACCOUNT_RANGE_PAGE_SIZE, true, true, false);

        let response: AccountRangeQueryResponse = provider
            .client()
            .request("debug_accountRange", params)
            .await
            .map_err(|e| eyre!("debug_accountRange failed: {e}"))?;

        if response.accounts.is_empty() {
            return Err(eyre!("No accounts returned from debug_accountRange"));
        }

        if let Some((hashed_address, entry)) = response
            .accounts
            .iter()
            .find(|(hash, _)| hash_has_prefix(hash, prefix))
        {
            let address = entry.address.ok_or_else(|| {
                eyre!(
                    "Address preimage missing from debug_accountRange response for hash {hashed_address}. \
                     The node may not have preimage storage enabled. State orphan resolution requires \
                     a node with --preimage flag or equivalent."
                )
            })?;
            let computed_hash = keccak256(address);
            ensure!(
                computed_hash == *hashed_address,
                "debug_accountRange returned inconsistent preimage: expected hash {hashed_address}, got {computed_hash}"
            );
            return Ok(address);
        }

        let next_key = response.next.ok_or_else(|| {
            eyre!(
                "No account exists at prefix {:?} (range exhausted before match)",
                prefix
            )
        })?;

        if !hash_has_prefix(&next_key, prefix) {
            return Err(eyre!(
                "No account exists at prefix {:?} (next key outside prefix range)",
                prefix
            ));
        }

        pages += 1;
        if pages >= ACCOUNT_RANGE_MAX_PAGES {
            return Err(eyre!(
                "debug_accountRange pagination exceeded {} pages without a match",
                ACCOUNT_RANGE_MAX_PAGES
            ));
        }

        start_key = next_key;
    }
}

/// Unpacks bytes into nibbles.
fn unpack_nibbles(bytes: &[u8]) -> Vec<u8> {
    let mut nibbles = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        nibbles.push(byte >> 4);
        nibbles.push(byte & 0x0f);
    }
    nibbles
}

/// Returns true if the hash starts with the given nibble prefix.
fn hash_has_prefix(hash: &B256, prefix: &[u8]) -> bool {
    if prefix.is_empty() {
        return true;
    }
    let byte_len = prefix.len().div_ceil(2);
    let hash_nibbles = unpack_nibbles(&hash[..byte_len]);
    hash_nibbles[..prefix.len()] == *prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_range_response_parsing() {
        // Test parsing a valid response with storage entry
        let json = r#"{
            "storage": {
                "0x0000000000000000000000000000000000000000000000000000000000000001": {
                    "key": "0x0000000000000000000000000000000000000000000000000000000000000005",
                    "value": "0x64"
                }
            },
            "nextKey": null
        }"#;

        let response: StorageRangeQueryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.storage.len(), 1);
        assert!(response.next_key.is_none());

        let (_, entry) = response.storage.into_iter().next().unwrap();
        assert!(entry.key.is_some());
        assert_eq!(entry.value, alloy_primitives::U256::from(100));
    }

    #[test]
    fn test_storage_range_response_without_preimage() {
        // Test parsing a response without storage key preimage
        let json = r#"{
            "storage": {
                "0x0000000000000000000000000000000000000000000000000000000000000001": {
                    "key": null,
                    "value": "0x64"
                }
            }
        }"#;

        let response: StorageRangeQueryResponse = serde_json::from_str(json).unwrap();
        let (_, entry) = response.storage.into_iter().next().unwrap();
        assert!(entry.key.is_none());
    }

    #[test]
    fn test_account_range_response_parsing() {
        // Test parsing a valid response with account entry
        let json = r#"{
            "accounts": {
                "0x0000000000000000000000000000000000000000000000000000000000000001": {
                    "address": "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
                    "balance": "0x1234",
                    "nonce": 42,
                    "codeHash": "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
                }
            },
            "next": null
        }"#;

        let response: AccountRangeQueryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.accounts.len(), 1);
        assert!(response.next.is_none());

        let (_, entry) = response.accounts.into_iter().next().unwrap();
        assert!(entry.address.is_some());
        assert_eq!(entry.nonce, 42);
    }

    #[test]
    fn test_account_range_response_without_preimage() {
        // Test parsing a response without address preimage (common case)
        let json = r#"{
            "accounts": {
                "0x0000000000000000000000000000000000000000000000000000000000000001": {
                    "balance": "0x1234",
                    "nonce": 42
                }
            }
        }"#;

        let response: AccountRangeQueryResponse = serde_json::from_str(json).unwrap();
        let (_, entry) = response.accounts.into_iter().next().unwrap();
        assert!(entry.address.is_none());
    }

    #[test]
    fn test_unpack_nibbles() {
        assert_eq!(unpack_nibbles(&[0xab, 0xcd]), vec![0x0a, 0x0b, 0x0c, 0x0d]);
        assert_eq!(unpack_nibbles(&[0x12]), vec![0x01, 0x02]);
        assert_eq!(unpack_nibbles(&[]), Vec::<u8>::new());
    }
}
