//! Arena-based Merkle Patricia Trie (MPT) optimized for zkVM execution.
//!
//! Key ideas:
//! - Nodes are stored in a flat arena (`Vec`) and referenced by `NodeId` indices for cache locality
//!   and compact serialization.
//! - Hex-prefix paths are stored compactly and decoded to nibbles on-demand for traversal.
//! - Hash/reference caching avoids repeated re-encoding and hashing. Cache is invalidated upwards
//!   during `insert`/`delete` recursion.
//! - RLP is used for (de)serialization; decoding is near zero-copy by borrowing from an input
//!   buffer and allocating subsequent mutations into a bump arena.
//!
//! This module is performance sensitive on hot paths (hashing, traversal, encode/decode), but
//! remains readable with small helpers and targeted documentation.
use alloy_rlp::{Buf, Encodable};
use bumpalo::Bump;
use core::fmt::Debug;
use revm::primitives::B256;
use revm_primitives::{b256, keccak256};
use serde::{de, ser, Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use eyre::Result;

use crate::word_bytes::OptimizedBytes;
use smallvec::SmallVec;
use thiserror::Error as ThisError;

use crate::hp::{
    encoded_path_eq_nibs, encoded_path_strip_prefix, lcp, prefix_to_small_nibs, to_nibs,
};

pub type NodeId = u32;

/// Sentinel index representing the null node when decoding and in internal references.
/// In a default arena, `nodes[0]` starts as `Null`, but the root may later be changed to a
/// non-null node (e.g. `Digest`) for convenience. `NULL_NODE_ID` is still used by the decoder
/// as the canonical "no node" identifier.
pub const NULL_NODE_ID: NodeId = 0;

/// Root hash of an empty trie.
///
/// This is the Keccak-256 of the RLP-encoding of the empty string (""),
/// which is the canonical encoding of an empty node in Ethereum's MPT.
pub const EMPTY_ROOT: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

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

/// Arena-based implementation that stores all nodes in a flat vector
/// and uses indices for better memory layout and performance.
/// The lifetime parameter 'a allows zero-copy deserialization by borrowing from the input buffer.
#[derive(Clone, Debug)]
pub struct MptTrie<'a> {
    root_id: NodeId,
    nodes: Vec<NodeData<'a>>,
    /// One monotonically‑growing arena that owns every mutated byte slice.
    bump: Rc<Bump>,
    // Cache. Hashing/encoding often needs “what would this node look like in its parent"
    cached_references: Vec<RefCell<Option<NodeRef>>>,
    /// Scratch buffer used only for RLP encoding when a node's full RLP exceeds 32 bytes and we
    /// need to compute its keccak hash. Keeping it here avoids repeated allocations.
    rlp_scratch: RefCell<Vec<u8>>,
}

impl ser::Serialize for MptTrie<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // Serialize as a tuple of (node_count, rlp_blob) for efficient deserialization.
        (self.nodes.len(), OptimizedBytes(self.to_full_rlp())).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for MptTrie<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (num_nodes, OptimizedBytes(rlp_blob)) =
            <(usize, OptimizedBytes)>::deserialize(deserializer)?;
        // We need to leak the memory to get a 'de lifetime - this is a limitation of serde
        let leaked_bytes: &'de [u8] = Box::leak(rlp_blob.into_boxed_slice());
        MptTrie::decode_from_rlp(leaked_bytes, num_nodes).map_err(de::Error::custom)
    }
}

impl Default for MptTrie<'_> {
    fn default() -> Self {
        Self {
            nodes: vec![NodeData::Null],
            cached_references: vec![RefCell::new(None)],
            root_id: 0,
            bump: Rc::new(Bump::new()),
            rlp_scratch: RefCell::new(Vec::with_capacity(128)),
        }
    }
}

/// Represents the ways in which one node can reference another node inside the sparse Merkle
/// Patricia Trie (MPT).
///
/// Nodes in the MPT can reference other nodes either directly through their byte representation or
/// indirectly through a hash of their encoding.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum NodeRef {
    /// Represents a direct reference to another node using its byte encoding. Typically
    /// used for short encodings that are less than 32 bytes in length.
    Bytes(Vec<u8>),
    /// Represents an indirect reference to another node using the Keccak hash of its long
    /// encoding. Used for encodings that are not less than 32 bytes in length.
    Digest(B256),
}

/// Node data for arena-based trie with zero-copy optimization
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub enum NodeData<'a> {
    #[default]
    /// Absence of a node. Encoded as empty string in RLP.
    Null,
    /// 16-way branch. Each child is optional; the branch's value slot is unused in our state trie
    /// and must be empty, enforced during decoding.
    Branch([Option<NodeId>; 16]),
    /// Leaf node containing a compact hex-prefix path and a value. Both slices borrow from the
    /// input buffer or bump arena. The path encodes the remainder of the key.
    Leaf(&'a [u8], &'a [u8]),
    /// Extension node containing a compact hex-prefix path and a single child. Path encodes a
    /// shared prefix to skip before continuing at `child`.
    Extension(&'a [u8], NodeId),
    /// Unresolved reference to a node by its Keccak-256 digest (32 bytes). Encountering this in
    /// `get`/`insert`/`delete` is an error; resolution happens in `build_mpt` helpers.
    Digest(B256),
}

// Constructors & Capacity Management
impl<'a> MptTrie<'a> {
    /// Creates a new arena with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap.max(1));
        let mut cached_references = Vec::with_capacity(cap.max(1));
        nodes.push(NodeData::Null);
        cached_references.push(RefCell::new(None));

        Self {
            nodes,
            cached_references,
            root_id: 0,
            bump: Rc::new(Bump::new()),
            rlp_scratch: RefCell::new(Vec::with_capacity(128)),
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
        self.cached_references.reserve(additional);
    }

    /// Decodes an RLP-encoded node directly into an ArenaBasedMptNode with zero-copy optimization
    /// Decodes an RLP-encoded MPT into an arena-backed structure.
    /// `num_nodes` is a hint used to pre-allocate the arena capacity to avoid early re-allocations
    /// during later updates.
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
        let mut arena = MptTrie::with_capacity(capacity);

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
impl<'a> MptTrie<'a> {
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
impl<'a> MptTrie<'a> {
    #[inline]
    fn get_recursive<'s>(
        &'s self,
        node_id: NodeId,
        key_nibs: &[u8],
    ) -> Result<Option<&'a [u8]>, Error> {
        match &self.nodes[node_id as usize] {
            NodeData::Null => Ok(None),
            NodeData::Branch(nodes) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match nodes[*i as usize] {
                        Some(id) => self.get_recursive(id, tail),
                        None => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            NodeData::Leaf(path_bytes, value) => {
                // Compare compact path to key nibbles without allocating
                if encoded_path_eq_nibs(path_bytes, key_nibs) {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            NodeData::Extension(path_bytes, child_id) => {
                // Strip compact path prefix without allocating
                if let Some(tail) = encoded_path_strip_prefix(path_bytes, key_nibs) {
                    self.get_recursive(*child_id, tail)
                } else {
                    Ok(None)
                }
            }
            NodeData::Digest(digest) => Err(Error::NodeNotResolved(*digest)),
        }
    }

    #[inline]
    fn insert_recursive(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
        value: &[u8],
    ) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            NodeData::Null => {
                let path_slice = self.add_encoded_path_slice(key_nibs, true);
                let value_slice = self.alloc_in_bump(value);
                self.nodes[node_id as usize] = NodeData::Leaf(path_slice, value_slice);
                Ok(true)
            }
            NodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let updated = match children[*i as usize] {
                        Some(id) => self.insert_recursive(id, tail, value)?,
                        None => {
                            let path_slice = self.add_encoded_path_slice(tail, true);
                            let value_slice = self.alloc_in_bump(value);
                            let new_leaf_id =
                                self.add_node(NodeData::Leaf(path_slice, value_slice));
                            children[*i as usize] = Some(new_leaf_id);
                            self.nodes[node_id as usize] = NodeData::Branch(children);
                            true
                        }
                    };
                    Ok(updated)
                } else {
                    Err(Error::ValueInBranch)
                }
            }
            NodeData::Leaf(path_bytes, old_value) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                let common_len = lcp(&path_nibs, key_nibs);

                if common_len == path_nibs.len() && common_len == key_nibs.len() {
                    if old_value == value {
                        return Ok(false);
                    }
                    let value_slice = self.alloc_in_bump(value);
                    self.nodes[node_id as usize] = NodeData::Leaf(path_bytes, value_slice);
                    Ok(true)
                } else if common_len == path_nibs.len() || common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    let leaf1_path_slice =
                        self.add_encoded_path_slice(&path_nibs[split_point..], true);
                    let leaf1_value_slice = self.alloc_in_bump(old_value);
                    let leaf1_id =
                        self.add_node(NodeData::Leaf(leaf1_path_slice, leaf1_value_slice));

                    let leaf2_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf2_value_slice = self.alloc_in_bump(value);
                    let leaf2_id =
                        self.add_node(NodeData::Leaf(leaf2_path_slice, leaf2_value_slice));

                    children[path_nibs[common_len] as usize] = Some(leaf1_id);
                    children[key_nibs[common_len] as usize] = Some(leaf2_id);

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(NodeData::Branch(children));
                        let ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[..common_len], false);
                        NodeData::Extension(ext_path_slice, branch_id)
                    } else {
                        NodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    Ok(true)
                }
            }
            NodeData::Extension(path_bytes, child_id) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                let common_len = lcp(&path_nibs, key_nibs);

                if common_len == path_nibs.len() {
                    self.insert_recursive(child_id, &key_nibs[common_len..], value)
                } else if common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    if split_point < path_nibs.len() {
                        let ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[split_point..], false);
                        let ext_id = self.add_node(NodeData::Extension(ext_path_slice, child_id));
                        children[path_nibs[common_len] as usize] = Some(ext_id);
                    } else {
                        children[path_nibs[common_len] as usize] = Some(child_id);
                    }

                    let leaf_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf_value_slice = self.alloc_in_bump(value);
                    let leaf_id = self.add_node(NodeData::Leaf(leaf_path_slice, leaf_value_slice));
                    children[key_nibs[common_len] as usize] = Some(leaf_id);

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(NodeData::Branch(children));
                        let parent_ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[..common_len], false);
                        NodeData::Extension(parent_ext_path_slice, branch_id)
                    } else {
                        NodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    Ok(true)
                }
            }
            NodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }?;

        if updated {
            self.invalidate_ref_cache(node_id);
        }

        Ok(updated)
    }

    #[inline]
    fn delete_recursive(&mut self, node_id: NodeId, key_nibs: &[u8]) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            NodeData::Null => Ok(false),
            NodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            if !self.delete_recursive(id, tail)? {
                                return Ok(false);
                            }

                            if matches!(self.nodes[id as usize], NodeData::Null) {
                                children[*i as usize] = None;
                            }
                        }
                        None => return Ok(false),
                    }
                } else {
                    return Err(Error::ValueInBranch);
                }

                let mut remaining_iter = children.iter().enumerate().filter(|(_, n)| n.is_some());

                if let Some(first_remaining) = remaining_iter.next() {
                    // One child found, check if there are more.
                    if remaining_iter.next().is_none() {
                        // Exactly one child remains, collapse the branch node.
                        let (index, &child_id) = first_remaining;
                        let child_id = child_id.unwrap();
                        let child_node_data = self.nodes[child_id as usize].clone();

                        let new_node_data = match child_node_data {
                            NodeData::Leaf(path_bytes, value) => {
                                let path_nibs = prefix_to_small_nibs(path_bytes);
                                let mut new_nibs: SmallVec<[u8; 64]> =
                                    SmallVec::with_capacity(1 + path_nibs.len());
                                new_nibs.push(index as u8);
                                new_nibs.extend_from_slice(&path_nibs);
                                let new_path_slice = self.add_encoded_path_slice(&new_nibs, true);
                                let new_value_slice = self.alloc_in_bump(value);
                                NodeData::Leaf(new_path_slice, new_value_slice)
                            }
                            NodeData::Extension(path_bytes, child_child_id) => {
                                let path_nibs = prefix_to_small_nibs(path_bytes);
                                let mut new_nibs: SmallVec<[u8; 64]> =
                                    SmallVec::with_capacity(1 + path_nibs.len());
                                new_nibs.push(index as u8);
                                new_nibs.extend_from_slice(&path_nibs);
                                let new_path_slice = self.add_encoded_path_slice(&new_nibs, false);
                                NodeData::Extension(new_path_slice, child_child_id)
                            }
                            NodeData::Branch(_) | NodeData::Digest(_) => {
                                let ext_nibs: SmallVec<[u8; 1]> =
                                    SmallVec::from_slice(&[index as u8]);
                                let new_path_slice = self.add_encoded_path_slice(&ext_nibs, false);
                                NodeData::Extension(new_path_slice, child_id)
                            }
                            NodeData::Null => unreachable!(),
                        };
                        self.nodes[node_id as usize] = new_node_data;
                    } else {
                        // More than one child remains, just update the branch node.
                        self.nodes[node_id as usize] = NodeData::Branch(children);
                    }
                } else {
                    // No children left, update to an empty branch node.
                    self.nodes[node_id as usize] = NodeData::Branch(children);
                }

                Ok(true)
            }
            NodeData::Leaf(path_bytes, _) => {
                let path_nibs = prefix_to_small_nibs(path_bytes);
                if path_nibs.as_slice() != key_nibs {
                    return Ok(false);
                }
                self.nodes[node_id as usize] = NodeData::Null;
                Ok(true)
            }
            NodeData::Extension(path_bytes, child_id) => {
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
                    NodeData::Null => NodeData::Null,
                    NodeData::Leaf(child_path_bytes, value) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path_slice = self.add_encoded_path_slice(&combined_nibs, true);
                        let new_value_slice = self.alloc_in_bump(value);
                        NodeData::Leaf(new_path_slice, new_value_slice)
                    }
                    NodeData::Extension(child_path_bytes, grandchild_id) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path_slice = self.add_encoded_path_slice(&combined_nibs, false);
                        NodeData::Extension(new_path_slice, grandchild_id)
                    }
                    NodeData::Branch(_) | NodeData::Digest(_) => {
                        NodeData::Extension(path_bytes, child_id)
                    }
                };
                self.nodes[node_id as usize] = new_node_data;
                Ok(true)
            }
            NodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }?;

        if updated {
            self.invalidate_ref_cache(node_id);
        }
        Ok(updated)
    }

    fn decode_node_recursive(&mut self, buf: &mut &'a [u8]) -> Result<NodeId, Error> {
        if buf.is_empty() {
            return Ok(NULL_NODE_ID); // Return the null node index
        }

        let header = alloy_rlp::Header::decode(buf).map_err(Error::Rlp)?;

        if !header.list {
            // Single data item
            if header.payload_length == 0 {
                return Ok(NULL_NODE_ID); // Null node
            }
            if header.payload_length == 32 {
                if buf.len() < 32 {
                    return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
                }
                let digest = B256::from_slice(&buf[..32]);
                buf.advance(32);
                return Ok(self.add_node(NodeData::Digest(digest)));
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
            let mut children = [None; 16];
            for child in children.iter_mut() {
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                if child_id != NULL_NODE_ID {
                    *child = Some(child_id);
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

            Ok(self.add_node(NodeData::Branch(children)))
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

                Ok(self.add_node(NodeData::Leaf(path_slice, value_slice)))
            } else {
                // Extension node (prefix 0x00 or 0x01)
                // The second item is a child node, which we must parse recursively.
                // We cannot reuse h2 because decode_node_recursive needs to see the full RLP.
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                Ok(self.add_node(NodeData::Extension(path_slice, child_id)))
            }
        }
    }

    #[inline]
    fn add_node(&mut self, data: NodeData<'a>) -> NodeId {
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
            NodeData::Null => EMPTY_ROOT,
            _ => {
                match self.cached_references[node_id as usize]
                    .borrow_mut()
                    .get_or_insert_with(|| self.calc_reference(node_id))
                {
                    NodeRef::Digest(digest) => *digest,
                    NodeRef::Bytes(bytes) => keccak256(bytes),
                }
            }
        }
    }

    #[inline]
    fn calc_reference(&self, node_id: NodeId) -> NodeRef {
        match &self.nodes[node_id as usize] {
            NodeData::Null => NodeRef::Bytes(vec![alloy_rlp::EMPTY_STRING_CODE]),
            NodeData::Digest(digest) => NodeRef::Digest(*digest),
            _ => {
                let payload_length = self.payload_length_id(node_id);
                let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

                if rlp_length < 32 {
                    let mut encoded = Vec::with_capacity(rlp_length);
                    self.encode_id_with_payload_len(node_id, payload_length, &mut encoded);
                    debug_assert_eq!(encoded.len(), rlp_length);
                    NodeRef::Bytes(encoded)
                } else {
                    let mut scratch = self.rlp_scratch.borrow_mut();
                    scratch.clear();
                    scratch.reserve(rlp_length);
                    self.encode_id_with_payload_len(node_id, payload_length, &mut *scratch);
                    debug_assert_eq!(scratch.len(), rlp_length);
                    NodeRef::Digest(keccak256(&*scratch))
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
            NodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            NodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                for child_id in nodes.iter() {
                    match child_id {
                        Some(id) => self.reference_encode_id(*id, out),
                        None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                    }
                }
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            NodeData::Leaf(encoded_path, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                value.encode(out);
            }
            NodeData::Extension(encoded_path, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                self.reference_encode_id(*child_id, out);
            }
            NodeData::Digest(digest) => {
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
            NodeRef::Bytes(bytes) => out.put_slice(bytes),
            // if the reference is a digest, RLP-encode it with its fixed known length
            NodeRef::Digest(digest) => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE + 32);
                out.put_slice(digest.as_slice());
            }
        }
    }

    fn payload_length_id(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id as usize] {
            NodeData::Null => 0,
            NodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
                    .map(|child| child.map_or(1, |id| self.reference_length(id)))
                    .sum::<usize>()
            }
            NodeData::Leaf(encoded_path, value) => encoded_path.length() + value.length(),
            NodeData::Extension(encoded_path, node_id) => {
                encoded_path.length() + self.reference_length(*node_id)
            }
            NodeData::Digest(_) => 32,
        }
    }

    #[inline]
    fn reference_length(&self, node_id: NodeId) -> usize {
        match self.cached_references[node_id as usize]
            .borrow_mut()
            .get_or_insert_with(|| self.calc_reference(node_id))
        {
            NodeRef::Bytes(bytes) => bytes.len(),
            NodeRef::Digest(_) => 1 + 32,
        }
    }
}

impl MptTrie<'_> {
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn encode_trie(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        self.encode_trie_internal(self.root_id, &mut payload);

        let mut encoded = Vec::new();
        alloy_rlp::Header { list: true, payload_length: payload.len() }.encode(&mut encoded);
        encoded.append(&mut payload);
        encoded
    }

    fn encode_trie_internal(&self, id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.payload_length_id(id);
        self.encode_id_with_payload_len(id, payload_length, out);

        match self.nodes[id as usize] {
            NodeData::Branch(childs) => childs.iter().for_each(|c| {
                if let Some(node) = c {
                    self.encode_trie_internal(*node, out);
                } else {
                    out.put_u8(alloy_rlp::EMPTY_STRING_CODE)
                }
            }),
            NodeData::Extension(_, node) => {
                self.encode_trie_internal(node, out);
            }
            _ => {}
        }
    }
}

// Serialization. Not performance critical.
impl MptTrie<'_> {
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
            NodeData::Null => 0,
            NodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
                    .map(|child| child.map_or(1, |id| self.full_length(id)))
                    .sum::<usize>()
            }
            NodeData::Leaf(encoded_path, value) => encoded_path.length() + value.length(),
            NodeData::Extension(encoded_path, node_id) => {
                encoded_path.length() + self.full_length(*node_id)
            }
            // For digests we keep the string-encoding as-is. The payload length here refers to the
            // raw 32 bytes; callers computing the full RLP length will add the single-byte prefix
            // for strings < 56 bytes, yielding 33 total bytes when inlined into a parent list.
            NodeData::Digest(_) => 32,
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
            NodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            NodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                nodes.iter().for_each(|child_id| match child_id {
                    Some(id) => self.encode_full(*id, out), // INLINE children, never use digest!
                    None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                });
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            NodeData::Leaf(encoded_path, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                value.encode(out);
            }
            NodeData::Extension(encoded_path, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                encoded_path.encode(out);
                self.encode_full(*child_id, out); // INLINE child, never use digest!
            }
            NodeData::Digest(digest) => {
                // Keep digest nodes as-is (they represent external/unresolved nodes)
                digest.encode(out);
            }
        }
    }
}

// This code runs on host so it is not as performance critical as the rest of mpt
#[cfg(feature = "build_mpt")]
pub mod build_mpt;

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    impl MptTrie<'_> {
        pub fn is_empty(&self) -> bool {
            matches!(&self.nodes[self.root_id as usize], NodeData::Null)
        }
    }

    #[test]
    fn test_empty() {
        let trie = MptTrie::default();

        assert!(trie.is_empty());
        let expected = hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
        assert_eq!(expected, trie.hash().0);
    }

    #[test]
    fn test_clear() {
        let mut trie = MptTrie::default();
        trie.insert(b"dog", b"puppy").unwrap();
        assert!(!trie.is_empty());
        assert_ne!(trie.hash(), EMPTY_ROOT);

        trie.clear();
        assert!(trie.is_empty());
        assert_eq!(trie.hash(), EMPTY_ROOT);
    }

    #[test]
    fn test_insert() {
        let mut trie = MptTrie::default();
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

    // build_mpt feature tests moved to tests/ folder
}
