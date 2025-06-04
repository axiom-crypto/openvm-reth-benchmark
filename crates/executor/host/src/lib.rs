use alloy_consensus::{BlockBody, EthereumTxEnvelope};
use alloy_primitives::Bytes;
use alloy_provider::{
    ext::DebugApi,
    network::{AnyNetwork, AnyRpcBlock, BlockResponse, TransactionResponse},
    Provider,
};
use alloy_rpc_types::trace::geth::{
    CallFrame, GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
    GethDefaultTracingOptions,
};
use alloy_rpc_types_debug::ExecutionWitness;
use eyre::{eyre, OptionExt};
use openvm_client_executor::io::StatelessInput;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use openvm_rpc_db::RpcDb;
use reth_chainspec::MAINNET;
use reth_evm::{execute::Executor, ConfigureEvm};
use reth_evm_ethereum::execute::EthExecutorProvider;
use reth_primitives::{Block, RecoveredBlock};
use reth_revm::witness::ExecutionWitnessRecord;

use revm::database::CacheDB;
use revm_primitives::hash_map::HashMap;
use revm_primitives::Address;
use revm_primitives::B256;
use revm_primitives::U256;
use rustc_hash::FxBuildHasher;

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
    pub async fn execute(&self, block_number: u64) -> eyre::Result<StatelessInput> {
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
        let _previous_block = self
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

        // Execute with witness recording (like reth's implementation)
        let mut witness_record = ExecutionWitnessRecord::default();
        let executor_output = EthExecutorProvider::new(spec.clone())
            .executor(cache_db)
            .execute_with_state_closure(&executor_block_input, |statedb| {
                witness_record.record_executed_state(statedb);
            })?;

        // Generate ExecutionWitness using reth's approach
        tracing::info!("generating execution witness using reth's approach");
        let witness =
            self.generate_execution_witness(&rpc_db, block_number, witness_record).await?;

        // Convert current_block to the correct type for ClientExecutorInput
        let processed_block = self.process_block_for_client(&current_block)?;

        // Create the client input.
        let client_input = StatelessInput { block: processed_block, witness };
        tracing::info!("successfully generated client input");

        Ok(client_input)
    }

    /// Generate ExecutionWitness using reth's ExecutionWitnessRecord approach
    async fn generate_execution_witness(
        &self,
        rpc_db: &RpcDb<P>,
        block_number: u64,
        witness_record: ExecutionWitnessRecord,
    ) -> eyre::Result<ExecutionWitness> {
        tracing::info!("building execution witness using reth's ExecutionWitnessRecord");

        // Extract recorded data from ExecutionWitnessRecord (exactly like reth does)
        let ExecutionWitnessRecord { hashed_state, codes, keys, lowest_block_number } =
            witness_record;

        tracing::info!(
            "extracted from witness record: {} codes, {} keys, lowest_block: {:?}",
            codes.len(),
            keys.len(),
            lowest_block_number
        );

        // Convert reth's witness data to the format expected by ExecutionWitness
        let keys_bytes: Vec<Bytes> = keys.into_iter().map(|key| key.to_vec().into()).collect();

        let codes_bytes: Vec<Bytes> = if codes.is_empty() {
            // Fallback: ExecutionWitnessRecord didn't capture codes, use our proven approach
            tracing::warn!(
                "ExecutionWitnessRecord returned 0 codes, falling back to RPC collection"
            );
            self.extract_accessed_codes(&rpc_db, block_number).await?
        } else {
            codes
        };

        // Build state trie nodes using RPC fallback (since no database transaction available)
        let state =
            self.build_state_witness_from_hashed_state(&hashed_state, block_number, rpc_db).await?;

        // Build headers based on lowest_block_number (exactly like reth does)
        let headers =
            self.build_headers_for_witness_range(block_number, lowest_block_number).await?;

        tracing::info!(
            "built witness: {} state nodes, {} codes, {} keys, {} headers",
            state.len(),
            codes_bytes.len(),
            keys_bytes.len(),
            headers.len()
        );

        Ok(ExecutionWitness { state, codes: codes_bytes, keys: keys_bytes, headers })
    }

    /// Build state witness from hashed state (RPC fallback approach)
    async fn build_state_witness_from_hashed_state(
        &self,
        _hashed_state: &reth_trie_common::HashedPostState,
        block_number: u64,
        rpc_db: &RpcDb<P>,
    ) -> eyre::Result<Vec<Bytes>> {
        // Since we don't have database access for DatabaseTrieWitness::overlay_witness,
        // fall back to RPC-based state witness generation using the state that was tracked
        tracing::info!(
            "using RPC-based state witness generation (no database transaction available)"
        );

        // Use the RPC db's tracked state requests for witness generation
        // This ensures we get the same state that was actually accessed during execution
        let state_requests = rpc_db.get_state_requests();

        if state_requests.is_empty() {
            tracing::warn!("no RPC state requests found - using empty state witness");
            return Ok(Vec::new());
        }

        // Generate witness using our proven RPC approach
        self.build_state_witness(&state_requests, block_number).await
    }

    /// Build state witness from storage proofs (parallelized for speed) - RPC fallback
    async fn build_state_witness(
        &self,
        state_requests: &HashMap<Address, Vec<U256>, FxBuildHasher>,
        block_number: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        use futures::future::join_all;

        let mut state_nodes = Vec::new();

        // Parallelize proof requests for better performance
        let proof_futures: Vec<_> = state_requests
            .iter()
            .map(|(address, storage_keys)| {
                let provider = self.provider.clone();
                let address = *address;
                let keys: Vec<B256> = storage_keys.iter().map(|k| B256::from(*k)).collect();

                async move {
                    let proof = provider
                        .get_proof(address, keys)
                        .block_id((block_number - 1).into())
                        .await?;
                    Ok::<_, eyre::Error>(eip1186_proof_to_account_proof(proof))
                }
            })
            .collect();

        let account_proofs = join_all(proof_futures).await;

        for proof_result in account_proofs {
            let account_proof = proof_result?;

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

        tracing::info!("built state witness with {} trie nodes (parallelized)", state_nodes.len());
        Ok(state_nodes)
    }

    /// Extract contract codes that were accessed during execution (parallelized)
    async fn extract_accessed_codes(
        &self,
        rpc_db: &RpcDb<P>,
        block_number: u64,
    ) -> eyre::Result<Vec<Bytes>> {
        use futures::future::join_all;

        // Start with bytecodes that were cached during execution
        let cached_bytecodes = rpc_db.get_bytecodes();
        let mut codes: Vec<Bytes> =
            cached_bytecodes.into_iter().map(|bytecode| bytecode.bytecode().clone()).collect();

        // Get ALL addresses that were accessed during block execution using debug_traceTransaction
        let all_addresses = self.get_all_accessed_addresses(block_number).await?;

        tracing::info!(
            "found {} unique addresses accessed during block execution",
            all_addresses.len()
        );

        // Parallelize bytecode fetching for much better performance
        let code_futures: Vec<_> = all_addresses
            .into_iter()
            .map(|address| {
                let provider = self.provider.clone();
                async move {
                    let code =
                        provider.get_code_at(address).block_id((block_number - 1).into()).await?;
                    Ok::<_, eyre::Error>((address, code))
                }
            })
            .collect();

        let code_results = join_all(code_futures).await;

        // Collect all non-empty bytecodes
        for result in code_results {
            let (address, code) = result?;
            if !code.is_empty() {
                let code_len = code.len();
                codes.push(code);
                tracing::debug!("added bytecode for address {address}: {} bytes", code_len);
            }
        }

        // Remove duplicates
        codes.sort();
        codes.dedup();

        tracing::info!("extracted {} contract codes from execution", codes.len());
        Ok(codes)
    }

    /// Get ALL addresses that were accessed during block execution using comprehensive collection
    async fn get_all_accessed_addresses(
        &self,
        block_number: u64,
    ) -> eyre::Result<std::collections::HashSet<Address>> {
        use std::collections::HashSet;

        let mut all_addresses = HashSet::new();

        // Get the current block to extract all transactions
        let current_block = self
            .provider
            .get_block_by_number(block_number.into())
            .full()
            .await?
            .ok_or_eyre("couldn't fetch current block")?;

        let txs = current_block.transactions().clone().into_transactions();

        for tx in txs {
            let tx_hash = tx.tx_hash();

            // Trace with built-in call tracer.
            let call_options = GethDebugTracingOptions {
                config: GethDefaultTracingOptions {
                    disable_storage: Some(true),
                    enable_memory: Some(false),
                    ..Default::default()
                },
                tracer: Some(GethDebugTracerType::BuiltInTracer(
                    GethDebugBuiltInTracerType::CallTracer,
                )),
                ..Default::default()
            };
            let result = self.provider.debug_trace_transaction(tx_hash, call_options).await?;

            // Extract the CallFrame from the GethTrace
            if let Ok(call_frame) = result.try_into_call_frame() {
                self.extract_addresses_from_call_frame(&call_frame, &mut all_addresses);
            } else {
                tracing::warn!("unexpected trace result for transaction {}", tx_hash);
            }
        }

        tracing::info!(
            "collected {} unique addresses from comprehensive collection",
            all_addresses.len()
        );
        Ok(all_addresses)
    }

    /// Extract all addresses from a call frame recursively
    fn extract_addresses_from_call_frame(
        &self,
        call_frame: &CallFrame,
        addresses: &mut std::collections::HashSet<Address>,
    ) {
        // Extract 'to' and 'from' addresses
        if let Some(to_addr) = call_frame.to {
            addresses.insert(to_addr);
        }

        addresses.insert(call_frame.from);

        // Extract addresses from nested calls (recursive)
        for call in &call_frame.calls {
            self.extract_addresses_from_call_frame(call, addresses);
        }

        // Extract addresses from input data (look for 20-byte sequences that might be addresses)
        if !call_frame.input.is_empty() {
            let input_bytes = call_frame.input.as_ref();
            // Look for potential addresses in the input data (20-byte aligned sequences)
            for i in (0..input_bytes.len().saturating_sub(19)).step_by(1) {
                if i + 20 <= input_bytes.len() {
                    let potential_addr_bytes: [u8; 20] = input_bytes[i..i + 20].try_into().unwrap();
                    let potential_addr = Address::from(potential_addr_bytes);

                    // Only add if it looks like a real address (not all zeros, not all 0xff)
                    if !potential_addr.is_zero() && potential_addr != Address::from([0xff; 20]) {
                        addresses.insert(potential_addr);
                    }
                }
            }
        }

        // Extract addresses from log data
        for log in &call_frame.logs {
            if let Some(log_address) = log.address {
                addresses.insert(log_address);
            }

            // Extract addresses from log topics
            if let Some(topics) = &log.topics {
                for topic in topics {
                    // The last 20 bytes of a topic might be an address
                    if topic.len() >= 20 {
                        let addr_bytes: [u8; 20] = topic[12..32].try_into().unwrap_or_default();
                        let potential_addr = Address::from(addr_bytes);
                        if !potential_addr.is_zero() {
                            addresses.insert(potential_addr);
                        }
                    }
                }
            }
        }
    }

    /// Build headers for witness verification based on lowest_block_number (like reth)
    async fn build_headers_for_witness_range(
        &self,
        block_number: u64,
        lowest_block_number: Option<u64>,
    ) -> eyre::Result<Vec<Bytes>> {
        use alloy_rlp::Encodable;

        let smallest = match lowest_block_number {
            Some(smallest) => smallest,
            None => {
                // Return only the parent header, if there were no calls to the
                // BLOCKHASH opcode.
                block_number.saturating_sub(1)
            }
        };

        let range = smallest..block_number;
        tracing::info!("building headers for range: {}..{}", smallest, block_number);

        let mut headers = Vec::new();
        for block_num in range {
            let block = self
                .provider
                .get_block_by_number(block_num.into())
                .await?
                .ok_or_eyre("couldn't fetch block in header range")?;

            // Convert to proper header format and RLP encode (like reth does)
            let header = alloy_consensus::Header {
                parent_hash: block.header.parent_hash,
                ommers_hash: block.header.ommers_hash,
                beneficiary: block.header.beneficiary,
                state_root: block.header.state_root,
                transactions_root: block.header.transactions_root,
                receipts_root: block.header.receipts_root,
                logs_bloom: block.header.logs_bloom,
                difficulty: block.header.difficulty,
                number: block.header.number,
                gas_limit: block.header.gas_limit,
                gas_used: block.header.gas_used,
                timestamp: block.header.timestamp,
                extra_data: block.header.extra_data.clone(),
                mix_hash: block.header.mix_hash.unwrap_or_default(),
                nonce: block.header.nonce.map(|n| n.into()).unwrap_or_default(),
                base_fee_per_gas: block.header.base_fee_per_gas,
                withdrawals_root: block.header.withdrawals_root,
                blob_gas_used: block.header.blob_gas_used,
                excess_blob_gas: block.header.excess_blob_gas,
                parent_beacon_block_root: block.header.parent_beacon_block_root,
                requests_hash: None,
            };

            // RLP encode the header (like reth does)
            let mut header_bytes = Vec::new();
            header.encode(&mut header_bytes);
            headers.push(header_bytes.into());
        }

        tracing::info!("added {} RLP-encoded headers for witness verification", headers.len());
        Ok(headers)
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
) -> RecoveredBlock<Block<EthereumTxEnvelope<alloy_consensus::TxEip4844Variant>>> {
    let block = block.clone();
    let current_header =
        block.inner.header.inner.clone().try_into_header().expect("failed to convert header");

    let current_transactions = block
        .inner
        .transactions
        .clone()
        .map(|t| {
            let envelope = t.as_envelope().expect("only Ethereum transactions are supported");

            // Handle different transaction types without converting to pooled
            // EIP-4844 transactions don't have blob sidecar in RPC responses
            match envelope {
                EthereumTxEnvelope::Eip4844(signed_tx) => {
                    // For EIP-4844 transactions, we need to create a transaction without sidecar
                    // since RPC responses don't include the blob sidecar data
                    let tx_4844 = signed_tx.tx().clone();
                    let signature = signed_tx.signature().clone();

                    // Create the signed transaction
                    let signed = alloy_consensus::Signed::new_unchecked(
                        tx_4844,
                        signature,
                        *signed_tx.hash(),
                    );
                    EthereumTxEnvelope::Eip4844(signed)
                }
                _ => {
                    // For non-EIP-4844 transactions, try converting to pooled first
                    match envelope.clone().try_into_pooled() {
                        Ok(pooled) => pooled.into(),
                        Err(_) => envelope.clone(), // Fallback to original envelope if pooled conversion fails
                    }
                }
            }
        })
        .into_transactions_vec();

    let current_body = BlockBody {
        transactions: current_transactions,
        // TODO: can be restored from current_block.uncles but it's not needed?
        ommers: vec![],
        withdrawals: block.withdrawals.clone(),
    };

    let current_sealed_block = Block::new(current_header, current_body);
    let recovered_block =
        RecoveredBlock::try_recover(current_sealed_block).expect("failed to recover block");

    recovered_block
}
