#![allow(unreachable_pub)]
#![allow(dead_code)]

use alloy_primitives::B256;
use alloy_rlp::Encodable;
use core::{cell::RefCell, fmt::Debug};
use reth_trie::AccountProof;
use revm::primitives::HashMap;
use revm_primitives::{keccak256, Address};
use serde::{Deserialize, Serialize};

use rlp::{Decodable, DecoderError, Prototype, Rlp};

use eyre::Result;

use crate::{
    mpt::{Error, MptNodeReference, EMPTY_ROOT},
    utils::{lcp, prefix_nibs, to_encoded_path, to_nibs},
    EthereumState, EthereumState2, StorageTries, StorageTries2,
};

pub type NodeId = usize;

/// Represents the root node of a sparse Merkle Patricia Trie.
///
/// This is the new arena-based implementation that stores all nodes in a flat vector
/// and uses indices instead of boxed pointers for better memory layout and performance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaBasedMptNode {
    nodes: Vec<ArenaNodeData>,
    cached_references: Vec<RefCell<Option<MptNodeReference>>>,
    root_id: NodeId,
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
    Leaf(Vec<u8>, Vec<u8>),
    Extension(Vec<u8>, NodeId),
    Digest(B256),
}

impl From<PreMptNodeData> for PreMptNode {
    fn from(value: PreMptNodeData) -> Self {
        Self { data: value, cached_reference: RefCell::new(None) }
    }
}

// TODO: migrate to alloy_rlp
impl Decodable for PreMptNode {
    /// Decodes an RLP-encoded node from the provided `rlp` buffer.
    ///
    /// The method handles different RLP prototypes and reconstructs the `MptNode` based
    /// on the encoded data. If the RLP data does not match any known prototype or if
    /// there's an error during decoding, an error is returned.
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        match rlp.prototype()? {
            Prototype::Null | Prototype::Data(0) => Ok(PreMptNodeData::Null.into()),
            Prototype::List(2) => {
                let path: Vec<u8> = rlp.val_at(0)?;
                let prefix = path[0];
                if (prefix & (2 << 4)) == 0 {
                    let node: PreMptNode = Decodable::decode(&rlp.at(1)?)?;
                    Ok(PreMptNodeData::Extension(path, Box::new(node)).into())
                } else {
                    Ok(PreMptNodeData::Leaf(path, rlp.val_at(1)?).into())
                }
            }
            Prototype::List(17) => {
                let mut node_list = Vec::with_capacity(16);
                for node_rlp in rlp.iter().take(16) {
                    match node_rlp.prototype()? {
                        Prototype::Null | Prototype::Data(0) => {
                            node_list.push(None);
                        }
                        _ => node_list.push(Some(Box::new(Decodable::decode(&node_rlp)?))),
                    }
                }
                let value: Vec<u8> = rlp.val_at(16)?;
                if value.is_empty() {
                    Ok(PreMptNodeData::Branch(node_list.try_into().unwrap()).into())
                } else {
                    Err(DecoderError::Custom("branch node with value"))
                }
            }
            Prototype::Data(32) => {
                let bytes: Vec<u8> = rlp.as_val()?;
                Ok(PreMptNodeData::Digest(B256::from_slice(&bytes)).into())
            }
            _ => Err(DecoderError::RlpIncorrectListLen),
        }
    }
}

impl PreMptNode {
    /// Decodes an RLP-encoded [MptNode] from the provided byte slice.
    ///
    /// This method allows for the deserialization of a previously serialized [MptNode].
    #[inline]
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        rlp::decode(bytes.as_ref()).map_err(Error::from)
    }

    /// Computes and returns the 256-bit hash of the node.
    #[inline]
    fn hash(&self) -> B256 {
        match self.data {
            PreMptNodeData::Null => EMPTY_ROOT,
            _ => match self
                .cached_reference
                .borrow_mut()
                .get_or_insert_with(|| self.calc_reference())
            {
                MptNodeReference::Digest(digest) => *digest,
                MptNodeReference::Bytes(bytes) => keccak256(bytes),
            },
        }
    }

    fn calc_reference(&self) -> MptNodeReference {
        match &self.data {
            PreMptNodeData::Null => MptNodeReference::Bytes(vec![alloy_rlp::EMPTY_STRING_CODE]),
            PreMptNodeData::Digest(digest) => MptNodeReference::Digest(*digest),
            _ => {
                let encoded = self.to_rlp();
                if encoded.len() < 32 {
                    MptNodeReference::Bytes(encoded)
                } else {
                    MptNodeReference::Digest(keccak256(encoded))
                }
            }
        }
    }

    fn to_rlp(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode(&mut out);
        out
    }

    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        match &self.data {
            PreMptNodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            PreMptNodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length() }.encode(out);
                nodes.iter().for_each(|child| match child {
                    Some(node) => node.reference_encode(out),
                    None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                });
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            PreMptNodeData::Leaf(prefix, value) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length() }.encode(out);
                prefix.as_slice().encode(out);
                value.as_slice().encode(out);
            }
            PreMptNodeData::Extension(prefix, node) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length() }.encode(out);
                prefix.as_slice().encode(out);
                node.reference_encode(out);
            }
            PreMptNodeData::Digest(digest) => {
                digest.encode(out);
            }
        }
    }

    fn payload_length(&self) -> usize {
        match &self.data {
            PreMptNodeData::Null => 0,
            PreMptNodeData::Branch(nodes) => {
                1 + nodes
                    .iter()
                    .map(|child| child.as_ref().map_or(1, |node| node.reference_length()))
                    .sum::<usize>()
            }
            PreMptNodeData::Leaf(prefix, value) => {
                prefix.as_slice().length() + value.as_slice().length()
            }
            PreMptNodeData::Extension(prefix, node) => {
                prefix.as_slice().length() + node.reference_length()
            }
            PreMptNodeData::Digest(_) => 32,
        }
    }

    fn reference_encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        match self.cached_reference.borrow_mut().get_or_insert_with(|| self.calc_reference()) {
            // if the reference is an RLP-encoded byte slice, copy it directly
            MptNodeReference::Bytes(bytes) => out.put_slice(bytes),
            // if the reference is a digest, RLP-encode it with its fixed known length
            MptNodeReference::Digest(digest) => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE + 32);
                out.put_slice(digest.as_slice());
            }
        }
    }

    fn reference_length(&self) -> usize {
        match self.cached_reference.borrow_mut().get_or_insert_with(|| self.calc_reference()) {
            MptNodeReference::Bytes(bytes) => bytes.len(),
            MptNodeReference::Digest(_) => 1 + 32,
        }
    }

    #[inline]
    fn reference(&self) -> MptNodeReference {
        self.cached_reference.borrow_mut().get_or_insert_with(|| self.calc_reference()).clone()
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

    fn from_pre_node(pre_node: &PreMptNode) -> Self {
        let mut trie = Self { nodes: Vec::new(), cached_references: Vec::new(), root_id: 0 };
        let root_id = trie.add_pre_node_recursive(pre_node);
        trie.root_id = root_id;
        trie
    }

    fn add_pre_node_recursive(&mut self, pre_node: &PreMptNode) -> NodeId {
        let node_data = match &pre_node.data {
            PreMptNodeData::Null => ArenaNodeData::Null,
            PreMptNodeData::Branch(children) => {
                let mut new_children: [Option<NodeId>; 16] = Default::default();
                for (i, child) in children.iter().enumerate() {
                    if let Some(pre_child) = child {
                        new_children[i] = Some(self.add_pre_node_recursive(pre_child));
                    }
                }
                ArenaNodeData::Branch(new_children)
            }
            PreMptNodeData::Leaf(p, v) => ArenaNodeData::Leaf(p.clone(), v.clone()),
            PreMptNodeData::Extension(p, pre_child) => {
                let child_id = self.add_pre_node_recursive(pre_child);
                ArenaNodeData::Extension(p.clone(), child_id)
            }
            PreMptNodeData::Digest(d) => ArenaNodeData::Digest(*d),
        };
        self.add_node(node_data)
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
                let mut encoded = Vec::new();
                self.encode_id(node_id, &mut encoded);
                if encoded.len() < 32 {
                    MptNodeReference::Bytes(encoded)
                } else {
                    MptNodeReference::Digest(keccak256(encoded))
                }
            }
        }
    }

    fn encode_id(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        match &self.nodes[node_id] {
            ArenaNodeData::Null => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Branch(nodes) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length_id(node_id) }
                    .encode(out);
                nodes.iter().for_each(|child_id| match child_id {
                    Some(id) => self.reference_encode_id(*id, out),
                    None => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
                });
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            ArenaNodeData::Leaf(prefix, value) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length_id(node_id) }
                    .encode(out);
                prefix.as_slice().encode(out);
                value.as_slice().encode(out);
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                alloy_rlp::Header { list: true, payload_length: self.payload_length_id(node_id) }
                    .encode(out);
                prefix.as_slice().encode(out);
                self.reference_encode_id(*child_id, out);
            }
            ArenaNodeData::Digest(digest) => {
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
        match self.nodes[node_id].clone() {
            ArenaNodeData::Null => Ok(None),
            ArenaNodeData::Branch(nodes) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match nodes[*i as usize] {
                        Some(ref id) => self.get_internal(*id, tail),
                        None => Ok(None),
                    }
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Leaf(prefix, value) => {
                if prefix_nibs(&prefix) == key_nibs {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                if let Some(tail) = key_nibs.strip_prefix(prefix_nibs(&prefix).as_slice()) {
                    self.get_internal(child_id, tail)
                } else {
                    Ok(None)
                }
            }
            ArenaNodeData::Digest(digest) => Err(Error::NodeNotResolved(digest)),
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

    fn insert_internal(&mut self, key_nibs: &[u8], value: Vec<u8>) -> Result<bool, Error> {
        let updated = self.insert_recursive(self.root_id, key_nibs, value)?;
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
    ) -> Result<bool, Error> {
        let node_data = self.nodes[node_id].clone();
        match node_data {
            ArenaNodeData::Null => {
                self.nodes[node_id] = ArenaNodeData::Leaf(to_encoded_path(key_nibs, true), value);
                Ok(true)
            }
            ArenaNodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            let updated = self.insert_recursive(id, tail, value)?;
                            if updated {
                                self.cached_references[node_id].borrow_mut().take();
                            }
                            Ok(updated)
                        }
                        None => {
                            let new_leaf_id = self
                                .add_node(ArenaNodeData::Leaf(to_encoded_path(tail, true), value));
                            children[*i as usize] = Some(new_leaf_id);
                            self.nodes[node_id] = ArenaNodeData::Branch(children);
                            self.cached_references[node_id].borrow_mut().take();
                            Ok(true)
                        }
                    }
                } else {
                    Err(Error::ValueInBranch)
                }
            }
            ArenaNodeData::Leaf(prefix, mut old_value) => {
                let self_nibs = prefix_nibs(&prefix);
                let common_len = lcp(&self_nibs, key_nibs);
                if common_len == self_nibs.len() && common_len == key_nibs.len() {
                    // if self_nibs == key_nibs, update the value if it is different
                    if old_value == value {
                        return Ok(false);
                    }
                    old_value = value;
                    self.nodes[node_id] = ArenaNodeData::Leaf(prefix, old_value);
                    self.cached_references[node_id].borrow_mut().take();
                    Ok(true)
                } else if common_len == self_nibs.len() || common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    // create a branch with two children
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    let leaf1_id = self.add_node(ArenaNodeData::Leaf(
                        to_encoded_path(&self_nibs[split_point..], true),
                        old_value,
                    ));
                    let leaf2_id = self.add_node(ArenaNodeData::Leaf(
                        to_encoded_path(&key_nibs[split_point..], true),
                        value,
                    ));

                    children[self_nibs[common_len] as usize] = Some(leaf1_id);
                    children[key_nibs[common_len] as usize] = Some(leaf2_id);

                    if common_len > 0 {
                        // create parent extension for new branch
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        self.nodes[node_id] = ArenaNodeData::Extension(
                            to_encoded_path(&self_nibs[..common_len], false),
                            branch_id,
                        );
                    } else {
                        self.nodes[node_id] = ArenaNodeData::Branch(children);
                    }
                    self.cached_references[node_id].borrow_mut().take();
                    Ok(true)
                }
            }
            ArenaNodeData::Extension(prefix, child_id) => {
                let self_nibs = prefix_nibs(&prefix);
                let common_len = lcp(&self_nibs, key_nibs);
                if common_len == self_nibs.len() {
                    // traverse down for update
                    let updated =
                        self.insert_recursive(child_id, &key_nibs[common_len..], value)?;
                    if updated {
                        self.cached_references[node_id].borrow_mut().take();
                    }
                    Ok(updated)
                } else if common_len == key_nibs.len() {
                    Err(Error::ValueInBranch)
                } else {
                    let split_point = common_len + 1;
                    // create a branch with two children
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    if split_point < self_nibs.len() {
                        let ext_id = self.add_node(ArenaNodeData::Extension(
                            to_encoded_path(&self_nibs[split_point..], false),
                            child_id,
                        ));
                        children[self_nibs[common_len] as usize] = Some(ext_id);
                    } else {
                        children[self_nibs[common_len] as usize] = Some(child_id);
                    }

                    let leaf_id = self.add_node(ArenaNodeData::Leaf(
                        to_encoded_path(&key_nibs[split_point..], true),
                        value,
                    ));
                    children[key_nibs[common_len] as usize] = Some(leaf_id);

                    if common_len > 0 {
                        // Create parent extension for new branch
                        let branch_id = self.add_node(ArenaNodeData::Branch(children));
                        self.nodes[node_id] = ArenaNodeData::Extension(
                            to_encoded_path(&self_nibs[..common_len], false),
                            branch_id,
                        );
                    } else {
                        self.nodes[node_id] = ArenaNodeData::Branch(children);
                    }
                    self.cached_references[node_id].borrow_mut().take();
                    Ok(true)
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
            ArenaNodeData::Leaf(prefix, value) => {
                prefix.as_slice().length() + value.as_slice().length()
            }
            ArenaNodeData::Extension(prefix, node_id) => {
                prefix.as_slice().length() + self.reference_length(*node_id)
            }
            ArenaNodeData::Digest(_) => 32,
        }
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
}

/// A temporary structure to decode nodes from RLP before adding them to the arena.
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct PreMptNode {
    /// The type and data of the node.
    pub data: PreMptNodeData,
    /// Cache for a previously computed reference of this node. This is skipped during
    /// serialization.
    #[serde(skip)]
    pub cached_reference: RefCell<Option<MptNodeReference>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PreMptNodeData {
    /// Represents an empty trie node.
    #[default]
    Null,
    /// A node that can have up to 16 children. Each child is an optional boxed [MptNode].
    Branch([Option<Box<PreMptNode>>; 16]),
    /// A leaf node that contains a key and a value, both represented as byte vectors.
    Leaf(Vec<u8>, Vec<u8>),
    /// A node that has exactly one child and is used to represent a shared prefix of
    /// several keys.
    Extension(Vec<u8>, Box<PreMptNode>),
    /// Represents a sub-trie by its hash, allowing for efficient storage of large
    /// sub-tries without storing their entire content.
    Digest(B256),
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
    pub fn test_tiny() {
        // trie consisting of an extension, a branch and two leafs
        let mut trie = ArenaBasedMptNode::default();
        trie.insert_rlp(b"a", 0u8).unwrap();
        trie.insert_rlp(b"b", 1u8).unwrap();

        assert!(!trie.is_empty());
        let exp_hash = hex!("6fbf23d6ec055dd143ff50d558559770005ff44ae1d41276f1bd83affab6dd3b");
        assert_eq!(trie.hash().0, exp_hash);
    }

    #[test]
    pub fn test_get_rlp() {
        let mut trie = ArenaBasedMptNode::default();
        trie.insert_rlp(b"key1", 42u32).unwrap();
        trie.insert_rlp(b"key2", 100u64).unwrap();

        let val1: Option<u32> = trie.get_rlp(b"key1").unwrap();
        let val2: Option<u64> = trie.get_rlp(b"key2").unwrap();
        let val3: Option<u32> = trie.get_rlp(b"nonexistent").unwrap();

        assert_eq!(val1, Some(42u32));
        assert_eq!(val2, Some(100u64));
        assert_eq!(val3, None);
    }

    #[test]
    pub fn test_keccak_trie() {
        const N: usize = 128;

        // insert
        let mut trie = ArenaBasedMptNode::default();
        for i in 0..N {
            let hash = keccak256(i.to_be_bytes().as_slice());
            assert!(trie.insert_rlp(hash.as_slice(), i).unwrap());

            // check hash against trie build in reverse
            let mut reference = ArenaBasedMptNode::default();
            for j in (0..=i).rev() {
                let hash = keccak256(j.to_be_bytes().as_slice());
                reference.insert_rlp(hash.as_slice(), j).unwrap();
            }
            assert_eq!(trie.hash(), reference.hash());
        }

        // get
        for i in 0..N {
            let hash = keccak256(i.to_be_bytes().as_slice());
            assert_eq!(trie.get_rlp(hash.as_slice()).unwrap(), Some(i));
            let hash = keccak256((i + N).to_be_bytes().as_slice());
            assert!(trie.get(hash.as_slice()).unwrap().is_none());
        }
    }

    #[test]
    pub fn test_index_trie() {
        const N: usize = 128;

        // insert
        let mut trie = ArenaBasedMptNode::default();
        for i in 0..N {
            let mut key_bytes = Vec::new();
            i.encode(&mut key_bytes);
            assert!(trie.insert_rlp(&key_bytes, i).unwrap());

            // check hash against trie build in reverse
            let mut reference = ArenaBasedMptNode::default();
            for j in (0..=i).rev() {
                let mut ref_key_bytes = Vec::new();
                j.encode(&mut ref_key_bytes);
                reference.insert_rlp(&ref_key_bytes, j).unwrap();
            }
            assert_eq!(trie.hash(), reference.hash());
        }

        // get
        for i in 0..N {
            let mut key_bytes = Vec::new();
            i.encode(&mut key_bytes);
            assert_eq!(trie.get_rlp(&key_bytes).unwrap(), Some(i));

            let mut nonexist_key_bytes = Vec::new();
            (i + N).encode(&mut nonexist_key_bytes);
            assert!(trie.get(&nonexist_key_bytes).unwrap().is_none());
        }
    }

    #[test]
    pub fn test_to_encoded_path() {
        // extension node with an even path length
        let nibbles = vec![0x0a, 0x0b, 0x0c, 0x0d];
        assert_eq!(to_encoded_path(&nibbles, false), vec![0x00, 0xab, 0xcd]);
        // extension node with an odd path length
        let nibbles = vec![0x0a, 0x0b, 0x0c];
        assert_eq!(to_encoded_path(&nibbles, false), vec![0x1a, 0xbc]);
        // leaf node with an even path length
        let nibbles = vec![0x0a, 0x0b, 0x0c, 0x0d];
        assert_eq!(to_encoded_path(&nibbles, true), vec![0x20, 0xab, 0xcd]);
        // leaf node with an odd path length
        let nibbles = vec![0x0a, 0x0b, 0x0c];
        assert_eq!(to_encoded_path(&nibbles, true), vec![0x3a, 0xbc]);
    }

    #[test]
    pub fn test_lcp() {
        let cases = [
            (vec![], vec![], 0),
            (vec![0xa], vec![0xa], 1),
            (vec![0xa, 0xb], vec![0xa, 0xc], 1),
            (vec![0xa, 0xb], vec![0xa, 0xb], 2),
            (vec![0xa, 0xb], vec![0xa, 0xb, 0xc], 2),
            (vec![0xa, 0xb, 0xc], vec![0xa, 0xb, 0xc], 3),
            (vec![0xa, 0xb, 0xc], vec![0xa, 0xb, 0xc, 0xd], 3),
            (vec![0xa, 0xb, 0xc, 0xd], vec![0xa, 0xb, 0xc, 0xd], 4),
        ];
        for (a, b, cpl) in cases {
            assert_eq!(lcp(&a, &b), cpl)
        }
    }

    #[test]
    pub fn test_rlp_nodes_collection() {
        let mut trie = ArenaBasedMptNode::default();
        trie.insert(b"key1", b"value1".to_vec()).unwrap();
        trie.insert(b"key2", b"value2".to_vec()).unwrap();
        trie.insert(b"key3", b"value3".to_vec()).unwrap();

        let mut nodes = HashMap::default();
        trie.rlp_nodes(&mut nodes);

        // Should have collected all nodes in the trie
        assert!(!nodes.is_empty());

        // All nodes should be unique (no duplicates)
        let mut unique_rlp_bytes: std::collections::HashSet<Vec<u8>> =
            std::collections::HashSet::new();
        for rlp_bytes in nodes.values() {
            assert!(unique_rlp_bytes.insert(rlp_bytes.clone()), "Found duplicate RLP bytes");
        }
    }

    #[test]
    pub fn test_conversion_from_pre_node() {
        // Create a PreMptNode structure and convert it to ArenaBasedMptNode
        let pre_node = PreMptNode {
            data: PreMptNodeData::Leaf(vec![0x20, 0x12, 0x34], b"test_value".to_vec()),
            cached_reference: RefCell::new(None),
        };

        let arena_trie = ArenaBasedMptNode::from_pre_node(&pre_node);

        // Verify that it's not empty and has the expected structure
        assert!(!arena_trie.is_empty());
        assert_eq!(arena_trie.nodes.len(), 1);

        match &arena_trie.nodes[arena_trie.root_id] {
            ArenaNodeData::Leaf(prefix, value) => {
                assert_eq!(*prefix, vec![0x20, 0x12, 0x34]);
                assert_eq!(*value, b"test_value".to_vec());
            }
            _ => panic!("Expected leaf node"),
        }
    }

    #[test]
    pub fn test_hash_consistency() {
        // Test that the hash function produces consistent results
        let mut trie1 = ArenaBasedMptNode::default();
        let mut trie2 = ArenaBasedMptNode::default();

        let test_data = [("apple", "fruit"), ("banana", "yellow"), ("cherry", "red")];

        // Insert in the same order
        for (key, value) in &test_data {
            trie1.insert(key.as_bytes(), value.as_bytes().to_vec()).unwrap();
            trie2.insert(key.as_bytes(), value.as_bytes().to_vec()).unwrap();
        }

        assert_eq!(trie1.hash(), trie2.hash());

        // Insert in different order - should still have same hash
        let mut trie3 = ArenaBasedMptNode::default();
        for (key, value) in test_data.iter().rev() {
            trie3.insert(key.as_bytes(), value.as_bytes().to_vec()).unwrap();
        }

        assert_eq!(trie1.hash(), trie3.hash());
    }

    #[test]
    pub fn test_resolve_nodes_arena() {
        // Create a trie with some data
        let mut original_trie = ArenaBasedMptNode::default();
        original_trie.insert(b"test_key", b"test_value".to_vec()).unwrap();

        // Create a trie with a digest node
        let mut partial_trie = ArenaBasedMptNode::default();
        let digest = original_trie.hash();
        partial_trie.nodes[0] = ArenaNodeData::Digest(digest);

        // Create a node store with the resolved content
        let mut node_store = HashMap::default();
        let pre_node = PreMptNode {
            data: PreMptNodeData::Leaf(
                to_encoded_path(&to_nibs(b"test_key"), true),
                b"test_value".to_vec(),
            ),
            cached_reference: RefCell::new(None),
        };
        node_store.insert(MptNodeReference::Digest(digest), pre_node);

        // Resolve the partial trie
        let resolved_trie = resolve_nodes_arena(&partial_trie, &node_store);

        // The resolved trie should have the same hash as the original
        assert_eq!(original_trie.hash(), resolved_trie.hash());

        // Should be able to retrieve the value
        let retrieved = resolved_trie.get(b"test_key").unwrap();
        assert_eq!(retrieved, Some(b"test_value".to_vec()));
    }
}

/// Parses proof bytes into a vector of PreMptNodes (for conversion to arena-based).
pub fn parse_proof(proof: &[impl AsRef<[u8]>]) -> Result<Vec<PreMptNode>, Error> {
    Ok(proof.iter().map(|bytes| PreMptNode::decode(bytes)).collect::<Result<Vec<_>, _>>()?)
}

/// Creates an arena-based Merkle Patricia trie from an EIP-1186 proof.
pub fn mpt_from_proof(proof_nodes: &[PreMptNode]) -> Result<ArenaBasedMptNode, Error> {
    if proof_nodes.is_empty() {
        return Ok(ArenaBasedMptNode::default());
    }

    // Convert the proof nodes to arena-based and build the trie
    // We'll start with the last node and work our way back
    let mut next: Option<PreMptNode> = None;
    for (i, node) in proof_nodes.iter().enumerate().rev() {
        // There is nothing to replace for the last node
        let Some(replacement) = next else {
            next = Some(node.clone());
            continue;
        };

        // The next node must have a digest reference
        let replacement_ref = replacement.reference();
        let MptNodeReference::Digest(ref child_ref) = replacement_ref else {
            panic!("node {} in proof is not referenced by hash", i + 1);
        };

        // Find the child that references the next node
        let resolved: PreMptNode = match node.data.clone() {
            PreMptNodeData::Branch(mut children) => {
                if let Some(child) = children.iter_mut().flatten().find(
                    |child| matches!(child.data, PreMptNodeData::Digest(d) if d == *child_ref),
                ) {
                    *child = Box::new(replacement);
                } else {
                    panic!("node {} does not reference the successor", i);
                }
                PreMptNodeData::Branch(children).into()
            }
            PreMptNodeData::Extension(prefix, child) => {
                if !matches!(child.data, PreMptNodeData::Digest(d) if d == *child_ref) {
                    panic!("node {} does not reference the successor", i);
                }
                PreMptNodeData::Extension(prefix, Box::new(replacement)).into()
            }
            PreMptNodeData::Null | PreMptNodeData::Leaf(_, _) | PreMptNodeData::Digest(_) => {
                panic!("node {} has no children to replace", i);
            }
        };

        next = Some(resolved);
    }

    // Convert the final trie to arena-based
    let final_node = next.unwrap_or_default();
    Ok(ArenaBasedMptNode::from_pre_node(&final_node))
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
    node_store: &HashMap<MptNodeReference, PreMptNode>,
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
    node_store: &HashMap<MptNodeReference, PreMptNode>,
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
                // Convert the resolved PreMptNode to arena format and add it
                let resolved_arena = ArenaBasedMptNode::from_pre_node(resolved_node);
                return resolve_node_recursive(
                    &resolved_arena,
                    resolved_arena.root_id,
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

/// Builds Ethereum state tries from relevant proofs before and after a state transition using arena-based MPT.
/// This version returns EthereumState2 with arena-based nodes directly for better performance.
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

    let mut state_nodes = HashMap::<_, _>::default();
    let mut state_root_node = PreMptNode::default();

    for (address, proof) in parent_proofs {
        let proof_nodes = parse_proof(&proof.proof)?;
        let _trie = mpt_from_proof(&proof_nodes)?;

        // The first node in the proof is the root
        if let Some(node) = proof_nodes.first() {
            state_root_node = node.clone();
        }

        proof_nodes.into_iter().for_each(|node| {
            state_nodes.insert(node.reference(), node);
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

        let mut storage_nodes = HashMap::<_, _>::default();
        let mut storage_root_node = PreMptNode::default();

        for storage_proof in &proof.storage_proofs {
            let proof_nodes = parse_proof(&storage_proof.proof)?;
            let _trie = mpt_from_proof(&proof_nodes)?;

            // The first node in the proof is the root
            if let Some(node) = proof_nodes.first() {
                storage_root_node = node.clone();
            }

            proof_nodes.into_iter().for_each(|node| {
                storage_nodes.insert(node.reference(), node);
            });
        }

        // Assure that slots can be deleted from the storage trie
        for storage_proof in &fini_proofs.storage_proofs {
            add_orphaned_leafs(storage_proof.key.0, &storage_proof.proof, &mut storage_nodes)?;
        }

        // Create the storage trie from all the relevant nodes - keep as arena-based
        let storage_trie = resolve_nodes_arena(
            &ArenaBasedMptNode::from_pre_node(&storage_root_node),
            &storage_nodes,
        );

        storage.insert(B256::from(keccak256(address)), storage_trie);
    }

    // Create the state trie from all the relevant nodes - keep as arena-based
    let state_trie =
        resolve_nodes_arena(&ArenaBasedMptNode::from_pre_node(&state_root_node), &state_nodes);

    Ok(EthereumState2 { state_trie, storage_tries: StorageTries2(storage) })
}

/// Adds all the leaf nodes of non-inclusion proofs to the nodes.
fn add_orphaned_leafs(
    key: impl AsRef<[u8]>,
    proof: &[impl AsRef<[u8]>],
    nodes_by_reference: &mut HashMap<MptNodeReference, PreMptNode>,
) -> Result<(), Error> {
    if !proof.is_empty() {
        let proof_nodes = parse_proof(proof)?;
        if is_not_included(keccak256(key).as_slice(), &proof_nodes)? {
            // Add the leaf node to the nodes
            let leaf = proof_nodes.last().unwrap();
            shorten_node_path_arena(leaf).into_iter().for_each(|node| {
                nodes_by_reference.insert(node.reference(), node);
            });
        }
    }

    Ok(())
}

/// Verifies that the given proof is a valid proof of exclusion for the given key.
pub fn is_not_included(key: &[u8], proof_nodes: &[PreMptNode]) -> Result<bool, Error> {
    let proof_trie = mpt_from_proof(proof_nodes)?;
    // For valid proofs, the get must not fail
    let value = proof_trie.get(key)?;

    Ok(value.is_none())
}

/// Returns a list of all possible nodes that can be created by shortening the path of the given node.
pub fn shorten_node_path_arena(node: &PreMptNode) -> Vec<PreMptNode> {
    let mut res = Vec::new();
    let nibs = match &node.data {
        PreMptNodeData::Leaf(prefix, _) | PreMptNodeData::Extension(prefix, _) => {
            prefix_nibs(prefix)
        }
        _ => vec![],
    };

    match &node.data {
        PreMptNodeData::Null | PreMptNodeData::Branch(_) | PreMptNodeData::Digest(_) => {}
        PreMptNodeData::Leaf(_, value) => {
            for i in 0..=nibs.len() {
                res.push(
                    PreMptNodeData::Leaf(to_encoded_path(&nibs[i..], true), value.clone()).into(),
                )
            }
        }
        PreMptNodeData::Extension(_, child) => {
            for i in 0..=nibs.len() {
                res.push(
                    PreMptNodeData::Extension(to_encoded_path(&nibs[i..], false), child.clone())
                        .into(),
                )
            }
        }
    };
    res
}
