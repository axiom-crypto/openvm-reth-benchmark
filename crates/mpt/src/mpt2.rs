#![allow(unreachable_pub)]
#![allow(dead_code)]

use alloy_primitives::B256;
use alloy_rlp::{Buf, Encodable};
use core::{cell::RefCell, fmt::Debug};
use reth_trie::AccountProof;
use revm::primitives::HashMap;
use revm_primitives::{keccak256, Address};
use serde::{de, ser, Deserialize, Serialize};

use eyre::Result;

use crate::word_bytes::OptimizedBytes;
use crate::{
    mpt::{Error, MptNodeReference, EMPTY_ROOT},
    utils::{lcp, prefix_nibs, to_encoded_path, to_nibs},
    EthereumState2, StorageTries2,
};
use smallvec::SmallVec;

pub type NodeId = usize;

/// Represents the root node of a sparse Merkle Patricia Trie.
///
/// This is the new arena-based implementation that stores all nodes in a flat vector
/// and uses indices instead of boxed pointers for better memory layout and performance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArenaBasedMptNode {
    nodes: Vec<ArenaNodeData>,
    cached_references: Vec<RefCell<Option<MptNodeReference>>>,
    root_id: NodeId,
}

impl ser::Serialize for ArenaBasedMptNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // Serialize as a compact RLP blob with ALL children inlined!
        // This is much smaller and faster than serializing the arena structure
        OptimizedBytes(self.to_full_rlp()).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for ArenaBasedMptNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // Deserialize the RLP blob and use our fast streaming decoder!
        let rlp_blob: OptimizedBytes = OptimizedBytes::deserialize(deserializer)?;
        ArenaBasedMptNode::decode_from_rlp(&rlp_blob.0).map_err(de::Error::custom)
    }
}

impl Default for ArenaBasedMptNode {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let mut cached_references = Vec::new();

        // Add the initial null node
        nodes.push(ArenaNodeData::Null);
        cached_references.push(RefCell::new(None));

        Self { nodes, cached_references, root_id: 0 }
    }
}

/// Node data for arena-based trie
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ArenaNodeData {
    #[default]
    Null,
    Branch([Option<NodeId>; 16]),
    Leaf(SmallVec<[u8; 64]>, Vec<u8>),     // (nibbles, value)
    Extension(SmallVec<[u8; 64]>, NodeId), // (nibbles, child_id)
    Digest(B256),
}

/// Custom RLP decoder that builds ArenaBasedMptNode directly without intermediate boxed structures
impl ArenaBasedMptNode {
    /// Creates a new arena with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        let mut cached_references = Vec::with_capacity(capacity);

        // Add the initial null node
        nodes.push(ArenaNodeData::Null);
        cached_references.push(RefCell::new(None));

        Self { nodes, cached_references, root_id: 0 }
    }

    /// Decodes an RLP-encoded node directly into an ArenaBasedMptNode
    pub fn decode_from_rlp(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();
        // Heuristic: average node is ~40 bytes, so estimate node count
        let estimated_nodes = (bytes.len() / 40).max(16);
        let mut arena = ArenaBasedMptNode::with_capacity(estimated_nodes);

        let mut buf = bytes;
        let root_id = arena.decode_node_recursive(&mut buf)?;
        if !buf.is_empty() {
            return Err(Error::Rlp(alloy_rlp::Error::Custom("trailing data")));
        }
        arena.root_id = root_id;
        Ok(arena)
    }

    fn decode_node_recursive(&mut self, buf: &mut &[u8]) -> Result<NodeId, Error> {
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

        // Extract the list payload
        let mut payload_buf = &buf[..header.payload_length];
        buf.advance(header.payload_length);

        // Probe the first two items to determine if this is a 2-item or 17-item node
        // without scanning the entire payload
        let mut probe_buf = payload_buf;

        // Try to parse the first item from the probe
        let h1 = alloy_rlp::Header::decode(&mut probe_buf).map_err(Error::Rlp)?;
        if probe_buf.len() < h1.payload_length {
            return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
        }
        probe_buf.advance(h1.payload_length);

        // Try to parse the second item
        let h2 = alloy_rlp::Header::decode(&mut probe_buf).map_err(Error::Rlp)?;
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
            for child_opt in children.iter_mut() {
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                if child_id != 0 {
                    *child_opt = Some(child_id);
                }
            }

            // Manually decode the final value (should be empty for MPT)
            let value_header = alloy_rlp::Header::decode(&mut payload_buf).map_err(Error::Rlp)?;
            if value_header.list {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for value")));
            }
            if payload_buf.len() < value_header.payload_length {
                return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
            }
            let value = payload_buf[..value_header.payload_length].to_vec();
            payload_buf.advance(value_header.payload_length);

            if !value.is_empty() {
                return Err(Error::ValueInBranch);
            }

            Ok(self.add_node(ArenaNodeData::Branch(children)))
        } else {
            // Leaf or Extension node (2 items)
            // Either extension or leaf node
            // Manually decode the first item (path)
            let path_header = alloy_rlp::Header::decode(&mut payload_buf).map_err(Error::Rlp)?;
            if path_header.list {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for path")));
            }
            if payload_buf.len() < path_header.payload_length {
                return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
            }
            let path = payload_buf[..path_header.payload_length].to_vec();
            payload_buf.advance(path_header.payload_length);

            if path.is_empty() {
                return Err(Error::Rlp(alloy_rlp::Error::Custom("empty path")));
            }

            let prefix = path[0];
            if (prefix & 0x20) != 0 {
                // Leaf node (prefix 0x20 or 0x21)
                // Manually decode the second item (value)
                let value_header =
                    alloy_rlp::Header::decode(&mut payload_buf).map_err(Error::Rlp)?;
                if value_header.list {
                    return Err(Error::Rlp(alloy_rlp::Error::Custom("expected string for value")));
                }
                if payload_buf.len() < value_header.payload_length {
                    return Err(Error::Rlp(alloy_rlp::Error::InputTooShort));
                }
                let value = payload_buf[..value_header.payload_length].to_vec();
                payload_buf.advance(value_header.payload_length);

                // Convert encoded path to nibbles for internal storage
                let nibbles = prefix_nibs(&path);
                Ok(self.add_node(ArenaNodeData::Leaf(nibbles, value)))
            } else {
                // Extension node (prefix 0x00 or 0x01)
                let child_id = self.decode_node_recursive(&mut payload_buf)?;
                // Convert encoded path to nibbles for internal storage
                let nibbles = prefix_nibs(&path);
                Ok(self.add_node(ArenaNodeData::Extension(nibbles, child_id)))
            }
        }
    }
}

impl ArenaBasedMptNode {
    fn new(root_id: NodeId, nodes: Vec<ArenaNodeData>) -> Self {
        let cached_references = (0..nodes.len()).map(|_| RefCell::new(None)).collect();
        Self { nodes, cached_references, root_id }
    }

    fn add_node(&mut self, data: ArenaNodeData) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(data);
        self.cached_references.push(RefCell::new(None));
        id
    }

    /// Computes and returns the 256-bit hash of the node.
    #[inline]
    pub fn hash(&self) -> B256 {
        self.hash_id(self.root_id)
    }

    fn hash_id(&self, node_id: NodeId) -> B256 {
        match self.nodes[node_id] {
            ArenaNodeData::Null => EMPTY_ROOT,
            _ => {
                match self.cached_references[node_id]
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
        match &self.nodes[node_id] {
            ArenaNodeData::Null => MptNodeReference::Bytes(vec![alloy_rlp::EMPTY_STRING_CODE]),
            ArenaNodeData::Digest(digest) => MptNodeReference::Digest(*digest),
            _ => {
                let payload_length = self.payload_length_id(node_id);
                let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

                let mut encoded = Vec::with_capacity(rlp_length);
                self.encode_id_with_payload_len(node_id, payload_length, &mut encoded);
                debug_assert_eq!(encoded.len(), rlp_length);

                if rlp_length < 32 {
                    MptNodeReference::Bytes(encoded)
                } else {
                    MptNodeReference::Digest(keccak256(encoded))
                }
            }
        }
    }

    fn encode_id(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.payload_length_id(node_id);
        self.encode_id_with_payload_len(node_id, payload_length, out);
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
        match &self.nodes[node_id] {
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
            ArenaNodeData::Leaf(path_nibs, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                let encoded_path = to_encoded_path(path_nibs, true);
                encoded_path.as_slice().encode(out);
                value.as_slice().encode(out);
            }
            ArenaNodeData::Extension(path_nibs, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                let encoded_path = to_encoded_path(path_nibs, false);
                encoded_path.as_slice().encode(out);
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
        match &self.nodes[node_id] {
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
            ArenaNodeData::Leaf(path_nibs, value) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                let encoded_path = to_encoded_path(path_nibs, true);
                encoded_path.as_slice().encode(out);
                value.as_slice().encode(out);
            }
            ArenaNodeData::Extension(path_nibs, child_id) => {
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                let encoded_path = to_encoded_path(path_nibs, false);
                encoded_path.as_slice().encode(out);
                self.encode_full(*child_id, out); // INLINE child, never use digest!
            }
            ArenaNodeData::Digest(digest) => {
                // Keep digest nodes as-is (they represent external/unresolved nodes)
                digest.encode(out);
            }
        }
    }

    fn reference_encode_id(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        match self.cached_references[node_id]
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

    fn reference_length(&self, node_id: NodeId) -> usize {
        match self.cached_references[node_id]
            .borrow_mut()
            .get_or_insert_with(|| self.calc_reference(node_id))
        {
            MptNodeReference::Bytes(bytes) => bytes.len(),
            MptNodeReference::Digest(_) => 1 + 32,
        }
    }

    /// Determines if the trie is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(&self.nodes[self.root_id], ArenaNodeData::Null)
    }

    /// Returns the RLP-encoded bytes of this trie
    #[inline]
    pub fn to_rlp_bytes(&self) -> Vec<u8> {
        self.to_rlp_id(self.root_id)
    }

    /// Returns the RLP-encoded bytes with ALL children inlined (never replaced by digest).
    /// This produces a compact, fully-expanded representation perfect for serialization.
    #[inline]
    pub fn to_full_rlp(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.approx_full_rlp_len());
        self.encode_full(self.root_id, &mut out);
        out
    }

    /// Estimates the upper bound for full RLP length (for Vec capacity)
    fn approx_full_rlp_len(&self) -> usize {
        // Rough estimate: each node ~100 bytes average, plus some overhead
        self.nodes.len() * 100
    }

    /// Clears the trie, replacing its data with an empty node.
    #[inline]
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.cached_references.clear();
        self.nodes.push(ArenaNodeData::Null);
        self.cached_references.push(RefCell::new(None));
        self.root_id = 0;
    }

    /// Retrieves the value associated with a given key in the trie.
    #[inline]
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.get_internal(self.root_id, &to_nibs(key))
    }

    /// Retrieves the RLP-decoded value corresponding to the key.
    #[inline]
    pub fn get_rlp<T: alloy_rlp::Decodable>(&self, key: &[u8]) -> Result<Option<T>, Error> {
        match self.get(key)? {
            Some(bytes) => Ok(Some(T::decode(&mut bytes.as_slice())?)),
            None => Ok(None),
        }
    }

    fn get_internal(&self, node_id: NodeId, key_nibs: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        match &self.nodes[node_id] {
            ArenaNodeData::Null => Ok(None),
            ArenaNodeData::Branch(nodes) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match nodes[*i as usize] {
                        Some(id) => self.get_internal(id, tail),
                        None => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Leaf(path_nibs, value) => {
                if path_nibs.as_slice() == key_nibs {
                    Ok(Some(value.clone()))
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Extension(path_nibs, child_id) => {
                if let Some(tail) = key_nibs.strip_prefix(path_nibs.as_slice()) {
                    self.get_internal(*child_id, tail)
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(*digest)),
        }
    }

    /// Inserts a key-value pair into the trie.
    #[inline]
    pub fn insert(&mut self, key: &[u8], value: Vec<u8>) -> Result<bool, Error> {
        if value.is_empty() {
            panic!("value must not be empty");
        }
        self.insert_internal(&to_nibs(key), value)
    }

    /// Inserts an RLP-encoded value into the trie.
    #[inline]
    pub fn insert_rlp(&mut self, key: &[u8], value: impl Encodable) -> Result<bool, Error> {
        let mut rlp_bytes = Vec::new();
        value.encode(&mut rlp_bytes);
        self.insert_internal(&to_nibs(key), rlp_bytes)
    }

    /// Removes a key from the trie.
    ///
    /// This method attempts to remove a key-value pair from the trie. If the key is
    /// present, it returns `true`. Otherwise, it returns `false`.
    #[inline]
    pub fn delete(&mut self, key: &[u8]) -> Result<bool, Error> {
        self.delete_internal(&to_nibs(key))
    }

    fn delete_internal(&mut self, key_nibs: &[u8]) -> Result<bool, Error> {
        let (updated, new_root_id) = self.delete_recursive(self.root_id, key_nibs)?;
        self.root_id = new_root_id;
        if updated {
            self.cached_references[self.root_id].borrow_mut().take();
        }
        Ok(updated)
    }

    fn delete_recursive(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
    ) -> Result<(bool, NodeId), Error> {
        let node_data = self.nodes[node_id].clone();
        match node_data {
            ArenaNodeData::Null => Ok((false, node_id)),
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            let (updated, new_child_id) = self.delete_recursive(id, tail)?;
                            if !updated {
                                return Ok((false, node_id));
                            }
                            children[*i as usize] = Some(new_child_id);

                            if matches!(self.nodes[new_child_id], ArenaNodeData::Null) {
                                children[*i as usize] = None;
                            }
                        }
                        None => return Ok((false, node_id)),
                    }
                } else {
                    return Err(Error::ValueInBranch);
                }

                let remaining: Vec<_> =
                    children.iter().enumerate().filter(|(_, n)| n.is_some()).collect();

                // if there is only exactly one node left, we need to convert the branch
                let new_node_data = if remaining.len() == 1 {
                    let (index, &child_id) = remaining[0];
                    let child_id = child_id.unwrap();
                    let child_data = self.nodes[child_id].clone();

                    match child_data {
                        // if the orphan is a leaf, prepend the corresponding nib to it
                        ArenaNodeData::Leaf(prefix_nibs, value) => {
                            let mut new_nibs = SmallVec::with_capacity(1 + prefix_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&prefix_nibs);
                            ArenaNodeData::Leaf(new_nibs, value)
                        }
                        // if the orphan is an extension, prepend the corresponding nib to it
                        ArenaNodeData::Extension(prefix_nibs, child_child_id) => {
                            let mut new_nibs = SmallVec::with_capacity(1 + prefix_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&prefix_nibs);
                            ArenaNodeData::Extension(new_nibs, child_child_id)
                        }
                        // if the orphan is a branch or digest, convert to an extension
                        ArenaNodeData::Branch(_) | ArenaNodeData::Digest(_) => {
                            let mut ext_nibs = SmallVec::new();
                            ext_nibs.push(index as u8);
                            ArenaNodeData::Extension(ext_nibs, child_id)
                        }
                        ArenaNodeData::Null => unreachable!(),
                    }
                } else {
                    // Update the branch with the modified children
                    ArenaNodeData::Branch(children)
                };

                let new_node_id = self.add_node(new_node_data);
                self.cached_references[new_node_id].borrow_mut().take();
                Ok((true, new_node_id))
            }
            ArenaNodeData::Leaf(prefix_nibs, _) => {
                if prefix_nibs.as_slice() != key_nibs {
                    return Ok((false, node_id));
                }
                let new_node_id = self.add_node(ArenaNodeData::Null);
                self.cached_references[new_node_id].borrow_mut().take();
                Ok((true, new_node_id))
            }
            ArenaNodeData::Extension(path_nibs, child_id) => {
                let (updated, new_child_id) =
                    if let Some(tail) = key_nibs.strip_prefix(path_nibs.as_slice()) {
                        let (updated, new_child_id) = self.delete_recursive(child_id, tail)?;
                        if !updated {
                            return Ok((false, node_id));
                        }
                        (true, new_child_id)
                    } else {
                        return Ok((false, node_id));
                    };

                // an extension can only point to a branch or a digest; since its sub trie was
                // modified, we need to make sure that this property still holds
                let child_data = self.nodes[new_child_id].clone();
                let new_node_data = match child_data {
                    // if the child is empty, remove the extension
                    ArenaNodeData::Null => ArenaNodeData::Null,
                    // for a leaf, replace the extension with the extended leaf
                    ArenaNodeData::Leaf(child_path_nibs, value) => {
                        let mut combined_nibs =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        ArenaNodeData::Leaf(combined_nibs, value)
                    }
                    // for an extension, replace the extension with the extended extension
                    ArenaNodeData::Extension(child_path_nibs, grandchild_id) => {
                        let mut combined_nibs =
                            SmallVec::with_capacity(path_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&path_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        ArenaNodeData::Extension(combined_nibs, grandchild_id)
                    }
                    // for a branch or digest, the extension is still correct
                    ArenaNodeData::Branch(_) | ArenaNodeData::Digest(_) => {
                        ArenaNodeData::Extension(path_nibs, new_child_id)
                    }
                };

                let new_node_id = self.add_node(new_node_data);
                self.cached_references[new_node_id].borrow_mut().take();
                Ok((updated, new_node_id))
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }
    }

    fn insert_internal(&mut self, key_nibs: &[u8], value: Vec<u8>) -> Result<bool, Error> {
        let (updated, new_root_id) = self.insert_recursive(self.root_id, key_nibs, value)?;
        self.root_id = new_root_id;
        if updated {
            self.cached_references[self.root_id].borrow_mut().take();
        }
        Ok(updated)
    }

    fn insert_recursive(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
        value: Vec<u8>,
    ) -> Result<(bool, NodeId), Error> {
        let node_data = self.nodes[node_id].clone();
        match node_data {
            ArenaNodeData::Null => {
                let path_nibs = SmallVec::from_slice(key_nibs);
                let new_node_id = self.add_node(ArenaNodeData::Leaf(path_nibs, value));
                Ok((true, new_node_id))
            }
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            let (updated, new_child_id) = self.insert_recursive(id, tail, value)?;
                            if updated {
                                children[*i as usize] = Some(new_child_id);
                                let new_node_id = self.add_node(ArenaNodeData::Branch(children));
                                self.cached_references[new_node_id].borrow_mut().take();
                                Ok((true, new_node_id))
                            } else {
                                Ok((false, node_id))
                            }
                        }
                        None => {
                            let path_nibs = SmallVec::from_slice(tail);
                            let new_leaf_id = self.add_node(ArenaNodeData::Leaf(path_nibs, value));
                            children[*i as usize] = Some(new_leaf_id);
                            let new_node_id = self.add_node(ArenaNodeData::Branch(children));
                            self.cached_references[new_node_id].borrow_mut().take();
                            Ok((true, new_node_id))
                        }
                    }
                } else {
                    Err(Error::ValueInBranch)
                }
            }
            ArenaNodeData::Leaf(path_nibs, old_value) => {
                let common_len = lcp(&path_nibs, key_nibs);
                if common_len == path_nibs.len() && common_len == key_nibs.len() {
                    // if path_nibs == key_nibs, update the value if it is different
                    if old_value == value {
                        return Ok((false, node_id));
                    }
                    let new_node_id = self.add_node(ArenaNodeData::Leaf(path_nibs, value));
                    self.cached_references[new_node_id].borrow_mut().take();
                    Ok((true, new_node_id))
                } else if common_len == path_nibs.len() || common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    // create a branch with two children
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    let leaf1_path = SmallVec::from_slice(&path_nibs[split_point..]);
                    let leaf1_id = self.add_node(ArenaNodeData::Leaf(leaf1_path, old_value));

                    let leaf2_path = SmallVec::from_slice(&key_nibs[split_point..]);
                    let leaf2_id = self.add_node(ArenaNodeData::Leaf(leaf2_path, value));

                    children[path_nibs[common_len] as usize] = Some(leaf1_id);
                    children[key_nibs[common_len] as usize] = Some(leaf2_id);

                    let new_node_data = if common_len > 0 {
                        // create parent extension for new branch
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        let ext_path = SmallVec::from_slice(&path_nibs[..common_len]);
                        ArenaNodeData::Extension(ext_path, branch_id)
                    } else {
                        ArenaNodeData::Branch(children)
                    };
                    let new_node_id = self.add_node(new_node_data);
                    self.cached_references[new_node_id].borrow_mut().take();
                    Ok((true, new_node_id))
                }
            }
            ArenaNodeData::Extension(path_nibs, child_id) => {
                let common_len = lcp(&path_nibs, key_nibs);
                if common_len == path_nibs.len() {
                    // traverse down for update
                    let (updated, new_child_id) =
                        self.insert_recursive(child_id, &key_nibs[common_len..], value)?;
                    if updated {
                        let new_node_id =
                            self.add_node(ArenaNodeData::Extension(path_nibs, new_child_id));
                        self.cached_references[new_node_id].borrow_mut().take();
                        Ok((true, new_node_id))
                    } else {
                        Ok((false, node_id))
                    }
                } else if common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    // create a branch with two children
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    if split_point < path_nibs.len() {
                        let ext_path = SmallVec::from_slice(&path_nibs[split_point..]);
                        let ext_id = self.add_node(ArenaNodeData::Extension(ext_path, child_id));
                        children[path_nibs[common_len] as usize] = Some(ext_id);
                    } else {
                        children[path_nibs[common_len] as usize] = Some(child_id);
                    }

                    let leaf_path = SmallVec::from_slice(&key_nibs[split_point..]);
                    let leaf_id = self.add_node(ArenaNodeData::Leaf(leaf_path, value));
                    children[key_nibs[common_len] as usize] = Some(leaf_id);

                    let new_node_data = if common_len > 0 {
                        // Create parent extension for new branch
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        let parent_ext_path = SmallVec::from_slice(&path_nibs[..common_len]);
                        ArenaNodeData::Extension(parent_ext_path, branch_id)
                    } else {
                        ArenaNodeData::Branch(children)
                    };
                    let new_node_id = self.add_node(new_node_data);
                    self.cached_references[new_node_id].borrow_mut().take();
                    Ok((true, new_node_id))
                }
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
        }
    }

    fn payload_length_id(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
                    .map(|child| child.map_or(1, |id| self.reference_length(id)))
                    .sum::<usize>()
            }
            ArenaNodeData::Leaf(path_nibs, value) => {
                let encoded_path = to_encoded_path(path_nibs, true);
                encoded_path.as_slice().length() + value.as_slice().length()
            }
            ArenaNodeData::Extension(path_nibs, node_id) => {
                let encoded_path = to_encoded_path(path_nibs, false);
                encoded_path.as_slice().length() + self.reference_length(*node_id)
            }
            ArenaNodeData::Digest(_) => 32,
        }
    }

    /// Calculates payload length for full inline encoding (never using digest references)
    fn payload_length_full(&self, node_id: NodeId) -> usize {
        match &self.nodes[node_id] {
            ArenaNodeData::Null => 0,
            ArenaNodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
                    .map(|child| child.map_or(1, |id| self.full_length(id)))
                    .sum::<usize>()
            }
            ArenaNodeData::Leaf(path_nibs, value) => {
                let encoded_path = to_encoded_path(path_nibs, true);
                encoded_path.as_slice().length() + value.as_slice().length()
            }
            ArenaNodeData::Extension(path_nibs, node_id) => {
                let encoded_path = to_encoded_path(path_nibs, false);
                encoded_path.as_slice().length() + self.full_length(*node_id)
            }
            ArenaNodeData::Digest(_) => 32, // Keep digests as-is
        }
    }

    /// Returns the full RLP length when inlined (never using digest references)
    fn full_length(&self, node_id: NodeId) -> usize {
        let payload_length = self.payload_length_full(node_id);
        payload_length + alloy_rlp::length_of_length(payload_length)
    }

    /// Recursively collects all RLP-encoded trie nodes into the provided HashMap.
    /// Each node is keyed by its Keccak-256 hash to avoid duplicates.
    pub fn rlp_nodes(&self, nodes: &mut HashMap<B256, Vec<u8>>) {
        self.rlp_nodes_recursive(self.root_id, nodes);
    }

    fn rlp_nodes_recursive(&self, node_id: NodeId, nodes: &mut HashMap<B256, Vec<u8>>) {
        let rlp_bytes = self.to_rlp_id(node_id);
        let hash = B256::from(keccak256(rlp_bytes.as_slice()));

        // Insert this node into the map (avoiding duplicates)
        if nodes.contains_key(&hash) {
            return;
        }
        nodes.insert(hash, rlp_bytes);

        // Recursively process child nodes
        match &self.nodes[node_id] {
            ArenaNodeData::Branch(children) => {
                for child in children.iter().flatten() {
                    self.rlp_nodes_recursive(*child, nodes);
                }
            }
            ArenaNodeData::Extension(_, child) => {
                self.rlp_nodes_recursive(*child, nodes);
            }
            ArenaNodeData::Leaf(_, _) | ArenaNodeData::Null | ArenaNodeData::Digest(_) => {
                // No child nodes to process
            }
        }
    }

    fn to_rlp_id(&self, node_id: NodeId) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_id(node_id, &mut out);
        out
    }

    /// Merges another trie into this arena and returns the root NodeId of the merged trie
    fn merge_trie(&mut self, other: &ArenaBasedMptNode) -> NodeId {
        let mut node_mapping = HashMap::new();

        // Copy all nodes from other trie into this arena
        for (other_id, other_node) in other.nodes.iter().enumerate() {
            let new_id = self.add_node(other_node.clone());
            node_mapping.insert(other_id, new_id);
        }

        // Update all the node references in the copied nodes
        for (&other_id, &new_id) in &node_mapping {
            match &other.nodes[other_id] {
                ArenaNodeData::Branch(children) => {
                    let mut new_children: [Option<NodeId>; 16] = Default::default();
                    for (i, child) in children.iter().enumerate() {
                        if let Some(child_id) = child {
                            new_children[i] = Some(node_mapping[child_id]);
                        }
                    }
                    self.nodes[new_id] = ArenaNodeData::Branch(new_children);
                }
                ArenaNodeData::Extension(prefix, child_id) => {
                    let new_child_id = node_mapping[child_id];
                    self.nodes[new_id] = ArenaNodeData::Extension(prefix.clone(), new_child_id);
                }
                _ => {} // Leaf, Null, and Digest nodes don't need reference updates
            }
        }

        // Return the root of the merged trie
        node_mapping[&other.root_id]
    }
}

/// Parses proof bytes into a vector of ArenaBasedMptNodes.
pub fn parse_proof(proof: &[impl AsRef<[u8]>]) -> Result<Vec<ArenaBasedMptNode>, Error> {
    Ok(proof
        .iter()
        .map(|bytes| ArenaBasedMptNode::decode_from_rlp(bytes))
        .collect::<Result<Vec<_>, _>>()?)
}

/// Creates an arena-based Merkle Patricia trie from an EIP-1186 proof.
pub fn mpt_from_proof(proof_nodes: &[ArenaBasedMptNode]) -> Result<ArenaBasedMptNode, Error> {
    if proof_nodes.is_empty() {
        return Ok(ArenaBasedMptNode::default());
    }

    let mut next: Option<ArenaBasedMptNode> = None;
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
        let resolved_root_data = &resolved.nodes[resolved.root_id].clone();

        match resolved_root_data {
            ArenaNodeData::Branch(children) => {
                let mut new_children = children.clone();
                let mut found = false;

                for (child_idx, child_id) in children.iter().enumerate() {
                    if let Some(child_id) = child_id {
                        if let ArenaNodeData::Digest(digest) = &resolved.nodes[*child_id] {
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

                resolved.nodes[resolved.root_id] = ArenaNodeData::Branch(new_children);
                resolved.cached_references[resolved.root_id].borrow_mut().take();
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                if let ArenaNodeData::Digest(digest) = &resolved.nodes[*child_id] {
                    if *digest != replacement_hash {
                        panic!("node {} does not reference the successor", i);
                    }
                    // Replace the child with the replacement trie
                    let replacement_root_in_resolved = resolved.merge_trie(&replacement);
                    resolved.nodes[resolved.root_id] =
                        ArenaNodeData::Extension(prefix.clone(), replacement_root_in_resolved);
                    resolved.cached_references[resolved.root_id].borrow_mut().take();
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

/// Creates a new arena-based MPT node from a digest.
fn node_from_digest(digest: B256) -> ArenaBasedMptNode {
    match digest {
        EMPTY_ROOT | B256::ZERO => ArenaBasedMptNode::default(),
        _ => {
            let mut trie = ArenaBasedMptNode::default();
            trie.nodes[0] = ArenaNodeData::Digest(digest);
            trie
        }
    }
}

/// Resolves digest nodes in an arena-based trie using the provided node store.
/// This rebuilds the arena, replacing any digest nodes with their resolved content.
pub fn resolve_nodes_arena(
    root: &ArenaBasedMptNode,
    node_store: &HashMap<MptNodeReference, ArenaBasedMptNode>,
) -> ArenaBasedMptNode {
    let mut new_arena =
        ArenaBasedMptNode { nodes: Vec::new(), cached_references: Vec::new(), root_id: 0 };

    let root_id = resolve_node_recursive(root, root.root_id, node_store, &mut new_arena);
    new_arena.root_id = root_id;

    // The root hash must not change after resolution
    debug_assert_eq!(root.hash(), new_arena.hash());

    new_arena
}

/// Recursively resolves a single node and its children, adding them to the new arena.
fn resolve_node_recursive(
    original_arena: &ArenaBasedMptNode,
    node_id: NodeId,
    node_store: &HashMap<MptNodeReference, ArenaBasedMptNode>,
    new_arena: &mut ArenaBasedMptNode,
) -> NodeId {
    let node_data = &original_arena.nodes[node_id];

    let resolved_data = match node_data {
        ArenaNodeData::Null => ArenaNodeData::Null,
        ArenaNodeData::Leaf(prefix, value) => ArenaNodeData::Leaf(prefix.clone(), value.clone()),
        ArenaNodeData::Branch(children) => {
            let mut resolved_children: [Option<NodeId>; 16] = Default::default();
            for (i, child_id) in children.iter().enumerate() {
                if let Some(child_id) = child_id {
                    let resolved_child_id =
                        resolve_node_recursive(original_arena, *child_id, node_store, new_arena);
                    resolved_children[i] = Some(resolved_child_id);
                }
            }
            ArenaNodeData::Branch(resolved_children)
        }
        ArenaNodeData::Extension(prefix, child_id) => {
            let resolved_child_id =
                resolve_node_recursive(original_arena, *child_id, node_store, new_arena);
            ArenaNodeData::Extension(prefix.clone(), resolved_child_id)
        }
        ArenaNodeData::Digest(digest) => {
            // Try to resolve the digest from the node store
            if let Some(resolved_node) = node_store.get(&MptNodeReference::Digest(*digest)) {
                // Convert the resolved node to arena format and add it
                return resolve_node_recursive(
                    &resolved_node,
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

    let mut storage: HashMap<B256, ArenaBasedMptNode> = HashMap::default();

    let mut state_nodes = HashMap::<MptNodeReference, ArenaBasedMptNode>::default();
    let mut state_root_node = ArenaBasedMptNode::default();

    for (address, proof) in parent_proofs {
        let proof_nodes = parse_proof(&proof.proof)?;

        // The first node in the proof is the root
        if let Some(node) = proof_nodes.first() {
            state_root_node = node.clone();
        }

        proof_nodes.into_iter().for_each(|node| {
            // Use a digest reference based on the node's hash
            state_nodes.insert(MptNodeReference::Digest(node.hash()), node);
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

        let mut storage_nodes = HashMap::<MptNodeReference, ArenaBasedMptNode>::default();
        let mut storage_root_node = ArenaBasedMptNode::default();

        for storage_proof in &proof.storage_proofs {
            let proof_nodes = parse_proof(&storage_proof.proof)?;

            // The first node in the proof is the root
            if let Some(node) = proof_nodes.first() {
                storage_root_node = node.clone();
            }

            proof_nodes.into_iter().for_each(|node| {
                // Use a digest reference based on the node's hash
                storage_nodes.insert(MptNodeReference::Digest(node.hash()), node);
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

/// Adds all the leaf nodes of non-inclusion proofs to the nodes.
fn add_orphaned_leafs(
    key: impl AsRef<[u8]>,
    proof: &[impl AsRef<[u8]>],
    nodes_by_reference: &mut HashMap<MptNodeReference, ArenaBasedMptNode>,
) -> Result<(), Error> {
    if !proof.is_empty() {
        let proof_nodes = parse_proof(proof)?;
        if is_not_included(keccak256(key).as_slice(), &proof_nodes)? {
            // Add the leaf node to the nodes
            let leaf = proof_nodes.last().unwrap();
            shorten_node_path_arena(leaf).into_iter().for_each(|node| {
                nodes_by_reference.insert(MptNodeReference::Digest(node.hash()), node);
            });
        }
    }

    Ok(())
}

/// Verifies that the given proof is a valid proof of exclusion for the given key.
pub fn is_not_included(key: &[u8], proof_nodes: &[ArenaBasedMptNode]) -> Result<bool, Error> {
    let proof_trie = mpt_from_proof(proof_nodes)?;
    // For valid proofs, the get must not fail
    let value = proof_trie.get(key)?;

    Ok(value.is_none())
}

/// Returns a list of all possible nodes that can be created by shortening the path of the given
/// node.
pub fn shorten_node_path_arena(node: &ArenaBasedMptNode) -> Vec<ArenaBasedMptNode> {
    let mut res = Vec::new();
    let nibs = match &node.nodes[node.root_id] {
        ArenaNodeData::Leaf(path_nibs, _) | ArenaNodeData::Extension(path_nibs, _) => {
            path_nibs.clone()
        }
        _ => SmallVec::new(),
    };

    match &node.nodes[node.root_id] {
        ArenaNodeData::Null | ArenaNodeData::Branch(_) | ArenaNodeData::Digest(_) => {}
        ArenaNodeData::Leaf(_, value) => {
            for i in 0..=nibs.len() {
                let mut new_node = ArenaBasedMptNode::default();
                let shortened_path = SmallVec::from_slice(&nibs[i..]);
                new_node.nodes[0] = ArenaNodeData::Leaf(shortened_path, value.clone());
                res.push(new_node);
            }
        }
        ArenaNodeData::Extension(_, child_id) => {
            for i in 0..=nibs.len() {
                let mut new_node = ArenaBasedMptNode::default();
                let shortened_path = SmallVec::from_slice(&nibs[i..]);
                new_node.nodes[0] = ArenaNodeData::Extension(shortened_path, *child_id);
                res.push(new_node);
            }
        }
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    pub fn test_empty() {
        let trie = ArenaBasedMptNode::default();

        assert!(trie.is_empty());
        let expected = hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
        assert_eq!(expected, trie.hash().0);
    }

    #[test]
    pub fn test_clear() {
        let mut trie = ArenaBasedMptNode::default();
        trie.insert(b"dog", b"puppy".to_vec()).unwrap();
        assert!(!trie.is_empty());
        assert_ne!(trie.hash(), EMPTY_ROOT);

        trie.clear();
        assert!(trie.is_empty());
        assert_eq!(trie.hash(), EMPTY_ROOT);
    }

    #[test]
    pub fn test_insert() {
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
            assert!(trie.insert(key.as_bytes(), val.as_bytes().to_vec()).unwrap());
        }

        let expected = hex!("2bab6cdf91a23ebf3af683728ea02403a98346f99ed668eec572d55c70a4b08f");
        assert_eq!(expected, trie.hash().0);

        for (key, value) in &vals {
            let retrieved = trie.get(key.as_bytes()).unwrap().unwrap();
            assert_eq!(retrieved, value.as_bytes());
        }

        // check inserting duplicate keys
        assert!(trie.insert(vals[0].0.as_bytes(), b"new".to_vec()).unwrap());
        assert!(!trie.insert(vals[0].0.as_bytes(), b"new".to_vec()).unwrap());
    }

    #[test]
    pub fn test_direct_rlp_decoding() {
        // Test that we can decode RLP directly into ArenaBasedMptNode
        let mut trie = ArenaBasedMptNode::default();
        trie.insert(b"test", b"value".to_vec()).unwrap();

        let rlp_bytes = trie.to_rlp_id(trie.root_id);
        let decoded = ArenaBasedMptNode::decode_from_rlp(&rlp_bytes).unwrap();

        // The decoded trie should have the same hash as the original
        assert_eq!(trie.hash(), decoded.hash());
    }

    #[test]
    pub fn test_mpt_from_proof_reconstruction() {
        // Create a test proof scenario
        // This mimics how proofs work: we have a sequence of nodes where later nodes
        // reference earlier nodes by digest

        // Create the deepest node (a leaf) - using nibbles directly
        let mut leaf_trie = ArenaBasedMptNode::default();
        leaf_trie.nodes[0] =
            ArenaNodeData::Leaf(SmallVec::from_slice(&[0x03]), b"test_value".to_vec());

        // Create a parent extension that references the leaf by digest
        let mut ext_trie = ArenaBasedMptNode::default();
        let leaf_digest = leaf_trie.hash();
        ext_trie.nodes[0] = ArenaNodeData::Digest(leaf_digest);
        let ext_child_id = ext_trie.add_node(ArenaNodeData::Digest(leaf_digest));
        ext_trie.nodes[0] = ArenaNodeData::Extension(SmallVec::from_slice(&[0x01]), ext_child_id);
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
