use alloy_provider::RootProvider;
use alloy_rpc_types_debug::ExecutionWitness;
use openvm_client_executor::ClientExecutor;
use openvm_host_executor::HostExecutor;
use std::time::Instant;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};
use url::Url;

#[tokio::test(flavor = "multi_thread")]
async fn test_e2e_ethereum() {
    // run_e2e("RPC_1", 21345144).await; -- this is just 1M gas block
    run_e2e("RPC_1", 18884864).await;
}

async fn run_e2e(env_var_key: &str, block_number: u64) {
    // Initialize the environment variables.
    dotenv::dotenv().ok();

    // Initialize the logger with detailed output
    let _ = tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(EnvFilter::from_default_env())
        .try_init();

    tracing::info!("🚀 Starting E2E test with HostExecutor");
    tracing::info!("📊 Test Configuration:");
    tracing::info!("   - Block Number: {}", block_number);
    tracing::info!("   - RPC Environment Variable: {}", env_var_key);

    let start_time = Instant::now();

    // Setup the provider.
    tracing::info!("🔗 Setting up RPC provider...");
    let rpc_url =
        Url::parse(std::env::var(env_var_key).unwrap().as_str()).expect("invalid rpc url");
    tracing::info!("   - RPC URL: {}", rpc_url);

    let provider_setup_time = Instant::now();
    let provider = RootProvider::new_http(rpc_url);
    tracing::info!("   ✅ Provider setup completed in {:?}", provider_setup_time.elapsed());

    tracing::info!("🔧 Setting up the host executor");
    let host_executor = HostExecutor::new(provider);

    tracing::info!("⚡ Executing the host (generating ExecutionWitness)...");
    let host_execution_start = Instant::now();
    let client_input = host_executor.execute(block_number).await.expect("failed to execute host");
    let host_execution_time = host_execution_start.elapsed();

    tracing::info!("✅ Host execution completed in {:?}", host_execution_time);

    // Log detailed information about the generated ExecutionWitness
    log_execution_witness_details(&client_input.witness, "HostExecutor");
    log_block_details(&client_input.current_block);

    tracing::info!("🔧 Setting up the client executor");
    let client_executor = ClientExecutor;

    tracing::info!("⚡ Executing the client (validating ExecutionWitness)...");
    let client_execution_start = Instant::now();
    client_executor.execute(client_input.clone()).expect("failed to execute client");
    let client_execution_time = client_execution_start.elapsed();

    tracing::info!("✅ Client execution completed in {:?}", client_execution_time);

    let total_time = start_time.elapsed();
    tracing::info!("🎯 E2E Test Summary:");
    tracing::info!("   - Total Time: {:?}", total_time);
    tracing::info!("   - Host Execution: {:?}", host_execution_time);
    tracing::info!("   - Client Execution: {:?}", client_execution_time);
    tracing::info!(
        "   - Setup Overhead: {:?}",
        total_time - host_execution_time - client_execution_time
    );

    // // Save the client input to a buffer.
    // let buffer = bincode::serialize(&client_input).unwrap();

    // // Load the client input from a buffer.
    // let _: ClientExecutorInput = bincode::deserialize(&buffer).unwrap();
}

fn log_execution_witness_details(witness: &ExecutionWitness, executor_type: &str) {
    tracing::info!("📋 ExecutionWitness Details ({})", executor_type);
    tracing::info!("   🗂️  State Components:");
    tracing::info!("      - State trie nodes: {}", witness.state.len());
    tracing::info!("      - Contract codes: {}", witness.codes.len());
    tracing::info!("      - Key preimages: {}", witness.keys.len());
    tracing::info!("      - Headers: {}", witness.headers.len());

    // Calculate sizes
    let state_size: usize = witness.state.iter().map(|s| s.len()).sum();
    let codes_size: usize = witness.codes.iter().map(|c| c.len()).sum();
    let keys_size: usize = witness.keys.iter().map(|k| k.len()).sum();
    let headers_size: usize = witness.headers.iter().map(|h| h.len()).sum();
    let total_size = state_size + codes_size + keys_size + headers_size;

    tracing::info!("   📏 Size Breakdown:");
    tracing::info!("      - State data: {} bytes", state_size);
    tracing::info!("      - Code data: {} bytes", codes_size);
    tracing::info!("      - Key data: {} bytes", keys_size);
    tracing::info!("      - Header data: {} bytes", headers_size);
    tracing::info!(
        "      - Total witness size: {} bytes ({:.2} KB)",
        total_size,
        total_size as f64 / 1024.0
    );

    // Log sample data (first few items)
    if !witness.state.is_empty() {
        tracing::info!("   🔍 Sample State Nodes (first 3):");
        for (i, node) in witness.state.iter().take(3).enumerate() {
            tracing::info!(
                "      [{}]: {} bytes (0x{}...)",
                i,
                node.len(),
                hex::encode(&node[..std::cmp::min(8, node.len())])
            );
        }
    }

    if !witness.codes.is_empty() {
        tracing::info!("   🔍 Sample Contract Codes (first 3):");
        for (i, code) in witness.codes.iter().take(3).enumerate() {
            tracing::info!(
                "      [{}]: {} bytes (0x{}...)",
                i,
                code.len(),
                hex::encode(&code[..std::cmp::min(8, code.len())])
            );
        }
    }

    if !witness.keys.is_empty() {
        tracing::info!("   🔍 Sample Key Preimages (first 5):");
        for (i, key) in witness.keys.iter().take(5).enumerate() {
            tracing::info!("      [{}]: {} bytes (0x{})", i, key.len(), hex::encode(key));
        }
    }
}

fn log_block_details(
    block: &reth_primitives::Block<alloy_consensus::EthereumTxEnvelope<alloy_consensus::TxEip4844>>,
) {
    tracing::info!("📦 Block Details:");
    tracing::info!("   - Block Number: {}", block.header.number);
    tracing::info!("   - Block Hash: 0x{}", hex::encode(block.header.hash_slow()));
    tracing::info!("   - Parent Hash: 0x{}", hex::encode(block.header.parent_hash));
    tracing::info!("   - Transaction Count: {}", block.body.transactions.len());
    tracing::info!("   - Gas Used: {}", block.header.gas_used);
    tracing::info!("   - Gas Limit: {}", block.header.gas_limit);
    tracing::info!("   - Timestamp: {}", block.header.timestamp);

    // Log sample transactions
    if !block.body.transactions.is_empty() {
        tracing::info!("   🔍 Sample Transactions (first 3):");
        for (i, tx) in block.body.transactions.iter().take(3).enumerate() {
            let tx_hash = tx.hash();
            tracing::info!("      [{}]: 0x{}", i, hex::encode(tx_hash));
        }
    }
}
