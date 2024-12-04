use core::mem::transmute;

use axvm::io::{print, read_vec, reveal};
// #[allow(unused_imports)]
// use axvm_keccak256_guest; // trigger extern native-keccak256
use rsp_client_executor::{
    io::ClientExecutorInput, rsp_mpt::StorageTries, ClientExecutor, EthereumVariant,
};

// axvm::entry!(main);

pub fn main() {
    // Read the input. Implicitly uses bincode deserialize
    let input_vec = read_vec();
    // const INPUT: &[u8] = include_bytes!("../../../rpc-cache/input/1/20526624.bin");
    // print(format!("{:x}", INPUT[INPUT.len() - 4]));
    // print(format!("{:x}", INPUT[INPUT.len() - 3]));
    // print(format!("{:x}", INPUT[INPUT.len() - 2]));
    // print(format!("{:x}", INPUT[INPUT.len() - 1]));
    print("start bincode");
    let (input, len): (StorageTries, usize) =
        bincode::decode_from_slice(&input_vec[..], bincode::config::standard()).unwrap();
    print("finished reading input");
    assert_eq!(len, input_vec.len());

    // Execute the block.
    // let executor = ClientExecutor;
    // let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    // let block_hash = header.hash_slow();

    // Commit the block hash.
    // let block_hash = unsafe { transmute::<_, [u32; 8]>(block_hash) };

    // block_hash.into_iter().enumerate().for_each(|(i, x)| reveal(x, i));
}
