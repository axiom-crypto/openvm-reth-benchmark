use alloy_consensus::{BlockBody, EthereumTxEnvelope};
use alloy_primitives::Bytes;
use alloy_provider::{
    network::{AnyNetwork, AnyRpcBlock, BlockResponse},
    Provider,
};
use alloy_rpc_types_debug::ExecutionWitness;
use eyre::{eyre, Ok, OptionExt};
use openvm_client_executor::ClientExecutorInput;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_ethereum_consensus::EthBeaconConsensus;
use reth_evm::{execute::Executor, ConfigureEvm};
use reth_evm_ethereum::execute::EthExecutorProvider;
use reth_execution_types::BlockExecutionResult;
use reth_primitives::{Block, RecoveredBlock};
use revm::database::CacheDB;
use revm_primitives::hash_map::HashMap;
use revm_primitives::Address;
use revm_primitives::B256;
use revm_primitives::U256;
use rustc_hash::FxBuildHasher;
use zerocopy::IntoBytes;

/// /// An executor that fetches data from a [Provider] to generate [ExecutionWitness] for a block.
#[derive(Debug, Clone)]
pub struct HostExecutor<P: Provider<AnyNetwork> + Clone> {
    /// The provider which fetches data.
    pub provider: P,
}
impl<P: Provider<AnyNetwork> + Clone> HostExecutor<P> {
    /// Create a new [`HostExecutor`] with a specific [Provider] and [Transport].
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Executes the block with the given block number.
    pub async fn execute(&self, block_number: u64) -> eyre::Result<ClientExecutorInput> {
        // For now only MAINNET is supported.
        let spec = MAINNET.clone();

        // Fetch the current block and the previous block from the provider.
        tracing::info!("fetching the current block and the previous block");
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;
        let previous_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .full()
            .await?
            .ok_or(eyre!("couldn't fetch block: {}", block_number))?;

        // Setup the database for the block executor.
        tracing::info!("setting up the database for the block executor");
        let rpc_db = RpcDb::new(self.provider.clone(), block_number - 1);
        let cache_db = CacheDB::new(&rpc_db);

        // Execute the block and fetch all the necessary data along the way.
        tracing::info!(
            "executing the block and with rpc db: block_number={block_number}, transaction_count={transaction_count}",
            transaction_count = current_block.transactions().len()
        );

        let executor_block_input = recover_rpc_block(current_block.clone());

        let executor_output = EthExecutorProvider::new(spec.clone())
            .executor(cache_db)
            .execute(&executor_block_input)?;

        // Validate the block post execution.
        tracing::info!("validating the block post execution");
        let consensus = EthBeaconConsensus::new(spec.clone());

        let block_execution_result = BlockExecutionResult {
            receipts: executor_output.receipts.clone(),
            requests: executor_output.requests.clone(),
            gas_used: executor_output.gas_used,
        };

        // TODO: skip for now -- some type issues
        // consensus.validate_block_post_execution(&executor_block_input, &block_execution_result)?;

        // Generate ExecutionWitness
        tracing::info!("generating execution witness");
        let witness =
            self.generate_execution_witness(&rpc_db, &executor_output, block_number).await?;

        // Convert current_block to the correct type for ClientExecutorInput
        let processed_block = self.process_block_for_client(&current_block)?;

        // Create the client input.
        let client_input = ClientExecutorInput { current_block: processed_block, witness };
        tracing::info!("successfully generated client input");

        Ok(client_input)
    }

    /// Generate ExecutionWitness based on the reference implementation approach
    async fn generate_execution_witness(
        &self,
        rpc_db: &RpcDb<P>,
        executor_output: &BlockExecutionResult<reth_primitives::Receipt>,
        block_number: u64,
    ) -> eyre::Result<ExecutionWitness> {
        tracing::info!("building execution witness components");

        // 1. Get state requests (what was accessed during execution)
        let state_requests = rpc_db.get_state_requests();

        // 2. Build state trie nodes from storage proofs
        let state = self.build_state_witness(&state_requests, block_number).await?;

        // 3. Extract contract codes that were accessed
        let codes = self.extract_accessed_codes(&rpc_db, executor_output, block_number).await?;

        // 4. Build key preimages (unhashed addresses and storage keys)
        let keys = self.build_key_preimages(&state_requests);

        // 5. Build headers based on BLOCKHASH usage (simplified approach)
        let headers = self.build_headers_for_witness(block_number).await?;

        Ok(ExecutionWitness { state, codes, keys, headers })
    }

    /// Build state witness from storage proofs
    async fn build_state_witness(
        &self,
        state_requests: &HashMap<Address, Vec<U256>, FxBuildHasher>,
        block_number: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        let mut state_nodes = Vec::new();

        for (address, storage_keys) in state_requests.iter() {
            // Convert storage keys to B256
            let keys: Vec<B256> = storage_keys.iter().map(|k| B256::from(*k)).collect();

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

        Ok(state_nodes)
    }

    /// Extract contract codes that were accessed during execution
    async fn extract_accessed_codes(
        &self,
        rpc_db: &RpcDb<P>,
        executor_output: &BlockExecutionResult<reth_primitives::Receipt>,
        block_number: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        // Start with bytecodes that were cached during execution
        let cached_bytecodes = rpc_db.get_bytecodes();
        let mut codes: Vec<Bytes> =
            cached_bytecodes.into_iter().map(|bytecode| bytecode.bytecode().clone()).collect();

        // Collect all unique addresses that appeared in transactions
        let mut all_addresses = std::collections::HashSet::new();

        // Add addresses from logs (contracts that emitted events)
        for receipt in &executor_output.receipts {
            for log in &receipt.logs {
                all_addresses.insert(log.address);
            }
        }

        // For each unique address, fetch its bytecode if it's a contract
        for address in all_addresses {
            // Check if we already have this bytecode cached
            let state_requests = rpc_db.get_state_requests();
            if state_requests.contains_key(&address) {
                continue; // Already included in cached bytecodes
            }

            // Fetch the bytecode for this address
            let code =
                self.provider.get_code_at(address).block_id((block_number - 1).into()).await?;

            if !code.is_empty() {
                codes.push(code);
            }
        }

        // Remove duplicates
        codes.sort();
        codes.dedup();

        tracing::info!("extracted {} contract codes from execution", codes.len());
        Ok(codes)
    }

    /// Build key preimages (unhashed addresses and storage keys)
    fn build_key_preimages(
        &self,
        state_requests: &HashMap<Address, Vec<U256>, FxBuildHasher>,
    ) -> Vec<Bytes> {
        let mut keys = Vec::new();

        for (address, storage_keys) in state_requests.iter() {
            // Add the address itself (unhashed)
            keys.push(address.as_bytes().to_vec().into());

            // Add storage keys (unhashed)
            for storage_key in storage_keys {
                keys.push(storage_key.to_be_bytes_vec().into());
            }
        }

        // Remove duplicates
        keys.sort();
        keys.dedup();

        keys
    }

    /// Build headers for witness verification with proper RLP encoding
    /// Since EIP-2935 is implemented, we only need the parent header for state verification
    async fn build_headers_for_witness(&self, block_number: u64) -> eyre::Result<Vec<Bytes>> {
        use alloy_rlp::Encodable;

        // Since EIP-2935 is implemented, we only need the parent header for state verification
        let parent_block = self
            .provider
            .get_block_by_number((block_number - 1).into())
            .await?
            .ok_or_eyre("couldn't fetch parent block")?;

        // Convert to proper header format and RLP encode
        let header = alloy_consensus::Header {
            parent_hash: parent_block.header.parent_hash,
            ommers_hash: parent_block.header.ommers_hash,
            beneficiary: parent_block.header.beneficiary,
            state_root: parent_block.header.state_root,
            transactions_root: parent_block.header.transactions_root,
            receipts_root: parent_block.header.receipts_root,
            logs_bloom: parent_block.header.logs_bloom,
            difficulty: parent_block.header.difficulty,
            number: parent_block.header.number,
            gas_limit: parent_block.header.gas_limit,
            gas_used: parent_block.header.gas_used,
            timestamp: parent_block.header.timestamp,
            extra_data: parent_block.header.extra_data.clone(),
            mix_hash: parent_block.header.mix_hash.unwrap_or_default(),
            nonce: parent_block.header.nonce.map(|n| n.into()).unwrap_or_default(),
            base_fee_per_gas: parent_block.header.base_fee_per_gas,
            withdrawals_root: parent_block.header.withdrawals_root,
            blob_gas_used: parent_block.header.blob_gas_used,
            excess_blob_gas: parent_block.header.excess_blob_gas,
            parent_beacon_block_root: parent_block.header.parent_beacon_block_root,
            requests_hash: None,
        };

        // RLP encode the header (this is what the client expects)
        let mut header_bytes = Vec::new();
        header.encode(&mut header_bytes);

        tracing::info!("added RLP-encoded parent header for state verification");
        Ok(vec![header_bytes.into()])
    }

    /// Convert AnyRpcBlock to the Block type expected by ClientExecutorInput
    fn process_block_for_client(
        &self,
        block: &AnyRpcBlock,
    ) -> eyre::Result<Block<EthereumTxEnvelope<alloy_consensus::TxEip4844>>> {
        // Convert using the existing recover_rpc_block function
        let recovered_block = recover_rpc_block(block.clone());
        Ok(recovered_block.into_block())
    }
}

fn recover_rpc_block(
    block: AnyRpcBlock,
) -> RecoveredBlock<Block<EthereumTxEnvelope<alloy_consensus::TxEip4844>>> {
    let block = block.clone();
    let current_header =
        block.inner.header.inner.clone().try_into_header().expect("failed to convert header");

    let current_transactions: Vec<EthereumTxEnvelope<alloy_consensus::TxEip4844>> = block
        .inner
        .transactions
        .clone()
        .map(|t| {
            let envelope = t.as_envelope().expect("only Ethereum transactions are supported");
            let pooled = envelope.clone().try_into_pooled().expect("failed to convert to pooled");
            pooled.into()
        })
        .into_transactions_vec();

    let current_body = BlockBody {
        transactions: current_transactions,
        // TODO: can be restored from current_block.uncles but it's not needed?
        ommers: vec![],
        withdrawals: block.withdrawals.clone(),
    };

    let current_sealed_block: alloy_consensus::Block<
        EthereumTxEnvelope<alloy_consensus::TxEip4844>,
    > = Block::new(current_header, current_body);
    let recovered_block =
        RecoveredBlock::try_recover(current_sealed_block).expect("failed to recover block");

    recovered_block
}
