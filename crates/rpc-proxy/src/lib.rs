// The initial version of this crate is forked from Zeth <https://github.com/boundless-xyz/zeth/tree/main/crates/rpc-proxy> at tag v0.3.0.

//! This crate provides a proxy for the `debug_executionWitness` endpoint by using standard RPC
//! providers in environments without direct access to a Reth node. This crate is intended for usage
//! in development, testing, and benchmarking. Production use cases should prefer to use a direct
//! integration with Reth via an ExEx.

mod db;
mod lookup;
mod rpc;
mod trie;
mod witness;

pub use lookup::PreimageLookup;
pub use witness::execution_witness;
