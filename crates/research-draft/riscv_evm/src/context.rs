use std::collections::HashMap;

use revm::{
    Context as RevmEthContext,
    context::{BlockEnv, CfgEnv, TxEnv},
    database::{CacheDB, EmptyDB},
    primitives::{Address, Bytes},
};

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
