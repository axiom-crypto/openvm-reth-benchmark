use bumpalo::Bump;
use reth_trie::AccountProof;
use revm_primitives::{keccak256, Address, HashMap, B256};
use smallvec::SmallVec;

use crate::{
    hp::{
        encoded_path_eq_nibs, encoded_path_nibble_count, encoded_path_strip_prefix, prefix_to_nibs,
        to_encoded_path,
    },
    node::{NodeData, NodeId},
    owned::MptOwned,
    Error, EthereumState,
};

/// Nibble sequence type for key traversal.
type Nibbles = SmallVec<[u8; 64]>;

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

/// Represents an unresolvable orphan in the storage trie.
/// When a storage deletion would cause branch collapse but the sibling is unresolved,
/// we need to fetch additional proofs for a storage key with this prefix.
#[derive(Debug, Clone)]
pub struct StorageOrphan {
    /// The address of the account whose storage trie has the orphan.
    pub address: Address,
    /// The nibble prefix of the unresolved sibling.
    pub prefix: Vec<u8>,
}

/// Represents an unresolvable orphan in the state trie.
/// When an account deletion would cause branch collapse but the sibling is unresolved,
/// we need to fetch additional proofs for an account with this hash prefix.
#[derive(Debug, Clone)]
pub struct StateOrphan {
    /// The nibble prefix of the unresolved sibling in the state trie.
    pub prefix: Vec<u8>,
}

/// Detects accounts whose deletion would fail due to unresolved siblings in the state trie.
///
/// For each deleted account address, this function checks if the deletion would cause
/// a branch collapse with an unresolved sibling.
///
/// # Arguments
/// * `trie` - The state trie built from proofs
/// * `deleted_addresses` - Account addresses that will be deleted
fn detect_state_orphans(trie: &MptOwned, deleted_addresses: &[Address]) -> Vec<Vec<u8>> {
    let mut orphan_prefixes = Vec::new();

    for address in deleted_addresses {
        let hashed_address = keccak256(address);
        let key_nibs = to_nibbles(hashed_address.as_slice());

        if let Some(prefix) = find_unresolvable_sibling(trie, trie.root_id(), &key_nibs, &[]) {
            orphan_prefixes.push(prefix);
        }
    }

    orphan_prefixes
}

/// Detects storage slots whose deletion would fail due to unresolved siblings.
///
/// For each deleted storage slot (value goes from non-zero to zero), this function
/// checks if the deletion would cause a branch collapse with an unresolved sibling.
/// Returns the nibble prefixes of any unresolvable siblings.
///
/// # Arguments
/// * `trie` - The storage trie built from proofs
/// * `deleted_keys` - Storage keys (not hashed) that will be deleted
fn detect_storage_orphans(trie: &MptOwned, deleted_keys: &[B256]) -> Vec<Vec<u8>> {
    let mut orphan_prefixes = Vec::new();

    for key in deleted_keys {
        let hashed_key = keccak256(key);
        let key_nibs = to_nibbles(hashed_key.as_slice());

        if let Some(prefix) = find_unresolvable_sibling(trie, trie.root_id(), &key_nibs, &[]) {
            orphan_prefixes.push(prefix);
        }
    }

    orphan_prefixes
}

/// Converts a byte slice to nibbles.
fn to_nibbles(bytes: &[u8]) -> Nibbles {
    let mut result = SmallVec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(byte >> 4);
        result.push(byte & 0x0f);
    }
    result
}

/// Traverses the trie following the key path and checks if deleting this key
/// would cause a branch collapse with an unresolved sibling.
///
/// Returns Some(prefix) if an unresolvable sibling is found, None otherwise.
fn find_unresolvable_sibling(
    trie: &MptOwned,
    node_id: NodeId,
    remaining_key: &[u8],
    current_prefix: &[u8],
) -> Option<Vec<u8>> {
    let node = trie.get_node(node_id)?;

    match node {
        NodeData::Null => None,

        NodeData::Leaf(path, _) => {
            // If we reach a leaf that matches, deletion happens here.
            // No branch collapse at this point.
            if encoded_path_eq_nibs(path, remaining_key) {
                // Exact match - deletion at leaf level, no collapse issue
                None
            } else {
                // Key doesn't match leaf path - key not in trie
                None
            }
        }

        NodeData::Branch(children) => {
            if remaining_key.is_empty() {
                // Key ends at branch - no collapse
                return None;
            }

            let nibble = remaining_key[0] as usize;
            let child_id = children[nibble]?;

            // Count how many children this branch has
            let child_count = children.iter().filter(|c| c.is_some()).count();

            if child_count == 2 {
                // After deleting one child, branch would collapse.
                // Find the sibling (the other child).
                for (i, child) in children.iter().enumerate() {
                    if i != nibble {
                        if let Some(sibling_id) = child {
                            // Check if sibling is unresolved
                            if let Some(NodeData::Digest(_)) = trie.get_node(*sibling_id) {
                                // Sibling is unresolved! Build the prefix.
                                let mut prefix = current_prefix.to_vec();
                                prefix.push(i as u8);
                                return Some(prefix);
                            }
                        }
                    }
                }
            }

            // Continue traversing
            let mut new_prefix = current_prefix.to_vec();
            new_prefix.push(nibble as u8);
            find_unresolvable_sibling(trie, child_id, &remaining_key[1..], &new_prefix)
        }

        NodeData::Extension(path, child_id) => {
            // Strip the extension path from the remaining key
            if let Some(tail) = encoded_path_strip_prefix(path, remaining_key) {
                let path_len = encoded_path_nibble_count(path);
                let mut new_prefix = current_prefix.to_vec();
                new_prefix.extend_from_slice(&remaining_key[..path_len]);
                find_unresolvable_sibling(trie, *child_id, tail, &new_prefix)
            } else {
                // Key doesn't match extension path - key not in trie
                None
            }
        }

        NodeData::Digest(_) => {
            // We hit an unresolved node while traversing.
            // This means the witness is incomplete for this key.
            // Return the current prefix as needing resolution.
            Some(current_prefix.to_vec())
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

/// Result of building tries from proofs, including any unresolvable orphans.
#[derive(Debug)]
pub struct TransitionResult {
    /// The built Ethereum state.
    pub state: EthereumState,
    /// Unresolvable storage orphans that need additional proofs.
    pub storage_orphans: Vec<StorageOrphan>,
    /// Unresolvable state trie orphans that need additional proofs.
    pub state_orphans: Vec<StateOrphan>,
}

/// Storage deletions for an account.
#[derive(Debug, Clone, Default)]
pub struct StorageDeletions {
    /// Storage keys (not hashed) that will be deleted (value goes from non-zero to zero).
    pub deleted_keys: Vec<B256>,
}

/// Builds tries from proofs and detects any unresolvable orphans.
///
/// This is the main entry point for witness generation with orphan detection.
/// If orphans are returned, the caller should:
/// 1. Find keys/addresses whose hashes start with the orphan prefixes
/// 2. Fetch additional proofs for those keys
/// 3. Retry with the additional proofs
pub fn transition_proofs_to_tries_with_deletions(
    state_root: B256,
    parent_proofs: &HashMap<Address, AccountProof>,
    proofs: &HashMap<Address, AccountProof>,
    storage_deletions: &HashMap<Address, StorageDeletions>,
    deleted_accounts: &[Address],
) -> Result<TransitionResult, Error> {
    let bump = Box::leak(Box::new(Bump::new()));

    if parent_proofs.is_empty() {
        return Ok(TransitionResult {
            state: EthereumState {
                state_trie: node_from_digest(state_root).into_inner(),
                storage_tries: HashMap::default(),
                bump,
            },
            storage_orphans: vec![],
            state_orphans: vec![],
        });
    }

    let mut storage_tries = HashMap::default();
    let mut state_nodes = HashMap::default();
    let mut state_root_node = MptOwned::default();
    let mut all_storage_orphans = Vec::new();

    for (address, proof) in parent_proofs {
        if let Some(root) = process_proof(&proof.proof, &mut state_nodes)? {
            state_root_node = root;
        }

        let fini_proofs = proofs.get(address).unwrap();
        add_orphaned_leafs(address, &fini_proofs.proof, &mut state_nodes)?;

        let storage_trie = build_storage_trie(proof, fini_proofs)?;

        // Check for storage orphans if there are deletions for this account
        if let Some(account_deletions) = storage_deletions.get(address) {
            let orphan_prefixes =
                detect_storage_orphans(&storage_trie, &account_deletions.deleted_keys);
            for prefix in orphan_prefixes {
                all_storage_orphans.push(StorageOrphan { address: *address, prefix });
            }
        }

        storage_tries.insert(B256::from(keccak256(address)), storage_trie.into_inner());
    }

    let state_trie = resolve_nodes(&state_root_node, &state_nodes);

    // Check for state trie orphans from deleted accounts
    let state_orphan_prefixes = detect_state_orphans(&state_trie, deleted_accounts);
    let all_state_orphans: Vec<StateOrphan> =
        state_orphan_prefixes.into_iter().map(|prefix| StateOrphan { prefix }).collect();

    Ok(TransitionResult {
        state: EthereumState { state_trie: state_trie.into_inner(), storage_tries, bump },
        storage_orphans: all_storage_orphans,
        state_orphans: all_state_orphans,
    })
}

/// Original function for backwards compatibility (without orphan detection).
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

    let mut storage_tries = HashMap::default();
    let mut state_nodes = HashMap::default();
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
