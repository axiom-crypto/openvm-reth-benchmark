#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use bincode::config::standard;
use dhat::Profiler;
use openvm_client_executor::io::{ClientExecutorInput, ClientExecutorInputWithState};
use openvm_primitives::chain_spec::mainnet;
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives_traits::Block;
use reth_revm::db::CacheDB;
use std::{env, fs, sync::Arc};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_usage();
        return;
    }

    // Get operation from args
    let operation = if args.len() > 1 { args[1].as_str() } else { "all" };

    // Get block number from environment
    let block_number = env::var("BLOCK")
        .unwrap_or_else(|_| "23100006".to_string())
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Invalid BLOCK number"));

    let input_file = format!(
        "/Users/shayan/src/github.com/axiom-crypto/openvm-reth-benchmark/rpc-cache/input/1/{}.bin",
        block_number
    );

    println!("MPT Memory Profiler");
    println!("Operation: {}", operation);
    println!("Block: {}", block_number);
    println!("Input file: {}", input_file);
    println!();

    // Load the benchmark data file
    let buffer = fs::read(&input_file)
        .unwrap_or_else(|_| panic!("Failed to read benchmark data from '{}'. Run 'BLOCK={} cargo run --bin generate_benchmark_data' first to generate it.", input_file, block_number));

    println!("Loaded benchmark data: {} bytes", buffer.len());

    let bincode_config = standard();

    // Pre-compute the post-state once
    let (pre_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();
    let client_input = ClientExecutorInputWithState::build(pre_input.clone()).unwrap();
    let witness_db = client_input.witness_db().unwrap();
    let cache_db = CacheDB::new(&witness_db);
    let spec = Arc::new(mainnet());
    let current_block = client_input.input.current_block.clone().try_into_recovered().unwrap();
    let block_executor = BasicBlockExecutor::new(EthEvmConfig::new(spec), cache_db);
    let executor_output = block_executor.execute(&current_block).unwrap();
    let executor_outcome = ExecutionOutcome::new(
        executor_output.state,
        vec![executor_output.result.receipts],
        client_input.input.current_block.header.number,
        vec![executor_output.result.requests],
    );

    println!("Starting profiling...");

    match operation {
        "all" | "end-to-end" => {
            println!("Profiling: End-to-end workflow (without execution)");
            profile_end_to_end(&buffer, &executor_outcome);
        }
        "deserialize" => {
            println!("Profiling: Deserialization only");
            profile_deserialize(&buffer);
        }
        "witness" => {
            println!("Profiling: Witness DB creation only");
            profile_witness_db(pre_input);
        }
        "update" => {
            println!("Profiling: MPT update only");
            profile_update(client_input.state, &executor_outcome);
        }
        "state-root" => {
            println!("Profiling: Update and state root computation only");
            profile_state_root(client_input.state, &executor_outcome);
        }
        _ => {
            println!("Unknown operation: {}", operation);
            print_usage();
            return;
        }
    }

    println!("Profiling complete! Check the generated .dhat file.");
}

fn profile_end_to_end(buffer: &[u8], executor_outcome: &ExecutionOutcome) {
    let _profiler = Profiler::new_heap();
    let bincode_config = standard();

    // Deserialize
    let (pre_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(buffer, bincode_config).unwrap();

    let mut client_input = ClientExecutorInputWithState::build(pre_input).unwrap();

    // Create witness DB
    let _witness_db = client_input.witness_db().unwrap();

    // Update MPT with pre-computed post-state
    client_input.state.update_from_bundle_state(&executor_outcome.bundle).unwrap();
    let _state_root = client_input.state.state_trie.hash();
}

fn profile_deserialize(buffer: &[u8]) {
    let _profiler = Profiler::new_heap();
    let bincode_config = standard();

    let (_client_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(buffer, bincode_config).unwrap();
}

fn profile_witness_db(client_input: ClientExecutorInput) {
    let _profiler = Profiler::new_heap();

    let input = ClientExecutorInputWithState::build(client_input).unwrap();

    let _witness_db = input.witness_db().unwrap();
}

fn profile_update(mut parent_state: mptnew::EthereumState, executor_outcome: &ExecutionOutcome) {
    let _profiler = Profiler::new_heap();

    parent_state.update_from_bundle_state(&executor_outcome.bundle).unwrap();
}

fn profile_state_root(
    mut parent_state: mptnew::EthereumState,
    executor_outcome: &ExecutionOutcome,
) {
    let _profiler = Profiler::new_heap();

    parent_state.update_from_bundle_state(&executor_outcome.bundle).unwrap();
    let _state_root = parent_state.state_trie.hash();
}

fn print_usage() {
    println!("Usage: cargo run --bin mpt_profiler [operation]");
    println!("       BLOCK=18884864 cargo run --bin mpt_profiler update");
    println!();
    println!("Arguments:");
    println!("  operation    Operation to profile (default: all)");
    println!();
    println!("Environment:");
    println!("  BLOCK        Block number for data file (default: 23100006)");
    println!();
    println!("Operations:");
    println!("  all          Complete workflow (default)");
    println!("  deserialize  Deserialization only");
    println!("  witness      Witness DB creation only");
    println!("  update       MPT update only");
    println!("  state-root   State root computation only");
}
