use crate::lookup::PreimageLookup;
use alloy::{
    network::Network,
    primitives::{keccak256, map::B256Set, Address, B256},
    providers::Provider,
};
use eyre::{bail, Context, ContextCompat, Result};
use revm::database::StorageWithOriginalValues;
use risc0_ethereum_trie::{orphan, Nibbles, Trie};
use std::collections::HashSet;
use tracing::{debug, trace};

pub(crate) async fn handle_removed_account<P, N>(
    provider: &P,
    block_hash: B256,
    address: Address,
    state_trie: &mut Trie,
) -> Result<()>
where
    P: Provider<N>,
    N: Network,
{
    trace!(%address, "Hydrating proof for destroyed account");
    let proof = provider
        .get_proof(address, vec![])
        .hash(block_hash)
        .await
        .context("eth_getProof failed")?;
    state_trie.hydrate_from_rlp(&proof.account_proof)?;
    state_trie.resolve_orphan(keccak256(address), &proof.account_proof)?;

    Ok(())
}

pub(crate) async fn handle_new_account<P, N>(
    provider: &P,
    block_hash: B256,
    address: Address,
    state_trie: &mut Trie,
) -> Result<()>
where
    P: Provider<N>,
    N: Network,
{
    trace!(%address, "Hydrating proof for new account");
    let proof = provider
        .get_proof(address, vec![])
        .hash(block_hash)
        .await
        .context("eth_getProof failed")?;
    state_trie.hydrate_from_rlp(proof.account_proof)?;

    Ok(())
}

pub(crate) async fn handle_modified_account<P, N>(
    provider: &P,
    block_hash: B256,
    address: Address,
    storage: &StorageWithOriginalValues,
    storage_trie: &mut Trie,
    lookup: &PreimageLookup,
    accessed_keys: &B256Set,
) -> Result<()>
where
    P: Provider<N>,
    N: Network,
{
    // collect the storage keys for any new or removed slot
    let keys: Vec<B256> = storage
        .iter()
        .filter_map(|(key, slot)| {
            if slot.original_value().is_zero() != slot.present_value().is_zero() {
                Some(B256::from(*key))
            } else {
                None
            }
        })
        .collect();

    if keys.is_empty() {
        return Ok(());
    }

    trace!(%address, num_keys = keys.len(), "Hydrating proof for new or removed slots");
    let proof =
        provider.get_proof(address, keys).hash(block_hash).await.context("eth_getProof failed")?;

    let mut unresolvable: HashSet<Nibbles> = HashSet::default();
    for storage_proof in proof.storage_proof {
        let hashed_key = keccak256(storage_proof.key.as_b256());
        storage_trie.hydrate_from_rlp(&storage_proof.proof)?;
        if storage_proof.value.is_zero() {
            match storage_trie.resolve_orphan(hashed_key, &storage_proof.proof) {
                Ok(_) => {}
                Err(orphan::Error::Unresolvable(prefix)) => {
                    unresolvable.insert(prefix);
                }
                Err(err) => bail!(err),
            }
        }
    }

    if unresolvable.is_empty() {
        return Ok(());
    }

    let mut missing_storage_keys = B256Set::default();
    for prefix in &unresolvable {
        // First, try to find the preimage in the accessed storage keys
        let storage_key = find_preimage_in_accessed_keys(accessed_keys, prefix)
            // Then try the static preimage lookup table
            .or_else(|| lookup.find(prefix))
            // If not found, return an error
            .with_context(|| {
                format!(
                    "Cannot find storage key preimage for prefix {:?} at address {}. \
                     The preimage is not in the accessed keys or the static lookup table. \
                     Consider increasing --preimage-cache-nibbles (current lookup covers {} nibbles).",
                    prefix, address, lookup.nibbles()
                )
            })?;
        missing_storage_keys.insert(storage_key);
    }

    if !missing_storage_keys.is_empty() {
        trace!(%address, keys=?missing_storage_keys, "Fetching final proofs for missing storage keys");
        let proof = provider
            .get_proof(address, missing_storage_keys.into_iter().collect())
            .hash(block_hash)
            .await
            .context("eth_getProof failed")?;

        storage_trie.hydrate_from_rlp(proof.storage_proof.iter().flat_map(|p| &p.proof))?;
    }

    Ok(())
}

/// Searches through accessed storage keys to find one whose keccak256 hash starts with the given prefix.
fn find_preimage_in_accessed_keys(accessed_keys: &B256Set, prefix: &Nibbles) -> Option<B256> {
    for key in accessed_keys {
        let hash_nibbles = Nibbles::unpack(keccak256(key));
        if hash_nibbles.starts_with(prefix) {
            debug!(?prefix, %key, "Found preimage in accessed storage keys");
            return Some(*key);
        }
    }
    None
}
