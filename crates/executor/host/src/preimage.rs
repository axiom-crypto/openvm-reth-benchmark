use revm::database::BundleState;
use revm_primitives::{keccak256, Address, B256, U256};
use std::collections::HashMap;

/// Dictionary mapping hashed keys back to their preimages, used to resolve MPT orphan nodes.
///
/// Built from existing host data (`state_requests` + `BundleState`) with zero additional RPC
/// calls. The dictionary contains every account address and storage slot that was read or
/// written by any transaction in the block.
///
/// # Orphan resolution
///
/// When `eth_getProof` returns a proof for key K, the proof includes all trie nodes along
/// the path from the root to K. Sibling nodes not on this path appear as opaque `Digest`
/// references. If a later trie mutation (insert/delete) collapses a branch whose sibling is
/// such a `Digest`, the operation fails with `NodeNotResolved`.
///
/// To resolve the orphan, we need to find a key whose proof path traverses the orphaned
/// node. We locate the orphan's nibble prefix in the trie, then scan this dictionary for a
/// key whose keccak256 hash starts with that prefix. Fetching `eth_getProof` for that key
/// provides the missing node data.
///
/// # Cold sibling limitation
///
/// This dictionary only contains keys that were accessed during block execution. If the
/// orphaned sibling's subtree contains only keys that no transaction in the block touched
/// (a "cold sibling"), no matching preimage exists in the dictionary and resolution fails.
///
/// No standard Ethereum RPC endpoint exposes a preimage database (mapping keccak256 hashes
/// back to their inputs). The `debug_dbGet` endpoint exists in some node implementations
/// but is not available on any hosted RPC provider. As a result, the cold sibling case is
/// an inherent limitation of proof-based state reconstruction over standard RPC.
///
/// In practice, cold siblings are rare: the orphan's neighbor in the trie is usually a key
/// that was accessed by the same transaction or a related one in the block.
pub(crate) struct PreimageDictionary {
    /// keccak256(address) -> address
    account_keys: HashMap<B256, Address>,
    /// Per-address storage key preimages: address -> vec of (keccak256(slot), slot)
    storage_keys_by_address: HashMap<Address, Vec<(B256, U256)>>,
}

impl PreimageDictionary {
    /// Build the dictionary from state requests (RpcDb) and bundle state (executor outcome).
    pub(crate) fn build(
        state_requests: &revm_primitives::HashMap<Address, Vec<U256>>,
        bundle_state: &BundleState,
    ) -> Self {
        let mut account_keys = HashMap::new();
        let mut storage_keys_by_address: HashMap<Address, Vec<(B256, U256)>> = HashMap::new();

        for (address, slots) in state_requests {
            account_keys.insert(keccak256(address), *address);
            let entry = storage_keys_by_address.entry(*address).or_default();
            for slot in slots {
                let hashed = keccak256(slot.to_be_bytes::<32>());
                entry.push((hashed, *slot));
            }
        }

        for (address, account) in &bundle_state.state {
            account_keys.entry(keccak256(address)).or_insert(*address);
            let entry = storage_keys_by_address.entry(*address).or_default();
            for (slot, _) in &account.storage {
                let hashed = keccak256(slot.to_be_bytes::<32>());
                // Avoid duplicates from state_requests
                if !entry.iter().any(|(h, _)| *h == hashed) {
                    entry.push((hashed, *slot));
                }
            }
        }

        Self { account_keys, storage_keys_by_address }
    }

    /// Find the original address by its keccak256 hash.
    pub(crate) fn find_account_by_hash(&self, hashed_address: &B256) -> Option<Address> {
        self.account_keys.get(hashed_address).copied()
    }

    /// Find an account address whose keccak256 hash starts with the given nibble prefix.
    pub(crate) fn find_account_key_by_prefix(&self, nibble_prefix: &[u8]) -> Option<Address> {
        self.account_keys
            .iter()
            .find(|(hash, _)| hash_matches_nibble_prefix(hash, nibble_prefix))
            .map(|(_, addr)| *addr)
    }

    /// Find a storage slot for the given address whose keccak256 hash starts with
    /// the given nibble prefix.
    pub(crate) fn find_storage_key_by_prefix(
        &self,
        address: &Address,
        nibble_prefix: &[u8],
    ) -> Option<U256> {
        let slots = self.storage_keys_by_address.get(address)?;
        slots
            .iter()
            .find(|(hash, _)| hash_matches_nibble_prefix(hash, nibble_prefix))
            .map(|(_, slot)| *slot)
    }
}

/// Check if a B256 hash, interpreted as nibbles, starts with the given nibble prefix.
fn hash_matches_nibble_prefix(hash: &B256, nibble_prefix: &[u8]) -> bool {
    let bytes = hash.as_slice();
    for (i, &nib) in nibble_prefix.iter().enumerate() {
        let byte_idx = i / 2;
        if byte_idx >= bytes.len() {
            return false;
        }
        let hash_nib = if i % 2 == 0 { bytes[byte_idx] >> 4 } else { bytes[byte_idx] & 0x0f };
        if hash_nib != nib {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::database::BundleState;

    #[test]
    fn test_build_from_state_requests() {
        let addr = Address::from([0x01; 20]);
        let slot = U256::from(42);

        let mut state_requests = revm_primitives::HashMap::default();
        state_requests.insert(addr, vec![slot]);

        let bundle = BundleState::default();
        let dict = PreimageDictionary::build(&state_requests, &bundle);

        let hashed_addr = keccak256(addr);
        assert_eq!(dict.find_account_by_hash(&hashed_addr), Some(addr));

        let hashed_slot = keccak256(slot.to_be_bytes::<32>());
        let nibbles: Vec<u8> =
            hashed_slot.as_slice().iter().flat_map(|b| [b >> 4, b & 0x0f]).take(4).collect();
        assert_eq!(dict.find_storage_key_by_prefix(&addr, &nibbles), Some(slot));
    }

    #[test]
    fn test_prefix_match() {
        let addr = Address::from([0xAB; 20]);
        let hashed = keccak256(addr);
        let nibbles: Vec<u8> =
            hashed.as_slice().iter().flat_map(|b| [b >> 4, b & 0x0f]).take(6).collect();

        let mut state_requests = revm_primitives::HashMap::default();
        state_requests.insert(addr, vec![]);

        let dict = PreimageDictionary::build(&state_requests, &BundleState::default());

        assert_eq!(dict.find_account_key_by_prefix(&nibbles), Some(addr));

        // Wrong prefix should not match
        let wrong_nibbles = vec![0x0F, 0x0F, 0x0F];
        assert_eq!(dict.find_account_key_by_prefix(&wrong_nibbles), None);
    }

    #[test]
    fn test_prefix_match_with_address_filter() {
        let addr1 = Address::from([0x01; 20]);
        let addr2 = Address::from([0x02; 20]);
        let slot1 = U256::from(100);
        let slot2 = U256::from(200);

        let mut state_requests = revm_primitives::HashMap::default();
        state_requests.insert(addr1, vec![slot1]);
        state_requests.insert(addr2, vec![slot2]);

        let dict = PreimageDictionary::build(&state_requests, &BundleState::default());

        // Find slot1 for addr1
        let hashed_slot1 = keccak256(slot1.to_be_bytes::<32>());
        let nibbles1: Vec<u8> =
            hashed_slot1.as_slice().iter().flat_map(|b| [b >> 4, b & 0x0f]).take(4).collect();
        assert_eq!(dict.find_storage_key_by_prefix(&addr1, &nibbles1), Some(slot1));

        // Should NOT find slot1 for addr2 (addr2 has slot2, not slot1)
        assert_eq!(dict.find_storage_key_by_prefix(&addr2, &nibbles1), None);

        // Find slot2 for addr2
        let hashed_slot2 = keccak256(slot2.to_be_bytes::<32>());
        let nibbles2: Vec<u8> =
            hashed_slot2.as_slice().iter().flat_map(|b| [b >> 4, b & 0x0f]).take(4).collect();
        assert_eq!(dict.find_storage_key_by_prefix(&addr2, &nibbles2), Some(slot2));

        // Non-existent address should return None
        let addr3 = Address::from([0x03; 20]);
        assert_eq!(dict.find_storage_key_by_prefix(&addr3, &nibbles1), None);
    }
}
