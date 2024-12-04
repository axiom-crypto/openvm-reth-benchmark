use core::mem::transmute;

#[allow(unused_imports)]
use axvm;
// use axvm::io::print; // , read_vec, reveal};
// #[allow(unused_imports)]
// use axvm_keccak256_guest; // trigger extern native-keccak256
use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

// axvm::entry!(main);

pub fn main() {
    // Read the input. Implicitly uses bincode deserialize
    // let input = read_vec();
    const INPUT: &[u8] = include_bytes!("../../../rpc-cache/input/1/20526624.bin");
    // print(format!("{:x}", INPUT[INPUT.len() - 4]));
    // print(format!("{:x}", INPUT[INPUT.len() - 3]));
    // print(format!("{:x}", INPUT[INPUT.len() - 2]));
    // print(format!("{:x}", INPUT[INPUT.len() - 1]));
    // print("start bincode");
    let input = bincode::deserialize::<ClientExecutorInput>(INPUT).unwrap();
    // print("finished reading input");

    // Execute the block.
    // let executor = ClientExecutor;
    // let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    // let block_hash = header.hash_slow();

    // Commit the block hash.
    // let block_hash = unsafe { transmute::<_, [u32; 8]>(block_hash) };

    // block_hash.into_iter().enumerate().for_each(|(i, x)| reveal(x, i));
}
