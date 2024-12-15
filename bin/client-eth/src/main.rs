use core::mem::transmute;

use openvm::io::{println, read, reveal};
use openvm_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    openvm_bigint_guest, // trigger extern u256 (this may be unneeded)
    openvm_ecc_guest::k256::Secp256k1Coord,
    openvm_keccak256_guest, /* trigger extern native-keccak256
                             * openvm_pairing_guest::bn254::{Bn254, Fp, Fp2}, */
};

openvm_algebra_guest::moduli_setup::moduli_init! {
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 scalar field
    // "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // bn254 coordinate field
    // "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001" // bn254 scalar field
}
openvm_ecc_guest::sw_setup::sw_init! {
    Secp256k1Coord,
}
// openvm_algebra_complex_macros::complex_init! {
//     Fp2 { mod_idx = 0 },
// }

pub fn main() {
    println("client-eth starting");
    setup_all_moduli();
    setup_all_curves();

    // Read the input.
    let input: ClientExecutorInput = read();
    println("finished reading input");

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    let block_hash = unsafe { transmute::<_, [u32; 8]>(block_hash) };

    block_hash.into_iter().enumerate().for_each(|(i, x)| reveal(x, i));
}
