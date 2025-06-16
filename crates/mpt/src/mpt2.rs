use alloy_primitives::B256;
use alloy_rlp::{Buf, Encodable};
use bumpalo::Bump;
use core::fmt::Debug;
use reth_trie::AccountProof;
use revm::primitives::HashMap;
use revm_primitives::{keccak256, Address};
use serde::{de, ser};
use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;

use crate::word_bytes::OptimizedBytes;
use crate::{
    mpt::{Error, MptNodeReference, EMPTY_ROOT},
    utils::{lcp, to_nibs},
    EthereumState2, StorageTries2,
};
use smallvec::SmallVec;

pub type NodeId = u32;

/// Optimized conversion from hex-encoded path directly to nibbles SmallVec
/// Avoids the Vec<u8> -> SmallVec<u8> round-trip that was causing extra allocations
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

/// Represents the root node of a sparse Merkle Patricia Trie.
///
/// This is the new arena-based implementation that stores all nodes in a flat vector
/// and uses indices instead of boxed pointers for better memory layout and performance.
/// The lifetime parameter 'a allows zero-copy deserialization by borrowing from the input buffer.
#[derive(Clone, Debug)]
pub struct ArenaBasedMptNode<'a> {
    nodes: Vec<ArenaNodeData<'a>>,
    cached_references: Vec<RefCell<Option<MptNodeReference>>>,
    root_id: NodeId,
    /// One monotonically‑growing arena that owns every mutated byte slice.
    bump: Rc<Bump>,
    encoding_scratch: RefCell<Vec<u8>>,
}

impl ser::Serialize for ArenaBasedMptNode<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // Serialize as a compact RLP blob with ALL children inlined!
        // This is much smaller and faster than serializing the arena structure
        OptimizedBytes(self.to_full_rlp()).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for ArenaBasedMptNode<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // Deserialize the RLP blob and use our fast streaming decoder!
        let rlp_blob: OptimizedBytes = OptimizedBytes::deserialize(deserializer)?;
        // We need to leak the memory to get a 'de lifetime - this is a limitation of serde
        let leaked_bytes: &'de [u8] = Box::leak(rlp_blob.0.into_boxed_slice());
        ArenaBasedMptNode::decode_from_rlp(leaked_bytes).map_err(de::Error::custom)
    }
}

impl Default for ArenaBasedMptNode<'_> {
    fn default() -> Self {
        let bump = Rc::new(Bump::new());
        Self {
            nodes: vec![ArenaNodeData::Null],
            cached_references: vec![RefCell::new(None)],
            root_id: 0,
            bump,
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
        }
    }
}

/// Node data for arena-based trie with zero-copy optimization
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub enum ArenaNodeData<'a> {
    #[default]
    Null,
    Branch([Option<NodeId>; 16]),
    Leaf(&'a [u8], &'a [u8]), // (compact_path, value) - borrowed from input
    Extension(&'a [u8], NodeId), // (compact_path, child_id) - borrowed from input
    Digest(B256),
}

/// Custom RLP decoder that builds ArenaBasedMptNode directly without intermediate boxed structures
impl<'a> ArenaBasedMptNode<'a> {
    /// Creates a new arena with pre-allocated capacity
    pub fn with_capacity(cap: usize) -> Self {
        let bump = Rc::new(Bump::new());
        let mut nodes = Vec::with_capacity(cap.max(1));
        let mut cached_references = Vec::with_capacity(cap.max(1));
        nodes.push(ArenaNodeData::Null);
        cached_references.push(RefCell::new(None));

        Self {
            nodes,
            cached_references,
            root_id: 0,
            bump,
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
        }
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given arena.
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
        self.cached_references.reserve(additional);
    }

    /// Decodes an RLP-encoded node directly into an ArenaBasedMptNode with zero-copy optimization
    pub fn decode_from_rlp(bytes: &'a [u8]) -> Result<Self, Error> {
        // Improved heuristic: real Ethereum nodes average ~20-24 bytes, overshoot to avoid reallocations
        eprintln!("bytes len: {}", bytes.len());
        let estimated_nodes = bytes.len() / 29 + 3;
        let mut arena = ArenaBasedMptNode::with_capacity(estimated_nodes);

        let mut buf = bytes;
        let root_id = arena.decode_node_recursive(&mut buf)?;
        if !buf.is_empty() {
            return Err(Error::Rlp(alloy_rlp::Error::Custom("trailing data")));
        }
        arena.root_id = root_id;
        Ok(arena)
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
            let mut children = [None; 16];
            for i in 0..16 {
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                if child_id != 0 {
                    children[i] = Some(child_id);
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
}

impl<'a> ArenaBasedMptNode<'a> {
    #[allow(dead_code)]
    fn new(root_id: NodeId, nodes: Vec<ArenaNodeData<'a>>) -> Self {
        let cached_references = (0..nodes.len()).map(|_| RefCell::new(None)).collect();
        Self {
            nodes,
            cached_references,
            root_id,
            bump: Rc::new(Bump::new()),
            encoding_scratch: RefCell::new(Vec::with_capacity(128)),
        }
    }

    fn add_node(&mut self, data: ArenaNodeData<'a>) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(data);
        self.cached_references.push(RefCell::new(None));
        id
    }

    /// Encodes nibbles into the standard hex-prefix format directly into the bump arena.
    /// This avoids creating an intermediate `Vec`.
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
        self.add_owned_slice(&encoded)
    }

    /// Copies `bytes` into the bump arena and returns a `'a` slice.
    #[inline]
    fn alloc_in_bump(&self, bytes: &[u8]) -> &'a [u8] {
        // `Bump::alloc_slice_copy` only needs `&Bump`, no &mut.
        let slice = self.bump.alloc_slice_copy(bytes);
        // Sound because `slice` lives as long as `self.bump`.
        unsafe { std::mem::transmute::<&[u8], &'a [u8]>(slice) }
    }

    /// Former `add_owned_slice`, now zero heap‑fragments.
    #[inline]
    fn add_owned_slice(&mut self, bytes: impl AsRef<[u8]>) -> &'a [u8] {
        self.alloc_in_bump(bytes.as_ref())
    }

    #[inline]
    fn invalidate_ref_cache(&mut self, node_id: NodeId) {
        self.cached_references[node_id as usize].borrow_mut().take();
    }

    /// Computes and returns the 256-bit hash of the node.
    #[inline]
    pub fn hash(&self) -> B256 {
        self.hash_id(self.root_id)
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

    /// Encodes a node with ALL children inlined (never using digest references).
    /// This produces the fully-expanded RLP representation.
    fn encode_full(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.payload_length_full(node_id);
        self.encode_full_with_payload_len(node_id, payload_length, out);
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
            ArenaNodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                nodes.iter().for_each(|child_id| match child_id {
                    Some(id) => self.reference_encode_id(*id, out),
                    None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                });
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
            ArenaNodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                nodes.iter().for_each(|child_id| match child_id {
                    Some(id) => self.encode_full(*id, out), // INLINE children, never use digest!
                    None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                });
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

    /// Returns the RLP-encoded bytes with ALL children inlined (never replaced by digest).
    /// This produces a compact, fully-expanded representation perfect for serialization.
    #[inline]
    pub fn to_full_rlp(&self) -> Vec<u8> {
        // Rough estimate: each node ~100 bytes average, plus some overhead
        let mut out = Vec::with_capacity(self.nodes.len() * 100);
        self.encode_full(self.root_id, &mut out);
        out
    }

    /// Clears the trie, replacing its data with an empty node.
    /// Old `clear()` – keep the old arena for anyone still sharing it,
    /// switch `self` to a fresh one.
    #[inline]
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Retrieves the value associated with a given key in the trie.
    #[inline]
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.get_recursive(self.root_id, &to_nibs(key))
    }

    /// Retrieves the RLP-decoded value corresponding to the key.
    #[inline]
    pub fn get_rlp<T: alloy_rlp::Decodable>(&self, key: &[u8]) -> Result<Option<T>, Error> {
        match self.get(key)? {
            Some(bytes) => Ok(Some(T::decode(&mut bytes.as_slice())?)),
            None => Ok(None),
        }
    }

    fn get_recursive(&self, node_id: NodeId, key_nibs: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => Ok(None),
            ArenaNodeData::Branch(nodes) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match nodes[*i as usize] {
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
                    Ok(Some(value.to_vec()))
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

    fn insert_recursive(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
        value: &[u8],
    ) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            ArenaNodeData::Null => {
                let path_slice = self.add_encoded_path_slice(key_nibs, true);
                let value_slice = self.add_owned_slice(value);
                self.nodes[node_id as usize] = ArenaNodeData::Leaf(path_slice, value_slice);
                Ok(true)
            }
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let updated = match children[*i as usize] {
                        Some(id) => self.insert_recursive(id, tail, value)?,
                        None => {
                            let path_slice = self.add_encoded_path_slice(tail, true);
                            let value_slice = self.add_owned_slice(value);
                            let new_leaf_id =
                                self.add_node(ArenaNodeData::Leaf(path_slice, value_slice));
                            children[*i as usize] = Some(new_leaf_id);
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
                    let value_slice = self.add_owned_slice(value);
                    self.nodes[node_id as usize] = ArenaNodeData::Leaf(path_bytes, value_slice);
                    Ok(true)
                } else if common_len == path_nibs.len() || common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    let leaf1_path_slice =
                        self.add_encoded_path_slice(&path_nibs[split_point..], true);
                    let leaf1_value_slice = self.add_owned_slice(old_value);
                    let leaf1_id =
                        self.add_node(ArenaNodeData::Leaf(leaf1_path_slice, leaf1_value_slice));

                    let leaf2_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf2_value_slice = self.add_owned_slice(value);
                    let leaf2_id =
                        self.add_node(ArenaNodeData::Leaf(leaf2_path_slice, leaf2_value_slice));

                    children[path_nibs[common_len] as usize] = Some(leaf1_id);
                    children[key_nibs[common_len] as usize] = Some(leaf2_id);

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
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    if split_point < path_nibs.len() {
                        let ext_path_slice =
                            self.add_encoded_path_slice(&path_nibs[split_point..], false);
                        let ext_id =
                            self.add_node(ArenaNodeData::Extension(ext_path_slice, child_id));
                        children[path_nibs[common_len] as usize] = Some(ext_id);
                    } else {
                        children[path_nibs[common_len] as usize] = Some(child_id);
                    }

                    let leaf_path_slice =
                        self.add_encoded_path_slice(&key_nibs[split_point..], true);
                    let leaf_value_slice = self.add_owned_slice(value);
                    let leaf_id =
                        self.add_node(ArenaNodeData::Leaf(leaf_path_slice, leaf_value_slice));
                    children[key_nibs[common_len] as usize] = Some(leaf_id);

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

    /// Removes a key from the trie.
    ///
    /// This method attempts to remove a key-value pair from the trie. If the key is
    /// present, it returns `true`. Otherwise, it returns `false`.
    #[inline]
    pub fn delete(&mut self, key: &[u8]) -> Result<bool, Error> {
        let key_nibs = &to_nibs(key);
        self.delete_recursive(self.root_id, key_nibs)
    }

    fn delete_recursive(&mut self, node_id: NodeId, key_nibs: &[u8]) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            ArenaNodeData::Null => Ok(false),
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            if !self.delete_recursive(id, tail)? {
                                return Ok(false);
                            }

                            if matches!(self.nodes[id as usize], ArenaNodeData::Null) {
                                children[*i as usize] = None;
                            }
                        }
                        None => return Ok(false),
                    }
                } else {
                    return Err(Error::ValueInBranch);
                }

                let remaining: Vec<_> =
                    children.iter().enumerate().filter(|(_, n)| n.is_some()).collect();

                if remaining.len() == 1 {
                    let (index, &child_id) = remaining[0];
                    let child_id = child_id.unwrap();
                    let child_node_data = self.nodes[child_id as usize].clone();

                    let new_node_data = match child_node_data {
                        ArenaNodeData::Leaf(path_bytes, value) => {
                            let path_nibs = prefix_to_small_nibs(path_bytes);
                            let mut new_nibs: SmallVec<[u8; 64]> =
                                SmallVec::with_capacity(1 + path_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&path_nibs);
                            let new_path_slice = self.add_encoded_path_slice(&new_nibs, true);
                            let new_value_slice = self.add_owned_slice(value);
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
                            let ext_nibs: SmallVec<[u8; 1]> = SmallVec::from_slice(&[index as u8]);
                            let new_path_slice = self.add_encoded_path_slice(&ext_nibs, false);
                            ArenaNodeData::Extension(new_path_slice, child_id)
                        }
                        ArenaNodeData::Null => unreachable!(),
                    };
                    self.nodes[node_id as usize] = new_node_data;
                } else {
                    self.nodes[node_id as usize] = ArenaNodeData::Branch(children);
                };

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
                        let new_value_slice = self.add_owned_slice(value);
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

    fn payload_length_id(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
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

    /// Calculates payload length for full inline encoding (never using digest references)
    fn payload_length_full(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id as usize] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
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
}

// IMPORTANT: This code runs on host so it is not as performance critical as the rest of mpt
#[cfg(feature = "build_mpt")]
pub mod build_mpt {
    use super::*;

    impl<'a> ArenaBasedMptNode<'a> {
        /// Merges another trie into this arena and returns the root NodeId of the merged trie
        fn merge_trie(&mut self, other: &ArenaBasedMptNode<'a>) -> NodeId {
            let mut node_mapping = HashMap::<NodeId, NodeId>::new();

            // Copy all nodes from other trie into this arena
            for (other_id, other_node) in other.nodes.iter().enumerate() {
                let new_id = self.add_node(other_node.clone());
                node_mapping.insert(other_id as u32, new_id);
            }

            // Update all the node references in the copied nodes
            for (&other_id, &new_id) in &node_mapping {
                match &other.nodes[other_id as usize] {
                    ArenaNodeData::Branch(children) => {
                        let mut new_children: [Option<NodeId>; 16] = Default::default();
                        for (i, child) in children.iter().enumerate() {
                            if let Some(child_id) = child {
                                new_children[i] = Some(node_mapping[child_id]);
                            }
                        }
                        self.nodes[new_id as usize] = ArenaNodeData::Branch(new_children);
                    }
                    ArenaNodeData::Extension(prefix, child_id) => {
                        let new_child_id = node_mapping[child_id];
                        let new_prefix = self.add_owned_slice(*prefix);
                        self.nodes[new_id as usize] =
                            ArenaNodeData::Extension(new_prefix, new_child_id);
                    }
                    ArenaNodeData::Leaf(prefix, value) => {
                        let new_prefix = self.add_owned_slice(*prefix);
                        let new_value = self.add_owned_slice(*value);
                        self.nodes[new_id as usize] = ArenaNodeData::Leaf(new_prefix, new_value);
                    }
                    _ => {} // Null, and Digest nodes don't need reference updates
                }
            }

            // Return the root of the merged trie
            node_mapping[&other.root_id]
        }
    }

    /// Parses proof bytes into a vector of ArenaBasedMptNodes.
    pub fn parse_proof(proof: &[impl AsRef<[u8]>]) -> Result<Vec<ArenaBasedMptNode<'_>>, Error> {
        proof
            .iter()
            .map(|bytes| ArenaBasedMptNode::decode_from_rlp(bytes.as_ref()))
            .collect::<Result<Vec<_>, _>>()
    }

    /// Builds Ethereum state tries from relevant proofs before and after a state transition using
    /// arena-based MPT. This version returns EthereumState2 with arena-based nodes directly for better
    /// performance.
    pub fn transition_proofs_to_tries_arena(
        state_root: B256,
        parent_proofs: &HashMap<Address, AccountProof>,
        proofs: &HashMap<Address, AccountProof>,
    ) -> Result<EthereumState2, Error> {
        // If no addresses are provided, return the trie only consisting of the state root
        if parent_proofs.is_empty() {
            return Ok(EthereumState2 {
                state_trie: match state_root {
                    EMPTY_ROOT | B256::ZERO => ArenaBasedMptNode::default(),
                    _ => node_from_digest(state_root),
                },
                storage_tries: Default::default(),
            });
        }

        let mut storage: HashMap<B256, ArenaBasedMptNode<'static>> = HashMap::default();

        let mut state_nodes = HashMap::<MptNodeReference, ArenaBasedMptNode<'static>>::default();
        let mut state_root_node = ArenaBasedMptNode::default();

        for (address, proof) in parent_proofs {
            let proof_nodes = parse_proof(&proof.proof)?;

            // The first node in the proof is the root
            if let Some(node) = proof_nodes.first() {
                state_root_node = convert_to_static_lifetime(node.clone());
            }

            proof_nodes.into_iter().for_each(|node| {
                // Convert to static lifetime and use a digest reference based on the node's hash
                let static_node = convert_to_static_lifetime(node);
                state_nodes.insert(MptNodeReference::Digest(static_node.hash()), static_node);
            });

            let fini_proofs = proofs.get(address).unwrap();

            // Add orphaned leafs for deletions
            add_orphaned_leafs(address, &fini_proofs.proof, &mut state_nodes)?;

            // Handle storage proofs
            let storage_root = proof.storage_root;
            if proof.storage_proofs.is_empty() {
                let storage_root_node = match storage_root {
                    EMPTY_ROOT | B256::ZERO => ArenaBasedMptNode::default(),
                    _ => node_from_digest(storage_root),
                };
                storage.insert(B256::from(keccak256(address)), storage_root_node);
                continue;
            }

            let mut storage_nodes =
                HashMap::<MptNodeReference, ArenaBasedMptNode<'static>>::default();
            let mut storage_root_node = ArenaBasedMptNode::default();

            for storage_proof in &proof.storage_proofs {
                let proof_nodes = parse_proof(&storage_proof.proof)?;

                // The first node in the proof is the root
                if let Some(node) = proof_nodes.first() {
                    storage_root_node = convert_to_static_lifetime(node.clone());
                }

                proof_nodes.into_iter().for_each(|node| {
                    // Convert to static lifetime and use a digest reference based on the node's hash
                    let static_node = convert_to_static_lifetime(node);
                    storage_nodes.insert(MptNodeReference::Digest(static_node.hash()), static_node);
                });
            }

            // Assure that slots can be deleted from the storage trie
            for storage_proof in &fini_proofs.storage_proofs {
                add_orphaned_leafs(storage_proof.key.0, &storage_proof.proof, &mut storage_nodes)?;
            }

            // Create the storage trie from all the relevant nodes - keep as arena-based
            let storage_trie = resolve_nodes_arena(&storage_root_node, &storage_nodes);

            storage.insert(B256::from(keccak256(address)), storage_trie);
        }

        // Create the state trie from all the relevant nodes - keep as arena-based
        let state_trie = resolve_nodes_arena(&state_root_node, &state_nodes);

        Ok(EthereumState2 { state_trie, storage_tries: StorageTries2(storage) })
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
                let leaf = proof_nodes.last().unwrap();
                // Convert to static lifetime by cloning the data
                for node in shorten_node_path_arena(leaf) {
                    // Create a static version by copying all data
                    let static_node = convert_to_static_lifetime(node);
                    nodes_by_reference
                        .insert(MptNodeReference::Digest(static_node.hash()), static_node);
                }
            }
        }

        Ok(())
    }

    /// Helper function to convert a node with any lifetime to static lifetime
    /// by copying all borrowed data into owned storage
    fn convert_to_static_lifetime(node: ArenaBasedMptNode<'_>) -> ArenaBasedMptNode<'static> {
        // Use with_capacity to correctly initialize with a pre-sized Vec and a default Null node.
        let mut static_node = ArenaBasedMptNode::with_capacity(node.nodes.len());

        // Copy all nodes, converting borrowed data to owned
        for (i, node_data) in node.nodes.into_iter().enumerate() {
            let static_data = match node_data {
                ArenaNodeData::Null => ArenaNodeData::Null,
                ArenaNodeData::Branch(children) => ArenaNodeData::Branch(children),
                ArenaNodeData::Leaf(path, value) => {
                    let owned_path = static_node.add_owned_slice(path);
                    let owned_value = static_node.add_owned_slice(value);
                    ArenaNodeData::Leaf(owned_path, owned_value)
                }
                ArenaNodeData::Extension(path, child_id) => {
                    let owned_path = static_node.add_owned_slice(path);
                    ArenaNodeData::Extension(owned_path, child_id)
                }
                ArenaNodeData::Digest(digest) => ArenaNodeData::Digest(digest),
            };

            if i == 0 {
                // Overwrite the initial Null node.
                static_node.nodes[0] = static_data;
                // The OnceCell is already new, which is correct.
            } else {
                static_node.add_node(static_data);
            }
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
            _ => (None, false, None, None),
        };

        let Some(path_bytes) = path_bytes else { return res };
        let nibs = prefix_to_small_nibs(path_bytes);

        if is_leaf {
            let value = value.unwrap();
            for i in 0..=nibs.len() {
                let mut new_node = ArenaBasedMptNode::default();
                let shortened_nibs = &nibs[i..];
                let path_slice = new_node.add_encoded_path_slice(shortened_nibs, true);
                let value_slice = new_node.add_owned_slice(value);
                new_node.nodes[0] = ArenaNodeData::Leaf(path_slice, value_slice);
                res.push(new_node);
            }
        } else {
            let child_id = child_id.unwrap();
            for i in 0..=nibs.len() {
                let mut new_node = ArenaBasedMptNode::default();
                let shortened_nibs = &nibs[i..];
                let path_slice = new_node.add_encoded_path_slice(shortened_nibs, false);
                new_node.nodes[0] = ArenaNodeData::Extension(path_slice, child_id);
                res.push(new_node);
            }
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

        let mut next: Option<ArenaBasedMptNode<'a>> = None;
        for (i, node) in proof_nodes.iter().enumerate().rev() {
            // There is nothing to replace for the last node
            let Some(replacement) = next else {
                next = Some(node.clone());
                continue;
            };

            // The next node must have a digest reference
            let replacement_hash = replacement.hash();

            // Find the child that references the next node and replace it
            let mut resolved = node.clone();
            let resolved_root_data = &resolved.nodes[resolved.root_id as usize].clone();

            match resolved_root_data {
                ArenaNodeData::Branch(children) => {
                    let mut new_children = *children;
                    let mut found = false;

                    for (child_idx, child_id) in children.iter().enumerate() {
                        if let Some(child_id) = child_id {
                            if let ArenaNodeData::Digest(digest) =
                                &resolved.nodes[*child_id as usize]
                            {
                                if *digest == replacement_hash {
                                    // Replace this child with the replacement trie
                                    // We need to merge the replacement trie into our arena
                                    let replacement_root_in_resolved =
                                        resolved.merge_trie(&replacement);
                                    new_children[child_idx] = Some(replacement_root_in_resolved);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        panic!("node {} does not reference the successor", i);
                    }

                    resolved.nodes[resolved.root_id as usize] = ArenaNodeData::Branch(new_children);
                }
                ArenaNodeData::Extension(prefix, child_id) => {
                    if let ArenaNodeData::Digest(digest) = &resolved.nodes[*child_id as usize] {
                        if *digest != replacement_hash {
                            panic!("node {} does not reference the successor", i);
                        }
                        // Replace the child with the replacement trie
                        let replacement_root_in_resolved = resolved.merge_trie(&replacement);
                        resolved.nodes[resolved.root_id as usize] =
                            ArenaNodeData::Extension(prefix, replacement_root_in_resolved);
                    } else {
                        panic!("node {} does not reference the successor", i);
                    }
                }
                ArenaNodeData::Null | ArenaNodeData::Leaf(_, _) | ArenaNodeData::Digest(_) => {
                    panic!("node {} has no children to replace", i);
                }
            }

            next = Some(resolved);
        }

        // The last node in the proof should be the root
        Ok(next.unwrap_or_default())
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
                let new_prefix = new_arena.add_owned_slice(*prefix);
                let new_value = new_arena.add_owned_slice(*value);
                ArenaNodeData::Leaf(new_prefix, new_value)
            }
            ArenaNodeData::Branch(children) => {
                let mut resolved_children: [Option<NodeId>; 16] = Default::default();
                for (i, child_id) in children.iter().enumerate() {
                    if let Some(child_id) = child_id {
                        let resolved_child_id = resolve_node_recursive(
                            original_arena,
                            *child_id,
                            node_store,
                            new_arena,
                        );
                        resolved_children[i] = Some(resolved_child_id);
                    }
                }
                ArenaNodeData::Branch(resolved_children)
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                let resolved_child_id =
                    resolve_node_recursive(original_arena, *child_id, node_store, new_arena);
                let new_prefix = new_arena.add_owned_slice(*prefix);
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
        use crate::mpt2::build_mpt::mpt_from_proof;
        // Create a test proof scenario
        // This mimics how proofs work: we have a sequence of nodes where later nodes
        // reference earlier nodes by digest

        // Create the deepest node (a leaf) - using compact path directly
        let mut leaf_trie = ArenaBasedMptNode::default();
        let path_bytes = leaf_trie.add_encoded_path_slice(&[0x03], true); // compact encoding for nibble [0x03]
        let value_bytes = leaf_trie.add_owned_slice(b"test_value");
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
        assert_eq!(retrieved, Some(b"test_value".to_vec()));

        // The hash should match what we expect
        assert!(!reconstructed.is_empty());
    }
}
