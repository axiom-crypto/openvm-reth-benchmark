use bincode::config::standard;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::state::HashedPostState;
use openvm_primitives::chain_spec::mainnet;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives_traits::Block;
use reth_revm::db::CacheDB;
use std::{fs, sync::Arc};

fn benchmark_mpt_operations(c: &mut Criterion) {
    // Load the benchmark data file (this is not counted in benchmark timing)
    let buffer = fs::read("client_input_benchmark.bin")
        .expect("Failed to read benchmark data. Run the integration test first to generate it.");

    println!("Loaded benchmark data: {} bytes", buffer.len());

    let bincode_config = standard();

    // Pre-compute the post-state once for the MPT benchmarks (not timed)
    let (client_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();
    let witness_db = client_input.witness_db().unwrap();
    let cache_db = CacheDB::new(&witness_db);
    let spec = Arc::new(mainnet());
    let current_block = client_input.current_block.clone().try_into_recovered().unwrap();
    let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec), cache_db);
    let executor_output = block_executor.execute(&current_block).unwrap();
    let executor_outcome = ExecutionOutcome::new(
        executor_output.state,
        vec![executor_output.result.receipts],
        client_input.current_block.header.number,
        vec![executor_output.result.requests],
    );

    // Benchmark the realistic end-to-end workflow (deserialize -> witness_db -> mpt_update)
    // This excludes block execution since that's not what you want to measure
    c.bench_function("end_to_end_without_execution", |b| {
        b.iter(|| {
            // Deserialize (this happens in production)
            let (mut client_input, _): (ClientExecutorInput, _) =
                bincode::serde::decode_from_slice(black_box(&buffer), bincode_config).unwrap();

            // Create witness DB (this happens in production)
            let _witness_db = client_input.witness_db().unwrap();

            // Update MPT with pre-computed post-state (this happens in production)
            // Note: In production, the post-state comes from block execution, but we're
            // using pre-computed data to exclude execution time from the benchmark
            client_input.parent_state.update_from_bundle_state(&executor_outcome.bundle);
            let state_root = client_input.parent_state.state_root();
            black_box(state_root)
        })
    });
}

criterion_group!(benches, benchmark_mpt_operations);
criterion_main!(benches);
