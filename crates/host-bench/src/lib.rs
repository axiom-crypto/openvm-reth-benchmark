use alloy_provider::RootProvider;
use alloy_rpc_client::RpcClient;
use alloy_transport::layers::RetryBackoffLayer;
use clap::{ArgGroup, Parser};
use openvm_algebra_circuit::{Fp2Extension, ModularExtension};
use openvm_benchmarks_prove::util::BenchmarkCli;
use openvm_bigint_circuit::Int256;
use openvm_circuit::{
    arch::{
        instructions::exe::VmExe, DefaultSegmentationStrategy, SystemConfig, VmConfig, VmExecutor,
    },
    openvm_stark_sdk::{
        bench::run_with_metric_collection, config::baby_bear_poseidon2::BabyBearPoseidon2Config,
        openvm_stark_backend::p3_field::PrimeField32, p3_baby_bear::BabyBear,
    },
};
use openvm_client_executor::{
    io::ClientExecutorInput, ChainVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET,
    CHAIN_ID_OP_MAINNET,
};
use openvm_ecc_circuit::{WeierstrassExtension, SECP256K1_CONFIG};
use openvm_host_executor::HostExecutor;
use openvm_native_recursion::halo2::utils::CacheHalo2ParamsReader;
use openvm_pairing_circuit::{PairingCurve, PairingExtension};
use openvm_rv32im_circuit::Rv32M;
use openvm_sdk::{
    config::SdkVmConfig,
    prover::{AppProver, ContinuationProver},
    DefaultStaticVerifierPvHandler, GenericSdk, StdIn, SC,
};
use openvm_stark_sdk::engine::StarkFriEngine;
use openvm_transpiler::{elf::Elf, openvm_platform::memory::MEM_SIZE, FromElf};
pub use reth_primitives;
use reth_primitives::hex::ToHexExt;
use serde_json::json;
use std::{fs, path::PathBuf, sync::Arc};
use tracing::info_span;

mod execute;

mod cli;
use cli::ProviderArgs;

/// The arguments for the host executable.
#[derive(Debug, Parser)]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["prove", "execute", "tracegen", "prove_e2e"]),
))]
pub struct HostArgs {
    /// The block number of the block to execute.
    #[clap(long)]
    block_number: u64,
    #[clap(flatten)]
    provider: ProviderArgs,

    #[clap(long, group = "mode")]
    execute: bool,
    #[clap(long, group = "mode")]
    tracegen: bool,
    #[clap(long, group = "mode")]
    prove: bool,
    #[clap(long, group = "mode")]
    prove_e2e: bool,
    #[clap(long, group = "mode")]
    make_input: bool,

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
    #[arg(long, alias = "max_cells_per_chip_in_segment")]
    pub max_cells_per_chip_in_segment: Option<usize>,

    #[arg(long)]
    pub no_kzg_intrinsics: bool,

    /// Optional path to write the input to. Only needed for mode=make_input
    #[arg(long)]
    pub input_path: Option<PathBuf>,
}

pub fn reth_vm_config(
    app_log_blowup: usize,
    max_segment_length: usize,
    max_cells_per_chip_in_segment: usize,
    use_kzg_intrinsics: bool,
) -> SdkVmConfig {
    let mut system_config = SystemConfig::default()
        .with_continuations()
        .with_max_constraint_degree((1 << app_log_blowup) + 1)
        .with_public_values(32);
    system_config.set_segmentation_strategy(Arc::new(DefaultSegmentationStrategy::new(
        max_segment_length,
        max_cells_per_chip_in_segment,
    )));
    let int256 = Int256::default();
    let bn_config = PairingCurve::Bn254.curve_config();
    let bls_config = PairingCurve::Bls12_381.curve_config();
    // The builder will do this automatically, but we set it just in case.
    let rv32m = Rv32M { range_tuple_checker_sizes: int256.range_tuple_checker_sizes };
    let mut supported_moduli = vec![
        bn_config.modulus.clone(),
        bn_config.scalar.clone(),
        SECP256K1_CONFIG.modulus.clone(),
        SECP256K1_CONFIG.scalar.clone(),
    ];
    let mut supported_complex_moduli = vec![bn_config.modulus.clone()];
    let mut supported_curves = vec![bn_config.clone(), SECP256K1_CONFIG.clone()];
    let mut supported_pairing_curves = vec![PairingCurve::Bn254];
    if use_kzg_intrinsics {
        supported_moduli.push(bls_config.modulus.clone());
        supported_moduli.push(bls_config.scalar.clone());
        supported_complex_moduli.push(bls_config.modulus.clone());
        supported_curves.push(bls_config.clone());
        supported_pairing_curves.push(PairingCurve::Bls12_381);
    }
    SdkVmConfig::builder()
        .system(system_config.into())
        .rv32i(Default::default())
        .rv32m(rv32m)
        .io(Default::default())
        .keccak(Default::default())
        .sha256(Default::default())
        .bigint(int256)
        .modular(ModularExtension::new(supported_moduli))
        .fp2(Fp2Extension::new(supported_complex_moduli))
        .ecc(WeierstrassExtension::new(supported_curves))
        .pairing(PairingExtension::new(supported_pairing_curves))
        .build()
}

pub const RETH_DEFAULT_APP_LOG_BLOWUP: usize = 1;
pub const RETH_DEFAULT_LEAF_LOG_BLOWUP: usize = 1;

#[tokio::main]
pub async fn run_reth_benchmark<E: StarkFriEngine<SC>>(
    args: HostArgs,
    openvm_client_eth_elf: &[u8],
) -> eyre::Result<()> {
    // Initialize the environment variables.
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Parse the command line arguments.
    let mut args = args;
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
            let client =
                RpcClient::builder().layer(RetryBackoffLayer::new(5, 1000, 100)).http(rpc_url);
            let provider = RootProvider::new(client);

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

    if args.make_input {
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
    let max_segment_length = args.benchmark.max_segment_length.unwrap_or((1 << 23) - 100);
    let max_cells_per_chip_in_segment =
        args.max_cells_per_chip_in_segment.unwrap_or(((1 << 23) - 100) * 120);
    let leaf_log_blowup = args.benchmark.leaf_log_blowup.unwrap_or(RETH_DEFAULT_LEAF_LOG_BLOWUP);
    args.benchmark.leaf_log_blowup = Some(leaf_log_blowup);

    let vm_config = reth_vm_config(
        app_log_blowup,
        max_segment_length,
        max_cells_per_chip_in_segment,
        !args.no_kzg_intrinsics,
    );
    let sdk = GenericSdk::<E>::new();
    let elf = Elf::decode(openvm_client_eth_elf, MEM_SIZE as u32)?;
    let exe = VmExe::from_elf(elf, vm_config.transpiler()).unwrap();

    let mode = if args.execute {
        "execute"
    } else if args.tracegen {
        "tracegen"
    } else if args.prove {
        "prove"
    } else {
        "prove_e2e"
    };
    let program_name = format!("reth.{}.block_{}", mode, args.block_number);
    let app_config = args.benchmark.app_config(vm_config.clone());

    run_with_metric_collection("OUTPUT_PATH", || {
        info_span!("reth-block", block_number = args.block_number).in_scope(
            || -> eyre::Result<()> {
                if args.execute {
                    let pvs = info_span!("execute", group = program_name)
                        .in_scope(|| sdk.execute(exe, app_config.app_vm_config, stdin))?;
                    let block_hash: Vec<u8> = pvs
                        .iter()
                        .map(|x| x.as_canonical_u32().try_into().unwrap())
                        .collect::<Vec<_>>();
                    println!("block_hash: {}", ToHexExt::encode_hex(&block_hash));
                } else if args.tracegen {
                    let executor = VmExecutor::<_, _>::new(app_config.app_vm_config);
                    info_span!("tracegen", group = program_name).in_scope(|| {
                        executor.execute_and_generate::<BabyBearPoseidon2Config>(exe, stdin)
                    })?;
                } else if args.prove {
                    let app_pk = sdk.app_keygen(app_config)?;
                    let app_committed_exe = sdk.commit_app_exe(app_pk.app_fri_params(), exe)?;

                    let app_prover =
                        AppProver::<_, E>::new(app_pk.app_vm_pk.clone(), app_committed_exe)
                            .with_program_name(program_name);
                    let proof = app_prover.generate_app_proof(stdin);
                    let app_vk = app_pk.get_app_vk();
                    sdk.verify_app_proof(&app_vk, &proof)?;
                } else {
                    let halo2_params_reader = CacheHalo2ParamsReader::new_with_default_params_dir();
                    let mut agg_config = args.benchmark.agg_config();
                    agg_config.agg_stark_config.max_num_user_public_values =
                        VmConfig::<BabyBear>::system(&vm_config).num_public_values;

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

                    let mut prover = ContinuationProver::<_, E>::new(
                        &halo2_params_reader,
                        Arc::new(app_pk),
                        app_committed_exe,
                        full_agg_pk,
                    );
                    prover.set_program_name(program_name);
                    let evm_proof = prover.generate_proof_for_evm(stdin);
                    let block_hash = &evm_proof.user_public_values;
                    println!("block_hash: {}", ToHexExt::encode_hex(block_hash));
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
