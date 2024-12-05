use alloy_provider::ReqwestProvider;
use ax_stark_sdk::{
    ax_stark_backend::{
        self,
        engine::VerificationData,
        p3_field::{AbstractField, PrimeField32},
    },
    config::{baby_bear_poseidon2::BabyBearPoseidon2Engine, FriParameters},
    engine::{StarkFriEngine, VerificationDataWithFriParams},
};
use axvm_algebra_circuit::{ModularExtension, ModularExtensionExecutor, ModularExtensionPeriphery};
use axvm_algebra_transpiler::ModularTranspilerExtension;
use axvm_circuit::{
    arch::{
        instructions::exe::AxVmExe, SystemConfig, SystemExecutor, SystemPeriphery, VirtualMachine,
        VmChipComplex, VmConfig, VmInventoryError,
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
use axvm_rv32im_circuit::{
    Rv32I, Rv32IExecutor, Rv32IPeriphery, Rv32Io, Rv32IoExecutor, Rv32IoPeriphery, Rv32M,
    Rv32MExecutor, Rv32MPeriphery,
};
use axvm_rv32im_transpiler::{
    Rv32ITranspilerExtension, Rv32IoTranspilerExtension, Rv32MTranspilerExtension,
};
use axvm_transpiler::{axvm_platform::memory::MEM_SIZE, elf::Elf, transpiler::Transpiler, FromElf};
use clap::Parser;
use core::option::Option::None;
use derive_more::From;
use metrics::{counter, gauge, Gauge};
use rsp_client_executor::{
    io::ClientExecutorInput, ChainVariant, CHAIN_ID_ETH_MAINNET, CHAIN_ID_LINEA_MAINNET,
    CHAIN_ID_OP_MAINNET,
};
use rsp_host_executor::HostExecutor;
use std::{path::PathBuf, time::Instant};
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub use reth_primitives;

mod execute;

mod cli;
use cli::ProviderArgs;

/// The arguments for the host executable.
#[derive(Debug, Clone, Parser)]
struct HostArgs {
    /// The block number of the block to execute.
    #[clap(long)]
    block_number: u64,
    #[clap(flatten)]
    provider: ProviderArgs,
    /// Whether to generate a proof or just execute the block.
    #[clap(long)]
    prove: bool,
    /// Optional path to the directory containing cached client input. A new cache file will be
    /// created from RPC data if it doesn't already exist.
    #[clap(long)]
    cache_dir: Option<PathBuf>,
    /// The path to the CSV file containing the execution data.
    #[clap(long, default_value = "report.csv")]
    report_path: PathBuf,
}

const RSP_CLIENT_ETH_ELF: &[u8] = include_bytes!("../elf/rsp-client-eth");

#[derive(Clone, Debug, VmConfig)]
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

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Intialize the environment variables.
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Initialize the logger.
    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();

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

                bincode::encode_into_std_write(
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

    // Try smaller input
    // let mut test_input = client_input.clone();
    // test_input.ancestor_headers.clear();
    // test_input.parent_state.storage_tries.clear();
    // test_input.parent_state.state_trie.cached_reference.borrow_mut().take();
    // test_input.state_requests.clear();
    // test_input.bytecodes.clear();
    //
    // let test_input = client_input.parent_state.storage_tries.clone();

    let config = bincode::config::standard();
    let input_vec: Vec<u8> = bincode::serde::encode_to_vec(&client_input, config)?;
    let (decoded, _): (ClientExecutorInput, usize) =
        bincode::serde::decode_from_slice(&input_vec, config)?;
    assert_eq!(client_input, decoded);

    let input_stream = vec![input_vec.into_iter().map(AbstractField::from_canonical_u8).collect()];

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
    );
    let app_log_blowup = 2;
    tracing::info_span!("reth-block", group = "reth_block", block_number = args.block_number)
        .in_scope(|| -> eyre::Result<()> {
            let engine = BabyBearPoseidon2Engine::new(
                FriParameters::standard_with_100_bits_conjectured_security(app_log_blowup),
            );
            // bench_from_exe
            let mut config = Rv32RethConfig::default();
            // 1. Executes runtime once with full metric collection for flamegraphs (slow).
            // config.system_mut().collect_metrics = true;
            // let executor = VmExecutor::<Val<SC>, VC>::new(config.clone());
            // tracing::info_span!("execute_with_metrics", collect_metrics = true)
            //     .in_scope(|| executor.execute(exe.clone(), input_stream.clone()))?;
            // 2. Generate proving key from config.
            config.system.collect_metrics = false;
            counter!("fri.log_blowup").absolute(engine.fri_params().log_blowup as u64);
            let vm = VirtualMachine::new(engine, config);
            if !args.prove {
                // Execute runtime only without metrics collection.
                time(gauge!("execute_time_ms"), || vm.execute(exe, input_stream))?;
            } else {
                // 3. Commit to the exe by generating cached trace for program.
                let committed_exe = time(gauge!("commit_exe_time_ms"), || vm.commit_exe(exe));
                // 4. Executes runtime again without metric collection and generate trace.
                let results = time(gauge!("execute_and_trace_gen_time_ms"), || {
                    vm.execute_and_generate_with_cached_program(committed_exe, input_stream)
                })?;
                let pk = time(gauge!("keygen_time_ms"), || vm.keygen());
                // 5. Generate STARK proofs for each segment (segmentation is determined by
                //    `config`), with timer.
                // vm.prove will emit metrics for proof time of each segment
                let proofs = vm.prove(&pk, results);
                // 6. Verify STARK proofs.
                let vk = pk.get_vk();
                vm.verify(&vk, proofs.clone()).expect("Verification failed");
                let _vdata: Vec<_> = proofs
                    .into_iter()
                    .map(|proof| VerificationDataWithFriParams {
                        data: VerificationData { vk: vk.clone(), proof },
                        fri_params: vm.engine.fri_params(),
                    })
                    .collect();
            }

            Ok(())
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
                bincode::decode_from_std_read(&mut cache_file, bincode::config::standard())?;

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
