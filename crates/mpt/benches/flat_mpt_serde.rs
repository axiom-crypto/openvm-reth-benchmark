use alloy_primitives::B256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_mpt::{
    flat::{FlatNode, FlatTrieOwned},
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

fn create_synthetic_flat_trie(num_nodes: usize) -> FlatTrieOwned {
    let mut flat_trie = FlatTrieOwned::default();

    // Create synthetic nodes
    for i in 0..num_nodes {
        let node = FlatNode {
            kind: (i % 4 + 1) as u8, // Cycle through Branch, Leaf, Extension, Digest
            data: (i % 65536) as u16,
            rlp_offset: (i * 50) as u32, // Synthetic RLP offset
            rlp_len: 50,
            child_idx: if i > 0 { (i - 1) as u32 } else { u32::MAX },
        };
        flat_trie.index.push(node);
    }

    // Create synthetic blob data
    flat_trie.blob = vec![0u8; num_nodes * 50];

    // Create synthetic branch children
    flat_trie.branch_children = (0..num_nodes as u32).collect();

    flat_trie
}

fn bench_mpt_node_serde(c: &mut Criterion) {
    let ethereum_state = create_synthetic_ethereum_state(10);

    // Serialize once
    let serialized =
        bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard()).unwrap();

    c.bench_function("mpt_node_serialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard())
                    .unwrap(),
            )
        })
    });

    c.bench_function("mpt_node_deserialize", |b| {
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

fn bench_flat_trie_serde_current(c: &mut Criterion) {
    let flat_trie = create_synthetic_flat_trie(10000); // 10k nodes

    // Serialize once
    let serialized =
        bincode::serde::encode_to_vec(&flat_trie, bincode::config::standard()).unwrap();

    c.bench_function("flat_trie_current_serialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::encode_to_vec(&flat_trie, bincode::config::standard()).unwrap(),
            )
        })
    });

    c.bench_function("flat_trie_current_deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<FlatTrieOwned, _>(
                    &serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });
}

fn bench_flat_ethereum_state_serde(c: &mut Criterion) {
    // Create a realistic-sized flat state
    let ethereum_state = create_synthetic_ethereum_state(50);
    let flat_state = ethereum_state.to_flat();

    // Serialize once
    let serialized =
        bincode::serde::encode_to_vec(&flat_state, bincode::config::standard()).unwrap();

    c.bench_function("flat_ethereum_state_serialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::encode_to_vec(&flat_state, bincode::config::standard()).unwrap(),
            )
        })
    });

    c.bench_function("flat_ethereum_state_deserialize", |b| {
        b.iter(|| {
            black_box(
                bincode::serde::decode_from_slice::<FlatEthereumState, _>(
                    &serialized,
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            )
        })
    });
}

fn bench_size_comparison(c: &mut Criterion) {
    let ethereum_state = create_synthetic_ethereum_state(20);
    let flat_state = ethereum_state.to_flat();

    let mpt_serialized =
        bincode::serde::encode_to_vec(&ethereum_state, bincode::config::standard()).unwrap();
    let flat_serialized =
        bincode::serde::encode_to_vec(&flat_state, bincode::config::standard()).unwrap();

    println!("MPT serialized size: {} bytes", mpt_serialized.len());
    println!("Flat serialized size: {} bytes", flat_serialized.len());
    println!(
        "Size ratio (flat/mpt): {:.2}",
        flat_serialized.len() as f64 / mpt_serialized.len() as f64
    );
}

criterion_group!(
    benches,
    bench_mpt_node_serde,
    bench_flat_trie_serde_current,
    bench_flat_ethereum_state_serde,
    bench_size_comparison
);
criterion_main!(benches);
