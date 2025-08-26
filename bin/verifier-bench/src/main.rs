use openvm_circuit::arch::{ContinuationVmProof, VirtualMachine};
use openvm_continuations::verifier::{
    internal::types::InternalVmVerifierInput, leaf::types::LeafVmVerifierInput,
};
use openvm_native_circuit::{NativeCpuBuilder, NATIVE_MAX_TRACE_HEIGHTS};
use openvm_native_recursion::hints::Hintable;
use openvm_sdk::{
    config::{SdkVmConfig, DEFAULT_NUM_CHILDREN_INTERNAL, DEFAULT_NUM_CHILDREN_LEAF},
    keygen::{AggProvingKey, AppProvingKey},
    SC,
};
use openvm_stark_sdk::{
    config::baby_bear_poseidon2::BabyBearPoseidon2Engine,
    engine::{StarkEngine, StarkFriEngine},
    openvm_stark_backend::{proof::Proof, prover::hal::DeviceDataTransporter},
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, default_value = "false")]
    skip_leaf: bool,
    #[clap(long, default_value = "false")]
    skip_internal: bool,
}

fn main() {
    let args = Args::parse();
    let Fixtures { app_proof, leaf_proofs, app_pk, agg_pk } = read_fixtures();
    let AggProvingKey { leaf_vm_pk, internal_vm_pk, internal_committed_exe, .. } = agg_pk;
    if !args.skip_leaf {
        let start = std::time::Instant::now();
        let engine = BabyBearPoseidon2Engine::new(leaf_vm_pk.fri_params);
        let d_pk = engine.device().transport_pk_to_device(&leaf_vm_pk.vm_pk);
        let vm = VirtualMachine::new(engine, NativeCpuBuilder, leaf_vm_pk.vm_config.clone(), d_pk)
            .unwrap();
        let leaf_exe = app_pk.leaf_committed_exe.exe.clone();
        let mut interpreter = vm.preflight_interpreter(&leaf_exe).unwrap();
        let num_app_proofs = app_proof.per_segment.len();
        let leaf_inputs =
            LeafVmVerifierInput::chunk_continuation_vm_proof(&app_proof, DEFAULT_NUM_CHILDREN_LEAF);
        for (i, leaf_input) in leaf_inputs.into_iter().enumerate() {
            let start = std::time::Instant::now();
            let input_stream = leaf_input.write_to_stream();
            let state = vm.create_initial_state(&leaf_exe, input_stream);
            let out = vm
                .execute_preflight(&mut interpreter, state, None, NATIVE_MAX_TRACE_HEIGHTS)
                .expect("Failed to execute preflight");
            println!("end pc {}", out.to_state.pc);
            println!("Time to aggregate app proof chunk {i}, {}s", start.elapsed().as_secs_f64());
        }
        println!(
            "Preflight execution leaf verifier to aggregate {num_app_proofs} app proofs, {}s",
            start.elapsed().as_secs_f64()
        );
    }
    if !args.skip_internal {
        let start = std::time::Instant::now();
        let engine = BabyBearPoseidon2Engine::new(internal_vm_pk.fri_params);
        let d_pk = engine.device().transport_pk_to_device(&internal_vm_pk.vm_pk);
        let vm =
            VirtualMachine::new(engine, NativeCpuBuilder, internal_vm_pk.vm_config.clone(), d_pk)
                .unwrap();
        let internal_exe = internal_committed_exe.exe.clone();
        let mut interpreter = vm.preflight_interpreter(&internal_exe).unwrap();
        let num_leaf_proofs = leaf_proofs.len();
        let internal_inputs = InternalVmVerifierInput::chunk_leaf_or_internal_proofs(
            internal_committed_exe.get_program_commit().into(),
            &leaf_proofs,
            DEFAULT_NUM_CHILDREN_INTERNAL,
        );
        for (i, internal_proof) in internal_inputs.into_iter().enumerate() {
            let start = std::time::Instant::now();
            let input_stream = internal_proof.write();
            let state = vm.create_initial_state(&internal_exe, input_stream);
            let out = vm
                .execute_preflight(&mut interpreter, state, None, NATIVE_MAX_TRACE_HEIGHTS)
                .expect("Failed to execute preflight");
            println!("end pc {}", out.to_state.pc);
            println!("Time to aggregate leaf proof chunk {i}, {}s", start.elapsed().as_secs_f64());
        }
        println!(
            "Preflight execution for internal verifier to aggregate {num_leaf_proofs} leaf proofs, {}s",
            start.elapsed().as_secs_f64()
        );
    }
}

struct Fixtures {
    app_proof: ContinuationVmProof<SC>,
    leaf_proofs: Vec<Proof<SC>>,
    app_pk: AppProvingKey<SdkVmConfig>,
    agg_pk: AggProvingKey,
}

fn read_fixtures() -> Fixtures {
    let app_proof: ContinuationVmProof<SC> = {
        let content =
            std::fs::read(format!("{}/fixtures/app_proof.bitcode", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        bitcode::deserialize(&content).unwrap()
    };
    let leaf_proofs: Vec<Proof<SC>> = {
        let content =
            std::fs::read(format!("{}/fixtures/leaf_proofs.bitcode", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        bitcode::deserialize(&content).unwrap()
    };
    let app_pk: AppProvingKey<SdkVmConfig> = {
        let content =
            std::fs::read(format!("{}/fixtures/app_pk.bitcode", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        bitcode::deserialize(&content).unwrap()
    };
    let agg_pk: AggProvingKey = {
        let content =
            std::fs::read(format!("{}/fixtures/agg_pk.bitcode", env!("CARGO_MANIFEST_DIR")))
                .unwrap();
        bitcode::deserialize(&content).unwrap()
    };

    Fixtures { app_proof, leaf_proofs, app_pk, agg_pk }
}
