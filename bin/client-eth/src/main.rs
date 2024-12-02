use std::mem::transmute;

use axvm::io::{read, reveal};
#[allow(unused_imports)]
use axvm_keccak256_guest; // trigger extern native-keccak256
use rsp_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};

pub fn main() {
    // Read the input. Implicitly uses bincode deserialize
    let input: ClientExecutorInput = read();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    let block_hash = unsafe { transmute::<_, [u32; 8]>(block_hash) };

    block_hash.into_iter().enumerate().for_each(|(i, x)| reveal(x, i));
}
