use openvm::io::{println, read, reveal_bytes32};
#[allow(unused_imports)]
use openvm_client_executor::{io::ClientExecutorInput, ClientExecutor};
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    openvm_algebra_guest::IntMod,
    openvm_bigint_guest, // trigger extern u256 (this may be unneeded)
    openvm_ecc_guest::k256::Secp256k1Point,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing_guest::{bls12_381::Bls12_381G1Affine, bn254::Bn254G1Affine},
};

#[cfg(feature = "kzg-intrinsics")]
openvm_algebra_moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
    "0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab", // BLS12-381 Coordinate field
    "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001" // BLS12-381 Scalar field
}
#[cfg(feature = "kzg-intrinsics")]
openvm_ecc_sw_macros::sw_init! {
    Bn254G1Affine,
    Secp256k1Point,
    Bls12_381G1Affine,
}
#[cfg(feature = "kzg-intrinsics")]
openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
    Bls12_381Fp2 { mod_idx = 4 },
}

#[cfg(not(feature = "kzg-intrinsics"))]
openvm_algebra_moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
}
#[cfg(not(feature = "kzg-intrinsics"))]
openvm_ecc_sw_macros::sw_init! {
    Bn254G1Affine,
    Secp256k1Point,
}
#[cfg(not(feature = "kzg-intrinsics"))]
openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
}

pub fn main() {
    println("client-eth starting");
    // TODO: ??
    // Setup secp256k1 because it is always used for recover_signers
    // setup_2();
    // setup_3();
    // setup_sw_Secp256k1Point();

    // Read the input.
    let input: ClientExecutorInput = read();
    println("finished reading input");

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Reveal the block hash.
    reveal_bytes32(*block_hash);

    // TODO: ??
    // Setup can be called at any time.
    // if unsafe { USED_BN_ADD || USED_BN_MUL || USED_BN_PAIR } {
    //     setup_0(); // Bn254 coordinate field
    // }
    // if unsafe { USED_BN_ADD || USED_BN_MUL } {
    //     // pairing does not use ecc extension
    //     setup_sw_Bn254G1Affine();
    // }
    // if unsafe { USED_BN_MUL || USED_BN_PAIR } {
    //     setup_1(); // Bn254 scalar field
    // }
    // if unsafe { USED_BN_PAIR } {
    //     setup_complex_0(); // Bn254 complex extension of coordinate field
    // }
    // #[cfg(feature = "kzg-intrinsics")]
    // if unsafe { USED_KZG_PROOF } {
    //     setup_4(); // Bls12-381 coordinate field
    //     setup_5(); // Bls12-381 scalar field
    //     setup_sw_Bls12_381G1Affine();
    //     setup_complex_1(); // Bls12-381 complex extension of coordinate field
    // }
}
