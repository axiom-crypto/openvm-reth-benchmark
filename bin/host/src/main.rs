#![cfg_attr(feature = "tco", allow(incomplete_features))]
#![cfg_attr(feature = "tco", feature(explicit_tail_calls))]
use clap_builder::Parser;
#[cfg(feature = "cuda")]
use openvm_cuda_backend::engine::GpuBabyBearPoseidon2Engine as Engine;
use openvm_reth_benchmark::{
    run_reth_benchmark, HostArgs, NativeDeviceBuilder, SdkVmDeviceBuilder,
};
#[cfg(not(feature = "cuda"))]
use openvm_stark_sdk::config::baby_bear_poseidon2::BabyBearPoseidon2Engine as Engine;

const OPENVM_CLIENT_ETH_ELF: &[u8] = include_bytes!("../elf/openvm-client-eth");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = HostArgs::parse();
    run_reth_benchmark::<Engine, SdkVmDeviceBuilder, NativeDeviceBuilder>(
        args,
        OPENVM_CLIENT_ETH_ELF,
        SdkVmDeviceBuilder,
    )
    .await
}
