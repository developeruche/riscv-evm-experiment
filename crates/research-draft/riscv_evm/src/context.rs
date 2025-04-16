use std::collections::HashMap;

use revm::{
    Context as RevmEthContext,
    context::{
        BlockEnv, CfgEnv, TxEnv,
        transaction::{AccessList, SignedAuthorization},
    },
    database::{CacheDB, EmptyDB},
    primitives::{Address, B256, Bytes, TxKind, U256},
};

// #[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
// pub struct BlockEnv {
//     pub number: u64,
//     pub beneficiary: Address,
//     pub timestamp: u64,
//     pub gas_limit: u64,
//     pub basefee: u64,
//     pub difficulty: U256,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Default)]
// pub struct TxEnv {
//     pub tx_type: u8,
//     pub caller: Address,
//     pub gas_limit: u64,
//     pub gas_price: u128,
//     pub kind: TxKind,
//     pub value: U256,
//     pub data: Bytes,
//     pub nonce: u64,
//     pub chain_id: Option<u64>,
//     pub access_list: AccessList,
//     pub gas_priority_fee: Option<u128>,
//     pub blob_hashes: Vec<B256>,
//     pub max_fee_per_blob_gas: u128,
//     pub authorization_list: Vec<SignedAuthorization>,
// }

pub type StorageType = [u8; 32];
pub type EthContext = RevmEthContext<BlockEnv, TxEnv, CfgEnv, CacheDB<EmptyDB>>;

#[derive(Debug, Clone, Default)]
pub struct Storage {
    pub mapping: HashMap<StorageType, StorageType>,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub eth_context: EthContext,

    // frame related cotext (This is frame data would just be one for now, meaning only one extra frame can be spun bounded to this context, this can be increased by making the data a vec of reame related data, this would be done late, I need the reasearch done yesterday plus the benchmark test is an erc20 contract which is just one frame deep, this can handle two)
    pub address: Address,        // address(this)
    pub current_caller: Address, // msg.sender
    pub return_data: Bytes,
}

impl Context {
    pub fn new(eth_context: EthContext) -> Self {
        Self {
            eth_context,
            address: Default::default(),
            current_caller: Default::default(),
            return_data: Default::default(),
        }
    }
}
