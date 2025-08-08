// This code runs on host so it is not as performance critical as the rest of mpt
use crate::mpt::{ArenaBasedMptNode, ArenaNodeData, Error, MptNodeReference, NodeId};
use crate::{EthereumState, StorageTries};
use bumpalo::Bump;
use reth_trie::AccountProof;
use revm::primitives::{HashMap, B256};
use revm_primitives::{keccak256, Address};
use std::{cell::RefCell, rc::Rc};

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
        crate::mpt::EMPTY_ROOT | B256::ZERO => ArenaBasedMptNode::default(),
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
    let nibs = crate::hp::prefix_to_small_nibs(path_bytes);

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
        rlp_scratch: RefCell::new(Vec::with_capacity(128)),
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
