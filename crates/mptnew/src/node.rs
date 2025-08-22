use alloy_rlp::{Header, PayloadView};
use revm_primitives::B256;

use crate::Error;

pub(crate) type NodeId = u32;

/// Node data for arena-based trie with zero-copy optimization
#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum NodeData<'a> {
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

/// Represents the ways in which one node can reference another node inside the sparse Merkle
/// Patricia Trie (MPT).
///
/// Nodes in the MPT can reference other nodes either directly through their byte representation or
/// indirectly through a hash of their encoding.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) enum NodeRef {
    /// Represents a direct reference to another node using its byte encoding. Typically
    /// used for short encodings that are less than 32 bytes in length.
    Bytes(Vec<u8>),
    /// Represents an indirect reference to another node using the Keccak hash of its long
    /// encoding. Used for encodings that are not less than 32 bytes in length.
    Digest(B256),
}

#[derive(Debug, thiserror::Error)]
pub enum NodeRefError {
    #[error("rlp decode error: {0}")]
    RlpError(#[from] alloy_rlp::Error),
    #[error("digest length mismatch: {0}")]
    DigestLengthMismatch(#[from] core::array::TryFromSliceError),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum NodeRlpDecoded<'a> {
    #[default]
    /// Absence of a node. Encoded as empty string in RLP.
    Null,
    /// 16-way branch. Each child is optional; the branch's value slot is unused in our state trie
    /// and must be empty, enforced during decoding.
    Branch(Vec<&'a [u8]>),
    /// Leaf node containing a compact hex-prefix path and a value. Both slices borrow from the
    /// input buffer or bump arena. The path encodes the remainder of the key.
    Leaf(&'a [u8], &'a [u8]),
    /// Extension node containing a compact hex-prefix path and a single child. Path encodes a
    /// shared prefix to skip before continuing at `child`.
    Extension(&'a [u8], &'a [u8]),
    /// Unresolved reference to a node by its Keccak-256 digest (32 bytes). Encountering this in
    /// `get`/`insert`/`delete` is an error; resolution happens in `build_mpt` helpers.
    Digest(&'a [u8]),
}

impl<'a> NodeRlpDecoded<'a> {
    pub(crate) fn decode_rlp(bytes: &mut &'a [u8]) -> Result<Self, Error> {
        let payload_view = Header::decode_raw(bytes)?;
        let res = match payload_view {
            PayloadView::String(item) => match item.len() {
                0 => NodeRlpDecoded::Null,
                32 => NodeRlpDecoded::Digest(item),
                _ => {
                    return Err(Error::RlpError(alloy_rlp::Error::UnexpectedLength));
                }
            },
            PayloadView::List(mut items) => match items.len() {
                2 => {
                    let PayloadView::String(path) = Header::decode_raw(&mut items[0])? else {
                        return Err(Error::RlpError(alloy_rlp::Error::UnexpectedList));
                    };

                    let prefix = path[0];
                    if (prefix & (2 << 4)) == 0 {
                        // extension node
                        let ref_node_rlp = items[1];
                        NodeRlpDecoded::Extension(path, ref_node_rlp)
                    } else {
                        // leaf node
                        let PayloadView::String(value) = Header::decode_raw(&mut items[1])? else {
                            return Err(Error::RlpError(alloy_rlp::Error::UnexpectedList));
                        };
                        NodeRlpDecoded::Leaf(path, value)
                    }
                }
                17 => {
                    let PayloadView::String(value) = Header::decode_raw(&mut items[16])? else {
                        return Err(Error::RlpError(alloy_rlp::Error::UnexpectedList));
                    };
                    if !value.is_empty() {
                        return Err(Error::ValueInBranch);
                    }
                    items.pop();
                    NodeRlpDecoded::Branch(items)
                }
                _ => {
                    return Err(Error::RlpError(alloy_rlp::Error::UnexpectedLength));
                }
            },
        };
        Ok(res)
    }
}
