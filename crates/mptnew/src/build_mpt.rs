use alloy_rlp::{Decodable, Encodable};
use bumpalo::Bump;
use reth_trie::AccountProof;
use revm_primitives::{keccak256, Address, HashMap, B256};

use crate::{
    hp::{prefix_to_small_nibs, to_encoded_path},
    node::{NodeRef, NodeRlpDecoded},
    Error, EthereumState, MptTrie, EMPTY_ROOT,
};

#[derive(Clone, Debug)]
enum MptNodeOwned {
    /// Represents an empty trie node.
    Null,
    /// A node that can have up to 16 children. Each child is an optional boxed [MptNode].
    Branch(Vec<NodeRef>),
    /// A leaf node that contains a key and a value, both represented as byte vectors.
    Leaf(Vec<u8>, Vec<u8>),
    /// A node that has exactly one child and is used to represent a shared prefix of
    /// several keys.
    Extension(Vec<u8>, NodeRef),
    /// Represents a sub-trie by its hash, allowing for efficient storage of large
    /// sub-tries without storing their entire content.
    Digest(B256),
}

impl NodeRef {
    fn from_rlp_slice(slice: &[u8]) -> Self {
        if slice.len() == 33 {
            Self::Digest(B256::from_slice(&slice[1..]))
        } else {
            Self::Bytes(slice.to_vec())
        }
    }
}

impl alloy_rlp::Encodable for NodeRef {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match &self {
            NodeRef::Bytes(bytes) => out.put_slice(bytes),
            NodeRef::Digest(digest) => {
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE + 32);
                out.put_slice(digest.as_slice());
            }
        }
    }

    fn length(&self) -> usize {
        match self {
            NodeRef::Bytes(bytes) => bytes.len(),
            NodeRef::Digest(_) => 33,
        }
    }
}

impl alloy_rlp::Encodable for MptNodeOwned {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            MptNodeOwned::Null => out.put_u8(alloy_rlp::EMPTY_STRING_CODE),
            MptNodeOwned::Branch(child_refs) => {
                let payload_length: usize =
                    1 + child_refs.iter().map(|c| c.length()).sum::<usize>();
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                child_refs.iter().for_each(|c| c.encode(out));
                // in the MPT reference, branches have values so always add empty value
                out.put_u8(alloy_rlp::EMPTY_STRING_CODE);
            }
            MptNodeOwned::Leaf(prefix, value) => {
                let payload_length = prefix.as_slice().length() + value.as_slice().length();
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                prefix.as_slice().encode(out);
                value.as_slice().encode(out);
            }
            MptNodeOwned::Extension(prefix, node_ref) => {
                let payload_length = prefix.as_slice().length() + node_ref.length();
                alloy_rlp::Header { list: true, payload_length }.encode(out);
                prefix.as_slice().encode(out);
                node_ref.encode(out);
            }
            MptNodeOwned::Digest(digest) => {
                digest.encode(out);
            }
        }
    }
}

impl alloy_rlp::Decodable for MptNodeOwned {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let node_rlp_decoded = NodeRlpDecoded::decode_rlp(buf).unwrap();

        let t = match node_rlp_decoded {
            NodeRlpDecoded::Null => Self::Null,
            NodeRlpDecoded::Branch(child_slices) => {
                Self::Branch(child_slices.iter().map(|c| NodeRef::from_rlp_slice(c)).collect())
            }
            NodeRlpDecoded::Leaf(prefix, value) => Self::Leaf(prefix.to_vec(), value.to_vec()),
            NodeRlpDecoded::Extension(prefix, node_ref_slice) => {
                Self::Extension(prefix.to_vec(), NodeRef::from_rlp_slice(node_ref_slice))
            }
            NodeRlpDecoded::Digest(digest) => Self::Digest(B256::from_slice(digest)),
        };
        Ok(t)
    }
}

fn shorten_node_path(node: &MptNodeOwned) -> Vec<MptNodeOwned> {
    let bump = Bump::new();

    let mut res = Vec::new();
    match node {
        MptNodeOwned::Null | MptNodeOwned::Branch(_) | MptNodeOwned::Digest(_) => {}
        MptNodeOwned::Leaf(prefix, value) => {
            let nibs = prefix_to_small_nibs(prefix);
            for i in 0..=nibs.len() {
                res.push(MptNodeOwned::Leaf(
                    to_encoded_path(&bump, &nibs[i..], true).to_vec(),
                    value.clone(),
                ));
            }
        }
        MptNodeOwned::Extension(prefix, node_ref) => {
            let nibs = prefix_to_small_nibs(prefix);
            for i in 0..=nibs.len() {
                res.push(MptNodeOwned::Extension(
                    to_encoded_path(&bump, &nibs[i..], true).to_vec(),
                    node_ref.clone(),
                ));
            }
        }
    }

    res
}

fn orphaned_leafs(key: impl AsRef<[u8]>, proof: &[Vec<u8>], root_hash: B256) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    if !proof.is_empty() {
        let bump = Bump::new();
        let mpt = MptTrie::resolve_nodes(&bump, root_hash, 0, proof).unwrap();
        if mpt.get(keccak256(key).as_slice()).unwrap().is_none() {
            let last_node = MptNodeOwned::decode(&mut proof.last().unwrap().as_slice()).unwrap();
            shorten_node_path(&last_node).into_iter().for_each(|node| {
                let mut buf = Vec::new();
                node.encode(&mut buf.as_mut_slice());
                res.push(buf);
            });
        }
    }
    res
}

fn proof_rlp_nodes(rlp_node: Vec<u8>) -> Vec<Vec<u8>> {
    let mut res = vec![rlp_node.clone()];
    let node = MptNodeOwned::decode(&mut rlp_node.as_slice()).unwrap();
    match node {
        MptNodeOwned::Branch(node_refs) => {
            node_refs.into_iter().for_each(|node_ref| match node_ref {
                NodeRef::Bytes(bytes) => res.append(&mut proof_rlp_nodes(bytes)),
                NodeRef::Digest(_) => {
                    let mut buf = Vec::new();
                    node_ref.encode(&mut buf);
                    res.push(buf);
                }
            });
        }
        MptNodeOwned::Extension(_, node_ref) => match node_ref {
            NodeRef::Bytes(bytes) => res.append(&mut proof_rlp_nodes(bytes)),
            NodeRef::Digest(_) => {
                let mut buf = Vec::new();
                node_ref.encode(&mut buf);
                res.push(buf);
            }
        },
        _ => {}
    };
    res
}

pub fn transition_proofs_to_tries<'a>(
    bump: &'a Bump,
    parent_state_root: B256,
    parent_proofs: &HashMap<Address, AccountProof>,
    state_root: B256,
    proofs: &HashMap<Address, AccountProof>,
) -> Result<EthereumState<'a>, Error> {
    // If no addresses are provided, return the trie only consisting of the state root
    if parent_proofs.is_empty() {
        return Ok(EthereumState {
            state_trie: node_from_digest(bump, parent_state_root),
            // storage_tries: Default::default(),
        });
    }

    let mut state_trie_rlp_nodes = bumpalo::collections::Vec::new_in(bump);

    for (address, proof) in parent_proofs {
        for proof_rlp_node in &proof.proof {
            proof_rlp_nodes(proof_rlp_node.to_vec())
                .into_iter()
                .for_each(|n| state_trie_rlp_nodes.push(n));
        }

        // let finish_proof: Vec<_> =
        //     proofs.get(address).unwrap().proof.iter().map(|proof| proof.to_vec()).collect();
        // orphaned_leafs(address, &finish_proof, state_root)
        //     .into_iter()
        //     .for_each(|n| state_trie_rlp_nodes.push(n));
    }

    let state_trie =
        MptTrie::resolve_nodes(bump, parent_state_root, 0, state_trie_rlp_nodes.into_bump_slice())
            .unwrap();

    Ok(EthereumState { state_trie })
}

/// Creates a new arena-based MPT node from a digest.
fn node_from_digest<'a>(bump: &'a Bump, digest: B256) -> MptTrie<'a> {
    match digest {
        EMPTY_ROOT | B256::ZERO => MptTrie::new(bump),
        _ => MptTrie::with_root_hash(bump, digest),
    }
}
