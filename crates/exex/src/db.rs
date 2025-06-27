// In crates/host-bench/src/reth_executor.rs

use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use alloy_consensus::{TxEnvelope, TxReceipt};
use alloy_primitives::Bloom;
use openvm_client_executor::io::ClientExecutorInput;
use openvm_mpt::EthereumState;
use openvm_primitives::account_proof::eip1186_proof_to_account_proof;
use reth_chainspec::MAINNET;
use reth_consensus::{Consensus, HeaderValidator};
use reth_db::test_utils::create_test_rw_db;
use reth_ethereum_consensus::{validate_block_post_execution, EthBeaconConsensus};
use reth_evm::execute::{BasicBlockExecutor, Executor};
use reth_evm_ethereum::EthEvmConfig;
use reth_execution_types::ExecutionOutcome;
use reth_primitives::{Account, Block, Header};
use reth_primitives_traits::block::Block as _;
use reth_provider::{
    providers::StaticFileProvider, ProviderError, ProviderFactory, StateProvider,
    StateProviderFactory,
};
use revm::{primitives::HashMap, state::AccountInfo, DatabaseRef};
use revm_primitives::{b256, Address, B256, U256};

use alloy_provider::{network::Ethereum, Provider};
use alloy_rpc_types::BlockId;
use reth_revm::{db::CacheDB, state::Bytecode};
use reth_storage_errors::db::DatabaseError;

/// A [DatabaseRef] that reads from a [StateProvider] and records all accesses.
#[derive(Debug, Clone)]
pub struct RethDb<SP> {
    provider: SP,
    accounts: RefCell<HashMap<Address, AccountInfo>>,
    storage: RefCell<HashMap<Address, HashMap<U256, U256>>>,
    block_hashes: RefCell<HashMap<u64, B256>>,
    initial_block_number: u64,
}

impl<SP: StateProvider> RethDb<SP> {
    pub fn new(provider: SP, initial_block_number: u64) -> Self {
        Self {
            provider,
            accounts: RefCell::new(HashMap::default()),
            storage: RefCell::new(HashMap::default()),
            block_hashes: RefCell::new(HashMap::default()),
            initial_block_number,
        }
    }

    pub fn into_requests(self) -> (HashMap<Address, Vec<U256>>, Vec<Bytecode>, u64) {
        let accounts = self.accounts.borrow();
        let storage = self.storage.borrow();

        let state_requests = accounts
            .keys()
            .chain(storage.keys())
            .map(|&address| {
                let storage_keys_for_address: BTreeSet<U256> = storage
                    .get(&address)
                    .map(|storage_map| storage_map.keys().cloned().collect())
                    .unwrap_or_default();
                (address, storage_keys_for_address.into_iter().collect())
            })
            .collect();

        let bytecodes = accounts
            .values()
            .filter_map(|account| account.code.clone())
            .map(|code| (code.hash_slow(), code))
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect::<Vec<_>>();

        let oldest_ancestor =
            self.block_hashes.borrow().keys().min().cloned().unwrap_or(self.initial_block_number);

        (state_requests, bytecodes, oldest_ancestor)
    }
}

impl<SP: StateProvider> DatabaseRef for RethDb<SP> {
    type Error = ProviderError;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        if let Some(acc) = self.accounts.borrow().get(&address) {
            return Ok(Some(acc.clone()));
        }

        let acc = self.provider.basic_account(&address)?;
        let info = acc.map(|acc: Account| AccountInfo {
            balance: acc.balance,
            nonce: acc.nonce,
            code_hash: acc.bytecode_hash.unwrap_or(b256!(
                "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
            )),
            code: self.code_by_hash_ref(acc.bytecode_hash.unwrap()).ok(),
        });

        if let Some(info) = &info {
            self.accounts.borrow_mut().insert(address, info.clone());
        }

        Ok(info)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let bytecode = self.provider.bytecode_by_hash(&code_hash)?.unwrap_or_default();
        Ok(bytecode.0)
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        if let Some(val) = self.storage.borrow().get(&address).and_then(|s| s.get(&index)) {
            return Ok(*val);
        }
        let value = self.provider.storage(address, index.into())?.unwrap_or_default();
        self.storage.borrow_mut().entry(address).or_default().insert(index, value);
        Ok(value)
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        if let Some(hash) = self.block_hashes.borrow().get(&number) {
            return Ok(*hash);
        }
        let hash = self.provider.block_hash(number)?.unwrap_or_default();
        self.block_hashes.borrow_mut().insert(number, hash);
        Ok(hash)
    }
}

pub struct RethHostExecutor<P: Provider<Ethereum>> {
    provider: P,
}

impl<P: Provider<Ethereum>> RethHostExecutor<P> {
    pub fn new(provider: P) -> eyre::Result<Self> {
        Ok(Self { provider })
    }

    pub fn execute(&self, block_number: u64) -> eyre::Result<ClientExecutorInput> {
        let provider = self.provider;
        let spec = MAINNET.clone();

        let current_block: Block = provider.get_block_by_number(block_number.into());

        let previous_block = provider.get_block_by_number(block_number - 1).seal_slow();

        let parent_provider = self.factory.provider_at_or_latest(block_number - 1)?;
        let reth_db = RethDb::new(parent_provider, block_number - 1);
        let mut cache_db = CacheDB::new(reth_db);

        let block = current_block.clone().try_into_recovered()?;
        let consensus = EthBeaconConsensus::new(spec.clone());
        consensus.validate_header(block.sealed_header())?;
        consensus.validate_block_pre_execution(&block)?;

        let block_executor =
            BasicBlockExecutor::new(EthEvmConfig::new(spec.clone()), &mut cache_db);
        let executor_output = block_executor.execute(&block)?;
        validate_block_post_execution(
            &block,
            &spec,
            &executor_output.receipts,
            &executor_output.requests,
        )?;

        let (state_requests, bytecodes, oldest_ancestor) = cache_db.db.into_requests();

        let parent_provider = self.provider(block_number - 1)?;
        let mut before_storage_proofs = Vec::new();
        for (address, used_keys) in state_requests.iter() {
            let modified_keys = executor_output
                .state
                .get(address)
                .map(|account| {
                    account.storage.keys().map(|key| B256::from(*key)).collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();
            let keys = used_keys
                .iter()
                .map(|key| B256::from(*key))
                .chain(modified_keys.into_iter())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let storage_proof = parent_provider.get_proof(*address, keys)?;
            before_storage_proofs.push(eip1186_proof_to_account_proof(storage_proof));
        }

        let current_provider = self.factory.provider_at_or_latest(block_number)?;
        let mut after_storage_proofs = Vec::new();
        for (address, _) in state_requests.iter() {
            let modified_keys = executor_output
                .state
                .get(address)
                .map(|account| {
                    account.storage.keys().map(|key| B256::from(*key)).collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
                .into_iter()
                .collect::<Vec<_>>();
            let storage_proof = current_provider.get_proof(*address, modified_keys)?;
            after_storage_proofs.push(eip1186_proof_to_account_proof(storage_proof));
        }

        let parent_state = EthereumState::from_transition_proofs(
            previous_block.state_root,
            &before_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
            &after_storage_proofs.iter().map(|item| (item.address, item.clone())).collect(),
        )?;

        let mut ancestor_headers = vec![];
        for height in (oldest_ancestor..=(block_number - 1)).rev() {
            ancestor_headers.push(provider.header_by_number(height)?.unwrap());
        }

        let client_input = ClientExecutorInput {
            current_block,
            ancestor_headers,
            parent_state,
            state_requests,
            bytecodes,
        };

        Ok(client_input)
    }
}
