use reth_chainspec::{
    BaseFeeParams, BaseFeeParamsKind, Chain, ChainHardforks, ChainSpec, DepositContract,
    EthereumHardfork, ForkCondition, Hardfork,
};
use revm_primitives::{address, b256, U256};

/// Returns the [ChainSpec] for Ethereum mainnet.
/// It is much faster than using reth's MAINNET.clone()
pub fn mainnet() -> ChainSpec {
    // Spec extracted from:
    //
    // https://github.com/paradigmxyz/reth/blob/c228fe15808c3acbf18dc3af1a03ef5cbdcda07a/crates/chainspec/src/spec.rs#L35-L60
    let mut spec = ChainSpec {
        chain: Chain::mainnet(),
        // We don't need the genesis state. Using default to save cycles.
        genesis: Default::default(),
        //genesis_hash: Some(MAINNET_GENESIS_HASH),
        paris_block_and_final_difficulty: Some((0, U256::ZERO)),
        // For some reasons a state root mismatch error arises if we don't force activate everything
        // before and including Shanghai.
        hardforks: ChainHardforks::new(vec![
            (EthereumHardfork::Frontier.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Homestead.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Dao.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Tangerine.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::SpuriousDragon.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Byzantium.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Constantinople.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Petersburg.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Istanbul.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::MuirGlacier.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::Berlin.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::London.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::ArrowGlacier.boxed(), ForkCondition::Block(0)),
            (EthereumHardfork::GrayGlacier.boxed(), ForkCondition::Block(0)),
            (
                EthereumHardfork::Paris.boxed(),
                ForkCondition::TTD {
                    fork_block: Some(0),
                    total_difficulty: U256::ZERO,
                    activation_block_number: 0,
                },
            ),
            (EthereumHardfork::Shanghai.boxed(), ForkCondition::Timestamp(0)),
            (EthereumHardfork::Cancun.boxed(), ForkCondition::Timestamp(1710338135)),
        ]),
        deposit_contract: Some(DepositContract::new(
            address!("00000000219ab540356cbb839cbe05303d7705fa"),
            11052984,
            b256!("649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
        )),
        base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        //max_gas_limit: ETHEREUM_BLOCK_GAS_LIMIT,
        prune_delete_limit: 20000,
        genesis_header: Default::default(),
        blob_params: Default::default(),
    };
    spec.genesis.config.dao_fork_support = true;
    spec
}
