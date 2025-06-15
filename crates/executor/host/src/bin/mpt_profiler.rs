#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use bincode::config::standard;
use dhat::Profiler;
use openvm_client_executor::io::ClientExecutorInput;
use openvm_primitives::chain_spec::mainnet;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives_traits::Block;
use reth_revm::db::CacheDB;
use std::{env, fs, sync::Arc};

fn main() {
    let args: Vec<String> = env::args().collect();

    let operation = if args.len() > 1 { args[1].as_str() } else { "all" };

    let iterations = if args.len() > 2 { args[2].parse::<usize>().unwrap_or(1) } else { 1 };

    println!("MPT Memory Profiler");
    println!("Operation: {}", operation);
    println!("Iterations: {}", iterations);
    println!();

    // Load the benchmark data file
    let buffer = fs::read("client_input_benchmark.bin")
        .expect("Failed to read benchmark data. Run the integration test first to generate it.");

    println!("Loaded benchmark data: {} bytes", buffer.len());

    let bincode_config = standard();

    // Pre-compute the post-state once
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

    println!("Starting profiling...");

    match operation {
        "all" | "end-to-end" => {
            println!("Profiling: End-to-end workflow (without execution)");
            profile_end_to_end(&buffer, &executor_outcome, iterations);
        }
        "deserialize" => {
            println!("Profiling: Deserialization only");
            profile_deserialize(&buffer, iterations);
        }
        "witness" => {
            println!("Profiling: Witness DB creation only");
            profile_witness_db(&client_input, iterations);
        }
        "update" => {
            println!("Profiling: MPT update only");
            profile_update(&client_input, &executor_outcome, iterations);
        }
        "state-root" => {
            println!("Profiling: State root computation only");
            profile_state_root(&client_input, &executor_outcome, iterations);
        }
        _ => {
            println!("Unknown operation: {}", operation);
            print_usage();
            return;
        }
    }

    println!("Profiling complete! Check the generated .dhat file.");
}

fn profile_end_to_end(buffer: &[u8], executor_outcome: &ExecutionOutcome, iterations: usize) {
    let _profiler = Profiler::new_heap();
    let bincode_config = standard();

    for i in 0..iterations {
        if iterations > 1 {
            println!("  Iteration {}/{}", i + 1, iterations);
        }

        // Deserialize
        let (mut client_input, _): (ClientExecutorInput, _) =
            bincode::serde::decode_from_slice(buffer, bincode_config).unwrap();

        // Create witness DB
        let _witness_db = client_input.witness_db().unwrap();

        // Update MPT with pre-computed post-state
        client_input.parent_state.update_from_bundle_state(&executor_outcome.bundle);
        let _state_root = client_input.parent_state.state_root();
    }
}

fn profile_deserialize(buffer: &[u8], iterations: usize) {
    let _profiler = Profiler::new_heap();
    let bincode_config = standard();

    for i in 0..iterations {
        if iterations > 1 {
            println!("  Iteration {}/{}", i + 1, iterations);
        }

        let (_client_input, _): (ClientExecutorInput, _) =
            bincode::serde::decode_from_slice(buffer, bincode_config).unwrap();
    }
}

fn profile_witness_db(client_input: &ClientExecutorInput, iterations: usize) {
    let _profiler = Profiler::new_heap();

    for i in 0..iterations {
        if iterations > 1 {
            println!("  Iteration {}/{}", i + 1, iterations);
        }

        let _witness_db = client_input.witness_db().unwrap();
    }
}

fn profile_update(
    client_input: &ClientExecutorInput,
    executor_outcome: &ExecutionOutcome,
    iterations: usize,
) {
    let _profiler = Profiler::new_heap();

    for i in 0..iterations {
        if iterations > 1 {
            println!("  Iteration {}/{}", i + 1, iterations);
        }

        let mut parent_state = client_input.parent_state.clone();
        parent_state.update_from_bundle_state(&executor_outcome.bundle);
    }
}

fn profile_state_root(
    client_input: &ClientExecutorInput,
    executor_outcome: &ExecutionOutcome,
    iterations: usize,
) {
    let _profiler = Profiler::new_heap();

    for i in 0..iterations {
        if iterations > 1 {
            println!("  Iteration {}/{}", i + 1, iterations);
        }

        let mut parent_state = client_input.parent_state.clone();
        parent_state.update_from_bundle_state(&executor_outcome.bundle);
        let _state_root = parent_state.state_root();
    }
}

fn print_usage() {
    println!("Usage: mpt_profiler [operation] [iterations]");
    println!();
    println!("Operations:");
    println!("  all, end-to-end  - Profile the complete workflow (default)");
    println!("  deserialize      - Profile only deserialization");
    println!("  witness          - Profile only witness DB creation");
    println!("  update           - Profile only MPT update");
    println!("  state-root       - Profile only state root computation");
    println!();
    println!("Examples:");
    println!("  mpt_profiler");
    println!("  mpt_profiler update 5");
    println!("  mpt_profiler state-root 10");
}
