use alloy_rlp::{Buf, Encodable};
use bumpalo::Bump;
use core::fmt::Debug;
use revm::primitives::B256;
use revm_primitives::{b256, keccak256};
use serde::{de, ser, Deserialize, Serialize};
use std::{cell::RefCell, iter, rc::Rc};

use eyre::Result;

use crate::word_bytes::OptimizedBytes;
use smallvec::SmallVec;
use thiserror::Error as ThisError;

pub type NodeId = u32;

/// Root hash of an empty trie.
pub const EMPTY_ROOT: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

/// Represents the ways in which one node can reference another node inside the sparse Merkle
/// Patricia Trie (MPT).
///
/// Nodes in the MPT can reference other nodes either directly through their byte representation or
/// indirectly through a hash of their encoding.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum MptNodeReference {
    /// Represents a direct reference to another node using its byte encoding. Typically
    /// used for short encodings that are less than 32 bytes in length.
    Bytes(Vec<u8>),
    /// Represents an indirect reference to another node using the Keccak hash of its long
    /// encoding. Used for encodings that are not less than 32 bytes in length.
    Digest(B256),
}

/// Custom error types for the sparse Merkle Patricia Trie (MPT).
#[derive(Debug, ThisError)]
pub enum Error {
    /// Triggered when an operation reaches an unresolved node. The associated `B256`
    /// value provides details about the unresolved node.
    #[error("reached an unresolved node: {0:#}")]
    NodeNotResolved(B256),
    /// Occurs when a value is unexpectedly found in a branch node.
    #[error("branch node with value")]
    ValueInBranch,
    /// Represents errors related to the RLP encoding and decoding .
    #[error("RLP error")]
    Rlp(#[from] alloy_rlp::Error),
}

/// Returns the length of the common prefix
pub fn lcp(a: &[u8], b: &[u8]) -> usize {
    for (i, (a, b)) in iter::zip(a, b).enumerate() {
        if a != b {
            return i;
        }
    }
    std::cmp::min(a.len(), b.len())
}

/// Converts a byte slice into a vector of nibbles.
///
/// A nibble is 4 bits or half of an 8-bit byte. This function takes each byte from the
/// input slice, splits it into two nibbles, and appends them to the resulting vector.
/// Uses SmallVec to avoid heap allocation for typical key sizes (≤32 bytes = 64 nibbles).
pub fn to_nibs(slice: &[u8]) -> SmallVec<[u8; 64]> {
    let mut result = SmallVec::with_capacity(2 * slice.len());
    for byte in slice {
        result.push(byte >> 4);
        result.push(byte & 0xf);
    }
    result
}

/// Optimized conversion from hex-encoded path directly to nibbles SmallVec
fn prefix_to_small_nibs(encoded_path: &[u8]) -> SmallVec<[u8; 64]> {
    if encoded_path.is_empty() {
        return SmallVec::new();
    }

    let mut nibs = SmallVec::with_capacity(encoded_path.len() * 2);
    let first_byte = encoded_path[0];
    let is_odd = (first_byte & 0x10) != 0;

    // Handle the first nibble if odd length
    if is_odd {
        nibs.push(first_byte & 0x0f);
    }

    // Process remaining bytes, starting from index 1
    for &byte in &encoded_path[1..] {
        nibs.push(byte >> 4); // High nibble
        nibs.push(byte & 0x0f); // Low nibble
    }

    nibs
}

/// Arena-based implementation that stores all nodes in a flat vector
/// and uses indices for better memory layout and performance.
/// The lifetime parameter 'a allows zero-copy deserialization by borrowing from the input buffer.
#[derive(Clone, Debug)]
pub struct ArenaBasedMptNode<'a> {
    nodes: Vec<ArenaNodeData<'a>>,
    cached_references: Vec<RefCell<Option<MptNodeReference>>>,
    root_id: NodeId,
    /// One monotonically‑growing arena that owns every mutated byte slice.
    bump: Rc<Bump>,
    encoding_scratch: RefCell<Vec<u8>>,
    dirty_nodes: RefCell<Vec<NodeId>>,
}

impl ser::Serialize for ArenaBasedMptNode<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // Serialize as a tuple of (node_count, rlp_blob) for efficient deserialization.
        (self.nodes.len(), OptimizedBytes(self.to_full_rlp())).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for ArenaBasedMptNode<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (num_nodes, OptimizedBytes(rlp_blob)) =
            <(usize, OptimizedBytes)>::deserialize(deserializer)?;
        // We need to leak the memory to get a 'de lifetime - this is a limitation of serde
        let leaked_bytes: &'de [u8] = Box::leak(rlp_blob.into_boxed_slice());
        ArenaBasedMptNode::decode_from_rlp(leaked_bytes, num_nodes).map_err(de::Error::custom)
    }
}

impl Default for ArenaBasedMptNode<'_> {
    fn default() -> Self {
        Self {
            nodes: vec![ArenaNodeData::Null],
            cached_references: vec![RefCell::new(None)],
            root_id: 0,
            bump: Rc::new(Bump::new()),
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
            dirty_nodes: RefCell::new(Vec::new()),
        }
    }
}

/// An optimized representation for the 16 children of a branch node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C)]
pub struct BranchChildren {
    /// A bitmask where the i-th bit is set if a child is present at that index.
    mask: u16,
    /// An array of node IDs. The value is only valid if the corresponding bit in the mask is set.
    slots: [NodeId; 16],
}

impl Default for BranchChildren {
    fn default() -> Self {
        Self { mask: 0, slots: [0; 16] }
    }
}

impl BranchChildren {
    /// Returns the child node ID at a given index if it exists.
    #[inline]
    pub fn child(&self, index: u8) -> Option<NodeId> {
        if (self.mask >> index) & 1 == 1 {
            // SAFETY: The mask check ensures this memory access is valid.
            Some(unsafe { *self.slots.get_unchecked(index as usize) })
        } else {
            None
        }
    }

    /// Sets or removes a child at a given index.
    #[inline]
    pub fn set_child(&mut self, index: u8, node_id: Option<NodeId>) {
        if let Some(id) = node_id {
            self.mask |= 1 << index;
            self.slots[index as usize] = id;
        } else {
            self.mask &= !(1 << index);
        }
    }

    /// Returns an iterator over the existing children, yielding their index and node ID.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (usize, NodeId)> + '_ {
        let mut mask = self.mask;
        std::iter::from_fn(move || {
            if mask == 0 {
                return None;
            }
            let index = mask.trailing_zeros() as usize;
            mask &= mask - 1; // Clear the lowest set bit

            // SAFETY: The mask guarantees the index is valid and the slot is initialized.
            Some((index, unsafe { *self.slots.get_unchecked(index) }))
        })
    }

    /// Returns an iterator over all 16 potential child slots.
    #[inline]
    pub fn iter_all(&self) -> impl Iterator<Item = Option<NodeId>> + '_ {
        (0..16).map(|i| self.child(i))
    }
}

/// Node data for arena-based trie with zero-copy optimization
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub enum ArenaNodeData<'a> {
    #[default]
    Null,
    Branch(BranchChildren),
    Leaf(&'a [u8], &'a [u8]), // (compact_path, value) - borrowed from input
    Extension(&'a [u8], NodeId), // (compact_path, child_id) - borrowed from input
    Digest(B256),
}

// Constructors & Capacity Management
impl<'a> ArenaBasedMptNode<'a> {
    /// Creates a new arena with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap.max(1));
        let mut cached_references = Vec::with_capacity(cap.max(1));
        nodes.push(ArenaNodeData::Null);
        cached_references.push(RefCell::new(None));

        Self {
            nodes,
            cached_references,
            root_id: 0,
            bump: Rc::new(Bump::new()),
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
            dirty_nodes: RefCell::new(Vec::new()),
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
        self.cached_references.reserve(additional);
    }

    /// Decodes an RLP-encoded node directly into an ArenaBasedMptNode with zero-copy optimization
    pub fn decode_from_rlp(bytes: &'a [u8], num_nodes: usize) -> Result<Self, Error> {
        // A growth factor applied to the node vector's capacity during deserialization.
        // This is a pragmatic optimization to pre-allocate a buffer for nodes that will be
        // added during the `update` phase. It prevents a "reallocation storm" where the
        // main trie and dozens of storage tries all try to reallocate their full node
        // vectors on the first update.
        // TODO: this is imperfect solution and the constant is somewhat arbitrary (although
        // reasonable)       Simple improvement: run benchmark on a set of blocks (e.g. 100
        // blocks) and select the best constant.       More advanced improvement: either
        // pre-execute block at guest to know exact allocations in advance,
        //       or allocate a separate arena specifically for updates.
        const VEC_CAPACITY_GROWTH_FACTOR: f64 = 1.11;
        let capacity = (num_nodes as f64 * VEC_CAPACITY_GROWTH_FACTOR) as usize + 1;
        let mut arena = ArenaBasedMptNode::with_capacity(capacity);

        let mut buf = bytes;
        let root_id = arena.decode_node_recursive(&mut buf)?;
        if !buf.is_empty() {
            return Err(Error::Rlp(alloy_rlp::Error::Custom("trailing data")));
        }
        arena.root_id = root_id;
        Ok(arena)
    }
}

// Public API
impl<'a> ArenaBasedMptNode<'a> {
    /// Computes and returns the 256-bit hash of the node.
    #[inline]
    pub fn hash(&self) -> B256 {
        self.hash_id(self.root_id)
    }

    /// Retrieves the value associated with a given key in the trie.
    #[inline]
    pub fn get<'s>(&'s self, key: &[u8]) -> Result<Option<&'a [u8]>, Error> {
        self.get_recursive(self.root_id, &to_nibs(key))
    }

    /// Retrieves the RLP-decoded value corresponding to the key.
    #[inline]
    pub fn get_rlp<T: alloy_rlp::Decodable>(&self, key: &[u8]) -> Result<Option<T>, Error> {
        match self.get(key)? {
            Some(bytes) => {
                let mut slice = bytes;
                Ok(Some(T::decode(&mut slice)?))
            }
            None => Ok(None),
        }
    }

    /// Inserts a key-value pair into the trie.
    #[inline]
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<bool, Error> {
        let key_nibs = &to_nibs(key);
        self.insert_recursive(self.root_id, key_nibs, value)
    }

    /// Inserts an RLP-encoded value into the trie.
    #[inline]
    pub fn insert_rlp(&mut self, key: &[u8], value: impl Encodable) -> Result<bool, Error> {
        let mut rlp_bytes = Vec::new();
        value.encode(&mut rlp_bytes);
        self.insert(key, &rlp_bytes)
    }

    /// Inserts an RLP-encoded value into the trie, reusing a buffer for encoding.
    #[inline]
    pub fn insert_rlp_with_buf(
        &mut self,
        key: &[u8],
        value: impl Encodable,
        buf: &mut Vec<u8>,
    ) -> Result<bool, Error> {
        buf.clear();
        value.encode(buf);
        self.insert(key, buf)
    }

    /// Removes a key from the trie.
    ///
    /// This method attempts to remove a key-value pair from the trie. If the key is
    /// present, it returns `true`. Otherwise, it returns `false`.
    #[inline]
    pub fn delete(&mut self, key: &[u8]) -> Result<bool, Error> {
        let key_nibs = &to_nibs(key);
        self.delete_recursive(self.root_id, key_nibs)
    }

    /// Clears the trie, replacing its data with an empty node.
    /// Old `clear()` – keep the old arena for anyone still sharing it,
    /// switch `self` to a fresh one.
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

// Internal Implementation
impl<'a> ArenaBasedMptNode<'a> {
    fn get_recursive<'s>(
        &'s self,
        node_id: NodeId,
        key_nibs: &[u8],
    ) -> Result<Option<&'a [u8]>, Error> {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => Ok(None),
            ArenaNodeData::Branch(children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match children.child(*i) {
                        Some(id) => self.get_recursive(id, tail),
                        None => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Leaf(path_bytes, value) => {
                // Convert compact path to nibbles on-demand
                let path_nibs = prefix_to_small_nibs(path_bytes);
                if path_nibs.as_slice() == key_nibs {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Extension(path_bytes, child_id) => {
                // Convert compact path to nibbles on-demand
                let path_nibs = prefix_to_small_nibs(path_bytes);
                if let Some(tail) = key_nibs.strip_prefix(path_nibs.as_slice()) {
                    self.get_recursive(*child_id, tail)
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(*digest)),
        }
    }

    fn insert_recursive(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
        value: &[u8],
    ) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            ArenaNodeData::Null => {
                let path_slice = self.add_encoded_path_slice(key_nibs, true);
                let value_slice = self.alloc_in_bump(value);
                self.nodes[node_id as usize] = ArenaNodeData::Leaf(path_slice, value_slice);
                Ok(true)
            }
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let updated = match children.child(*i) {
                        Some(id) => self.insert_recursive(id, tail, value)?,
                        None => {
                            let path_slice = self.add_encoded_path_slice(tail, true);
                            let value_slice = self.alloc_in_bump(value);
                            let new_leaf_id =
                                self.add_node(ArenaNodeData::Leaf(path_slice, value_slice));
                            children.set_child(*i, Some(new_leaf_id));
                            self.nodes[node_id as usize] = ArenaNodeData::Branch(children);
                            true
                        }
                    };
                    Ok(updated)
                } else {
                    Err(Error::ValueInBranch)
                }
            }
            ArenaNodeData::Leaf(path_bytes, old_value) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                let common_len = lcp(&path_nibs, key_nibs);

                if common_len == path_nibs.len() && common_len == key_nibs.len() {
                    if old_value == value {
                        return Ok(false);
                    }
                    let value_slice = self.alloc_in_bump(value);
                    self.nodes[node_id as usize] = ArenaNodeData::Leaf(path_bytes, value_slice);
                    Ok(true)
                } else if common_len == path_nibs.len() || common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    let mut children: BranchChildren = Default::default();

                    let leaf1_path_slice =
                        self.add_encoded_path_slice(&path_nibs[split_point..], true);
                    let leaf1_value_slice = self.alloc_in_bump(old_value);
                    let leaf1_id =
                        self.add_node(ArenaNodeData::Leaf(leaf1_path_slice, leaf1_value_slice));

                    let leaf2_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf2_value_slice = self.alloc_in_bump(value);
                    let leaf2_id =
                        self.add_node(ArenaNodeData::Leaf(leaf2_path_slice, leaf2_value_slice));

                    children.set_child(path_nibs[common_len], Some(leaf1_id));
                    children.set_child(key_nibs[common_len], Some(leaf2_id));

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        let ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[..common_len], false);
                        ArenaNodeData::Extension(ext_path_slice, branch_id)
                    } else {
                        ArenaNodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    Ok(true)
                }
            }
            ArenaNodeData::Extension(path_bytes, child_id) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                let common_len = lcp(&path_nibs, key_nibs);

                if common_len == path_nibs.len() {
                    self.insert_recursive(child_id, &key_nibs[common_len..], value)
                } else if common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    let mut children: BranchChildren = Default::default();

                    if split_point < path_nibs.len() {
                        let ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[split_point..], false);
                        let ext_id =
                            self.add_node(ArenaNodeData::Extension(ext_path_slice, child_id));
                        children.set_child(path_nibs[common_len], Some(ext_id));
                    } else {
                        children.set_child(path_nibs[common_len], Some(child_id));
                    }

                    let leaf_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf_value_slice = self.alloc_in_bump(value);
                    let leaf_id =
                        self.add_node(ArenaNodeData::Leaf(leaf_path_slice, leaf_value_slice));
                    children.set_child(key_nibs[common_len], Some(leaf_id));

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        let parent_ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[..common_len], false);
                        ArenaNodeData::Extension(parent_ext_path_slice, branch_id)
                    } else {
                        ArenaNodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    Ok(true)
                }
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }?;

        if updated {
            self.invalidate_ref_cache(node_id);
        }

        Ok(updated)
    }

    fn delete_recursive(&mut self, node_id: NodeId, key_nibs: &[u8]) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            ArenaNodeData::Null => Ok(false),
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children.child(*i);
                    match child_id {
                        Some(id) => {
                            if !self.delete_recursive(id, tail)? {
                                return Ok(false);
                            }

                            if matches!(self.nodes[id as usize], ArenaNodeData::Null) {
                                children.set_child(*i, None);
                            }
                        }
                        None => return Ok(false),
                    }
                } else {
                    return Err(Error::ValueInBranch);
                }

                let mut remaining_iter = children.iter();

                if let Some(first_remaining) = remaining_iter.next() {
                    // One child found, check if there are more.
                    if remaining_iter.next().is_none() {
                        // Exactly one child remains, collapse the branch node.
                        let (index, child_id) = first_remaining;
                        let child_node_data = self.nodes[child_id as usize].clone();

                        let new_node_data = match child_node_data {
                            ArenaNodeData::Leaf(path_bytes, value) => {
                                let path_nibs = prefix_to_small_nibs(path_bytes);
                                let mut new_nibs: SmallVec<[u8; 64]> =
                                    SmallVec::with_capacity(1 + path_nibs.len());
                                new_nibs.push(index as u8);
                                new_nibs.extend_from_slice(&path_nibs);
                                let new_path_slice = self.add_encoded_path_slice(&new_nibs, true);
                                let new_value_slice = self.alloc_in_bump(value);
                                ArenaNodeData::Leaf(new_path_slice, new_value_slice)
                            }
                            ArenaNodeData::Extension(path_bytes, child_child_id) => {
                                let path_nibs = prefix_to_small_nibs(path_bytes);
                                let mut new_nibs: SmallVec<[u8; 64]> =
                                    SmallVec::with_capacity(1 + path_nibs.len());
                                new_nibs.push(index as u8);
                                new_nibs.extend_from_slice(&path_nibs);
                                let new_path_slice = self.add_encoded_path_slice(&new_nibs, false);
                                ArenaNodeData::Extension(new_path_slice, child_child_id)
                            }
                            ArenaNodeData::Branch(_) | ArenaNodeData::Digest(_) => {
                                let ext_nibs: SmallVec<[u8; 1]> =
                                    SmallVec::from_slice(&[index as u8]);
                                let new_path_slice = self.add_encoded_path_slice(&ext_nibs, false);
                                ArenaNodeData::Extension(new_path_slice, child_id)
                            }
                            ArenaNodeData::Null => unreachable!(),
                        };
                        self.nodes[node_id as usize] = new_node_data;
                    } else {
                        // More than one child remains, just update the branch node.
                        self.nodes[node_id as usize] = ArenaNodeData::Branch(children);
                    }
                } else {
                    // No children left, update to an empty branch node.
                    self.nodes[node_id as usize] = ArenaNodeData::Branch(children);
                }

                Ok(true)
            }
            ArenaNodeData::Leaf(path_bytes, _) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                if path_nibs.as_slice() != key_nibs {
                    return Ok(false);
                }
                self.nodes[node_id as usize] = ArenaNodeData::Null;
                Ok(true)
            }
            ArenaNodeData::Extension(path_bytes, child_id) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                if let Some(tail) = key_nibs.strip_prefix(path_nibs.as_slice()) {
                    if !self.delete_recursive(child_id, tail)? {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                };

                let child_node_data = self.nodes[child_id as usize].clone();
                let new_node_data = match child_node_data {
                    ArenaNodeData::Null => ArenaNodeData::Null,
                    ArenaNodeData::Leaf(child_path_bytes, value) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path_slice = self.add_encoded_path_slice(&combined_nibs, true);
                        let new_value_slice = self.alloc_in_bump(value);
                        ArenaNodeData::Leaf(new_path_slice, new_value_slice)
                    }
                    ArenaNodeData::Extension(child_path_bytes, grandchild_id) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path_slice = self.add_encoded_path_slice(&combined_nibs, false);
                        ArenaNodeData::Extension(new_path_slice, grandchild_id)
                    }
                    ArenaNodeData::Branch(_) | ArenaNodeData::Digest(_) => {
                        ArenaNodeData::Extension(path_bytes, child_id)
                    }
                };
                self.nodes[node_id as usize] = new_node_data;
                Ok(true)
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }?;

        if updated {
            self.invalidate_ref_cache(node_id);
        }
        Ok(updated)
    }

    fn decode_node_recursive(&mut self, buf: &mut &'a [u8]) -> Result<NodeId, Error> {
        if buf.is_empty() {
            return Ok(0); // Return the null node index
        }

        let header = alloy_rlp::Header::decode(buf).map_err(Error::Rlp)?;

        if !header.list {
            // Single data item
            if header.payload_length == 0 {
                return Ok(0); // Null node
            }
            if header.payload_length == 32 {
                if buf.len() < 32 {
                    return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
                }
                let digest = B256::from_slice(&buf[..32]);
                buf.advance(32);
                return Ok(self.add_node(ArenaNodeData::Digest(digest)));
            }
            return Err(Error::Rlp(alloy_rlp::Error::Custom("invalid string node")));
        }

        // Extract the list payload - zero-copy slice
        let payload = &buf[..header.payload_length];
        buf.advance(header.payload_length);
        let mut payload_buf = payload;

        // Probe the first two items to determine if this is a 2-item or 17-item node
        // without scanning the entire payload
        let mut probe_buf = payload_buf;

        // Try to parse the first item from the probe
        let h1_start_ptr = probe_buf.as_ptr();
        let h1 = alloy_rlp::Header::decode(&mut probe_buf).map_err(Error::Rlp)?;
        let h1_header_len = probe_buf.as_ptr() as usize - h1_start_ptr as usize;
        if probe_buf.len() < h1.payload_length {
            return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
        }
        probe_buf.advance(h1.payload_length);

        if probe_buf.is_empty() {
            return Err(Error::Rlp(alloy_rlp::Error::Custom("invalid 1-item list node")));
        }

        // Try to parse the second item
        let h2_start_ptr = probe_buf.as_ptr();
        let h2 = alloy_rlp::Header::decode(&mut probe_buf).map_err(Error::Rlp)?;
        let h2_header_len = probe_buf.as_ptr() as usize - h2_start_ptr as usize;
        if probe_buf.len() < h2.payload_length {
            return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
        }
        probe_buf.advance(h2.payload_length);

        // If the probe buffer is now empty, it was a 2-item node
        // Otherwise, it's a 17-item branch node
        let is_branch = !probe_buf.is_empty();

        if is_branch {
            // Branch node (17 items)
            let mut children = BranchChildren::default();
            for i in 0..16 {
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                if child_id != 0 {
                    children.set_child(i, Some(child_id));
                }
            }

            // Skip the final value (should be empty for MPT) - avoid allocation
            let value_header = alloy_rlp::Header::decode(&mut payload_buf).map_err(Error::Rlp)?;
            if value_header.list {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for value")));
            }
            if value_header.payload_length != 0 {
                return Err(Error::ValueInBranch);
            }
            payload_buf.advance(value_header.payload_length);

            Ok(self.add_node(ArenaNodeData::Branch(children)))
        } else {
            // Leaf or Extension node (2 items)
            // Reuse headers from the probe to avoid re-parsing.

            // 1. Path
            let path_header = h1;
            if path_header.list {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for path")));
            }
            payload_buf.advance(h1_header_len);
            if payload_buf.len() < path_header.payload_length {
                return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
            }
            let path_slice = &payload_buf[..path_header.payload_length];
            payload_buf.advance(path_header.payload_length);

            if path_slice.is_empty() {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("empty path")));
            }

            let prefix = path_slice[0];
            if (prefix & 0x20) != 0 {
                // Leaf node (prefix 0x20 or 0x21)

                // 2. Value
                let value_header = h2;
                if value_header.list {
                    return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for value")));
                }
                payload_buf.advance(h2_header_len);
                if payload_buf.len() < value_header.payload_length {
                    return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
                }
                let value_slice = &payload_buf[..value_header.payload_length];
                payload_buf.advance(value_header.payload_length);

                Ok(self.add_node(ArenaNodeData::Leaf(path_slice, value_slice)))
            } else {
                // Extension node (prefix 0x00 or 0x01)
                // The second item is a child node, which we must parse recursively.
                // We cannot reuse h2 because decode_node_recursive needs to see the full RLP.
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                Ok(self.add_node(ArenaNodeData::Extension(path_slice, child_id)))
            }
        }
    }

    fn add_node(&mut self, data: ArenaNodeData<'a>) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(data);
        self.cached_references.push(RefCell::new(None));
        id
    }

    /// Encodes nibbles into the standard hex-prefix format directly into the bump arena.
    fn add_encoded_path_slice(&mut self, nibs: &[u8], is_leaf: bool) -> &'a [u8] {
        let is_odd = nibs.len() % 2 != 0;
        // Max path is 64 nibs (32 bytes) + 1 prefix byte = 33 bytes.
        // SmallVec will keep this on the stack, avoiding heap allocations.
        let mut encoded = SmallVec::<[u8; 33]>::new();

        let mut prefix = if is_leaf { 0x20 } else { 0x00 };
        if is_odd {
            prefix |= 0x10;
            encoded.push(prefix | nibs[0]);
            for i in (1..nibs.len()).step_by(2) {
                encoded.push((nibs[i] << 4) | nibs[i + 1]);
            }
        } else {
            encoded.push(prefix);
            for i in (0..nibs.len()).step_by(2) {
                encoded.push((nibs[i] << 4) | nibs[i + 1]);
            }
        }
        self.alloc_in_bump(&encoded)
    }

    /// Copies `bytes` into the bump arena and returns a `'a` slice.
    #[inline]
    fn alloc_in_bump(&mut self, bytes: &[u8]) -> &'a [u8] {
        let slice = self.bump.alloc_slice_copy(bytes);
        // Sound because `slice` lives as long as `self.bump`.
        unsafe { std::mem::transmute::<&[u8], &'a [u8]>(slice) }
    }

    #[inline]
    fn invalidate_ref_cache(&mut self, node_id: NodeId) {
        self.cached_references[node_id as usize].borrow_mut().take();
    }

    fn hash_id(&self, node_id: NodeId) -> B256 {
        match self.nodes[node_id as usize] {
            ArenaNodeData::Null => EMPTY_ROOT,
            _ => {
                match self.cached_references[node_id as usize]
                    .borrow_mut()
                    .get_or_insert_with(|| self.calc_reference(node_id))
                {
                    MptNodeReference::Digest(digest) => *digest,
                    MptNodeReference::Bytes(bytes) => keccak256(bytes),
                }
            }
        }
    }

    fn calc_reference(&self, node_id: NodeId) -> MptNodeReference {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => MptNodeReference::Bytes(vec![alloy_rlp::EMPTY_STRING_CODE]),
            ArenaNodeData::Digest(digest) => MptNodeReference::Digest(*digest),
            _ => {
                let payload_length = self.payload_length_id(node_id);
                let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

                if rlp_length < 32 {
                    let mut encoded = Vec::with_capacity(rlp_length);
                    self.encode_id_with_payload_len(node_id, payload_length, &mut encoded);
                    debug_assert_eq!(encoded.len(), rlp_length);
                    MptNodeReference::Bytes(encoded)
                } else {
                    let mut scratch = self.encoding_scratch.borrow_mut();
                    scratch.clear();
                    scratch.reserve(rlp_length);
                    self.encode_id_with_payload_len(node_id, payload_length, &mut *scratch);
                    debug_assert_eq!(scratch.len(), rlp_length);
                    MptNodeReference::Digest(keccak256(&*scratch))
                }
            }
        }
    }

    fn encode_id_with_payload_len(
        &self,
        node_id: NodeId,
        payload_length: usize,
        out: &mut dyn alloy_rlp::BufMut,
    ) {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Branch(children) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                for child_id in children.iter_all() {
                    match child_id {
                        Some(id) => self.reference_encode_id(id, out),
                        None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                    }
                }
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Leaf(encoded_path, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                value.encode(out);
            }
            ArenaNodeData::Extension(encoded_path, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                self.reference_encode_id(*child_id, out);
            }
            ArenaNodeData::Digest(digest) => {
                digest.encode(out);
            }
        }
    }

    #[inline]
    fn reference_encode_id(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        match self.cached_references[node_id as usize]
            .borrow_mut()
            .get_or_insert_with(|| self.calc_reference(node_id))
        {
            // if the reference is an RLP-encoded byte slice, copy it directly
            MptNodeReference::Bytes(bytes) => out.put_slice(bytes),
            // if the reference is a digest, RLP-encode it with its fixed known length
            MptNodeReference::Digest(digest) => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE + 32);
                out.put_slice(digest.as_slice());
            }
        }
    }

    fn payload_length_id(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(children) => {
                1 + children
                    .iter_all()
                    .map(|child| child.map_or(1, |id| self.reference_length(id)))
                    .sum::<usize>()
            }
            ArenaNodeData::Leaf(encoded_path, value) => encoded_path.length() + value.length(),
            ArenaNodeData::Extension(encoded_path, node_id) => {
                encoded_path.length() + self.reference_length(*node_id)
            }
            ArenaNodeData::Digest(_) => 32,
        }
    }

    #[inline]
    fn reference_length(&self, node_id: NodeId) -> usize {
        match self.cached_references[node_id as usize]
            .borrow_mut()
            .get_or_insert_with(|| self.calc_reference(node_id))
        {
            MptNodeReference::Bytes(bytes) => bytes.len(),
            MptNodeReference::Digest(_) => 1 + 32,
        }
    }
}

// Serialization. Not performance critical.
impl ArenaBasedMptNode<'_> {
    /// Returns the RLP-encoded bytes with ALL children inlined (never replaced by digest).
    /// This produces a compact, fully-expanded representation perfect for serialization.
    #[inline]
    pub fn to_full_rlp(&self) -> Vec<u8> {
        // Rough estimate: each node ~100 bytes average, plus some overhead
        let mut out = Vec::with_capacity(self.nodes.len() * 100);
        self.encode_full(self.root_id, &mut out);
        out
    }

    /// Encodes a node with ALL children inlined (never using digest references).
    /// Produces the fully-expanded RLP representation.
    fn encode_full(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.payload_length_full(node_id);
        self.encode_full_with_payload_len(node_id, payload_length, out);
    }

    /// Calculates payload length for full inline encoding (never using digest references)
    fn payload_length_full(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(children) => {
                1 + children
                    .iter_all()
                    .map(|child| child.map_or(1, |id| self.full_length(id)))
                    .sum::<usize>()
            }
            ArenaNodeData::Leaf(encoded_path, value) => encoded_path.length() + value.length(),
            ArenaNodeData::Extension(encoded_path, node_id) => {
                encoded_path.length() + self.full_length(*node_id)
            }
            ArenaNodeData::Digest(_) => 32, // Keep digests as-is
        }
    }

    /// Returns the full RLP length when inlined (never using digest references)
    fn full_length(&self, node_id: NodeId) -> usize {
        let payload_length = self.payload_length_full(node_id);
        payload_length + alloy_rlp::length_of_length(payload_length)
    }

    /// Encodes a node with full inline children (never using digest references)
    fn encode_full_with_payload_len(
        &self,
        node_id: NodeId,
        payload_length: usize,
        out: &mut dyn alloy_rlp::BufMut,
    ) {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Branch(children) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                for child_id in children.iter_all() {
                    match child_id {
                        Some(id) => self.encode_full(id, out), // INLINE children, never use digest!
                        None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                    }
                }
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Leaf(encoded_path, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                value.encode(out);
            }
            ArenaNodeData::Extension(encoded_path, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                self.encode_full(*child_id, out); // INLINE child, never use digest!
            }
            ArenaNodeData::Digest(digest) => {
                // Keep digest nodes as-is (they represent external/unresolved nodes)
                digest.encode(out);
            }
        }
    }
}

// This code runs on host so it is not as performance critical as the rest of mpt
#[cfg(feature = "build_mpt")]
pub mod build_mpt {
    use super::*;
    use crate::{EthereumState, StorageTries};
    use reth_trie::AccountProof;
    use revm::primitives::HashMap;
    use revm_primitives::Address;

    /// Parses proof bytes into a vector of ArenaBasedMptNodes.
    pub fn parse_proof(proof: &[impl AsRef<[u8]>]) -> Result<Vec<ArenaBasedMptNode<'_>>, Error> {
        proof
            .iter()
            .map(|bytes| ArenaBasedMptNode::decode_from_rlp(bytes.as_ref(), 0))
            .collect::<Result<Vec<_>, _>>()
    }

    /// Helper to process proof nodes, convert them to static lifetime, and add to a node map.
    fn process_proof(
        proof_data: &[impl AsRef<[u8]>],
        nodes: &mut HashMap<MptNodeReference, ArenaBasedMptNode<'static>>,
    ) -> Result<Option<ArenaBasedMptNode<'static>>, Error> {
        let proof_nodes = parse_proof(proof_data)?;
        let root_node = proof_nodes.first().map(|node| convert_to_static_lifetime(node));

        for node in proof_nodes {
            let static_node = convert_to_static_lifetime(&node);
            nodes.insert(MptNodeReference::Digest(static_node.hash()), static_node);
        }
        Ok(root_node)
    }

    /// Builds a single storage trie from its proofs.
    fn build_storage_trie(
        proof: &AccountProof,
        fini_proofs: &AccountProof,
    ) -> Result<ArenaBasedMptNode<'static>, Error> {
        if proof.storage_proofs.is_empty() {
            return Ok(node_from_digest(proof.storage_root));
        }

        let mut storage_nodes = HashMap::default();
        let mut storage_root_node = ArenaBasedMptNode::default();

        for storage_proof in &proof.storage_proofs {
            if let Some(root) = process_proof(&storage_proof.proof, &mut storage_nodes)? {
                storage_root_node = root;
            }
        }

        for storage_proof in &fini_proofs.storage_proofs {
            add_orphaned_leafs(storage_proof.key.0, &storage_proof.proof, &mut storage_nodes)?;
        }

        Ok(resolve_nodes_arena(&storage_root_node, &storage_nodes))
    }

    /// Builds Ethereum state tries from relevant proofs before and after a state transition using
    /// arena-based MPT. This version returns EthereumState2 with arena-based nodes directly for
    /// better performance.
    pub fn transition_proofs_to_tries_arena(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<EthereumState, Error> {
        // If no addresses are provided, return the trie only consisting of the state root
        if parent_proofs.is_empty() {
            return Ok(EthereumState {
                state_trie: node_from_digest(state_root),
                storage_tries: Default::default(),
            });
        }

        let mut storage_tries = HashMap::default();
        let mut state_nodes = HashMap::default();
        let mut state_root_node = ArenaBasedMptNode::default();

        for (address, proof) in parent_proofs {
            if let Some(root) = process_proof(&proof.proof, &mut state_nodes)? {
                state_root_node = root;
            }

            let fini_proofs = proofs.get(address).unwrap();
            add_orphaned_leafs(address, &fini_proofs.proof, &mut state_nodes)?;

            let storage_trie = build_storage_trie(proof, fini_proofs)?;
            storage_tries.insert(B256::from(keccak256(address)), storage_trie);
        }

        // Create the state trie from all the relevant nodes
        let state_trie = resolve_nodes_arena(&state_root_node, &state_nodes);
        Ok(EthereumState { state_trie, storage_tries: StorageTries(storage_tries) })
    }

    /// Creates a new arena-based MPT node from a digest.
    fn node_from_digest(digest: B256) -> ArenaBasedMptNode<'static> {
        match digest {
            EMPTY_ROOT | B256::ZERO => ArenaBasedMptNode::default(),
            _ => {
                let mut trie = ArenaBasedMptNode::default();
                trie.nodes[0] = ArenaNodeData::Digest(digest);
                trie
            }
        }
    }

    /// Adds all the leaf nodes of non-inclusion proofs to the nodes.
    fn add_orphaned_leafs(
        key: impl AsRef<[u8]>,
        proof: &[impl AsRef<[u8]>],
        nodes_by_reference: &mut HashMap<MptNodeReference, ArenaBasedMptNode<'static>>,
    ) -> Result<(), Error> {
        if !proof.is_empty() {
            let proof_nodes = parse_proof(proof)?;
            if is_not_included(keccak256(key).as_slice(), &proof_nodes)? {
                // Add the leaf node to the nodes
                if let Some(leaf) = proof_nodes.last() {
                    for node in shorten_node_path_arena(leaf) {
                        let static_node = convert_to_static_lifetime(&node);
                        nodes_by_reference
                            .insert(MptNodeReference::Digest(static_node.hash()), static_node);
                    }
                }
            }
        }

        Ok(())
    }

    /// Helper function to convert a node with any lifetime to static lifetime
    /// by copying all borrowed data into owned storage
    fn convert_to_static_lifetime(node: &ArenaBasedMptNode<'_>) -> ArenaBasedMptNode<'static> {
        let mut static_node = ArenaBasedMptNode::with_capacity(node.nodes.len());
        static_node.nodes.clear();
        static_node.cached_references.clear();

        for node_data in &node.nodes {
            let static_data = match *node_data {
                ArenaNodeData::Null => ArenaNodeData::Null,
                ArenaNodeData::Branch(children) => ArenaNodeData::Branch(children),
                ArenaNodeData::Leaf(path, value) => {
                    let owned_path = static_node.alloc_in_bump(path);
                    let owned_value = static_node.alloc_in_bump(value);
                    ArenaNodeData::Leaf(owned_path, owned_value)
                }
                ArenaNodeData::Extension(path, child_id) => {
                    let owned_path = static_node.alloc_in_bump(path);
                    ArenaNodeData::Extension(owned_path, child_id)
                }
                ArenaNodeData::Digest(digest) => ArenaNodeData::Digest(digest),
            };
            static_node.add_node(static_data);
        }

        static_node.root_id = node.root_id;
        static_node
    }

    /// Verifies that the given proof is a valid proof of exclusion for the given key.
    pub fn is_not_included<'a>(
        key: &[u8],
        proof_nodes: &'a [ArenaBasedMptNode<'a>],
    ) -> Result<bool, Error> {
        let proof_trie = mpt_from_proof(proof_nodes)?;
        // For valid proofs, the get must not fail
        let value = proof_trie.get(key)?;

        Ok(value.is_none())
    }

    /// Returns a list of all possible nodes that can be created by shortening the path of the given
    /// node.
    pub fn shorten_node_path_arena<'a>(node: &ArenaBasedMptNode<'a>) -> Vec<ArenaBasedMptNode<'a>> {
        let mut res = Vec::new();
        let (path_bytes, is_leaf, value, child_id) = match &node.nodes[node.root_id as usize] {
            ArenaNodeData::Leaf(path_bytes, value) => (Some(*path_bytes), true, Some(*value), None),
            ArenaNodeData::Extension(path_bytes, child_id) => {
                (Some(*path_bytes), false, None, Some(*child_id))
            }
            _ => return res,
        };

        let path_bytes = path_bytes.unwrap();
        let nibs = prefix_to_small_nibs(path_bytes);

        for i in 0..=nibs.len() {
            let mut new_node = ArenaBasedMptNode::default();
            let shortened_nibs = &nibs[i..];
            let path_slice = new_node.add_encoded_path_slice(shortened_nibs, is_leaf);
            let new_node_data = if is_leaf {
                let value_slice = new_node.alloc_in_bump(value.unwrap());
                ArenaNodeData::Leaf(path_slice, value_slice)
            } else {
                ArenaNodeData::Extension(path_slice, child_id.unwrap())
            };
            new_node.nodes[0] = new_node_data;
            res.push(new_node);
        }
        res
    }

    /// Creates an arena-based Merkle Patricia trie from an EIP-1186 proof.
    pub fn mpt_from_proof<'a>(
        proof_nodes: &'a [ArenaBasedMptNode<'a>],
    ) -> Result<ArenaBasedMptNode<'a>, Error> {
        if proof_nodes.is_empty() {
            return Ok(ArenaBasedMptNode::default());
        }

        let node_store: HashMap<MptNodeReference, ArenaBasedMptNode<'a>> = proof_nodes
            .iter()
            .map(|node| (MptNodeReference::Digest(node.hash()), node.clone()))
            .collect();

        let root_node = proof_nodes.first().unwrap();

        Ok(resolve_nodes_arena(root_node, &node_store))
    }

    /// Resolves digest nodes in an arena-based trie using the provided node store.
    /// This rebuilds the arena, replacing any digest nodes with their resolved content.
    pub fn resolve_nodes_arena<'a>(
        root: &ArenaBasedMptNode<'a>,
        node_store: &HashMap<MptNodeReference, ArenaBasedMptNode<'a>>,
    ) -> ArenaBasedMptNode<'a> {
        let mut new_arena = ArenaBasedMptNode {
            nodes: Vec::new(),
            cached_references: Vec::new(),
            root_id: 0,
            bump: Rc::new(Bump::new()),
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
            dirty_nodes: RefCell::new(Vec::new()),
        };

        let root_id = resolve_node_recursive(root, root.root_id, node_store, &mut new_arena);
        new_arena.root_id = root_id;

        // The root hash must not change after resolution
        debug_assert_eq!(root.hash(), new_arena.hash());

        new_arena
    }

    /// Recursively resolves a single node and its children, adding them to the new arena.
    fn resolve_node_recursive<'a>(
        original_arena: &ArenaBasedMptNode<'a>,
        node_id: NodeId,
        node_store: &HashMap<MptNodeReference, ArenaBasedMptNode<'a>>,
        new_arena: &mut ArenaBasedMptNode<'a>,
    ) -> NodeId {
        let node_data = &original_arena.nodes[node_id as usize];

        let resolved_data = match node_data {
            ArenaNodeData::Null => ArenaNodeData::Null,
            ArenaNodeData::Leaf(prefix, value) => {
                // Copy the data into the new arena's owned storage
                let new_prefix = new_arena.alloc_in_bump(prefix);
                let new_value = new_arena.alloc_in_bump(value);
                ArenaNodeData::Leaf(new_prefix, new_value)
            }
            ArenaNodeData::Branch(children) => {
                let mut resolved_children: BranchChildren = Default::default();
                for (i, child_id) in children.iter() {
                    let resolved_child_id =
                        resolve_node_recursive(original_arena, child_id, node_store, new_arena);
                    resolved_children.set_child(i as u8, Some(resolved_child_id));
                }
                ArenaNodeData::Branch(resolved_children)
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                let resolved_child_id =
                    resolve_node_recursive(original_arena, *child_id, node_store, new_arena);
                let new_prefix = new_arena.alloc_in_bump(prefix);
                ArenaNodeData::Extension(new_prefix, resolved_child_id)
            }
            ArenaNodeData::Digest(digest) => {
                // Try to resolve the digest from the node store
                if let Some(resolved_node) = node_store.get(&MptNodeReference::Digest(*digest)) {
                    // Convert the resolved node to arena format and add it
                    return resolve_node_recursive(
                        resolved_node,
                        resolved_node.root_id,
                        node_store,
                        new_arena,
                    );
                } else {
                    // If not found in store, keep it as a digest
                    ArenaNodeData::Digest(*digest)
                }
            }
        };

        new_arena.add_node(resolved_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    impl ArenaBasedMptNode<'_> {
        pub fn is_empty(&self) -> bool {
            matches!(&self.nodes[self.root_id as usize], ArenaNodeData::Null)
        }
    }

    #[test]
    fn test_empty() {
        let trie = ArenaBasedMptNode::default();

        assert!(trie.is_empty());
        let expected = hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
        assert_eq!(expected, trie.hash().0);
    }

    #[test]
    fn test_clear() {
        let mut trie = ArenaBasedMptNode::default();
        trie.insert(b"dog", b"puppy").unwrap();
        assert!(!trie.is_empty());
        assert_ne!(trie.hash(), EMPTY_ROOT);

        trie.clear();
        assert!(trie.is_empty());
        assert_eq!(trie.hash(), EMPTY_ROOT);
    }

    #[test]
    fn test_insert() {
        let mut trie = ArenaBasedMptNode::default();
        let vals = vec![
            ("painting", "place"),
            ("guest", "ship"),
            ("mud", "leave"),
            ("paper", "call"),
            ("gate", "boast"),
            ("tongue", "gain"),
            ("baseball", "wait"),
            ("tale", "lie"),
            ("mood", "cope"),
            ("menu", "fear"),
        ];
        for (key, val) in &vals {
            assert!(trie.insert(key.as_bytes(), val.as_bytes()).unwrap());
        }

        let expected = hex!("2bab6cdf91a23ebf3af683728ea02403a98346f99ed668eec572d55c70a4b08f");
        assert_eq!(expected, trie.hash().0);

        for (key, value) in &vals {
            let retrieved = trie.get(key.as_bytes()).unwrap().unwrap();
            assert_eq!(retrieved, value.as_bytes());
        }

        // check inserting duplicate keys
        assert!(trie.insert(vals[0].0.as_bytes(), b"new").unwrap());
        assert!(!trie.insert(vals[0].0.as_bytes(), b"new").unwrap());
    }

    #[test]
    #[cfg(feature = "build_mpt")]
    fn test_mpt_from_proof_reconstruction() {
        use crate::mpt::build_mpt::mpt_from_proof;
        // Create a test proof scenario
        // This mimics how proofs work: we have a sequence of nodes where later nodes
        // reference earlier nodes by digest

        // Create the deepest node (a leaf) - using compact path directly
        let mut leaf_trie = ArenaBasedMptNode::default();
        let path_bytes = leaf_trie.add_encoded_path_slice(&[0x03], true); // compact encoding for nibble [0x03]
        let value_bytes = leaf_trie.alloc_in_bump(b"test_value");
        leaf_trie.nodes[0] = ArenaNodeData::Leaf(path_bytes, value_bytes);

        // Create a parent extension that references the leaf by digest
        let mut ext_trie = ArenaBasedMptNode::default();
        let leaf_digest = leaf_trie.hash();
        ext_trie.nodes[0] = ArenaNodeData::Digest(leaf_digest);
        let ext_child_id = ext_trie.add_node(ArenaNodeData::Digest(leaf_digest));
        let ext_path_bytes = ext_trie.add_encoded_path_slice(&[0x01], false); // compact encoding for nibble [0x01]
        ext_trie.nodes[0] = ArenaNodeData::Extension(ext_path_bytes, ext_child_id);
        ext_trie.root_id = 0;

        // Create the proof nodes (in the order they would appear in a real proof)
        let proof_nodes = vec![ext_trie, leaf_trie];

        // Reconstruct the trie from the proof
        let reconstructed = mpt_from_proof(&proof_nodes).unwrap();

        // The reconstructed trie should be able to retrieve the value
        // Key would be [0x01] + [0x03] = nibbles [0x01, 0x03] = key bytes [0x13]
        let retrieved = reconstructed.get(b"\x13").unwrap();
        assert_eq!(retrieved, Some(&b"test_value"[..]));

        // The hash should match what we expect
        assert!(!reconstructed.is_empty());
    }
}
