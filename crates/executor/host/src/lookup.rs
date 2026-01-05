//! Preimage lookup table for finding storage keys with specific keccak256 hash prefixes.
//!
//! This is used to resolve "orphan" nodes in the MPT when building witnesses from eth_getProof.
//! When a deletion causes branch collapse and the sibling is unresolved, we need to find a
//! storage key whose hash starts with the sibling's prefix to fetch its proof.

use alloy_primitives::{keccak256, B256, U256};
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    OnceLock,
};
use tracing::info;

/// A lookup table for Keccak256 pre-images.
///
/// Given a nibble prefix, finds a 32-byte value whose keccak256 hash starts with that prefix.
/// This is used to find storage keys that can be used to fetch additional proofs.
#[derive(Debug, Clone)]
pub struct PreimageLookup {
    table: Vec<u64>,
    nibbles: usize,
}

impl PreimageLookup {
    /// Creates a new lookup table by precomputing hashes in parallel.
    ///
    /// The `prefix_length` determines how many nibbles the table covers.
    /// Table size is 16^prefix_length * 8 bytes:
    /// - 2 nibbles: 2 KB
    /// - 3 nibbles: 32 KB
    /// - 4 nibbles: 512 KB
    /// - 5 nibbles: 8 MB
    pub fn new(prefix_length: u8) -> Self {
        if prefix_length == 0 {
            return Self { table: vec![], nibbles: 0 };
        }

        info!(%prefix_length, "Generating preimage cache");
        let prefix_count = 16usize.checked_pow(prefix_length as u32).expect("nibbles too large");

        let table: Vec<OnceLock<u64>> = (0..prefix_count).map(|_| OnceLock::new()).collect();
        let found = AtomicUsize::new(0);

        // Use Rayon to parallelize the search over the nonce space
        let _ = (0..=u64::MAX).into_par_iter().try_for_each(|nonce| {
            // Stop processing if we have found all prefixes
            if found.load(Ordering::Relaxed) >= prefix_count {
                return Err(());
            }

            let hash = keccak256(B256::from(U256::from(nonce)));
            let idx = get_index_from_hash(hash, prefix_length as usize);

            // If we successfully set the cell (it was empty), increment the counter
            if table[idx].set(nonce).is_ok() {
                found.fetch_add(1, Ordering::Relaxed);
            }

            Ok(())
        });
        info!("Preimage cache generated");

        let final_table = table.into_iter().map(|nonce| nonce.into_inner().unwrap()).collect();

        Self { table: final_table, nibbles: prefix_length as usize }
    }

    /// Creates a disabled lookup table that always returns None.
    pub fn disabled() -> Self {
        Self { table: vec![], nibbles: 0 }
    }

    /// Finds a pre-image for a given nibble prefix.
    ///
    /// Returns a 32-byte value whose keccak256 hash starts with the given prefix.
    /// Returns None if the prefix is longer than the table supports.
    pub fn find(&self, prefix: &[u8]) -> Option<B256> {
        if prefix.len() > self.nibbles || self.table.is_empty() {
            return None;
        }

        let idx = get_index_unchecked(prefix);
        let nonce = self.table.get(idx).copied()?;

        Some(B256::from(U256::from(nonce)))
    }

    /// Returns the number of nibbles this table covers.
    pub fn nibbles(&self) -> usize {
        self.nibbles
    }
}

fn get_index_from_hash(hash: B256, prefix_length: usize) -> usize {
    let nibbles = unpack_nibbles(&hash[..prefix_length.div_ceil(2)]);
    get_index_unchecked(&nibbles[..prefix_length])
}

/// Calculate the little-endian index from the input nibbles.
/// E.g., for [A, B, C], the index will be 0x...CBA.
fn get_index_unchecked(nibbles: &[u8]) -> usize {
    nibbles.iter().rfold(0, |a, n| (a << 4) | *n as usize)
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

/// Packs nibbles into bytes (right-padded with zeros if odd length).
pub(crate) fn pack_nibbles(nibbles: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(nibbles.len().div_ceil(2));
    for chunk in nibbles.chunks(2) {
        let high = chunk[0];
        let low = chunk.get(1).copied().unwrap_or(0);
        bytes.push((high << 4) | low);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preimage_lookup() {
        let lookup = PreimageLookup::new(2);

        // Find preimage for prefix 0xAB
        let preimage = lookup.find(&[0x0a, 0x0b]).unwrap();
        let hash = keccak256(preimage);
        let hash_nibbles = unpack_nibbles(&hash[..1]);
        assert_eq!(hash_nibbles[0], 0x0a);
        assert_eq!(hash_nibbles[1], 0x0b);

        // Find preimage for single nibble prefix
        let preimage = lookup.find(&[0x0a]).unwrap();
        let hash = keccak256(preimage);
        let hash_nibbles = unpack_nibbles(&hash[..1]);
        assert_eq!(hash_nibbles[0], 0x0a);

        // Empty prefix should work
        assert!(lookup.find(&[]).is_some());

        // Prefix longer than table should return None
        assert!(lookup.find(&[0x0a, 0x0b, 0x0c]).is_none());
    }

    #[test]
    fn disabled_lookup() {
        let lookup = PreimageLookup::disabled();
        assert!(lookup.find(&[0x0a]).is_none());
        assert!(lookup.find(&[]).is_none());
    }
}
