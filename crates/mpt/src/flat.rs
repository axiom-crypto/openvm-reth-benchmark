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
use serde::{Deserialize, Serialize};

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
    /// For Leaf/Extension, this stores the length of the prefix nibbles.
    pub data: u16,
    /// The offset of the node's RLP data within the `blob`.
    pub rlp_offset: u32,
    /// The length of the node's RLP data within the `blob`.
    pub rlp_len: u16,
    /// For a Branch or Extension node, this is the index of its first child in the `index` slice.
    /// For a Branch, its children are stored contiguously from this index.
    pub child_idx: u32,
}

/// An owned, serializable representation of a flat MPT.
///
/// The host builds this structure from a traditional `MptNode` tree. It can then be
/// serialized and sent to the guest.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FlatTrieOwned {
    pub index: Vec<FlatNode>,
    pub blob: Vec<u8>,
    /// Stores the flat node indices of all branch children, concatenated.
    pub branch_children: Vec<u32>,
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
}

impl FlatTrieOwned {
    /// Creates a zero-copy view of the owned flat trie.
    pub fn view(&self) -> FlatTrie<'_> {
        FlatTrie { index: &self.index, blob: &self.blob, branch_children: &self.branch_children }
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
                // Digest node. The value is stored directly in the RLP blob.
                let rlp_slice = &self.blob[root_node.rlp_offset as usize
                    ..(root_node.rlp_offset + root_node.rlp_len as u32) as usize];
                let rlp = Rlp::new(rlp_slice);
                let bytes: Vec<u8> = rlp.as_val().unwrap();
                B256::from_slice(&bytes)
            }
            _ => {
                let rlp_slice = &self.blob[root_node.rlp_offset as usize
                    ..(root_node.rlp_offset + root_node.rlp_len as u32) as usize];
                B256::from(keccak(rlp_slice))
            }
        }
    }

    /// Retrieves the value associated with a given key.
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rlp::DecoderError> {
        if self.index.is_empty() {
            return Ok(None);
        }

        let mut nibbles = to_nibs(key);
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
                    // Leaf
                    let rlp_slice = &self.blob[node.rlp_offset as usize
                        ..(node.rlp_offset + node.rlp_len as u32) as usize];
                    let rlp = Rlp::new(rlp_slice);
                    let prefix_bytes: Vec<u8> = rlp.val_at(0)?;
                    let leaf_nibbles = prefix_nibs(&prefix_bytes);

                    if leaf_nibbles.as_slice() == nibbles_view {
                        let value: Vec<u8> = rlp.val_at(1)?;
                        return Ok(Some(value));
                    } else {
                        return Ok(None);
                    }
                }
                3 => {
                    // Extension
                    let rlp_slice = &self.blob[node.rlp_offset as usize
                        ..(node.rlp_offset + node.rlp_len as u32) as usize];
                    let rlp = Rlp::new(rlp_slice);
                    let prefix_bytes: Vec<u8> = rlp.val_at(0)?;
                    let ext_nibbles = prefix_nibs(&prefix_bytes);

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
