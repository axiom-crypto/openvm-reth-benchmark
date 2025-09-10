use std::{cell::RefCell, mem::MaybeUninit};

use alloy_rlp::Encodable;
use bumpalo::Bump;
use bytes::Buf;
use revm_primitives::{keccak256, B256};
use smallvec::SmallVec;

use crate::{
    bump_bufmut::BumpBytesMut,
    hp::{
        encoded_path_eq_nibs, encoded_path_strip_prefix, lcp, prefix_to_nibs, to_encoded_path,
        to_nibs,
    },
    node::{NodeData, NodeId, NodeRef},
};

/// OpenVM memory alignment word size.
const MIN_ALIGN: usize = 4;

/// Initial capacity of [`MptTrie`]'s `rlp_scratch`.
const RLP_SCRATCH_INIT_CAPACITY: usize = 600;

const VALUE_RLP_BUFFER_CAPACITY: usize = 200;

/// Sentinel index representing the null node when decoding and in internal references.
/// In a default MPT, `nodes[0]` starts as `Null`, but the root may later be changed to a
/// non-null node (e.g. `Digest`) for convenience. `NULL_NODE_ID` is still used by the decoder
/// as the canonical "no node" identifier.
const NULL_NODE_ID: NodeId = 0;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Node reference does not match the expected node reference in the parent.
    #[error("node reference mismatch")]
    NodeRefMismatch,
    /// Triggered when an operation reaches an unresolved node. The associated `B256`
    /// value provides details about the unresolved node.
    #[error("reached an unresolved node: {0:#}")]
    NodeNotResolved(B256),
    /// Represents errors related to the RLP encoding and decoding.
    #[error("rlp decode error: {0}")]
    RlpError(#[from] alloy_rlp::Error),
    /// Occurs when a value is unexpectedly found in a branch node.
    #[error("branch node with value")]
    ValueInBranch,
}

/// Arena-based implementation that stores all nodes in a flat vector and uses indices for better
/// memory layout and performance. The lifetime parameter `'a` allows zero-copy deserialization by
/// borrowing from the input buffer.
#[derive(Debug, Clone)]
pub struct MptTrie<'a> {
    root_id: NodeId,

    /// List of MPT nodes.
    nodes: Vec<NodeData<'a>>,

    /// Cache. Hashing/encoding often needs "what would this node look like in its parent"
    cached_references: Vec<RefCell<Option<NodeRef<'a>>>>,

    /// Scratch buffer used only for RLP encoding when a node's full RLP exceeds 32 bytes and we
    /// need to compute its keccak hash. Keeping it here avoids repeated allocations.
    rlp_scratch: RefCell<Vec<u8>>,

    /// Bump allocation area.
    bump: &'a Bump,
}

impl<'a> MptTrie<'a> {
    pub fn new(bump: &'a Bump) -> Self {
        Self::with_capacity(bump, 1)
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn with_capacity(bump: &'a Bump, capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        let mut cached_references = Vec::with_capacity(capacity);
        nodes.push(NodeData::Null);
        cached_references.push(RefCell::new(None));

        Self {
            nodes,
            rlp_scratch: RefCell::new(Vec::with_capacity(RLP_SCRATCH_INIT_CAPACITY)),
            cached_references,
            bump,
            root_id: 0,
        }
    }
}

/// Same as `let (bytes, rest) = buf.split_at(cnt); *buf = rest; bytes`.
#[inline(always)]
unsafe fn advance_unchecked<'a>(buf: &mut &'a [u8], cnt: usize) -> &'a [u8] {
    let bytes = &buf[..cnt];
    buf.advance(cnt);
    bytes
}

impl<'a> MptTrie<'a> {
    /// Encodes the MPT into an array of bytes. This is only used in the host, as a result it's not
    /// performance-critical.
    pub fn encode_trie(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        self.encode_trie_internal(self.root_id, &mut encoded);
        encoded
    }

    fn encode_trie_internal(&self, node_id: NodeId, out: &mut dyn alloy_rlp::BufMut) {
        let payload_length = self.payload_length(node_id);
        self.encode_with_payload_len(node_id, payload_length, out);

        // Pad the RLP encoding so its total length is divisible by `MIN_ALIGN`.
        // This let us to avoid memcpy operations when calculating keccak(rlp_encoded) during decoding.
        let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);
        let padding_len = (MIN_ALIGN - (rlp_length % MIN_ALIGN)) % MIN_ALIGN;
        for _ in 0..padding_len {
            out.put_u8(0);
        }

        match self.nodes[node_id as usize] {
            NodeData::Branch(childs) => {
                childs.iter().for_each(|c| {
                    if let Some(child_id) = c {
                        self.encode_trie_internal(*child_id, out)
                    }
                });
            }
            NodeData::Extension(_, ext_id) => {
                self.encode_trie_internal(ext_id, out);
            }
            _ => {}
        }
    }

    /// Decodes the given `bytes` into and creates an `MptTrie`.
    pub fn decode_trie(
        bump: &'a Bump,
        bytes: &mut &'a [u8],
        num_nodes: usize,
    ) -> Result<Self, Error> {
        // A growth factor applied to the node vector's capacity during deserialization.
        // This is a pragmatic optimization to pre-allocate a buffer for nodes that will be
        // added during the `update` phase. It prevents a "reallocation storm" where the
        // main trie and dozens of storage tries all try to reallocate their full node
        // vectors on the first update.
        // TODO: this is imperfect solution and the constant is somewhat arbitrary (although
        // reasonable)
        //
        // Simple improvement: run benchmark on a set of blocks (e.g. 100
        // blocks) and select the best constant.
        //
        // More advanced improvement: either pre-execute block at guest to know exact allocations in
        // advance, or allocate a separate arena specifically for updates.
        let capacity = num_nodes + (num_nodes / 2);
        let mut trie = Self::with_capacity(bump, capacity);

        // construct the expected root reference
        let root_ref = {
            let mut buf = *bytes;
            let rlp_node_header_start = buf;
            let alloy_rlp::Header { list, payload_length } = alloy_rlp::Header::decode(&mut buf)?;
            let payload = unsafe { advance_unchecked(&mut buf, payload_length) };
            let rlp_node_length = rlp_node_header_start.len() - buf.len();

            let rlp_node = &rlp_node_header_start[..rlp_node_length];
            if rlp_node.len() < 32 {
                NodeRef::Bytes(rlp_node)
            } else if !list {
                NodeRef::Digest(payload)
            } else {
                let digest = keccak256(rlp_node);
                let digest_slice = bump.alloc_slice_copy(digest.as_slice());
                NodeRef::Digest(digest_slice)
            }
        };

        let root_id = trie.decode_trie_internal(bytes, root_ref)?;
        trie.root_id = root_id;

        Ok(trie)
    }

    fn decode_trie_internal(
        &mut self,
        bytes: &mut &'a [u8],
        expected_node_ref: NodeRef<'a>,
    ) -> Result<NodeId, Error> {
        let rlp_node_header_start = *bytes;
        let alloy_rlp::Header { list, payload_length } = alloy_rlp::Header::decode(bytes)?;

        let mut payload = unsafe { advance_unchecked(bytes, payload_length) };
        let rlp_node_length = rlp_node_header_start.len() - bytes.len();

        let rlp_node = &rlp_node_header_start[..rlp_node_length];

        let padding_len = (MIN_ALIGN - (rlp_node_length % MIN_ALIGN)) % MIN_ALIGN;
        unsafe { advance_unchecked(bytes, padding_len) };

        // calculate node's reference and ensure it matches the `expected_node_ref` from parent.
        let node_ref = {
            if rlp_node_length < 32 {
                if rlp_node != expected_node_ref.as_slice() {
                    return Err(Error::NodeRefMismatch);
                }
                NodeRef::Bytes(rlp_node)
            } else if payload_length == 32 && !list {
                if payload != expected_node_ref.as_slice() {
                    return Err(Error::NodeRefMismatch);
                }
                expected_node_ref
            } else {
                let digest = keccak256(rlp_node);
                if digest != expected_node_ref.as_slice() {
                    return Err(Error::NodeRefMismatch);
                }
                expected_node_ref
            }
        };

        if !list {
            let node_id = match payload_length {
                0 => NULL_NODE_ID,
                32 => self.add_node(NodeData::Digest(payload), Some(NodeRef::Digest(payload))),
                _ => {
                    return Err(Error::RlpError(alloy_rlp::Error::UnexpectedLength));
                }
            };
            return Ok(node_id);
        }

        // first payload item
        let item0_header_start = payload;
        let alloy_rlp::Header { payload_length: item0_payload_length, .. } =
            alloy_rlp::Header::decode(&mut payload)?;
        let item0_payload_start = unsafe { advance_unchecked(&mut payload, item0_payload_length) };
        let item0_length = item0_header_start.len() - payload.len();

        // second payload item
        let item1_header_start = payload;
        let alloy_rlp::Header { payload_length: item1_payload_length, .. } =
            alloy_rlp::Header::decode(&mut payload)?;
        let item1_payload_start = unsafe { advance_unchecked(&mut payload, item1_payload_length) };
        let item1_length = item1_header_start.len() - payload.len();

        if payload.is_empty() {
            // either an extension or leaf
            let path = &item0_payload_start[..item0_payload_length];
            let prefix = path[0];
            if (prefix & (2 << 4)) == 0 {
                // extension node
                let ext_node_expected_ref =
                    NodeRef::from_rlp_slice(&item1_header_start[..item1_length]);
                let ext_node_id = self.decode_trie_internal(bytes, ext_node_expected_ref)?;
                let node_data = NodeData::Extension(path, ext_node_id);
                return Ok(self.add_node(node_data, Some(node_ref)));
            } else {
                // leaf node
                let value = &item1_payload_start[..item1_payload_length];
                let node_data = NodeData::Leaf(path, value);
                return Ok(self.add_node(node_data, Some(node_ref)));
            }
        }

        // branch
        let child0_expected_node_ref = NodeRef::from_rlp_slice(&item0_header_start[..item0_length]);
        let child0 = {
            if child0_expected_node_ref.as_slice() == NULL_NODE_REF_SLICE {
                None
            } else {
                Some(self.decode_trie_internal(bytes, child0_expected_node_ref)?)
            }
        };

        let child1_expected_node_ref = NodeRef::from_rlp_slice(&item1_header_start[..item1_length]);
        let child1 = {
            if child1_expected_node_ref.as_slice() == NULL_NODE_REF_SLICE {
                None
            } else {
                Some(self.decode_trie_internal(bytes, child1_expected_node_ref)?)
            }
        };

        // Create an uninitialized array to avoid wasteful default initialization
        let mut childs: [MaybeUninit<Option<NodeId>>; 16] =
            unsafe { MaybeUninit::uninit().assume_init() };

        // Initialize first two elements
        childs[0] = MaybeUninit::new(child0);
        childs[1] = MaybeUninit::new(child1);

        // Initialize remaining elements
        for child in &mut childs[2..] {
            let item_header_start = payload;
            let alloy_rlp::Header { payload_length: item_payload_length, .. } =
                alloy_rlp::Header::decode(&mut payload)?;
            unsafe { advance_unchecked(&mut payload, item_payload_length) };
            let item_length = item_header_start.len() - payload.len();

            let child_expected_node_ref =
                NodeRef::from_rlp_slice(&item_header_start[..item_length]);

            *child = MaybeUninit::new({
                if child_expected_node_ref.as_slice() == NULL_NODE_REF_SLICE {
                    None
                } else {
                    Some(self.decode_trie_internal(bytes, child_expected_node_ref)?)
                }
            });
        }

        // Transmute the fully initialized array to the final type
        let childs: [Option<NodeId>; 16] = unsafe {
            std::mem::transmute::<[MaybeUninit<Option<NodeId>>; 16], [Option<NodeId>; 16]>(childs)
        };

        if payload != NULL_NODE_REF_SLICE {
            return Err(Error::ValueInBranch);
        }

        let node_data = NodeData::Branch(childs);
        let node_id = self.add_node(node_data, Some(node_ref));
        Ok(node_id)
    }
}

const NULL_NODE_REF_SLICE: &[u8] = &[alloy_rlp::EMPTY_STRING_CODE];

impl<'a> MptTrie<'a> {
    #[inline]
    fn calc_reference(&self, node_id: NodeId) -> NodeRef<'a> {
        match &self.nodes[node_id as usize] {
            NodeData::Null => NodeRef::Bytes(NULL_NODE_REF_SLICE),
            NodeData::Digest(digest) => NodeRef::Digest(digest),
            _ => {
                let payload_length = self.payload_length(node_id);
                let rlp_length = payload_length + alloy_rlp::length_of_length(payload_length);

                if rlp_length < 32 {
                    let mut encoded = BumpBytesMut::with_capacity_in(rlp_length, self.bump);
                    self.encode_with_payload_len(node_id, payload_length, &mut encoded);
                    debug_assert_eq!(encoded.len(), rlp_length);

                    NodeRef::Bytes(encoded.into_inner().into_bump_slice())
                } else {
                    let mut scratch = self.rlp_scratch.borrow_mut();
                    scratch.clear();

                    self.encode_with_payload_len(node_id, payload_length, &mut *scratch);
                    debug_assert_eq!(scratch.len(), rlp_length);

                    let digest = keccak256(&*scratch);
                    // [shayanh]: This copy could have been avoided if we had allocated keccak's output
                    // in the bump area from the beginning. This needs to have a
                    // separate keccak implementation. Since I wanted to stick
                    // to alloy's keccak, I didn't create a new one.
                    let digest_slice = self.bump.alloc_slice_copy(digest.as_slice());
                    NodeRef::Digest(digest_slice)
                }
            }
        }
    }

    #[inline]
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
                out.put_slice(digest);
            }
        }
    }

    /// Returns the length of the RLP payload of the node.
    #[inline]
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
    /// Root hash of the MPT.
    #[inline]
    pub fn hash(&self) -> B256 {
        match self.nodes[self.root_id as usize] {
            NodeData::Null => reth_trie::EMPTY_ROOT_HASH,
            _ => match self.cached_references[self.root_id as usize]
                .borrow_mut()
                .get_or_insert_with(|| self.calc_reference(self.root_id))
            {
                NodeRef::Digest(digest) => B256::from_slice(digest),
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

    /// Inserts an RLP-encoded value into the trie.
    #[inline]
    pub fn insert_rlp(
        &mut self,
        key: &[u8],
        value: impl alloy_rlp::Encodable,
    ) -> Result<bool, Error> {
        let mut rlp_bytes = BumpBytesMut::with_capacity_in(VALUE_RLP_BUFFER_CAPACITY, self.bump);
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
    pub fn is_empty(&self) -> bool {
        matches!(&self.nodes[self.root_id as usize], NodeData::Null)
    }

    /// Reserves additional capacity for the trie.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
        self.cached_references.reserve(additional);
    }
}

// Internal Implementation
impl<'a> MptTrie<'a> {
    #[inline]
    fn add_node(&mut self, data: NodeData<'a>, node_ref: Option<NodeRef<'a>>) -> NodeId {
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
            NodeData::Digest(digest) => Err(Error::NodeNotResolved(B256::from_slice(digest))),
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
                let self_nibs = prefix_to_nibs(prefix);
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
                let self_nibs = prefix_to_nibs(prefix);
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
                return Err(Error::NodeNotResolved(B256::from_slice(digest)));
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
                            let leaf_nibs = prefix_to_nibs(prefix);
                            let mut new_nibs: SmallVec<[u8; 64]> =
                                SmallVec::with_capacity(1 + leaf_nibs.len());
                            new_nibs.push(index as u8);
                            new_nibs.extend_from_slice(&leaf_nibs);
                            let new_path = to_encoded_path(self.bump, &new_nibs, true);
                            NodeData::Leaf(new_path, value)
                        }
                        NodeData::Extension(prefix, child_child_id) => {
                            let ext_nibs = prefix_to_nibs(prefix);
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
                let leaf_nibs = prefix_to_nibs(prefix);
                if leaf_nibs.as_slice() != key_nibs {
                    return Ok(false);
                }
                self.nodes[node_id as usize] = NodeData::Null;
                true
            }
            NodeData::Extension(prefix, child_id) => {
                let self_nibs = prefix_to_nibs(prefix);
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
                        let child_path_nibs = prefix_to_nibs(child_path_bytes);
                        let mut combined_nibs: SmallVec<[u8; 64]> =
                            SmallVec::with_capacity(self_nibs.len() + child_path_nibs.len());
                        combined_nibs.extend_from_slice(&self_nibs);
                        combined_nibs.extend_from_slice(&child_path_nibs);
                        let new_path = to_encoded_path(self.bump, &combined_nibs, true);
                        NodeData::Leaf(new_path, value)
                    }
                    // for an extension, replace the extension with the extended extension
                    NodeData::Extension(child_path_bytes, grandchild_id) => {
                        let child_path_nibs = prefix_to_nibs(child_path_bytes);
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
                return Err(Error::NodeNotResolved(B256::from_slice(digest)));
            }
        };

        if updated {
            self.invalidate_ref_cache(node_id);
        }
        Ok(updated)
    }
}

impl MptTrie<'_> {
    pub fn print_trie(&self) {
        self.print_trie_internal(self.root_id, 0);
    }

    fn print_trie_internal(&self, node_id: NodeId, depth: usize) {
        let indent = "  ".repeat(depth);
        match &self.nodes[node_id as usize] {
            NodeData::Null => {
                println!("{}Null", indent);
            }
            NodeData::Branch(children) => {
                println!("{}Branch", indent);
                for (i, child) in children.iter().enumerate() {
                    if let Some(child_id) = child {
                        println!("{}  [{}]:", indent, i);
                        self.print_trie_internal(*child_id, depth + 2);
                    }
                }
            }
            NodeData::Leaf(path, value) => {
                let path_nibs = prefix_to_nibs(path);
                println!(
                    "{}Leaf path={:?} value_len={}",
                    indent,
                    path_nibs.as_slice(),
                    value.len()
                );
            }
            NodeData::Extension(path, child_id) => {
                let path_nibs = prefix_to_nibs(path);
                println!("{}Extension path={:?}", indent, path_nibs.as_slice());
                self.print_trie_internal(*child_id, depth + 1);
            }
            NodeData::Digest(digest) => {
                println!("{}Digest {:?}", indent, B256::from_slice(digest));
            }
        }
    }
}
