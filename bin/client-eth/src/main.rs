use core::mem::transmute;

use axvm::io::{println, read, reveal};
#[allow(unused_imports)]
use axvm_bigint_guest; // trigger extern u256 (this may be unneeded)
use axvm_ecc_guest::k256::Secp256k1Coord;
#[allow(unused_imports)]
use axvm_keccak256_guest; // trigger extern native-keccak256
use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

axvm_algebra_guest::moduli_setup::moduli_init! {
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F",
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141"
}
axvm_ecc_guest::sw_setup::sw_init! {
    Secp256k1Coord,
}

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
