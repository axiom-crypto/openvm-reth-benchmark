use openvm::io::{println, read, reveal_bytes32};
use openvm_client_executor::ClientExecutor;
// Imports needed by the linker, but clippy can't tell:
use reth_stateless::StatelessInput;
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    k256::Secp256k1Point,
    openvm_algebra_guest::IntMod,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing::{bls12_381::Bls12_381G1Affine, bn254::Bn254G1Affine},
};

openvm::init!();

pub fn main() {
    println("client-eth starting");
    // Read the input.
    let input: StatelessInput = read();
    println("finished reading input");

    // Execute the block.
    let executor = ClientExecutor;
    let block_hash = executor.execute(input).expect("failed to execute client");

    // Reveal the block hash.
    reveal_bytes32(*block_hash);
}
