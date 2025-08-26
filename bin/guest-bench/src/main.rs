use std::fs;

use bincode::config::standard;
use openvm_client_executor::{io::NewClientExecutorInput, ClientExecutor};

fn main() {
    let block_number = 23100006;
    let input_file = format!(
        "/Users/shayan/src/github.com/axiom-crypto/openvm-reth-benchmark/rpc-cache/input/1/{}.bin",
        block_number
    );
    let bincode_config = standard();

    let buffer = fs::read(&input_file)
        .unwrap_or_else(|_| panic!("Failed to read benchmark data from '{}'. Run 'BLOCK={} cargo run --bin generate_benchmark_data' first to generate it.", input_file, block_number));
    let (pre_input, _): (NewClientExecutorInput, _) =
        bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();

    let executor = ClientExecutor;
    let header = executor.execute(pre_input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    println!("{block_hash}");
}
