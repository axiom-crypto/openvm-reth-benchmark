use alloy_provider::ReqwestProvider;
use ax_stark_sdk::{
    ax_stark_backend::{self, p3_field::PrimeField32},
    bench::run_with_metric_collection,
    config::FriParameters,
};
use axvm_algebra_circuit::{ModularExtension, ModularExtensionExecutor, ModularExtensionPeriphery};
use axvm_algebra_transpiler::ModularTranspilerExtension;
use axvm_benchmarks::utils::BenchmarkCli;
use axvm_circuit::{
    arch::{
        instructions::exe::AxVmExe, SystemConfig, SystemExecutor, SystemPeriphery, VmChipComplex,
        VmConfig, VmExecutor, VmInventoryError,
    },
    circuit_derive::{Chip, ChipUsageGetter},
    derive::{AnyEnum, InstructionExecutor, VmConfig},
};
use axvm_ecc_circuit::{
    WeierstrassExtension, WeierstrassExtensionExecutor, WeierstrassExtensionPeriphery,
    SECP256K1_CONFIG,
};
use axvm_ecc_transpiler::EccTranspilerExtension;
use axvm_keccak256_circuit::{Keccak256, Keccak256Executor, Keccak256Periphery};
use axvm_keccak256_transpiler::Keccak256TranspilerExtension;
use axvm_native_compiler::conversion::CompilerOptions;
use axvm_rv32im_circuit::{
    Rv32I, Rv32IExecutor, Rv32IPeriphery, Rv32Io, Rv32IoExecutor, Rv32IoPeriphery, Rv32M,
    Rv32MExecutor, Rv32MPeriphery,
};
use axvm_rv32im_transpiler::{
    Rv32ITranspilerExtension, Rv32IoTranspilerExtension, Rv32MTranspilerExtension,
};
use axvm_sdk::{
    commit::commit_app_exe,
    config::{AggConfig, AppConfig, FullAggConfig, Halo2Config},
    Sdk, StdIn,
};
use axvm_transpiler::{axvm_platform::memory::MEM_SIZE, elf::Elf, transpiler::Transpiler, FromElf};
use clap::{ArgGroup, Parser};
use core::option::Option::None;
use derive_more::From;
use metrics::{gauge, Gauge};
use rsp_client_executor::{
    io::ClientExecutorInput, ChainVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET,
    CHAIN_ID_OP_MAINNET,
};
use rsp_host_executor::HostExecutor;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Instant};

pub use reth_primitives;

mod execute;

mod cli;
use cli::ProviderArgs;

/// The arguments for the host executable.
#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["prove", "execute", "prove_e2e"]),
))]
struct HostArgs {
    /// The block number of the block to execute.
    #[clap(long)]
    block_number: u64,
    #[clap(flatten)]
    provider: ProviderArgs,

    #[clap(long, group = "mode")]
    execute: bool,
    #[clap(long, group = "mode")]
    prove: bool,
    #[clap(long, group = "mode")]
    prove_e2e: bool,

    #[clap(long)]
    collect_metrics: bool,

    /// Optional path to the directory containing cached client input. A new cache file will be
    /// created from RPC data if it doesn't already exist.
    #[clap(long)]
    cache_dir: Option<PathBuf>,
    /// The path to the CSV file containing the execution data.
    #[clap(long, default_value = "report.csv")]
    report_path: PathBuf,

    #[clap(flatten)]
    benchmark: BenchmarkCli,
}

const RSP_CLIENT_ETH_ELF: &[u8] = include_bytes!("../elf/rsp-client-eth");

#[derive(Clone, Debug, VmConfig, Serialize, Deserialize)]
pub struct Rv32RethConfig {
    #[system]
    pub system: SystemConfig,
    #[extension]
    pub base: Rv32I,
    #[extension]
    pub mul: Rv32M,
    #[extension]
    pub io: Rv32Io,
    #[extension]
    pub keccak: Keccak256,
    #[extension]
    pub modular: ModularExtension,
    #[extension]
    pub weierstrass: WeierstrassExtension,
}

impl Default for Rv32RethConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig::default()
                .with_continuations()
                .with_public_values(32)
                .with_max_segment_len((1 << 23) - 100),
            base: Rv32I,
            mul: Rv32M::default(),
            io: Rv32Io,
            keccak: Keccak256,
            modular: ModularExtension::new(vec![
                SECP256K1_CONFIG.modulus.clone(),
                SECP256K1_CONFIG.scalar.clone(),
            ]),
            weierstrass: WeierstrassExtension::new(vec![SECP256K1_CONFIG.clone()]),
        }
    }
}

impl Rv32RethConfig {
    fn new(app_log_blowup: usize) -> Self {
        let mut config = Self::default();
        config.system.max_constraint_degree = (1 << app_log_blowup) + 1;
        config
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Intialize the environment variables.
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Parse the command line arguments.
    let args = HostArgs::parse();
    let provider_config = args.provider.into_provider().await?;

    let variant = match provider_config.chain_id {
        CHAIN_ID_ETH_MAINNET => ChainVariant::Ethereum,
        CHAIN_ID_OP_MAINNET => ChainVariant::Optimism,
        CHAIN_ID_LINEA_MAINNET => ChainVariant::Linea,
        _ => {
            eyre::bail!("unknown chain ID: {}", provider_config.chain_id);
        }
    };

    let client_input_from_cache = try_load_input_from_cache(
        args.cache_dir.as_ref(),
        provider_config.chain_id,
        args.block_number,
    )?;

    let client_input = match (client_input_from_cache, provider_config.rpc_url) {
        (Some(client_input_from_cache), _) => client_input_from_cache,
        (None, Some(rpc_url)) => {
            // Cache not found but we have RPC
            // Setup the provider.
            let provider = ReqwestProvider::new_http(rpc_url);

            // Setup the host executor.
            let host_executor = HostExecutor::new(provider);

            // Execute the host.
            let client_input = host_executor
                .execute(args.block_number, variant)
                .await
                .expect("failed to execute host");

            if let Some(cache_dir) = args.cache_dir {
                let input_folder = cache_dir.join(format!("input/{}", provider_config.chain_id));
                if !input_folder.exists() {
                    std::fs::create_dir_all(&input_folder)?;
                }

                let input_path = input_folder.join(format!("{}.bin", args.block_number));
                let mut cache_file = std::fs::File::create(input_path)?;

                bincode::serde::encode_into_std_write(
                    &client_input,
                    &mut cache_file,
                    bincode::config::standard(),
                )?;
            }

            client_input
        }
        (None, None) => {
            eyre::bail!("cache not found and RPC URL not provided")
        }
    };

    let mut stdin = StdIn::default();
    stdin.write(&client_input);

    let sdk = Sdk;
    let elf = Elf::decode(RSP_CLIENT_ETH_ELF, MEM_SIZE as u32)?;
    let exe = AxVmExe::from_elf(
        elf,
        Transpiler::default()
            .with_extension(Rv32ITranspilerExtension)
            .with_extension(Rv32MTranspilerExtension)
            .with_extension(Rv32IoTranspilerExtension)
            .with_extension(Keccak256TranspilerExtension)
            .with_extension(ModularTranspilerExtension)
            .with_extension(EccTranspilerExtension),
        // add more extensions
    )
    .unwrap();
    let app_log_blowup = args.benchmark.app_log_blowup.unwrap_or(2);
    let agg_log_blowup = args.benchmark.agg_log_blowup.unwrap_or(2);
    let internal_log_blowup = args.benchmark.internal_log_blowup.unwrap_or(2);
    let root_log_blowup = args.benchmark.root_log_blowup.unwrap_or(3);
    let max_segment_length = args.benchmark.max_segment_length.unwrap_or((1 << 23) - 100);

    let mut compiler_options = CompilerOptions::default();
    if args.collect_metrics {
        compiler_options.enable_cycle_tracker = true;
    }
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let leaf_fri_params =
        FriParameters::standard_with_100_bits_conjectured_security(agg_log_blowup);
    let mut vm_config = Rv32RethConfig::new(app_log_blowup);
    vm_config.system.collect_metrics = args.collect_metrics;
    vm_config.system.max_segment_len = max_segment_length;

    let app_config = AppConfig {
        app_vm_config: vm_config.clone(),
        app_fri_params,
        leaf_fri_params: leaf_fri_params.into(),
        compiler_options,
    };

    run_with_metric_collection("OUTPUT_PATH", || {
        tracing::info_span!("reth-block", group = "reth_block", block_number = args.block_number)
            .in_scope(|| -> eyre::Result<()> {
                if args.execute {
                    let executor = VmExecutor::<_, _>::new(vm_config);
                    time(gauge!("execute_time_ms"), || executor.execute(exe, stdin))?;
                } else if args.prove {
                    let app_pk = sdk.app_keygen(app_config)?;
                    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe)?;

                    let _proof = sdk.generate_app_proof(app_pk, app_committed_exe, stdin)?;
                    // TODO: fix sdk to not consume app_pk
                    // sdk.verify_app_proof(&app_pk, &proof)?;
                } else {
                    let full_agg_config = FullAggConfig {
                        agg_config: AggConfig {
                            max_num_user_public_values: vm_config.system.num_public_values,
                            leaf_fri_params,
                            internal_fri_params:
                                FriParameters::standard_with_100_bits_conjectured_security(
                                    internal_log_blowup,
                                ),
                            root_fri_params:
                                FriParameters::standard_with_100_bits_conjectured_security(
                                    root_log_blowup,
                                ),
                            compiler_options,
                        },
                        halo2_config: Halo2Config { verifier_k: 24, wrapper_k: None },
                    };

                    let app_pk = sdk.app_keygen(app_config)?;
                    let full_agg_pk = sdk.agg_keygen(full_agg_config)?;
                    let app_committed_exe = commit_app_exe(app_fri_params, exe);

                    let _evm_proof =
                        sdk.generate_evm_proof(app_pk, app_committed_exe, full_agg_pk, stdin)?;
                }

                Ok(())
            })
    })?;

    Ok(())
}

fn try_load_input_from_cache(
    cache_dir: Option<&PathBuf>,
    chain_id: u64,
    block_number: u64,
) -> eyre::Result<Option<ClientExecutorInput>> {
    Ok(if let Some(cache_dir) = cache_dir {
        let cache_path = cache_dir.join(format!("input/{}/{}.bin", chain_id, block_number));

        if cache_path.exists() {
            // TODO: prune the cache if invalid instead
            let mut cache_file = std::fs::File::open(cache_path)?;
            let client_input: ClientExecutorInput =
                bincode::serde::decode_from_std_read(&mut cache_file, bincode::config::standard())?;

            Some(client_input)
        } else {
            None
        }
    } else {
        None
    })
}

fn time<F: FnOnce() -> R, R>(gauge: Gauge, f: F) -> R {
    let start = Instant::now();
    let res = f();
    gauge.set(start.elapsed().as_millis() as f64);
    res
}
