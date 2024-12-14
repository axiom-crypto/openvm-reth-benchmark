use alloy_provider::ReqwestProvider;
use clap::{ArgGroup, Parser};
use core::option::Option::None;
use derive_more::From;
use openvm_algebra_circuit::ModularExtension;
use openvm_benchmarks::utils::BenchmarkCli;
use openvm_circuit::arch::{instructions::exe::VmExe, SystemConfig, VmConfig, VmExecutor};
use openvm_ecc_circuit::{WeierstrassExtension, SECP256K1_CONFIG};
use openvm_native_compiler::conversion::CompilerOptions;
use openvm_native_recursion::halo2::utils::CacheHalo2ParamsReader;
use openvm_sdk::{
    commit::commit_app_exe,
    config::{AggConfig, AggStarkConfig, AppConfig, Halo2Config, SdkVmConfig},
    prover::{AppProver, ContinuationProver},
    Sdk, StdIn,
};
use openvm_stark_sdk::{
    bench::run_with_metric_collection, config::FriParameters, p3_baby_bear::BabyBear,
};
use openvm_transpiler::{elf::Elf, openvm_platform::memory::MEM_SIZE, FromElf};
use rsp_client_executor::{
    io::ClientExecutorInput, ChainVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET,
    CHAIN_ID_OP_MAINNET,
};
use rsp_host_executor::HostExecutor;
use std::{path::PathBuf, sync::Arc};

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

fn reth_vm_config(
    app_log_blowup: usize,
    max_segment_length: usize,
    collect_metrics: bool,
) -> SdkVmConfig {
    let mut system_config = SystemConfig::default()
        .with_continuations()
        .with_max_constraint_degree((1 << app_log_blowup) + 1)
        .with_public_values(32)
        .with_max_segment_len(max_segment_length);
    if collect_metrics {
        system_config.collect_metrics = true;
    }
    SdkVmConfig::builder()
        .system(system_config.into())
        .rv32i(Default::default())
        .rv32m(Default::default())
        .io(Default::default())
        .keccak(Default::default())
        .modular(ModularExtension::new(vec![
            SECP256K1_CONFIG.modulus.clone(),
            SECP256K1_CONFIG.scalar.clone(),
        ]))
        .ecc(WeierstrassExtension::new(vec![SECP256K1_CONFIG.clone()]))
        .build()
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

    let app_log_blowup = args.benchmark.app_log_blowup.unwrap_or(2);
    let agg_log_blowup = args.benchmark.agg_log_blowup.unwrap_or(2);
    let internal_log_blowup = args.benchmark.internal_log_blowup.unwrap_or(2);
    let root_log_blowup = args.benchmark.root_log_blowup.unwrap_or(3);
    let max_segment_length = args.benchmark.max_segment_length.unwrap_or((1 << 23) - 100);

    let vm_config = reth_vm_config(app_log_blowup, max_segment_length, args.collect_metrics);
    let sdk = Sdk;
    let elf = Elf::decode(RSP_CLIENT_ETH_ELF, MEM_SIZE as u32)?;
    let exe = VmExe::from_elf(elf, vm_config.transpiler()).unwrap();

    let mut compiler_options = CompilerOptions::default();
    if args.collect_metrics {
        compiler_options.enable_cycle_tracker = true;
    }
    let app_fri_params = FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup);
    let leaf_fri_params =
        FriParameters::standard_with_100_bits_conjectured_security(agg_log_blowup);

    let program_name = "reth_block";
    let app_config = AppConfig {
        app_vm_config: vm_config.clone(),
        app_fri_params,
        leaf_fri_params: leaf_fri_params.into(),
        compiler_options,
    };

    run_with_metric_collection("OUTPUT_PATH", || {
        tracing::info_span!("reth-block", block_number = args.block_number).in_scope(
            || -> eyre::Result<()> {
                if args.execute {
                    let executor = VmExecutor::<_, _>::new(vm_config);
                    executor.execute(exe, stdin)?;
                } else if args.prove {
                    let app_pk = sdk.app_keygen(app_config)?;
                    let app_committed_exe = sdk.commit_app_exe(app_fri_params, exe)?;

                    let mut app_prover =
                        AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe)
                            .with_program_name(program_name);
                    app_prover.set_profile(args.collect_metrics);
                    let proof = app_prover.generate_app_proof(stdin);
                    let app_vk = app_pk.get_vk();
                    sdk.verify_app_proof(&app_vk, &proof)?;
                } else {
                    let halo2_params_reader = CacheHalo2ParamsReader::new_with_default_params_dir();
                    let full_agg_config = AggConfig {
                        agg_stark_config: AggStarkConfig {
                            max_num_user_public_values: VmConfig::<BabyBear>::system(&vm_config)
                                .num_public_values,
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
                    let full_agg_pk = sdk.agg_keygen(full_agg_config, &halo2_params_reader)?;
                    let app_committed_exe = commit_app_exe(app_fri_params, exe);

                    let mut prover = ContinuationProver::new(
                        &halo2_params_reader,
                        Arc::new(app_pk),
                        app_committed_exe,
                        full_agg_pk,
                    );
                    prover.set_program_name("reth_block");
                    prover.set_profile(args.collect_metrics);
                    let _evm_proof = prover.generate_proof_for_evm(stdin);
                }

                Ok(())
            },
        )
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
