use alloy_provider::RootProvider;
use bincode::config::standard;
use openvm_client_executor::{io::ClientExecutorInput, ClientExecutor};
use openvm_host_executor::HostExecutor;
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};
use url::Url;

#[tokio::test(flavor = "multi_thread")]
async fn test_e2e_ethereum() {
    color_eyre::install().unwrap();
    // Initialize the logger with default configuration
    let _ = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init();

    let env_var_key = "RPC_1";
    let block_number = 22796011;

    tracing::info!("Starting test for block {}", block_number);

    // Initialize the environment variables.
    dotenv::dotenv().ok();

    // Setup the provider.
    let rpc_url =
        Url::parse(std::env::var(env_var_key).unwrap().as_str()).expect("invalid rpc url");
    let provider = RootProvider::new_http(rpc_url);

    // Setup the host executor.
    let host_executor = HostExecutor::new(provider);

    tracing::info!("Executing block {}", block_number);

    // Execute the host.
    let client_input = host_executor.execute(block_number).await.expect("failed to execute host");

    // Setup the client executor.
    let client_executor = ClientExecutor;

    // Test serialization/deserialization round-trip
    let bincode_config = standard();
    let buffer = bincode::serde::encode_to_vec(&client_input, bincode_config).unwrap();
    let (deserialized_input, _): (ClientExecutorInput, _) =
        bincode::serde::decode_from_slice(&buffer, bincode_config).unwrap();

    tracing::info!("Going to execute on client now");

    // Execute the client with the original input
    client_executor.execute(client_input).expect("failed to execute client");

    // Execute the client with the deserialized input to test round-trip
    client_executor
        .execute(deserialized_input)
        .expect("failed to execute client with deserialized input");
}
