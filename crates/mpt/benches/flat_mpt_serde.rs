use alloy_primitives::B256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_mpt::{
    mpt::{MptNode, MptNodeData},
    EthereumState, FlatEthereumState, StorageTries,
};
use revm::primitives::HashMap;
use revm_primitives::map::DefaultHashBuilder;

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

fn create_synthetic_ethereum_state(num_storage_tries: usize) -> EthereumState {
    let state_trie = create_synthetic_mpt_node(4, 8); // Depth 4, breadth 8

    let mut storage_tries = HashMap::with_hasher(DefaultHashBuilder::default());
    for i in 0..num_storage_tries {
        let storage_trie = create_synthetic_mpt_node(3, 6); // Smaller storage tries
        storage_tries.insert(B256::from([i as u8; 32]), storage_trie);
    }

    EthereumState { state_trie, storage_tries: StorageTries(storage_tries) }
}

fn bench_ethereum_state_comparison(c: &mut Criterion) {
    // Create a realistic-sized Ethereum state
    let ethereum_state = create_synthetic_ethereum_state(20);
    let flat_state = ethereum_state.to_flat();

    // Serialize with different methods
    let mpt_serialized =
        bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard()).unwrap();
    let flat_serialized =
        bincode::serde::encode_to_vec(&flat_state, bincode::config::standard()).unwrap();
    let rkyv_serialized = flat_state.to_rkyv_bytes().unwrap();

    // Print size comparison
    println!("\n=== Size Comparison ===");
    println!("Original MPT serialized size: {} bytes", mpt_serialized.len());
    println!("Flat MPT (serde) serialized size: {} bytes", flat_serialized.len());
    println!("Flat MPT (rkyv) serialized size: {} bytes", rkyv_serialized.len());
    println!(
        "Size ratio (flat-serde/original): {:.2}x",
        flat_serialized.len() as f64 / mpt_serialized.len() as f64
    );
    println!(
        "Size ratio (rkyv/original): {:.2}x",
        rkyv_serialized.len() as f64 / mpt_serialized.len() as f64
    );

    // Benchmark serialization
    let mut group = c.benchmark_group("ethereum_state_serialize");

    group.bench_function("original_mpt", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard())
                    .unwrap(),
            )
        })
    });

    group.bench_function("flat_mpt_serde", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::encode_to_vec(&flat_state, bincode::config::standard()).unwrap(),
            )
        })
    });

    group.bench_function("flat_mpt_rkyv", |b| {
        b.iter(|| black_box(flat_state.to_rkyv_bytes().unwrap()))
    });

    group.finish();

    // Benchmark deserialization
    let mut group = c.benchmark_group("ethereum_state_deserialize");

    group.bench_function("original_mpt", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<EthereumState, _>(
                    &mpt_serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });

    group.bench_function("flat_mpt_serde", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<FlatEthereumState, _>(
                    &flat_serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });

    group.bench_function("flat_mpt_rkyv", |b| {
        b.iter(|| {
            // Test zero-copy access - this is the main advantage of rkyv
            // Just measure the cost of accessing the archived data
            match FlatEthereumState::access_rkyv_bytes(&rkyv_serialized) {
                Ok(archived) => {
                    // Zero-copy success - this is what we want to measure
                    black_box(archived);
                    true
                }
                Err(_) => {
                    // Fall back to owned access if needed
                    let _ = black_box(FlatEthereumState::from_rkyv_bytes_owned(&rkyv_serialized));
                    false
                }
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_ethereum_state_comparison);
criterion_main!(benches);
