use alloy_consensus::{BlockBody, EthereumTxEnvelope, Transaction};
use alloy_primitives::Bytes;
use alloy_provider::{
    network::{AnyNetwork, AnyRpcBlock, BlockResponse, TransactionResponse},
    Provider,
};
use alloy_rpc_types_debug::ExecutionWitness;
use eyre::{eyre, Ok};
use openvm_client_executor::ClientExecutorInput;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use revm_primitives::hash_map::HashMap;
use revm_primitives::Address;
use revm_primitives::B256;
use revm_primitives::U256;
use rustc_hash::FxBuildHasher;
use zerocopy::IntoBytes;

/// A lightweight executor that generates ExecutionWitness using external RPC providers
/// No local node sync required - uses network RPC calls to construct witnesses
#[derive(Debug, Clone)]
pub struct LightweightHostExecutor<P: Provider<AnyNetwork> + Clone> {
    /// The RPC provider (can be external like 1rpc, Alchemy, etc.)
    pub provider: P,
}

impl<P: Provider<AnyNetwork> + Clone> LightweightHostExecutor<P> {
    /// Create a new lightweight executor with an external RPC provider
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Generate ExecutionWitness for a block using only RPC calls
    /// This approach doesn't require a local node - works with any RPC provider
    pub async fn generate_execution_witness(
        &self,
        block_number: u64,
    ) -> eyre::Result<ClientExecutorInput> {
        tracing::info!(
            "generating execution witness for block {} using external RPC",
            block_number
        );

        // 1. Fetch the current block
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        // 2. Extract addresses and storage keys from transactions
        let (addresses, storage_keys) = self.extract_accessed_state(&current_block).await?;

        // 3. Build witness components
        let state = self.build_state_witness(&addresses, &storage_keys, block_number).await?;
        let codes = self.extract_contract_codes(&addresses).await?;
        let keys = self.build_key_preimages(&addresses, &storage_keys);
        let headers = self.build_minimal_headers(block_number).await?;

        // 4. Create ExecutionWitness
        let witness = ExecutionWitness { state, codes, keys, headers };

        // 5. Convert block format
        let processed_block = self.convert_block_format(&current_block)?;

        Ok(ClientExecutorInput { current_block: processed_block, witness })
    }

    /// Extract addresses and storage keys that would be accessed during block execution
    /// This is a heuristic approach since we don't have actual execution traces
    async fn extract_accessed_state(
        &self,
        block: &AnyRpcBlock,
    ) -> eyre::Result<(Vec<Address>, HashMap<Address, Vec<U256>, FxBuildHasher>)> {
        let mut addresses = Vec::new();
        let mut storage_keys: HashMap<Address, Vec<U256>, FxBuildHasher> = HashMap::default();

        let txs = block.transactions().clone().into_transactions();
        // Extract from transactions
        for tx in txs {
            // Add sender and receiver addresses
            addresses.push(tx.from());
            if let Some(to) = tx.to() {
                addresses.push(to);
            }

            // For contract calls, we might need to analyze the input data
            // This is a simplified approach - in reality you'd need more sophisticated analysis
            if let Some(to) = tx.to() {
                let code = self.provider.get_code_at(to).await?;
                if !code.is_empty() {
                    // This is a contract - add some common storage slots
                    let common_slots = vec![
                        U256::from(0), // Often stores important data
                        U256::from(1), // Often stores important data
                        U256::from(2), // Often stores important data
                    ];
                    storage_keys.insert(to, common_slots);
                }
            }
        }

        // Remove duplicates
        addresses.sort();
        addresses.dedup();

        tracing::info!("extracted {} addresses for witness generation", addresses.len());
        Ok((addresses, storage_keys))
    }

    /// Build state witness using storage proofs from RPC
    async fn build_state_witness(
        &self,
        addresses: &[Address],
        storage_keys: &HashMap<Address, Vec<U256>, FxBuildHasher>,
        block_number: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        let mut state_nodes = Vec::new();

        for address in addresses {
            // Get storage keys for this address (if any)
            let keys: Vec<B256> = storage_keys
                .get(address)
                .map(|keys| keys.iter().map(|k| B256::from(*k)).collect())
                .unwrap_or_default();

            // Get storage proof for the previous block (pre-state)
            let proof =
                self.provider.get_proof(*address, keys).block_id((block_number - 1).into()).await?;

            let account_proof = eip1186_proof_to_account_proof(proof);

            // Extract trie nodes from account proof
            for node in &account_proof.proof {
                state_nodes.push(node.clone().into());
            }

            // Extract trie nodes from storage proofs
            for storage_proof in &account_proof.storage_proofs {
                for node in &storage_proof.proof {
                    state_nodes.push(node.clone().into());
                }
            }
        }

        // Remove duplicates
        state_nodes.sort();
        state_nodes.dedup();

        tracing::info!("collected {} state trie nodes", state_nodes.len());
        Ok(state_nodes)
    }

    /// Extract contract codes that were accessed
    async fn extract_contract_codes(&self, addresses: &[Address]) -> eyre::Result<Vec<Bytes>> {
        let mut codes = Vec::new();

        for address in addresses {
            let code = self.provider.get_code_at(*address).await?;
            if !code.is_empty() {
                codes.push(code);
            }
        }

        // Remove duplicates
        codes.sort();
        codes.dedup();

        tracing::info!("collected {} contract codes", codes.len());
        Ok(codes)
    }

    /// Build key preimages (unhashed addresses and storage keys)
    fn build_key_preimages(
        &self,
        addresses: &[Address],
        storage_keys: &HashMap<Address, Vec<U256>, FxBuildHasher>,
    ) -> Vec<Bytes> {
        let mut keys = Vec::new();

        for address in addresses {
            // Add the address itself (unhashed)
            keys.push(address.as_bytes().to_vec().into());

            // Add storage keys for this address (if any)
            if let Some(address_storage_keys) = storage_keys.get(address) {
                for storage_key in address_storage_keys {
                    keys.push(storage_key.to_be_bytes_vec().into());
                }
            }
        }

        // Remove duplicates
        keys.sort();
        keys.dedup();

        tracing::info!("collected {} key preimages", keys.len());
        keys
    }

    /// Build minimal headers (just parent for state verification, since EIP-2935 is implemented)
    async fn build_minimal_headers(&self, block_number: u64) -> eyre::Result<Vec<Bytes>> {
        // Since EIP-2935 is implemented, we only need the parent header for state verification
        let parent_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .await?
            .ok_or(eyre!("couldn't fetch parent block"))?;

        // Create a simple header representation
        // In a full implementation, you'd properly encode the header
        let header_data = serde_json::to_vec(&parent_block.header)?;

        tracing::info!("added parent header for state verification");
        Ok(vec![header_data.into()])
    }

    /// Convert AnyRpcBlock to the required block format
    fn convert_block_format(
        &self,
        _block: &AnyRpcBlock,
    ) -> eyre::Result<reth_primitives::Block<EthereumTxEnvelope<alloy_consensus::TxEip4844>>> {
        // Simplified conversion - in a full implementation you'd need proper type conversion
        // For now, create a minimal block structure
        use alloy_consensus::TxEip4844;
        use reth_primitives::{Block, Header};

        // This is a placeholder - you'd need to implement proper block conversion
        // based on your specific requirements

        tracing::warn!("using simplified block conversion - may need enhancement for production");

        // For now, return an error to indicate this needs proper implementation
        Err(eyre!(
            "Block conversion not fully implemented - needs proper AnyRpcBlock to Block conversion"
        ))
    }
}
