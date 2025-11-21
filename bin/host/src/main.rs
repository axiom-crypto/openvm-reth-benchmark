#![cfg_attr(feature = "aot", allow(incomplete_features))]
#![cfg_attr(feature = "aot", feature(explicit_tail_calls))]
use clap_builder::Parser;
use openvm_reth_benchmark::{run_reth_benchmark, HostArgs};

const OPENVM_CLIENT_ETH_ELF: &[u8] = include_bytes!("../elf/openvm-client-eth");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = HostArgs::parse();
    run_reth_benchmark(args, OPENVM_CLIENT_ETH_ELF).await
}
