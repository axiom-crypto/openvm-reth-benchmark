use alloy_primitives::B256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_mpt::{
    mpt::{MptNode, MptNodeData},
    mpt2::ArenaBasedMptNode,
    EthereumState, EthereumState2, StorageTries, StorageTries2,
};
use revm::primitives::HashMap;
use revm_primitives::{keccak256, map::DefaultHashBuilder};

fn create_synthetic_mpt_node(depth: usize, breadth: usize) -> MptNode {
    if depth == 0 {
        return MptNodeData::Leaf(
            format!("key_{}", rand::random::<u32>()).into_bytes(),
            format!("value_{}", rand::random::<u32>()).into_bytes(),
        )
        .into();
    }

    let mut children: [Option<Box<MptNode>>; 16] = Default::default();
    for i in 0..breadth.min(16) {
        children[i] = Some(Box::new(create_synthetic_mpt_node(depth - 1, breadth)));
    }
    MptNodeData::Branch(children).into()
}

fn create_synthetic_arena_mpt_node<'a>(depth: usize, breadth: usize) -> ArenaBasedMptNode<'a> {
    let mut trie = ArenaBasedMptNode::default();

    // Create some synthetic data to populate the arena-based trie
    // Use longer keys to avoid branch collisions that would create values in branches
    for i in 0..(breadth.pow(depth as u32).min(1000)) {
        let key = format!("long_key_prefix_{:08x}", i);
        let value = format!("value_{}", i);
        let _ = trie.insert(key.as_bytes(), value.into_bytes());
    }

    trie
}

// NEW: Create identical tries in both formats using the same key set
fn create_identical_tries<'a>(num_keys: usize) -> (MptNode, ArenaBasedMptNode<'a>) {
    let mut mpt_node = MptNode::default();
    let mut arena_node = ArenaBasedMptNode::default();

    // Generate deterministic keys to ensure identical trie structure
    let keys: Vec<_> = (0..num_keys)
        .map(|i| {
            let key = keccak256(format!("test_key_{:08x}", i).as_bytes());
            let value = format!("value_{}", i).into_bytes();
            (key.to_vec(), value)
        })
        .collect();

    // Insert same keys into both tries
    for (key, value) in &keys {
        mpt_node.insert(key, value.clone()).unwrap();
        arena_node.insert(key, value.clone()).unwrap();
    }

    (mpt_node, arena_node)
}

// NEW: RLP-specific benchmarks - this is what arena was designed for!
fn bench_rlp_codec_comparison(c: &mut Criterion) {
    let (mpt_node, arena_node) = create_identical_tries(100);

    // Generate RLP bytes for both
    let mpt_rlp = alloy_rlp::encode(&mpt_node);
    let arena_rlp = arena_node.to_rlp_bytes();

    println!("RLP sizes - MptNode: {} bytes, ArenaNode: {} bytes", mpt_rlp.len(), arena_rlp.len());

    // Benchmark RLP encoding (serialization)
    // c.bench_function("rlp_encode_mpt_node", |b| b.iter(|| black_box(alloy_rlp::encode(&mpt_node))));
    // c.bench_function("rlp_encode_arena_node", |b| b.iter(|| black_box(arena_node.to_rlp_bytes())));

    // Benchmark RLP decoding (deserialization) - the main target!
    c.bench_function("rlp_decode_mpt_node", |b| {
        b.iter(|| black_box(MptNode::decode(&mpt_rlp).unwrap()))
    });

    c.bench_function("rlp_decode_arena_node", |b| {
        b.iter(|| black_box(ArenaBasedMptNode::decode_from_rlp(&arena_rlp).unwrap()))
    });
}

// NEW: Compare lookup performance on identical tries
fn bench_lookup_performance(c: &mut Criterion) {
    let (mpt_node, arena_node) = create_identical_tries(1000);

    // Create test keys for lookup (some exist, some don't)
    let lookup_keys: Vec<_> =
        (0..100).map(|i| keccak256(format!("test_key_{:08x}", i).as_bytes()).to_vec()).collect();

    c.bench_function("lookup_mpt_node", |b| {
        b.iter(|| {
            for key in &lookup_keys {
                black_box(mpt_node.get(key).unwrap());
            }
        })
    });

    c.bench_function("lookup_arena_node", |b| {
        b.iter(|| {
            for key in &lookup_keys {
                black_box(arena_node.get(key).unwrap());
            }
        })
    });
}

// NEW: Compare hash computation on identical tries
fn bench_hash_computation(c: &mut Criterion) {
    let (mpt_node, arena_node) = create_identical_tries(500);

    // Verify they have the same hash first
    let mpt_hash = mpt_node.hash();
    let arena_hash = arena_node.hash();
    println!(
        "Hash verification - MptNode: {}, ArenaNode: {}, Equal: {}",
        mpt_hash,
        arena_hash,
        mpt_hash == arena_hash
    );

    c.bench_function("hash_mpt_node", |b| b.iter(|| black_box(mpt_node.hash())));

    c.bench_function("hash_arena_node", |b| b.iter(|| black_box(arena_node.hash())));
}

fn create_synthetic_ethereum_state(num_storage_tries: usize) -> EthereumState {
    // Use identical data generation for fair comparison
    let mut state_trie = MptNode::default();
    for i in 0..1000 {
        let key = format!("long_key_prefix_{:08x}", i);
        let value = format!("value_{}", i);
        let _ = state_trie.insert(key.as_bytes(), value.into_bytes());
    }

    let mut storage_tries = HashMap::with_hasher(DefaultHashBuilder::default());
    for i in 0..num_storage_tries {
        let mut storage_trie = MptNode::default();
        for j in 0..100 {
            let key = format!("storage_key_{:08x}", j);
            let value = format!("storage_value_{}", j);
            let _ = storage_trie.insert(key.as_bytes(), value.into_bytes());
        }
        storage_tries.insert(B256::from([i as u8; 32]), storage_trie);
    }

    EthereumState { state_trie, storage_tries: StorageTries(storage_tries) }
}

fn create_synthetic_ethereum_state2(num_storage_tries: usize) -> EthereumState2 {
    // Use identical data generation for fair comparison
    let mut state_trie = ArenaBasedMptNode::default();
    for i in 0..1000 {
        let key = format!("long_key_prefix_{:08x}", i);
        let value = format!("value_{}", i);
        let _ = state_trie.insert(key.as_bytes(), value.into_bytes());
    }

    let mut storage_tries = HashMap::with_hasher(DefaultHashBuilder::default());
    for i in 0..num_storage_tries {
        let mut storage_trie = ArenaBasedMptNode::default();
        for j in 0..100 {
            let key = format!("storage_key_{:08x}", j);
            let value = format!("storage_value_{}", j);
            let _ = storage_trie.insert(key.as_bytes(), value.into_bytes());
        }
        storage_tries.insert(B256::from([i as u8; 32]), storage_trie);
    }

    EthereumState2 { state_trie, storage_tries: StorageTries2(storage_tries) }
}

fn bench_ethereum_state_serde(c: &mut Criterion) {
    let ethereum_state = create_synthetic_ethereum_state(10);

    // Serialize once
    let serialized =
        bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard()).unwrap();

    // c.bench_function("ethereum_state_v1_serialize", |b| {
    //     b.iter(|| {
    //         black_box(
    //             bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard())
    //                 .unwrap(),
    //         )
    //     })
    // });

    c.bench_function("ethereum_state_v1_deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<EthereumState, _>(
                    &serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });
}

fn bench_ethereum_state2_serde(c: &mut Criterion) {
    let ethereum_state2 = create_synthetic_ethereum_state2(10);

    // Serialize once
    let serialized =
        bincode::serde::encode_to_vec(&ethereum_state2, bincode::config::standard()).unwrap();

    // c.bench_function("ethereum_state_v2_serialize", |b| {
    //     b.iter(|| {
    //         black_box(
    //             bincode::serde::encode_to_vec(&ethereum_state2, bincode::config::standard())
    //                 .unwrap(),
    //         )
    //     })
    // });

    c.bench_function("ethereum_state_v2_deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<EthereumState2, _>(
                    &serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });
}

fn bench_mpt_node_comparison(c: &mut Criterion) {
    let mpt_node = create_synthetic_mpt_node(3, 4);
    let arena_mpt_node = create_synthetic_arena_mpt_node(3, 4);

    // Test insertion performance with non-conflicting keys
    c.bench_function("mpt_node_v1_insert", |b| {
        b.iter(|| {
            let mut node = mpt_node.clone();
            for i in 0..50 {
                let key = format!("unique_insert_key_prefix_{:08x}", i);
                let value = format!("insert_value_{}", i);
                black_box(node.insert(key.as_bytes(), value.into_bytes()).unwrap());
            }
        })
    });

    c.bench_function("mpt_node_v2_insert", |b| {
        b.iter(|| {
            let mut node = arena_mpt_node.clone();
            for i in 0..50 {
                let key = format!("unique_insert_key_prefix_{:08x}", i);
                let value = format!("insert_value_{}", i);
                black_box(node.insert(key.as_bytes(), value.into_bytes()).unwrap());
            }
        })
    });

    // Test hash computation performance
    c.bench_function("mpt_node_v1_hash", |b| b.iter(|| black_box(mpt_node.hash())));

    c.bench_function("mpt_node_v2_hash", |b| b.iter(|| black_box(arena_mpt_node.hash())));
}

fn bench_size_comparison(_c: &mut Criterion) {
    let ethereum_state = create_synthetic_ethereum_state(20);
    let ethereum_state2 = create_synthetic_ethereum_state2(20);

    let v1_serialized =
        bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard()).unwrap();
    let v2_serialized =
        bincode::serde::encode_to_vec(&ethereum_state2, bincode::config::standard()).unwrap();

    println!("EthereumState (v1) serialized size: {} bytes", v1_serialized.len());
    println!("EthereumState2 (v2) serialized size: {} bytes", v2_serialized.len());
    println!("Size ratio (v2/v1): {:.2}", v2_serialized.len() as f64 / v1_serialized.len() as f64);

    // Also compare memory usage by counting nodes
    println!("EthereumState (v1) state trie size: {} nodes", ethereum_state.state_trie.size());
    println!("EthereumState2 (v2) is arena-based - single allocation");
}

fn bench_rlp_nodes_extraction(c: &mut Criterion) {
    let ethereum_state2 = create_synthetic_ethereum_state2(5);

    c.bench_function("ethereum_state_v2_extract_rlp_nodes", |b| {
        b.iter(|| black_box(ethereum_state2.all_rlp_nodes()))
    });
}

criterion_group!(
    benches,
    bench_rlp_codec_comparison, // NEW: The main test!
    bench_lookup_performance,   // NEW: Fair lookup comparison
    bench_hash_computation,     // NEW: Fair hash comparison
    bench_ethereum_state_serde,
    bench_ethereum_state2_serde,
    bench_mpt_node_comparison,
    bench_size_comparison,
    bench_rlp_nodes_extraction
);
criterion_main!(benches);
