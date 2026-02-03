// Stub crate to be deleted
use std::sync::Arc;

use alloy_consensus::{Header, TxEnvelope};
use alloy_provider::{network::Ethereum, Provider};
use alloy_rpc_types::BlockNumberOrTag;
use eyre::{eyre, Ok, OptionExt};
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::{resolver::MptResolver, EthereumState};
use openvm_rpc_proxy::{execution_witness, PreimageLookup};
use reth_chainspec::MAINNET;
use reth_ethereum::trie::{TrieAccount, EMPTY_ROOT_HASH};
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::Block;
use revm_primitives::{keccak256, Bytes, HashMap, B256};

/// An executor that fetches data from a [Provider] to execute blocks in the [ClientExecutor].
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct HostExecutor<P: Provider<Ethereum> + Clone> {
    /// The provider which fetches data.
    provider: P,
    evm_config: Arc<EthEvmConfig>,
    lookup: Arc<PreimageLookup>,
}

impl<P> HostExecutor<P>
where
    P: Provider<Ethereum> + Clone + 'static,
{
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P, preimage_cache_nibbles: u8) -> Self {
        let evm_config = Arc::new(EthEvmConfig::ethereum(MAINNET.clone()));
        // Initialize the preimage lookup table
        let lookup = Arc::new(PreimageLookup::new(preimage_cache_nibbles));
        Self { provider, evm_config, lookup }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<ClientExecutorInput> {
        let block_id = BlockNumberOrTag::Number(block_number);
        let witness =
            execution_witness(self.evm_config.clone(), &self.provider, block_id, &self.lookup)
                .await?;
        // TODO(refactor): use generate_block_input_from_witness instead
        let parent_block_number = block_number - 1;
        let parent_block = self
            .provider
            .get_block_by_number(parent_block_number.into())
            .await?
            .ok_or_eyre("parent block not found")?;
        let current_block = self
            .provider
            .get_block_by_number(block_id)
            .full()
            .await?
            .map(into_primitive_block)
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;
        let ethereum_state =
            resolve_ethereum_state(parent_block.header.state_root, witness.state, witness.keys)?;

        let bytecodes = {
            let codes = witness.codes;
            let mut bytecodes = Vec::with_capacity(codes.len());
            for code in codes {
                bytecodes.push(revm::state::Bytecode::new_raw(code));
            }
            bytecodes
        };

        let parent_state_bytes = ethereum_state.encode_to_state_bytes();
        let headers = witness.headers;
        let mut ancestor_headers = Vec::with_capacity(headers.len());
        for header_bytes in headers {
            let sealed = Header::decode_sealed(&mut &header_bytes[..])?;
            ancestor_headers.push(sealed.into_inner());
        }
        // Ancestor headers start from most recent
        ancestor_headers.reverse();

        // Create the client input.
        let client_input =
            ClientExecutorInput { current_block, ancestor_headers, parent_state_bytes, bytecodes };
        tracing::info!("successfully generated client input");

        Ok(client_input)
    }
}

fn into_primitive_block(block: alloy_rpc_types::Block) -> Block {
    let block = block.map_transactions(|tx| TxEnvelope::from(tx).into());
    block.into_consensus()
}

fn resolve_ethereum_state(
    state_root: B256,
    reth_state: Vec<Bytes>,
    keys: Vec<Bytes>,
) -> eyre::Result<EthereumState> {
    let mut node_store = Vec::with_capacity(reth_state.len());
    for node in reth_state {
        node_store.push((keccak256(&node), node));
    }
    let mpt_resolver = MptResolver::from_iter(node_store);

    let state_trie = mpt_resolver.resolve(&state_root)?;
    assert_eq!(state_trie.hash(), state_root);
    tracing::debug!(state_root=%state_root, num_nodes=state_trie.num_nodes(), "resolved state trie");

    let mut storage_tries = HashMap::new();

    // Filter accounts
    for key in keys.iter().filter(|k| k.len() == 20) {
        let hashed_address = keccak256(key);
        let storage_root = state_trie
            .get_rlp::<TrieAccount>(hashed_address.as_slice())?
            .map_or(EMPTY_ROOT_HASH, |a| a.storage_root);

        let storage_trie = mpt_resolver.resolve(&storage_root)?;
        assert_eq!(storage_trie.hash(), storage_root);
        tracing::debug!(
            account=%key,
            storage_root=%storage_root,
            num_nodes=storage_trie.num_nodes(),
            "resolved storage trie"
        );

        storage_tries.insert(hashed_address, storage_trie);
    }
    Ok(EthereumState::from_tries(state_trie, storage_tries))
}
