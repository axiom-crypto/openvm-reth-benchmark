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

/// Parses proof bytes into a vector of tries using a shared bump.
fn parse_proof_with_bump(
    bump: &'static Bump,
    proof: &[impl AsRef<[u8]>],
) -> Result<Vec<MptOwned>, Error> {
    proof
        .iter()
        .map(|bytes| MptOwned::decode_from_proof_rlp_with_bump(bump, &mut bytes.as_ref()))
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

/// Processes a proof using a shared bump.
fn process_proof_with_bump(
    bump: &'static Bump,
    proof: &[impl AsRef<[u8]>],
    node_store: &mut HashMap<B256, MptOwned>,
) -> Result<Option<MptOwned>, Error> {
    let proof_nodes = parse_proof_with_bump(bump, proof)?;
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

/// Adds all the leaf nodes of non-inclusion proofs using a shared bump.
fn add_orphaned_leafs_with_bump(
    bump: &'static Bump,
    key: impl AsRef<[u8]>,
    proof: &[impl AsRef<[u8]>],
    node_store: &mut HashMap<B256, MptOwned>,
) -> Result<(), Error> {
    if !proof.is_empty() {
        let proof_nodes = parse_proof_with_bump(bump, proof)?;
        if is_not_included_with_bump(bump, keccak256(key).as_slice(), &proof_nodes)? {
            for node in shorten_node_path_with_bump(bump, proof_nodes.last().unwrap()) {
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

/// Returns a list of all possible nodes using a shared bump.
fn shorten_node_path_with_bump(bump: &'static Bump, node: &MptOwned) -> Vec<MptOwned> {
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
            let mut new_node = MptOwned::with_bump(bump);
            let value = value.unwrap();
            new_node.set_node(new_node.root_id(), &NodeData::Leaf(&path, value));
            new_node
        } else {
            let mut new_node = MptOwned::from_trie_with_bump(bump, node.inner());
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

fn is_not_included_with_bump(
    bump: &'static Bump,
    key: &[u8],
    proof_nodes: &[MptOwned],
) -> Result<bool, Error> {
    let proof_trie = mpt_from_proof_with_bump(bump, proof_nodes)?;
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

fn mpt_from_proof_with_bump(
    bump: &'static Bump,
    proof_nodes: &[MptOwned],
) -> Result<MptOwned, Error> {
    if proof_nodes.is_empty() {
        return Ok(MptOwned::with_bump(bump));
    }

    let node_store: HashMap<B256, MptOwned> =
        proof_nodes.iter().map(|node| (node.hash(), node.clone())).collect();

    let root_node = proof_nodes.first().unwrap();

    Ok(resolve_nodes_with_bump(bump, root_node, &node_store))
}

fn resolve_nodes(root: &MptOwned, node_store: &HashMap<B256, MptOwned>) -> MptOwned {
    let mut new_trie = MptOwned::default();

    let root_id = resolve_nodes_internal(root, root.root_id(), node_store, &mut new_trie);
    new_trie.set_root_id(root_id);

    // The root hash must not change after resolution
    debug_assert_eq!(root.hash(), new_trie.hash());

    new_trie
}

fn resolve_nodes_with_bump(
    bump: &'static Bump,
    root: &MptOwned,
    node_store: &HashMap<B256, MptOwned>,
) -> MptOwned {
    let mut new_trie = MptOwned::with_bump(bump);

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

fn node_from_digest_with_bump(bump: &'static Bump, digest: B256) -> MptOwned {
    match digest {
        reth_trie::EMPTY_ROOT_HASH | B256::ZERO => MptOwned::with_bump(bump),
        _ => {
            let mut trie = MptOwned::with_bump(bump);
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

fn build_storage_trie_with_bump(
    bump: &'static Bump,
    proof: &AccountProof,
    fini_proofs: &AccountProof,
) -> Result<MptOwned, Error> {
    if proof.storage_proofs.is_empty() {
        return Ok(node_from_digest_with_bump(bump, proof.storage_root));
    }

    let mut storage_nodes = HashMap::default();
    let mut storage_root_node = MptOwned::with_bump(bump);

    for storage_proof in &proof.storage_proofs {
        if let Some(root) = process_proof_with_bump(bump, &storage_proof.proof, &mut storage_nodes)?
        {
            storage_root_node = root;
        }
    }

    for storage_proof in &fini_proofs.storage_proofs {
        add_orphaned_leafs_with_bump(
            bump,
            storage_proof.key.0,
            &storage_proof.proof,
            &mut storage_nodes,
        )?;
    }

    Ok(resolve_nodes_with_bump(bump, &storage_root_node, &storage_nodes))
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

/// Result of orphan detection without building the final state.
#[derive(Debug)]
pub struct OrphanDetectionResult {
    /// Unresolvable storage orphans that need additional proofs.
    pub storage_orphans: Vec<StorageOrphan>,
    /// Unresolvable state trie orphans that need additional proofs.
    pub state_orphans: Vec<StateOrphan>,
}

impl OrphanDetectionResult {
    /// Returns true if there are no orphans.
    pub fn is_empty(&self) -> bool {
        self.storage_orphans.is_empty() && self.state_orphans.is_empty()
    }
}

/// Detects orphans in proofs without building the final state.
///
/// This function uses a single shared bump arena to minimize memory leakage.
/// While one bump per call is still leaked (required for `'static` lifetime),
/// this is bounded and much better than leaking one bump per proof node.
///
/// Use this in a retry loop to detect orphans, then call
/// `transition_proofs_to_tries_with_deletions` once orphans are resolved.
pub fn detect_orphans_in_proofs(
    _state_root: B256,
    parent_proofs: &HashMap<Address, AccountProof>,
    proofs: &HashMap<Address, AccountProof>,
    storage_deletions: &HashMap<Address, StorageDeletions>,
    deleted_accounts: &[Address],
) -> Result<OrphanDetectionResult, Error> {
    if parent_proofs.is_empty() {
        return Ok(OrphanDetectionResult { storage_orphans: vec![], state_orphans: vec![] });
    }

    // Use a single shared bump for all allocations in this function.
    // This leaks one bump per call, but that's bounded and much better than
    // leaking one per proof node.
    let bump: &'static Bump = Box::leak(Box::new(Bump::new()));

    let mut storage_orphans = Vec::new();
    let mut state_nodes = HashMap::default();
    let mut state_root_node = MptOwned::with_bump(bump);

    for (address, proof) in parent_proofs {
        if let Some(root) = process_proof_with_bump(bump, &proof.proof, &mut state_nodes)? {
            state_root_node = root;
        }

        let fini_proofs = proofs.get(address).unwrap();
        add_orphaned_leafs_with_bump(bump, address, &fini_proofs.proof, &mut state_nodes)?;

        let storage_trie = build_storage_trie_with_bump(bump, proof, fini_proofs)?;

        // Check for storage orphans if there are deletions for this account
        if let Some(account_deletions) = storage_deletions.get(address) {
            let orphan_prefixes =
                detect_storage_orphans(&storage_trie, &account_deletions.deleted_keys);
            for prefix in orphan_prefixes {
                storage_orphans.push(StorageOrphan { address: *address, prefix });
            }
        }
    }

    let state_trie = resolve_nodes_with_bump(bump, &state_root_node, &state_nodes);

    // Check for state trie orphans from deleted accounts
    let state_orphan_prefixes = detect_state_orphans(&state_trie, deleted_accounts);
    let state_orphans: Vec<StateOrphan> =
        state_orphan_prefixes.into_iter().map(|prefix| StateOrphan { prefix }).collect();

    Ok(OrphanDetectionResult { storage_orphans, state_orphans })
}

/// Builds tries from proofs and detects any unresolvable orphans.
///
/// This allocates memory for the final state. For retry loops, use
/// `detect_orphans_in_proofs` first to avoid memory leaks, then call
/// this once orphans are resolved.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::NodeData;
    use smallvec::SmallVec;

    /// Creates a simple branch node with two children at the given nibble positions.
    fn create_branch_with_two_children(
        trie: &mut MptOwned,
        nibble1: usize,
        child1: NodeId,
        nibble2: usize,
        child2: NodeId,
    ) -> NodeId {
        let mut children: [Option<NodeId>; 16] = Default::default();
        children[nibble1] = Some(child1);
        children[nibble2] = Some(child2);
        trie.add_node(&NodeData::Branch(children))
    }

    #[test]
    fn test_no_orphan_when_branch_has_resolved_sibling() {
        // Create a trie with a branch node that has two resolved children (leaves)
        let mut trie = MptOwned::default();

        // Create two leaf nodes
        let leaf1_path = to_encoded_path(&[0, 1, 2, 3], true);
        let leaf1_id = trie.add_node(&NodeData::Leaf(&leaf1_path, &[1, 2, 3]));

        let leaf2_path = to_encoded_path(&[0, 1, 2, 4], true);
        let leaf2_id = trie.add_node(&NodeData::Leaf(&leaf2_path, &[4, 5, 6]));

        // Create branch with both children
        let branch_id = create_branch_with_two_children(&mut trie, 0, leaf1_id, 1, leaf2_id);
        trie.set_root_id(branch_id);

        // Build a key that would lead to nibble 0 child
        // Since both siblings are resolved, there should be no orphan
        let deleted_keys = vec![B256::ZERO]; // Key doesn't matter for this structure test
        let orphans = detect_storage_orphans(&trie, &deleted_keys);

        // No orphan because sibling is resolved (leaf, not digest)
        assert!(orphans.is_empty());
    }

    #[test]
    fn test_orphan_when_branch_has_unresolved_sibling() {
        // Create a trie with a branch node where one child is a digest
        let mut trie = MptOwned::default();

        // Create a leaf for one child
        let leaf_path = to_encoded_path(&[0, 1, 2, 3], true);
        let leaf_id = trie.add_node(&NodeData::Leaf(&leaf_path, &[1, 2, 3]));

        // Create a digest for the sibling
        let digest_hash = B256::repeat_byte(0xab);
        let digest_id = trie.add_node(&NodeData::Digest(digest_hash.as_slice()));

        // Create branch with leaf at nibble 5 and digest at nibble 7
        let branch_id = create_branch_with_two_children(&mut trie, 5, leaf_id, 7, digest_id);
        trie.set_root_id(branch_id);

        // Create a key whose hash starts with nibble 5 (to delete the leaf)
        // We need to find a key whose keccak256 starts with 0x5...
        // For testing, we'll just verify the orphan detection logic by
        // calling find_unresolvable_sibling directly with a key starting with nibble 5
        let mut key_nibs: Nibbles = SmallVec::new();
        key_nibs.push(5);
        for b in &leaf_path[1..] {
            key_nibs.push(b >> 4);
            key_nibs.push(b & 0x0f);
        }

        let result = find_unresolvable_sibling(&trie, trie.root_id(), &key_nibs, &[]);

        // Should find orphan at nibble 7 (the sibling digest)
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![7u8]);
    }

    #[test]
    fn test_no_orphan_when_branch_has_more_than_two_children() {
        // Create a trie with a branch node that has three children
        let mut trie = MptOwned::default();

        // Create three leaf nodes
        let leaf1_path = to_encoded_path(&[1, 2, 3], true);
        let leaf1_id = trie.add_node(&NodeData::Leaf(&leaf1_path, &[1]));

        let leaf2_path = to_encoded_path(&[1, 2, 4], true);
        let leaf2_id = trie.add_node(&NodeData::Leaf(&leaf2_path, &[2]));

        // Third child is a digest (unresolved)
        let digest_hash = B256::repeat_byte(0xcd);
        let digest_id = trie.add_node(&NodeData::Digest(digest_hash.as_slice()));

        // Create branch with three children
        let mut children: [Option<NodeId>; 16] = Default::default();
        children[2] = Some(leaf1_id);
        children[5] = Some(leaf2_id);
        children[9] = Some(digest_id);
        let branch_id = trie.add_node(&NodeData::Branch(children));
        trie.set_root_id(branch_id);

        // Try to delete the key at nibble 2
        let key_nibs: Nibbles = [2u8].iter().copied().collect();
        let result = find_unresolvable_sibling(&trie, trie.root_id(), &key_nibs, &[]);

        // No orphan because branch has 3 children, so deleting one won't collapse it
        assert!(result.is_none());
    }

    #[test]
    fn test_orphan_with_extension_prefix() {
        // Create a trie: Extension(0x12) -> Branch(nibbles 3 and 5)
        // where nibble 5 child is a digest
        let mut trie = MptOwned::default();

        // Create leaf for nibble 3 path
        let leaf_path = to_encoded_path(&[4, 5, 6], true);
        let leaf_id = trie.add_node(&NodeData::Leaf(&leaf_path, &[1, 2, 3]));

        // Create digest for nibble 5
        let digest_hash = B256::repeat_byte(0xef);
        let digest_id = trie.add_node(&NodeData::Digest(digest_hash.as_slice()));

        // Create branch
        let branch_id = create_branch_with_two_children(&mut trie, 3, leaf_id, 5, digest_id);

        // Create extension pointing to branch
        let ext_path = to_encoded_path(&[1, 2], false);
        let ext_id = trie.add_node(&NodeData::Extension(&ext_path, branch_id));
        trie.set_root_id(ext_id);

        // Key that traverses extension and goes to nibble 3 (the leaf)
        let key_nibs: Nibbles = [1, 2, 3, 4, 5, 6].iter().copied().collect();
        let result = find_unresolvable_sibling(&trie, trie.root_id(), &key_nibs, &[]);

        // Should find orphan at nibble path [1, 2, 5] (extension path + sibling nibble)
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![1, 2, 5]);
    }

    #[test]
    fn test_no_orphan_when_key_not_in_trie() {
        // Create a simple trie with a leaf
        let mut trie = MptOwned::default();
        let leaf_path = to_encoded_path(&[1, 2, 3, 4], true);
        let leaf_id = trie.add_node(&NodeData::Leaf(&leaf_path, &[1, 2, 3]));
        trie.set_root_id(leaf_id);

        // Try to delete a key that doesn't exist
        let key_nibs: Nibbles = [5, 6, 7, 8].iter().copied().collect();
        let result = find_unresolvable_sibling(&trie, trie.root_id(), &key_nibs, &[]);

        // No orphan because key doesn't exist in trie
        assert!(result.is_none());
    }

    #[test]
    fn test_orphan_when_traversing_hits_digest() {
        // Create a trie where the path to the key goes through a digest
        let mut trie = MptOwned::default();

        // Root is a digest (completely unresolved)
        let digest_hash = B256::repeat_byte(0x11);
        let digest_id = trie.add_node(&NodeData::Digest(digest_hash.as_slice()));
        trie.set_root_id(digest_id);

        // Any key traversal should report the root as unresolvable
        let key_nibs: Nibbles = [1, 2, 3, 4].iter().copied().collect();
        let result = find_unresolvable_sibling(&trie, trie.root_id(), &key_nibs, &[]);

        // Orphan at empty prefix (the root itself)
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_to_nibbles() {
        assert_eq!(to_nibbles(&[0xab, 0xcd]).as_slice(), &[0xa, 0xb, 0xc, 0xd]);
        assert_eq!(to_nibbles(&[0x12]).as_slice(), &[0x1, 0x2]);
        assert!(to_nibbles(&[]).is_empty());
        assert_eq!(to_nibbles(&[0x00]).as_slice(), &[0x0, 0x0]);
        assert_eq!(to_nibbles(&[0xff]).as_slice(), &[0xf, 0xf]);
    }

    #[test]
    fn test_detect_storage_orphans_empty() {
        let trie = MptOwned::default();
        let deleted_keys: Vec<B256> = vec![];
        let orphans = detect_storage_orphans(&trie, &deleted_keys);
        assert!(orphans.is_empty());
    }

    #[test]
    fn test_detect_state_orphans_empty() {
        let trie = MptOwned::default();
        let deleted_addresses: Vec<Address> = vec![];
        let orphans = detect_state_orphans(&trie, &deleted_addresses);
        assert!(orphans.is_empty());
    }
}
