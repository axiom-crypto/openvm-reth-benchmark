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
pub(crate) async fn get_next_storage_key<P: Provider>(
    provider: &P,
    block_hash: B256,
    address: Address,
    prefix: &[u8],
) -> eyre::Result<B256> {
    tracing::debug!(%address, ?prefix, "debug_storageRangeAt");

    // Pack nibbles into bytes, right-padded with zeros
    let start_key = B256::right_padding_from(&pack_nibbles(prefix));
    let params = (block_hash, 0, address, start_key, 1);

    let response: StorageRangeQueryResponse = provider
        .client()
        .request("debug_storageRangeAt", params)
        .await
        .map_err(|e| eyre!("debug_storageRangeAt failed: {e}"))?;

    let (_, entry) = response
        .storage
        .into_iter()
        .next()
        .ok_or_else(|| eyre!("No storage slot returned from debug_storageRangeAt"))?;

    let storage_key =
        entry.key.ok_or_else(|| eyre!("Preimage storage key is missing from debug response"))?;

    // Perform simple sanity check
    let hash = keccak256(storage_key);
    let hash_nibbles = unpack_nibbles(&hash[..prefix.len().div_ceil(2)]);
    ensure!(
        hash_nibbles[..prefix.len()] == *prefix,
        "Invalid debug_storageRangeAt response: hash prefix doesn't match"
    );

    Ok(storage_key)
}

/// Fetches the next account address using `debug_accountRange`.
///
/// This is used to find an account whose keccak256(address) starts with the given prefix,
/// which is needed to fetch additional proofs for state trie orphan resolution.
///
/// Note: This requires the node to have preimage storage enabled. If the node doesn't
/// store address preimages, this will fail.
pub(crate) async fn get_next_account<P: Provider>(
    provider: &P,
    block_hash: B256,
    prefix: &[u8],
) -> eyre::Result<Address> {
    tracing::debug!(?prefix, "debug_accountRange");

    // Pack nibbles into bytes, right-padded with zeros
    let start_key = B256::right_padding_from(&pack_nibbles(prefix));
    // Params: (blockNrOrHash, start, maxResults, nocode, nostorage, incompletes)
    // We request 1 result, skip code/storage, and don't need incompletes
    let params = (block_hash, start_key, 1u64, true, true, false);

    let response: AccountRangeQueryResponse = provider
        .client()
        .request("debug_accountRange", params)
        .await
        .map_err(|e| eyre!("debug_accountRange failed: {e}"))?;

    let (hashed_address, entry) = response
        .accounts
        .into_iter()
        .next()
        .ok_or_else(|| eyre!("No account returned from debug_accountRange"))?;

    // Try to get the address preimage from the response
    let address = entry.address.ok_or_else(|| {
        eyre!(
            "Address preimage missing from debug_accountRange response for hash {hashed_address}. \
             The node may not have preimage storage enabled. State orphan resolution requires \
             a node with --preimage flag or equivalent."
        )
    })?;

    // Perform simple sanity check
    let hash = keccak256(address);
    let hash_nibbles = unpack_nibbles(&hash[..prefix.len().div_ceil(2)]);
    ensure!(
        hash_nibbles[..prefix.len()] == *prefix,
        "Invalid debug_accountRange response: hash prefix doesn't match"
    );

    Ok(address)
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
