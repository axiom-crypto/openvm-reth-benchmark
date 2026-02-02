use alloy::{
    primitives::{B256, U256},
    providers::{Network, Provider},
    serde::JsonStorageKey,
};
use alloy_primitives::{keccak256, Address};
use async_trait::async_trait;
use eyre::{ensure, Context, ContextCompat};
use risc0_ethereum_trie::Nibbles;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::trace;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRangeQueryResponse {
    pub storage: HashMap<B256, StorageRangeQueryResponseEntry>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub next_key: Option<B256>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRangeQueryResponseEntry {
    pub key: Option<JsonStorageKey>,
    pub value: U256,
}

/// An extension trait for Alloy providers that adds custom debug RPC methods.
#[async_trait]
pub trait DebugApi<N: Network>: Provider<N> {
    /// Fetches the next storage key for an address using `debug_storageRangeAt`.
    async fn get_next_storage_key(
        &self,
        block_hash: B256,
        address: Address,
        prefix: Nibbles,
    ) -> eyre::Result<B256>;
}

#[async_trait]
impl<P, N> DebugApi<N> for P
where
    P: Provider<N>,
    N: Network,
{
    async fn get_next_storage_key(
        &self,
        block_hash: B256,
        address: Address,
        prefix: Nibbles,
    ) -> eyre::Result<B256> {
        trace!(%address, ?prefix, "debug_storageRangeAt");

        let start_key = B256::right_padding_from(&prefix.pack());
        let params = (block_hash, 0, address, start_key, 1);

        let response: StorageRangeQueryResponse = self
            .client()
            .request("debug_storageRangeAt", params)
            .await
            .context("debug_storageRangeAt failed")?;

        let (_, entry) = response
            .storage
            .into_iter()
            .next()
            .context("No storage slot returned from debug_storageRangeAt")?;

        let storage_key =
            entry.key.context("Preimage storage key is missing from debug response")?.as_b256();

        // perform simple sanity checks, as this RPC is known to be wonky
        ensure!(
            Nibbles::unpack(keccak256(storage_key)).starts_with(&prefix),
            "Invalid debug_storageRangeAt response"
        );

        Ok(storage_key)
    }
}
