use bumpalo::Bump;
use reth_trie::AccountProof;
use revm_primitives::{keccak256, map::DefaultHashBuilder, Address, HashMap, B256};

use crate::{
    hp::{prefix_to_nibs, to_encoded_path},
    node::{NodeData, NodeId},
    owned::MptOwned,
    Error, EthereumState,
};

/// Parses proof bytes into a vector of tries.
fn parse_proof(proof: &[impl AsRef<[u8]>]) -> Result<Vec<MptOwned>, Error> {
    proof
        .iter()
        .map(|bytes| MptOwned::decode_from_proof_rlp(&mut bytes.as_ref()))
        .collect::<Result<Vec<_>, _>>()
}

/// Processes a proof by parsing it into a vector of tries and adding them to the given node store.
fn process_proof(
    proof: &[impl AsRef<[u8]>],
    node_store: &mut HashMap<B256, MptOwned>,
) -> Result<Option<MptOwned>, Error> {
    let proof_nodes = parse_proof(proof)?;
    if !proof_nodes.is_empty() {
        node_store.reserve(proof_nodes.len());
    }
    let root_node = proof_nodes.first().cloned();
    for node in proof_nodes {
        node_store.insert(node.hash(), node);
    }
    Ok(root_node)
}

/// Adds all the leaf nodes of non-inclusion proofs to the nodes.
fn add_orphaned_leafs(
    key: impl AsRef<[u8]>,
    proof: &[impl AsRef<[u8]>],
    node_store: &mut HashMap<B256, MptOwned>,
) -> Result<(), Error> {
    if !proof.is_empty() {
        let proof_nodes = parse_proof(proof)?;
        if is_not_included(keccak256(key).as_slice(), &proof_nodes)? {
            for node in shorten_node_path(proof_nodes.last().unwrap()) {
                node_store.insert(node.hash(), node);
            }
        }
    }
    Ok(())
}

/// Returns a list of all possible nodes that can be created by shortening the path of the
/// given node.
/// When nodes in an MPT are deleted, leaves or extensions may be extended. To still be
/// able to identify the original nodes, we create all shortened versions of the node.
fn shorten_node_path(node: &MptOwned) -> Vec<MptOwned> {
    let mut res = Vec::new();
    let (prefix, is_leaf, value, child_id) = match node.get_node(node.root_id()).unwrap() {
        NodeData::Leaf(prefix, value) => (*prefix, true, Some(*value), None),
        NodeData::Extension(prefix, child_id) => (*prefix, false, None, Some(*child_id)),
        _ => return res,
    };

    let nibs = prefix_to_nibs(prefix);

    for i in 0..=nibs.len() {
        let shortened_nibs = &nibs[i..];
        let path = to_encoded_path(shortened_nibs, is_leaf);
        let new_node = if is_leaf {
            let mut new_node = MptOwned::default();
            let value = value.unwrap();
            new_node.set_node(new_node.root_id(), &NodeData::Leaf(&path, value));
            new_node
        } else {
            let mut new_node = MptOwned::from_trie(node.inner());
            let child_id = child_id.unwrap();
            new_node.set_node(new_node.root_id(), &NodeData::Extension(&path, child_id));
            new_node
        };
        res.push(new_node);
    }
    res
}

fn is_not_included(key: &[u8], proof_nodes: &[MptOwned]) -> Result<bool, Error> {
    let proof_trie = mpt_from_proof(proof_nodes)?;
    // For valid proofs, the get must not fail
    let value = proof_trie.get(key)?;
    Ok(value.is_none())
}

fn mpt_from_proof(proof_nodes: &[MptOwned]) -> Result<MptOwned, Error> {
    if proof_nodes.is_empty() {
        return Ok(MptOwned::default());
    }

    let node_store: HashMap<B256, MptOwned> =
        proof_nodes.iter().map(|node| (node.hash(), node.clone())).collect();

    let root_node = proof_nodes.first().unwrap();

    Ok(resolve_nodes(root_node, &node_store))
}

fn resolve_nodes(root: &MptOwned, node_store: &HashMap<B256, MptOwned>) -> MptOwned {
    let mut new_trie = MptOwned::default();

    let root_id = resolve_nodes_internal(root, root.root_id(), node_store, &mut new_trie);
    new_trie.set_root_id(root_id);

    // The root hash must not change after resolution
    debug_assert_eq!(root.hash(), new_trie.hash());

    new_trie
}

fn resolve_nodes_internal(
    cur_trie: &MptOwned,
    node_id: NodeId,
    node_store: &HashMap<B256, MptOwned>,
    new_trie: &mut MptOwned,
) -> NodeId {
    let cur_data = &cur_trie.get_node(node_id).unwrap();
    let resolved_data = match cur_data {
        NodeData::Null => NodeData::Null,
        NodeData::Leaf(prefix, value) => NodeData::Leaf(prefix, value),
        NodeData::Branch(childs) => {
            let mut resolved_children: [Option<NodeId>; 16] = Default::default();
            for (i, child_id) in childs.iter().enumerate() {
                if let Some(child_id) = child_id {
                    let resolved_child_id =
                        resolve_nodes_internal(cur_trie, *child_id, node_store, new_trie);
                    resolved_children[i] = Some(resolved_child_id);
                }
            }
            NodeData::Branch(resolved_children)
        }
        NodeData::Extension(prefix, child_id) => {
            let resolved_child_id =
                resolve_nodes_internal(cur_trie, *child_id, node_store, new_trie);
            NodeData::Extension(prefix, resolved_child_id)
        }
        NodeData::Digest(digest) => {
            if let Some(trie) = node_store.get(&B256::from_slice(digest)) {
                return resolve_nodes_internal(trie, trie.root_id(), node_store, new_trie);
            } else {
                NodeData::Digest(digest)
            }
        }
    };
    new_trie.add_node(&resolved_data)
}

fn node_from_digest(digest: B256) -> MptOwned {
    match digest {
        reth_trie::EMPTY_ROOT_HASH | B256::ZERO => MptOwned::default(),
        _ => {
            let mut trie = MptOwned::default();
            trie.set_node(trie.root_id(), &NodeData::Digest(digest.as_slice()));
            trie
        }
    }
}

fn build_storage_trie(proof: &AccountProof, fini_proofs: &AccountProof) -> Result<MptOwned, Error> {
    if proof.storage_proofs.is_empty() {
        return Ok(node_from_digest(proof.storage_root));
    }

    let mut storage_nodes = HashMap::default();
    let mut storage_root_node = MptOwned::default();

    for storage_proof in &proof.storage_proofs {
        if let Some(root) = process_proof(&storage_proof.proof, &mut storage_nodes)? {
            storage_root_node = root;
        }
    }

    for storage_proof in &fini_proofs.storage_proofs {
        add_orphaned_leafs(storage_proof.key.0, &storage_proof.proof, &mut storage_nodes)?;
    }

    Ok(resolve_nodes(&storage_root_node, &storage_nodes))
}

pub fn transition_proofs_to_tries(
    state_root: B256,
    parent_proofs: &HashMap<Address, AccountProof>,
    proofs: &HashMap<Address, AccountProof>,
) -> Result<EthereumState, Error> {
    let bump = Box::leak(Box::new(Bump::new()));

    if parent_proofs.is_empty() {
        return Ok(EthereumState {
            state_trie: node_from_digest(state_root).into_inner(),
            storage_tries: HashMap::default(),
            bump,
        });
    }

    let mut storage_tries =
        HashMap::with_capacity_and_hasher(parent_proofs.len(), DefaultHashBuilder::default());
    let mut state_nodes =
        HashMap::with_capacity_and_hasher(parent_proofs.len(), DefaultHashBuilder::default());
    let mut state_root_node = MptOwned::default();

    for (address, proof) in parent_proofs {
        if let Some(root) = process_proof(&proof.proof, &mut state_nodes)? {
            state_root_node = root;
        }

        let fini_proofs = proofs.get(address).unwrap();
        add_orphaned_leafs(address, &fini_proofs.proof, &mut state_nodes)?;

        let storage_trie = build_storage_trie(proof, fini_proofs)?;
        storage_tries.insert(B256::from(keccak256(address)), storage_trie.into_inner());
    }

    let state_trie = resolve_nodes(&state_root_node, &state_nodes);
    Ok(EthereumState { state_trie: state_trie.into_inner(), storage_tries, bump })
}
