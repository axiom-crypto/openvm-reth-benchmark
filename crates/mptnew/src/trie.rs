use std::cell::RefCell;

use alloy_rlp::Encodable;
use bumpalo::Bump;
use revm_primitives::{b256, keccak256, map::DefaultHashBuilder, FixedBytes, HashMap, B256};
use smallvec::SmallVec;

use crate::{
    bump_bufmut::BumpBytesMut,
    hp::{
        encoded_path_eq_nibs, encoded_path_strip_prefix, lcp, prefix_to_small_nibs,
        to_encoded_path, to_nibs,
    },
    node::{NodeData, NodeId, NodeRef, NodeRefError, NodeRlpDecoded},
};

/// Root hash of an empty trie.
pub const EMPTY_ROOT: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

const NODE_RLP_MAX_LENGTH: usize = 600;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot resolve node reference: {0:?}")]
    NodeRefNotResolved(NodeRef),
    /// Triggered when an operation reaches an unresolved node. The associated `B256`
    /// value provides details about the unresolved node.
    #[error("reached an unresolved node: {0:#}")]
    NodeNotResolved(B256),
    #[error("rlp decode error: {0}")]
    RlpError(#[from] alloy_rlp::Error),
    #[error("branch node with value")]
    ValueInBranch,
    #[error("node reference error: {0}")]
    NodeRefError(#[from] NodeRefError),
}

#[derive(Debug)]
pub struct MptTrie<'a> {
    root_id: NodeId,

    nodes: Vec<NodeData<'a>>,

    /// Cache. Hashing/encoding often needs "what would this node look like in its parent"
    cached_references: Vec<RefCell<Option<NodeRef>>>,

    /// Scratch buffer used only for RLP encoding when a node's full RLP exceeds 32 bytes and we
    /// need to compute its keccak hash. Keeping it here avoids repeated allocations.
    rlp_scratch: RefCell<Vec<u8>>,

    bump: &'a Bump,
}

impl<'a> MptTrie<'a> {
    pub fn new(bump: &'a Bump) -> Self {
        Self::with_capacity(bump, 1)
    }

    /// Creates a new arena with pre-allocated capacity
    pub fn with_capacity(bump: &'a Bump, cap: usize) -> Self {
        let mut nodes = Vec::with_capacity(cap);
        let mut cached_references = Vec::with_capacity(cap);
        nodes.push(NodeData::Null);
        cached_references.push(RefCell::new(None));

        Self {
            nodes,
            rlp_scratch: RefCell::new(Vec::with_capacity(NODE_RLP_MAX_LENGTH)),
            cached_references,
            bump,
            root_id: 0,
        }
    }
}

impl<'a> MptTrie<'a> {
    pub fn rlp_nodes(&self) -> Vec<Vec<u8>> {
        let mut rlp_nodes = Vec::with_capacity(self.nodes.len());
        for node_id in 0..self.nodes.len() {
            let node_id = node_id as NodeId;
            let payload_length = self.payload_length(node_id);
            let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

            let mut encoded = Vec::with_capacity(rlp_length);
            self.encode_with_payload_len(node_id, payload_length, &mut encoded);
            rlp_nodes.push(encoded);
        }
        rlp_nodes
    }

    pub fn resolve_nodes(
        bump: &'a Bump,
        root_hash: B256,
        root_id: NodeId,
        rlp_nodes: &'a [Vec<u8>],
    ) -> Result<MptTrie<'a>, Error> {
        let nodes_len = rlp_nodes.len();

        let mut node_ref_mapping =
            HashMap::with_capacity_and_hasher(nodes_len, DefaultHashBuilder::default());

        for rlp_node in rlp_nodes {
            if rlp_node.len() < 32 {
                continue;
            }

            let digest = keccak256(rlp_node);
            node_ref_mapping.insert(digest, rlp_node.as_slice());
        }

        if !rlp_nodes.is_empty() && rlp_nodes[root_id as usize].len() < 32 {
            let root_hash = keccak256(&rlp_nodes[root_id as usize]);
            node_ref_mapping.insert(root_hash, &rlp_nodes[root_id as usize]);
        }

        // dbg!(node_ref_mapping.len());

        let mut mpt_trie = MptTrie::with_capacity(bump, nodes_len);

        let root_id = {
            let mut mpt_resolver = MptResolver { node_ref_mapping, mpt: &mut mpt_trie };
            let slice = bump.alloc_slice_copy(root_hash.as_slice());
            mpt_resolver.resolve(slice)?
        };
        mpt_trie.root_id = root_id;

        Ok(mpt_trie)
    }
}

struct MptResolver<'a, 'm> {
    node_ref_mapping: HashMap<B256, &'a [u8]>,
    mpt: &'m mut MptTrie<'a>,
}

impl<'a, 'm> MptResolver<'a, 'm> {
    fn to_node_ref_slice(rlp_bytes: &[u8]) -> &[u8] {
        if rlp_bytes.len() == 33 {
            &rlp_bytes[1..]
        } else {
            rlp_bytes
        }
    }

    fn resolve(&mut self, node_ref_slice: &'a [u8]) -> Result<NodeId, Error> {
        let (node_ref, mut rlp_bytes) = {
            if node_ref_slice.len() == 32 {
                let fixed_bytes = B256::from_slice(node_ref_slice);
                let rlp_bytes = self.node_ref_mapping.get(&fixed_bytes).unwrap();
                let node_ref = NodeRef::Digest(fixed_bytes);
                (node_ref, *rlp_bytes)
            } else {
                let node_ref = NodeRef::Bytes(node_ref_slice.to_vec());
                (node_ref, node_ref_slice)
            }
        };
        let decoded_node = NodeRlpDecoded::decode_rlp(&mut rlp_bytes)?;

        let node_id = match decoded_node {
            NodeRlpDecoded::Null => self.mpt.add_node(NodeData::Null, Some(node_ref)),
            NodeRlpDecoded::Branch(children) => {
                let mut children_ids: [Option<NodeId>; 16] = Default::default();
                for (i, child_rlp) in children.into_iter().enumerate() {
                    let child_ref_slice = Self::to_node_ref_slice(child_rlp);
                    if child_ref_slice == [alloy_rlp::EMPTY_STRING_CODE] {
                        children_ids[i] = None;
                    } else {
                        let child_id = self.resolve(child_ref_slice)?;
                        children_ids[i] = Some(child_id);
                    }
                }
                let data = NodeData::Branch(children_ids);
                self.mpt.add_node(data, Some(node_ref))
            }
            NodeRlpDecoded::Leaf(prefix, value) => {
                let data = NodeData::Leaf(prefix, value);
                self.mpt.add_node(data, Some(node_ref))
            }
            NodeRlpDecoded::Extension(prefix, ext_node_rlp) => {
                let ext_node_ref_slice = Self::to_node_ref_slice(ext_node_rlp);
                let ext_node_id = self.resolve(ext_node_ref_slice)?;
                let data = NodeData::Extension(prefix, ext_node_id);
                self.mpt.add_node(data, Some(node_ref))
            }
            NodeRlpDecoded::Digest(digest) => {
                let data = NodeData::Digest(
                    FixedBytes::try_from(digest).map_err(NodeRefError::DigestLengthMismatch)?,
                );
                self.mpt.add_node(data, Some(node_ref))
            }
        };

        Ok(node_id)
    }
}

impl<'a> MptTrie<'a> {
    #[inline]
    fn calc_reference(&self, node_id: NodeId) -> NodeRef {
        match &self.nodes[node_id as usize] {
            NodeData::Null => NodeRef::Bytes(vec![alloy_rlp::EMPTY_STRING_CODE]),
            NodeData::Digest(digest) => NodeRef::Digest(*digest),
            _ => {
                let payload_length = self.payload_length(node_id);
                let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

                if rlp_length < 32 {
                    let mut encoded = Vec::with_capacity(rlp_length);
                    self.encode_with_payload_len(node_id, payload_length, &mut encoded);
                    debug_assert_eq!(encoded.len(), rlp_length);

                    NodeRef::Bytes(encoded)
                } else {
                    let mut scratch = self.rlp_scratch.borrow_mut();
                    scratch.clear();

                    self.encode_with_payload_len(node_id, payload_length, &mut *scratch);
                    debug_assert_eq!(scratch.len(), rlp_length);

                    let digest = keccak256(&*scratch);
                    NodeRef::Digest(digest)
                }
            }
        }
    }

    fn encode_with_payload_len(
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
                        Some(id) => self.reference_encode(*id, out),
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
                self.reference_encode(*child_id, out);
            }
            NodeData::Digest(digest) => {
                digest.encode(out);
            }
        }
    }

    #[inline]
    fn reference_encode(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
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

    /// Returns the length of the RLP payload of the node.
    fn payload_length(&self, node_id: NodeId) -> usize {
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

    /// Returns the length of the encoded [NodeRef] of this node.
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

// Public API
impl<'a> MptTrie<'a> {
    pub fn hash(&self) -> B256 {
        match self.nodes[self.root_id as usize] {
            NodeData::Null => EMPTY_ROOT,
            _ => match self.cached_references[self.root_id as usize]
                .borrow_mut()
                .get_or_insert_with(|| self.calc_reference(self.root_id))
            {
                NodeRef::Digest(digest) => *digest,
                NodeRef::Bytes(bytes) => keccak256(bytes),
            },
        }
    }

    /// Retrieves the value associated with a given key in the trie.
    #[inline]
    pub fn get<'s>(&'s self, key: &[u8]) -> Result<Option<&'a [u8]>, Error> {
        self.get_internal(self.root_id, &to_nibs(key))
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
    pub fn insert(&mut self, key: &[u8], value: &'a [u8]) -> Result<bool, Error> {
        let key_nibs = &to_nibs(key);
        self.insert_internal(self.root_id, key_nibs, value)
    }

    pub fn insert_rlp(
        &mut self,
        key: &[u8],
        value: impl alloy_rlp::Encodable,
    ) -> Result<bool, Error> {
        let mut rlp_bytes = BumpBytesMut::new_in(self.bump);
        value.encode(&mut rlp_bytes);
        self.insert(key, rlp_bytes.into_inner().into_bump_slice())
    }

    /// Removes a key from the trie.
    ///
    /// This method attempts to remove a key-value pair from the trie. If the key is
    /// present, it returns `true`. Otherwise, it returns `false`.
    #[inline]
    pub fn delete(&mut self, key: &[u8]) -> Result<bool, Error> {
        let key_nibs = &to_nibs(key);
        self.delete_internal(self.root_id, key_nibs)
    }

    #[inline]
    pub fn root_id(&self) -> NodeId {
        self.root_id
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(&self.nodes[self.root_id as usize], NodeData::Null)
    }
}

// Internal Implementation
impl<'a> MptTrie<'a> {
    fn add_node(&mut self, data: NodeData<'a>, node_ref: Option<NodeRef>) -> NodeId {
        let id = self.nodes.len() as NodeId;
        self.nodes.push(data);
        self.cached_references.push(RefCell::new(node_ref));
        id
    }

    #[inline]
    fn invalidate_ref_cache(&mut self, node_id: NodeId) {
        self.cached_references[node_id as usize].borrow_mut().take();
    }

    #[inline]
    fn get_internal(&self, node_id: NodeId, key_nibs: &[u8]) -> Result<Option<&'a [u8]>, Error> {
        // dbg!(&node_id, &key_nibs);

        match &self.nodes[node_id as usize] {
            NodeData::Null => Ok(None),
            NodeData::Branch(nodes) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match nodes[*i as usize] {
                        Some(id) => self.get_internal(id, tail),
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
                    self.get_internal(*child_id, tail)
                } else {
                    Ok(None)
                }
            }
            NodeData::Digest(digest) => Err(Error::NodeNotResolved(*digest)),
        }
    }

    #[inline]
    fn insert_internal(
        &mut self,
        node_id: NodeId,
        key_nibs: &[u8],
        value: &'a [u8],
    ) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            NodeData::Null => {
                let path = to_encoded_path(self.bump, key_nibs, true);
                self.nodes[node_id as usize] = NodeData::Leaf(path, value);
                true
            }
            NodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    match children[*i as usize] {
                        Some(id) => self.insert_internal(id, tail, value)?,
                        None => {
                            let path = to_encoded_path(self.bump, tail, true);
                            let new_leaf_id = self.add_node(NodeData::Leaf(path, value), None);
                            children[*i as usize] = Some(new_leaf_id);
                            self.nodes[node_id as usize] = NodeData::Branch(children);
                            true
                        }
                    }
                } else {
                    return Err(Error::ValueInBranch);
                }
            }
            NodeData::Leaf(prefix, old_value) => {
                let self_nibs = prefix_to_small_nibs(prefix);
                let common_len = lcp(&self_nibs, key_nibs);

                if common_len == self_nibs.len() && common_len == key_nibs.len() {
                    // if self_nibs == key_nibs, update the value if it is different
                    if old_value == value {
                        return Ok(false);
                    }
                    self.nodes[node_id as usize] = NodeData::Leaf(prefix, value);
                    true
                } else if common_len == self_nibs.len() || common_len == key_nibs.len() {
                    return Err(Error::ValueInBranch);
                } else {
                    // otherwise, create a branch with two children
                    let split_point = common_len + 1;
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    let leaf1_path = to_encoded_path(self.bump, &self_nibs[split_point..], true);
                    let leaf1_id = self.add_node(NodeData::Leaf(leaf1_path, old_value), None);

                    let leaf2_path = to_encoded_path(self.bump, &key_nibs[split_point..], true);
                    let leaf2_id = self.add_node(NodeData::Leaf(leaf2_path, value), None);

                    children[self_nibs[common_len] as usize] = Some(leaf1_id);
                    children[key_nibs[common_len] as usize] = Some(leaf2_id);

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(NodeData::Branch(children), None);
                        let ext_path_slice =
                            to_encoded_path(self.bump, &self_nibs[..common_len], false);
                        NodeData::Extension(ext_path_slice, branch_id)
                    } else {
                        NodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    true
                }
            }
            NodeData::Extension(prefix, child_id) => {
                let self_nibs = prefix_to_small_nibs(prefix);
                let common_len = lcp(&self_nibs, key_nibs);

                if common_len == self_nibs.len() {
                    self.insert_internal(child_id, &key_nibs[common_len..], value)?
                } else if common_len == key_nibs.len() {
                    return Err(Error::ValueInBranch);
                } else {
                    let split_point = common_len + 1;
                    let mut children: [Option<NodeId>; 16] = Default::default();

                    if split_point < self_nibs.len() {
                        let ext_path = to_encoded_path(self.bump, &self_nibs[split_point..], false);
                        let ext_id = self.add_node(NodeData::Extension(ext_path, child_id), None);
                        children[self_nibs[common_len] as usize] = Some(ext_id);
                    } else {
                        children[self_nibs[common_len] as usize] = Some(child_id);
                    }

                    let leaf_path = to_encoded_path(self.bump, &key_nibs[split_point..], true);
                    let leaf_id = self.add_node(NodeData::Leaf(leaf_path, value), None);
                    children[key_nibs[common_len] as usize] = Some(leaf_id);

                    let new_node_data = if common_len > 0 {
                        let branch_id = self.add_node(NodeData::Branch(children), None);
                        let parent_ext_path_slice =
                            to_encoded_path(self.bump, &self_nibs[..common_len], false);
                        NodeData::Extension(parent_ext_path_slice, branch_id)
                    } else {
                        NodeData::Branch(children)
                    };
                    self.nodes[node_id as usize] = new_node_data;
                    true
                }
            }
            NodeData::Digest(digest) => {
                return Err(Error::NodeNotResolved(digest));
            }
        };

        if updated {
            self.invalidate_ref_cache(node_id);
        }

        Ok(updated)
    }

    #[inline]
    fn delete_internal(&mut self, node_id: NodeId, key_nibs: &[u8]) -> Result<bool, Error> {
        let updated = match self.nodes[node_id as usize] {
            NodeData::Null => false,
            NodeData::Branch(mut children) => {
                if let Some((i, tail)) = key_nibs.split_first() {
                    let child_id = children[*i as usize];
                    match child_id {
                        Some(id) => {
                            if !self.delete_internal(id, tail)? {
                                return Ok(false);
                            }

                            // if the node is now empty, remove it
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

                // there will always be at least one remaining node
                let (index, child_id) = remaining_iter.next().unwrap();
                let child_id = child_id.unwrap();

                // if there is only exactly one node left, we need to convert the branch
                if remaining_iter.next().is_none() {
                    let child_node_data = self.nodes[child_id as usize].clone();

                    let new_node_data = match child_node_data {
                        NodeData::Leaf(prefix, value) => {
                            let leaf_nibs = prefix_to_small_nibs(prefix);
                            let mut new_nibs: SmallVec<[u8; 64]> =
                                SmallVec::with_capacity(1 + leaf_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&leaf_nibs);
                            let new_path = to_encoded_path(self.bump, &new_nibs, true);
                            NodeData::Leaf(new_path, value)
                        }
                        NodeData::Extension(prefix, child_child_id) => {
                            let ext_nibs = prefix_to_small_nibs(prefix);
                            let mut new_nibs: SmallVec<[u8; 64]> =
                                SmallVec::with_capacity(1 + ext_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&ext_nibs);
                            let new_path = to_encoded_path(self.bump, &new_nibs, false);
                            NodeData::Extension(new_path, child_child_id)
                        }
                        NodeData::Branch(_) | NodeData::Digest(_) => {
                            let ext_nibs: SmallVec<[u8; 1]> = SmallVec::from_slice(&[index as u8]);
                            let new_path = to_encoded_path(self.bump, &ext_nibs, false);
                            NodeData::Extension(new_path, child_id)
                        }
                        NodeData::Null => unreachable!(),
                    };
                    self.nodes[node_id as usize] = new_node_data;
                } else {
                    self.nodes[node_id as usize] = NodeData::Branch(children);
                }

                true
            }
            NodeData::Leaf(prefix, _) => {
                let leaf_nibs = prefix_to_small_nibs(prefix);
                if leaf_nibs.as_slice() != key_nibs {
                    return Ok(false);
                }
                self.nodes[node_id as usize] = NodeData::Null;
                true
            }
            NodeData::Extension(prefix, child_id) => {
                let self_nibs = prefix_to_small_nibs(prefix);
                if let Some(tail) = key_nibs.strip_prefix(self_nibs.as_slice()) {
                    if !self.delete_internal(child_id, tail)? {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                };

                // an extension can only point to a branch or a digest; since it's sub trie was
                // modified, we need to make sure that this property still holds
                let child_node_data = &self.nodes[child_id as usize];
                let new_node_data = match child_node_data {
                    // if the child is empty, remove the extension
                    NodeData::Null => NodeData::Null,
                    // for a leaf, replace the extension with the extended leaf
                    NodeData::Leaf(child_path_bytes, value) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(self_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&self_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path = to_encoded_path(self.bump, &combined_nibs, true);
                        NodeData::Leaf(new_path, value)
                    }
                    // for an extension, replace the extension with the extended extension
                    NodeData::Extension(child_path_bytes, grandchild_id) => {
                        let child_path_nibs = prefix_to_small_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(self_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&self_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path = to_encoded_path(self.bump, &combined_nibs, false);
                        NodeData::Extension(new_path, *grandchild_id)
                    }
                    // for a branch or digest, the extension is still correct
                    NodeData::Branch(_) | NodeData::Digest(_) => {
                        NodeData::Extension(prefix, child_id)
                    }
                };
                self.nodes[node_id as usize] = new_node_data;
                true
            }
            NodeData::Digest(digest) => {
                return Err(Error::NodeNotResolved(digest));
            }
        };

        if updated {
            self.invalidate_ref_cache(node_id);
        }
        Ok(updated)
    }

    #[allow(unused)]
    pub(crate) fn print_tree(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!("{i} {node:?}");
        }
    }
}
