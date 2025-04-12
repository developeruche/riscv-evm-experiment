use std::collections::HashMap;

use revm::{
    Context as EthContext,
    context::transaction::{AccessList, SignedAuthorization},
    primitives::{Address, B256, Bytes, TxKind, U256},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct BlockEnv {
    pub number: u64,
    pub beneficiary: Address,
    pub timestamp: u64,
    pub gas_limit: u64,
    pub basefee: u64,
    pub difficulty: U256,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TxEnv {
    pub tx_type: u8,
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: u128,
    pub kind: TxKind,
    pub value: U256,
    pub data: Bytes,
    pub nonce: u64,
    pub chain_id: Option<u64>,
    pub access_list: AccessList,
    pub gas_priority_fee: Option<u128>,
    pub blob_hashes: Vec<B256>,
    pub max_fee_per_blob_gas: u128,
    pub authorization_list: Vec<SignedAuthorization>,
}

pub type StorageType = [u8; 32];

#[derive(Debug, Clone, Default)]
pub struct Storage {
    pub mapping: HashMap<StorageType, StorageType>,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub eth_context: EthContext,

    // frame related cotext
    pub address: Address,        // address(this)
    pub current_caller: Address, // msg.sender
}

impl Context {
    pub fn new(eth_context: EthContext) -> Self {
        Self {
            eth_context,
            address: Default::default(),
            current_caller: Default::default(),
        }
    }
}
