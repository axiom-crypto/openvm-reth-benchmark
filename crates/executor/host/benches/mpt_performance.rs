use bincode::config::standard;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::state::HashedPostState;
use reth_chainspec::MAINNET;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives_traits::Block;
use reth_revm::db::CacheDB;
use std::fs;

fn benchmark_mpt_operations(c: &mut Criterion) {
    // Load the benchmark data file (this is not counted in benchmark timing)
    let buffer = fs::read("client_input_benchmark.bin")
        .expect("Failed to read benchmark data. Run the integration test first to generate it.");

    println!("Loaded benchmark data: {} bytes", buffer.len());

    let bincode_config = standard();

    // Deserialize once for the MPT benchmarks
    let (client_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();

    c.bench_function("deserialize_and_witness_db", |b| {
        b.iter(|| {
            let (client_input, _): (ClientExecutorInput, _) =
                bincode::serde::decode_from_slice(black_box(&buffer), bincode_config).unwrap();
            let witness_db = client_input.witness_db().unwrap();
            black_box(witness_db)
        })
    });

    // Execute the block once to get the real post-state (not timed)
    let witness_db = client_input.witness_db().unwrap();
    let cache_db = CacheDB::new(&witness_db);
    let spec = MAINNET.clone();
    let current_block = client_input.current_block.clone().try_into_recovered().unwrap();
    let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec), cache_db);
    let executor_output = block_executor.execute(&current_block).unwrap();
    let executor_outcome = ExecutionOutcome::new(
        executor_output.state,
        vec![executor_output.result.receipts],
        client_input.current_block.header.number,
        vec![executor_output.result.requests],
    );
    let real_post_state = HashedPostState::from_bundle_state(&executor_outcome.bundle.state);

    // Benchmark the combined operation (what actually happens in client)
    c.bench_function("mpt_update_and_hash", |b| {
        b.iter(|| {
            let mut parent_state = client_input.parent_state.clone();
            parent_state.update(&real_post_state);
            let state_root = parent_state.state_root();
            black_box(state_root)
        })
    });
}

criterion_group!(benches, benchmark_mpt_operations);
criterion_main!(benches);
