use alloy_primitives::hex::ToHexExt;
use alloy_provider::RootProvider;
use alloy_rpc_client::RpcClient;
use alloy_transport::layers::RetryBackoffLayer;
use clap::Parser;
use openvm_benchmarks_prove::util::BenchmarkCli;
use openvm_circuit::{
    arch::{execution_mode::metered::segment_ctx::SegmentationLimits, instructions::exe::VmExe, *},
    openvm_stark_sdk::{
        bench::run_with_metric_collection, openvm_stark_backend::p3_field::PrimeField32,
    },
};
use openvm_client_executor::{io::ClientExecutorInput, CHAIN_ID_ETH_MAINNET};
use openvm_host_executor::HostExecutor;
pub use openvm_native_circuit::{NativeConfig, NativeCpuBuilder};
pub use openvm_sdk::config::SdkVmCpuBuilder;
use openvm_sdk::{
    config::{AppConfig, SdkVmConfig},
    prover::{AppProver, StarkProver},
    GenericSdk, StdIn, F, SC,
};
use openvm_stark_sdk::engine::StarkFriEngine;
use openvm_transpiler::{elf::Elf, openvm_platform::memory::MEM_SIZE, FromElf};
pub use reth_primitives;
use serde_json::json;
use std::{fs, path::PathBuf, sync::Arc};
use tracing::info_span;

mod execute;

mod cli;
use cli::ProviderArgs;

/// Enum representing the execution mode of the host executable.
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BenchMode {
    /// Execute the VM without generating a proof.
    Execute,
    /// Execute the VM with metering to get segments information.
    ExecuteMetered,
    /// Generate sequence of app proofs for continuation segments.
    ProveApp,
    /// Generate a full end-to-end STARK proof with aggregation.
    ProveStark,
    /// Generate a full end-to-end halo2 proof for EVM verifier.
    #[cfg(feature = "evm-verify")]
    ProveEvm,
    /// Generate input file only.
    MakeInput,
}

impl std::fmt::Display for BenchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Execute => write!(f, "execute"),
            Self::ExecuteMetered => write!(f, "execute_metered"),
            Self::ProveApp => write!(f, "prove_app"),
            Self::ProveStark => write!(f, "prove_stark"),
            #[cfg(feature = "evm-verify")]
            Self::ProveEvm => write!(f, "prove_evm"),
            Self::MakeInput => write!(f, "make_input"),
        }
    }
}

/// The arguments for the host executable.
#[derive(Debug, Parser)]
pub struct HostArgs {
    /// The block number of the block to execute.
    #[clap(long)]
    block_number: u64,
    #[clap(flatten)]
    provider: ProviderArgs,

    /// The execution mode.
    #[clap(long, value_enum)]
    mode: BenchMode,

    /// Optional path to the directory containing cached client input. A new cache file will be
    /// created from RPC data if it doesn't already exist.
    #[clap(long)]
    cache_dir: Option<PathBuf>,
    /// The path to the CSV file containing the execution data.
    #[clap(long, default_value = "report.csv")]
    report_path: PathBuf,

    #[clap(flatten)]
    benchmark: BenchmarkCli,

    /// Max cells per chip in segment for continuations
    #[arg(long)]
    pub segment_max_cells: Option<usize>,

    /// Optional path to write the input to. Only needed for mode=make_input
    #[arg(long)]
    pub input_path: Option<PathBuf>,
}

pub fn reth_vm_config(
    app_log_blowup: usize,
    segment_max_height: usize,
    segment_max_cells: usize,
) -> SdkVmConfig {
    let mut config = toml::from_str::<AppConfig<SdkVmConfig>>(include_str!(
        "../../../bin/client-eth/openvm.toml"
    ))
    .unwrap()
    .app_vm_config;
    config.system.config = config
        .system
        .config
        .with_max_constraint_degree((1 << app_log_blowup) + 1)
        .with_public_values(32);
    config.system.config.set_segmentation_limits(
        SegmentationLimits::default()
            .with_max_trace_height(segment_max_height as u32)
            .with_max_cells(segment_max_cells),
    );
    config
}

pub const RETH_DEFAULT_APP_LOG_BLOWUP: usize = 1;
pub const RETH_DEFAULT_LEAF_LOG_BLOWUP: usize = 1;

pub async fn run_reth_benchmark<E, VB, NativeBuilder>(
    args: HostArgs,
    openvm_client_eth_elf: &[u8],
    vm_builder: VB,
) -> eyre::Result<()>
where
    E: StarkFriEngine<SC = SC>,
    VB: VmBuilder<E, VmConfig = SdkVmConfig>,
    <VB::VmConfig as VmExecutionConfig<F>>::Executor:
        Executor<F> + MeteredExecutor<F> + PreflightExecutor<F, VB::RecordArena>,
    NativeBuilder: VmBuilder<E, VmConfig = NativeConfig> + Clone + Default,
    <NativeConfig as VmExecutionConfig<F>>::Executor:
        PreflightExecutor<F, <NativeBuilder as VmBuilder<E>>::RecordArena>,
{
    // Initialize the environment variables.
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Parse the command line arguments.
    let mut args = args;
    let provider_config = args.provider.into_provider().await?;

    match provider_config.chain_id {
        #[allow(non_snake_case)]
        CHAIN_ID_ETH_MAINNET => (),
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
            let client =
                RpcClient::builder().layer(RetryBackoffLayer::new(5, 1000, 100)).http(rpc_url);
            let provider = RootProvider::new(client);

            // Setup the host executor.
            let host_executor = HostExecutor::new(provider);

            // Execute the host.
            let client_input =
                host_executor.execute(args.block_number).await.expect("failed to execute host");

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

    if matches!(args.mode, BenchMode::MakeInput) {
        let words: Vec<u32> = openvm::serde::to_vec(&client_input).unwrap();
        let bytes: Vec<u8> = words.into_iter().flat_map(|w| w.to_le_bytes()).collect();
        let hex_bytes = String::from("0x01") + &hex::encode(&bytes);
        let input = json!({
            "input": [hex_bytes]
        });
        let input = serde_json::to_string(&input).unwrap();
        fs::write(args.input_path.unwrap(), input)?;
        return Ok(());
    }

    let app_log_blowup = args.benchmark.app_log_blowup.unwrap_or(RETH_DEFAULT_APP_LOG_BLOWUP);
    args.benchmark.app_log_blowup = Some(app_log_blowup);
    let segment_max_height = args.benchmark.max_segment_length.unwrap_or((1 << 23) - 100);
    let segment_max_cells = args.segment_max_cells.unwrap_or(u32::MAX as usize); // 2^32 u32's = 16gb
    let leaf_log_blowup = args.benchmark.leaf_log_blowup.unwrap_or(RETH_DEFAULT_LEAF_LOG_BLOWUP);
    args.benchmark.leaf_log_blowup = Some(leaf_log_blowup);

    let vm_config = reth_vm_config(app_log_blowup, segment_max_height, segment_max_cells);
    let sdk =
        GenericSdk::<E, NativeBuilder>::new().with_agg_tree_config(args.benchmark.agg_tree_config);
    let elf = Elf::decode(openvm_client_eth_elf, MEM_SIZE as u32)?;
    let exe = VmExe::from_elf(elf, vm_config.transpiler()).unwrap();

    let program_name = format!("reth.{}.block_{}", args.mode, args.block_number);
    // NOTE: args.benchmark.app_config resets SegmentationLimits if max_segment_length is set
    args.benchmark.max_segment_length = None;
    let app_config = args.benchmark.app_config(vm_config.clone());

    run_with_metric_collection("OUTPUT_PATH", || {
        info_span!("reth-block", block_number = args.block_number).in_scope(
            || -> eyre::Result<()> {
                // Always execute_e1 for benchmarking:
                {
                    let pvs = info_span!("execute_e1", group = program_name).in_scope(|| {
                        sdk.execute(exe.clone(), app_config.app_vm_config.clone(), stdin.clone())
                    })?;
                    let block_hash: Vec<u8> = pvs
                        .iter()
                        .map(|x| x.as_canonical_u32().try_into().unwrap())
                        .collect::<Vec<_>>();
                    println!("block_hash: {}", ToHexExt::encode_hex(&block_hash));
                }
                match args.mode {
                    BenchMode::Execute => {}
                    BenchMode::ExecuteMetered => {
                        let engine = E::new(app_config.app_fri_params.fri_params);
                        let (vm, _) = VirtualMachine::new_with_keygen(
                            engine,
                            vm_builder,
                            app_config.app_vm_config,
                        )?;
                        let executor_idx_to_air_idx = vm.executor_idx_to_air_idx();
                        let interpreter =
                            vm.executor().metered_instance(&exe, &executor_idx_to_air_idx)?;
                        let metered_ctx = vm.build_metered_ctx();
                        let (segments, _) = info_span!("execute_metered", group = program_name)
                            .in_scope(|| interpreter.execute_metered(stdin, metered_ctx))?;
                        println!("Number of segments: {}", segments.len());
                    }
                    BenchMode::ProveApp => {
                        let app_pk = sdk.app_keygen(app_config)?;
                        let app_committed_exe = sdk.commit_app_exe(app_pk.app_fri_params(), exe)?;

                        let mut app_prover = AppProver::<E, _>::new(
                            vm_builder,
                            app_pk.app_vm_pk.clone(),
                            app_committed_exe,
                        )?
                        .with_program_name(program_name);
                        let proof = app_prover.generate_app_proof(stdin)?;
                        let app_vk = app_pk.get_app_vk();
                        sdk.verify_app_proof(&app_vk, &proof)?;
                    }
                    BenchMode::ProveStark => {
                        let app_pk = sdk.app_keygen(app_config)?;
                        let app_committed_exe = sdk.commit_app_exe(app_pk.app_fri_params(), exe)?;
                        let agg_stark_config = args.benchmark.agg_config().agg_stark_config;
                        let agg_stark_pk = sdk.agg_stark_keygen(agg_stark_config)?;
                        let mut prover = StarkProver::<E, _, _>::new(
                            vm_builder,
                            NativeBuilder::default(),
                            Arc::new(app_pk),
                            app_committed_exe,
                            agg_stark_pk,
                            args.benchmark.agg_tree_config,
                        )?;
                        prover.set_program_name(program_name);
                        let proof = prover.generate_e2e_stark_proof(stdin)?;
                        let block_hash = proof
                            .user_public_values
                            .iter()
                            .map(|pv| pv.as_canonical_u32() as u8)
                            .collect::<Vec<u8>>();
                        println!("block_hash: {}", ToHexExt::encode_hex(&block_hash));
                    }
                    #[cfg(feature = "evm-verify")]
                    BenchMode::ProveEvm => {
                        use openvm_native_recursion::halo2::utils::CacheHalo2ParamsReader;
                        use openvm_sdk::{prover::EvmHalo2Prover, DefaultStaticVerifierPvHandler};

                        let halo2_params_reader = CacheHalo2ParamsReader::new(
                            args.benchmark
                                .kzg_params_dir
                                .as_ref()
                                .expect("must set --kzg-params-dir"),
                        );
                        let mut agg_config = args.benchmark.agg_config();
                        agg_config.agg_stark_config.max_num_user_public_values =
                            vm_config.as_ref().num_public_values;

                        let app_pk = sdk.app_keygen(app_config)?;
                        let full_agg_pk = sdk.agg_keygen(
                            agg_config,
                            &halo2_params_reader,
                            &DefaultStaticVerifierPvHandler,
                        )?;
                        tracing::info!(
                            "halo2_outer_k: {}",
                            full_agg_pk.halo2_pk.verifier.pinning.metadata.config_params.k
                        );
                        tracing::info!(
                            "halo2_wrapper_k: {}",
                            full_agg_pk.halo2_pk.wrapper.pinning.metadata.config_params.k
                        );
                        let app_committed_exe = sdk.commit_app_exe(app_pk.app_fri_params(), exe)?;

                        let mut prover = EvmHalo2Prover::<E, _, _>::new(
                            &halo2_params_reader,
                            vm_builder,
                            NativeBuilder::default(),
                            Arc::new(app_pk),
                            app_committed_exe,
                            full_agg_pk,
                            args.benchmark.agg_tree_config,
                        )?;
                        prover.set_program_name(program_name);
                        let evm_proof = prover.generate_proof_for_evm(stdin)?;
                        let block_hash = &evm_proof.user_public_values;
                        println!("block_hash: {}", ToHexExt::encode_hex(block_hash));
                    }
                    BenchMode::MakeInput => {
                        // This case is handled earlier and should not reach here
                        unreachable!();
                    }
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
