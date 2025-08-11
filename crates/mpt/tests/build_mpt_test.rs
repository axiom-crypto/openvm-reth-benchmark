#![cfg(feature = "build_mpt")]

use openvm_mpt::mpt::MptTrie;

#[test]
fn test_mpt_from_proof_reconstruction() {
    use openvm_mpt::mpt::build_mpt::mpt_from_proof;

    // Create a test proof scenario
    // This mimics how proofs work: we have a sequence of nodes where later nodes
    // reference earlier nodes by digest

    // Use the build_mpt helpers instead of touching private fields/methods.
    // We'll construct a tiny proof by serializing nodes via to_full_rlp and decoding them back.
    use openvm_mpt::mpt::NodeData;

    // Build a leaf trie and serialize it
    let mut leaf_trie = MptTrie::default();
    // Insert a key so that we get a leaf with compact path [0x03]
    // Key byte 0x03 => nibbles [0x00, 0x03], but for simplicity just use insert API
    leaf_trie.insert(b"\x03", b"test_value").unwrap();
    let leaf_rlp = leaf_trie.to_full_rlp();

    // Build an extension trie that references the leaf by digest
    let mut ext_trie = MptTrie::default();
    ext_trie.insert(b"\x01\x03", b"dummy").unwrap(); // ensure we have an extension-like structure
                                                     // Replace the child with a digest of the leaf
    let leaf_digest = leaf_trie.hash();
    // Serialize ext and then decode a minimal proof list [ext, leaf]
    let ext_rlp = ext_trie.to_full_rlp();

    // Create the proof nodes (in the order they would appear in a real proof)
    let proof_nodes = vec![
        MptTrie::decode_from_rlp(&ext_rlp, 0).unwrap(),
        MptTrie::decode_from_rlp(&leaf_rlp, 0).unwrap(),
    ];

    // Reconstruct the trie from the proof
    let reconstructed = mpt_from_proof(&proof_nodes).unwrap();

    // The reconstructed trie should be able to retrieve the value
    // Key would be [0x01] + [0x03] = nibbles [0x01, 0x03] = key bytes [0x13]
    let retrieved = reconstructed.get(b"\x13").unwrap();
    assert_eq!(retrieved, Some(&b"test_value"[..]));

    // The hash should be non-empty
    assert_ne!(reconstructed.hash(), openvm_mpt::mpt::EMPTY_ROOT);
}
