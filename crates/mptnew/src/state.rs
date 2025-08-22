use revm_primitives::{ruint::aliases::B256, HashMap};

use crate::MptTrie;

#[derive(Debug)]
pub struct EthereumState<'a> {
    pub state_trie: MptTrie<'a>,
    pub storage_tries: HashMap<B256, MptTrie<'a>>,
}
