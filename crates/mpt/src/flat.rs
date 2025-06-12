//! A flat, zero-copy Merkle Patricia Trie representation.
//!
//! This module provides a data structure for representing an MPT that can be
//! deserialized with zero allocations on the guest. The host is responsible for
//! converting the traditional pointer-based `MptNode` representation into this
//! flat format. The guest can then perform read-only operations like `get` and `hash`
//! by doing cheap lookups into the flat index and blob slices.

use crate::mpt::{keccak, prefix_nibs, to_nibs, EMPTY_ROOT};
use alloy_primitives::B256;
use rlp::Rlp;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A single node in the flat MPT index.
///
/// This struct is designed to be compact and C-compatible for efficient packing.
#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlatNode {
    /// The type of the node.
    /// 0: Null, 1: Branch, 2: Leaf, 3: Extension, 4: Digest.
    pub kind: u8,
    /// For a branch, this is a bitmask where the i-th bit is set if the i-th child exists.
    /// For Leaf/Extension with digest reference, this is the index into prefixes array.
    /// For Leaf/Extension with inline reference, this is unused (set to 0).
    pub data: u16,
    /// The offset of the node's reference data within the `blob`.
    pub ref_offset: u32,
    /// The length of the node's reference data within the `blob`.
    pub ref_len: u16,
    /// For a Branch or Extension node, this is the index of its first child in the `index` slice.
    /// For a Leaf node, this is the index into leaf_values.
    pub child_idx: u32,
}

/// An owned, serializable representation of a flat MPT.
///
/// The host builds this structure from a traditional `MptNode` tree. It can then be
/// serialized and sent to the guest.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct FlatTrieOwned {
    pub index: Vec<FlatNode>,
    /// Stores node references
    pub blob: Vec<u8>,
    /// Stores the flat node indices of all branch children, concatenated.
    pub branch_children: Vec<u32>,
    /// Stores leaf values separately for efficient access
    pub leaf_values: Vec<Vec<u8>>,
    /// Stores prefix data only for leaf and extension nodes with digest references
    pub prefixes: Vec<Vec<u8>>,
}

mod flat_trie_owned_serde {
    use super::{FlatNode, FlatTrieOwned};
    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeTuple,
        Deserializer, Serializer,
    };
    use std::{fmt, mem};

    struct FlatTrieOwnedVisitor;

    impl<'de> Visitor<'de> for FlatTrieOwnedVisitor {
        type Value = FlatTrieOwned;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a FlatTrieOwned struct")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let index_bytes: Vec<u8> =
                seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

            if index_bytes.len() % mem::size_of::<FlatNode>() != 0 {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Bytes(&index_bytes),
                    &"a byte slice whose length is a multiple of FlatNode size",
                ));
            }
            let num_nodes = index_bytes.len() / mem::size_of::<FlatNode>();
            let mut index = Vec::with_capacity(num_nodes);
            unsafe {
                let ptr = index.as_mut_ptr();
                std::ptr::copy_nonoverlapping(
                    index_bytes.as_ptr() as *const FlatNode,
                    ptr,
                    num_nodes,
                );
                index.set_len(num_nodes);
            }

            let blob: Vec<u8> =
                seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
            let branch_children: Vec<u32> =
                seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
            let leaf_values: Vec<Vec<u8>> =
                seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
            let prefixes: Vec<Vec<u8>> =
                seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;

            Ok(FlatTrieOwned { index, blob, branch_children, leaf_values, prefixes })
        }
    }

    pub fn serialize<S>(trie: &FlatTrieOwned, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(5)?;
        let index_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                trie.index.as_ptr() as *const u8,
                trie.index.len() * mem::size_of::<FlatNode>(),
            )
        };
        state.serialize_element(index_bytes)?;
        state.serialize_element(&trie.blob)?;
        state.serialize_element(&trie.branch_children)?;
        state.serialize_element(&trie.leaf_values)?;
        state.serialize_element(&trie.prefixes)?;
        state.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FlatTrieOwned, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(5, FlatTrieOwnedVisitor)
    }
}

impl Serialize for FlatTrieOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        flat_trie_owned_serde::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for FlatTrieOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        flat_trie_owned_serde::deserialize(deserializer)
    }
}

/// A zero-copy view of a flat MPT.
///
/// This struct can be created from a `FlatTrieOwned` and is what the guest will
/// use to perform read operations. It borrows its data, making deserialization
/// extremely fast.
#[derive(Debug, Clone, Copy)]
pub struct FlatTrie<'a> {
    pub index: &'a [FlatNode],
    pub blob: &'a [u8],
    pub branch_children: &'a [u32],
    pub leaf_values: &'a [Vec<u8>],
    pub prefixes: &'a [Vec<u8>],
}

impl FlatTrieOwned {
    /// Creates a zero-copy view of the owned flat trie.
    pub fn view(&self) -> FlatTrie<'_> {
        FlatTrie {
            index: &self.index,
            blob: &self.blob,
            branch_children: &self.branch_children,
            leaf_values: &self.leaf_values,
            prefixes: &self.prefixes,
        }
    }
}

impl<'a> FlatTrie<'a> {
    /// Returns the hash of the trie's root node.
    pub fn hash(&self) -> B256 {
        if self.index.is_empty() {
            return EMPTY_ROOT;
        }
        // The root is the last node added in our post-order traversal.
        let root_node = self.index.last().unwrap();
        match root_node.kind {
            0 => EMPTY_ROOT, // Null
            4 => {
                // Digest node. The digest is stored directly in the blob.
                let ref_slice = &self.blob[root_node.ref_offset as usize
                    ..(root_node.ref_offset + root_node.ref_len as u32) as usize];
                B256::from_slice(ref_slice)
            }
            _ => {
                // For other nodes, hash the reference data
                let ref_slice = &self.blob[root_node.ref_offset as usize
                    ..(root_node.ref_offset + root_node.ref_len as u32) as usize];
                if ref_slice.len() == 32 {
                    // This is a digest reference
                    B256::from_slice(ref_slice)
                } else {
                    // This is inline bytes, hash them
                    B256::from(keccak(ref_slice))
                }
            }
        }
    }

    /// Retrieves the value associated with a given key.
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rlp::DecoderError> {
        if self.index.is_empty() {
            return Ok(None);
        }

        let nibbles = to_nibs(key);
        let mut nibbles_view = nibbles.as_slice();

        // The root is the last node added.
        let mut current_node_idx = self.index.len() - 1;

        loop {
            let node = &self.index[current_node_idx];
            match node.kind {
                0 => return Ok(None), // Null
                1 => {
                    // Branch
                    if let Some((head, tail)) = nibbles_view.split_first() {
                        nibbles_view = tail;

                        let mask = node.data;
                        if (mask & (1u16 << head)) == 0 {
                            return Ok(None); // No child for this nibble
                        }

                        // Find the index of the child in the branch_children array.
                        let child_number_in_list = (mask & ((1u16 << head) - 1)).count_ones();
                        current_node_idx = self.branch_children
                            [node.child_idx as usize + child_number_in_list as usize]
                            as usize;
                    } else {
                        return Ok(None); // Empty key remaining for branch
                    }
                }
                2 => {
                    // Leaf - handle both inline RLP and digest references
                    let ref_slice = &self.blob[node.ref_offset as usize
                        ..(node.ref_offset + node.ref_len as u32) as usize];

                    let leaf_nibbles = if ref_slice.len() == 32 {
                        // Digest reference - get prefix from prefixes array
                        let prefix_idx = node.data as usize;
                        let prefix = &self.prefixes[prefix_idx];
                        prefix_nibs(prefix)
                    } else {
                        // Inline RLP - parse to get prefix
                        let rlp = Rlp::new(ref_slice);
                        let prefix_bytes: Vec<u8> = rlp.val_at(0)?;
                        prefix_nibs(&prefix_bytes)
                    };

                    if leaf_nibbles.as_slice() == nibbles_view {
                        // Get the value from leaf_values using the child_idx as index
                        let value_idx = node.child_idx as usize;
                        if value_idx < self.leaf_values.len() {
                            return Ok(Some(self.leaf_values[value_idx].clone()));
                        }
                    }
                    return Ok(None);
                }
                3 => {
                    // Extension - handle both inline RLP and digest references
                    let ref_slice = &self.blob[node.ref_offset as usize
                        ..(node.ref_offset + node.ref_len as u32) as usize];

                    let ext_nibbles = if ref_slice.len() == 32 {
                        // Digest reference - get prefix from prefixes array
                        let prefix_idx = node.data as usize;
                        let prefix = &self.prefixes[prefix_idx];
                        prefix_nibs(prefix)
                    } else {
                        // Inline RLP - parse to get prefix
                        let rlp = Rlp::new(ref_slice);
                        let prefix_bytes: Vec<u8> = rlp.val_at(0)?;
                        prefix_nibs(&prefix_bytes)
                    };

                    if let Some(remaining_nibbles) =
                        nibbles_view.strip_prefix(ext_nibbles.as_slice())
                    {
                        nibbles_view = remaining_nibbles;
                        current_node_idx = node.child_idx as usize;
                    } else {
                        return Ok(None);
                    }
                }
                4 => return Ok(None), // Should not encounter a digest during traversal of a resolved trie.
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_serde() {
        let mut flat_trie = FlatTrieOwned::default();
        flat_trie.index.push(FlatNode {
            kind: 1,
            data: 42,
            ref_offset: 0,
            ref_len: 10,
            child_idx: 0,
        });
        flat_trie.blob = vec![1, 2, 3, 4, 5];
        flat_trie.branch_children = vec![10, 20, 30];
        flat_trie.leaf_values = vec![vec![1, 2, 3]];
        flat_trie.prefixes = vec![vec![4, 5, 6]];

        // Test with bincode
        let serialized =
            bincode::serde::encode_to_vec(&flat_trie, bincode::config::standard()).unwrap();
        let deserialized: FlatTrieOwned =
            bincode::serde::decode_from_slice(&serialized, bincode::config::standard()).unwrap().0;

        assert_eq!(flat_trie, deserialized);
    }
}
