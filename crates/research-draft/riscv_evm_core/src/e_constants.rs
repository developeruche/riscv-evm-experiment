pub enum RiscvEVMECalls {
    /// [offset, size] -> hash
    Keccak256,
    /// Address of the current excecuting contract |-> address
    Address,
    /// Native balance of the current caller [address] -> balance
    Balance,
    /// Address of the transaction origin |-> address
    Origin,
    /// Address of the current calling address |-> address
    Caller,
    /// Deposit value for this Tx |-> value
    CallValue,
    /// Load a Word(256bits in this case) from the calldata to the stack
    /// [i] -> data[i]
    CallDataLoad,
    /// Returns  the size of the calldata |-> usize
    CallDataSize,
    /// Copy calldata from input to memory [destOffset, offset, size]
    CallDataCopy,
    /// Returns the size of the code |-> usize
    CodeSize,
    /// Copy code from input to memory [destOffset, offset, size]
    CodeCopy,
    /// Gas price now
    GasPrice,
    /// Get the size of an External account's code [address] -> usize
    ExtCodeSize,
    /// Get the code of an External account's code [address, destOffset, offset, size]
    ExtCodeCopy,
    /// Get size of output data from the previous call from the current environment
    ReturnDataSize,
    /// Copy output data from the previous call to memory [destOffset, offset, size]
    ReturnDataCopy,
    /// Get hash of an account's code [address] -> hash
    ExtCodeHash,
    /// Get the hash of one of the 256 most recent complete blocks [blockNumber] -> hash
    BlockHash,
    /// Get the block's beneficiary address |-> address
    Coinbase,
    /// Get the block's timestamp |-> timestamp
    Timestamp,
    /// Get the block's number |-> blockNumber
    Number,
    /// Get the block's difficulty |-> difficulty
    PrevRandao,
    /// Get the block's gas limit |-> gasLimit
    GasLimit,
    /// Get the chain ID |-> chainId
    ChainId,
    /// Get balance of currently executing account |-> balance
    SelfBalance,
    /// Get the base fee |-> baseFee
    BaseFee,
    /// Get versioned hashes [index] -> blobVersionedHashesAtIndex
    BlobHash,
    /// Returns the value of the blob base-fee of the current block |-> blobBaseFee
    BlobBaseFee,
    /// Amount of valiable gas
    Gas,
    /// Append log record with no topics [offset, size]
    Log0,
    /// Append log record with one topic [offset, size, topic]
    Log1,
    /// Append log record with two topics [offset, size, topic1, topic2]
    Log2,
    /// Append log record with three topics [offset, size, topic1, topic2, topic3]
    Log3,
    /// Append log record with four topics [offset, size, topic1, topic2, topic3, topic4]
    Log4,
    /// Create a new account with associated code [value, offset, size] -> address
    Create,
    /// Message-call into an account [gas, address, value, argsOffset, argsSize, retOffset, retSize] -> success
    Call,
    /// Message-call into this account with alternative account's code [gas, address, value, argsOffset, argsSize, retOffset, retSize] -> success
    CallCode,
    /// Halt execution returning output data [offset, size]
    Return,
    /// Message-call into this account with an alternative account's code, but persisting the current values for sender and value [gas, address, argsOffset, argsSize, retOffset, retSize] -> success
    DelegateCall,
    /// Create a new account with associated code at a predictable address [value, offset, size, salt] -> address
    Create2,
    /// Static message-call into an account [gas, address, argsOffset, argsSize, retOffset, retSize] -> success
    StaticCall,
    /// Halt execution reverting state changes but returning data and remaining gas [offset, size]
    Revert,
    /// Loads a word (32-bytes) from storage
    SLoad,
    /// Stores a word (32-bytes) from storage
    SStore,
}

impl RiscvEVMECalls {
    pub fn from_u32(ecode: u32) -> Option<Self> {
        match ecode {
            0x20 => Some(Self::Keccak256),
            0x30 => Some(Self::Address),
            0x31 => Some(Self::Balance),
            0x32 => Some(Self::Origin),
            0x33 => Some(Self::Caller),
            0x34 => Some(Self::CallValue),
            0x35 => Some(Self::CallDataLoad),
            0x36 => Some(Self::CallDataSize),
            0x37 => Some(Self::CallDataCopy),
            0x38 => Some(Self::CodeSize),
            0x39 => Some(Self::CodeCopy),
            0x3A => Some(Self::GasPrice),
            0x3B => Some(Self::ExtCodeSize),
            0x3C => Some(Self::ExtCodeCopy),
            0x3D => Some(Self::ReturnDataSize),
            0x3E => Some(Self::ReturnDataCopy),
            0x3F => Some(Self::ExtCodeHash),
            0x40 => Some(Self::BlockHash),
            0x41 => Some(Self::Coinbase),
            0x42 => Some(Self::Timestamp),
            0x43 => Some(Self::Number),
            0x44 => Some(Self::PrevRandao),
            0x45 => Some(Self::GasLimit),
            0x46 => Some(Self::ChainId),
            0x47 => Some(Self::SelfBalance),
            0x48 => Some(Self::BaseFee),
            0x49 => Some(Self::BlobHash),
            0x4A => Some(Self::BlobBaseFee),
            0x5A => Some(Self::Gas),
            0xA0 => Some(Self::Log0),
            0xA1 => Some(Self::Log1),
            0xA2 => Some(Self::Log2),
            0xA3 => Some(Self::Log3),
            0xA4 => Some(Self::Log4),
            0xF0 => Some(Self::Create),
            0xF1 => Some(Self::Call),
            0xF2 => Some(Self::CallCode),
            0xF3 => Some(Self::Return),
            0xF4 => Some(Self::DelegateCall),
            0xF5 => Some(Self::Create2),
            0xFA => Some(Self::StaticCall),
            0xFD => Some(Self::Revert),
            _ => None,
        }
    }
}

//==========================
// ECALL Constants
//==========================
pub const ECALL_CODE_REG: u32 = 31;

// Keccak256
pub const KECCAK256_OFFSET_REGISTER: u32 = 1;
pub const KECCAK256_SIZE_REGISTER: u32 = 2;

pub const KECCAK256_OUTPUT_REGITER_1: u32 = 3;
pub const KECCAK256_OUTPUT_REGITER_2: u32 = 4;
pub const KECCAK256_OUTPUT_REGITER_3: u32 = 5;
pub const KECCAK256_OUTPUT_REGITER_4: u32 = 6;
pub const KECCAK256_OUTPUT_REGITER_5: u32 = 7;
pub const KECCAK256_OUTPUT_REGITER_6: u32 = 8;
pub const KECCAK256_OUTPUT_REGITER_7: u32 = 9;
pub const KECCAK256_OUTPUT_REGITER_8: u32 = 10;

// Address
pub const ADDRESS_REGISTER_1: u32 = 1;
pub const ADDRESS_REGISTER_2: u32 = 2;
pub const ADDRESS_REGISTER_3: u32 = 3;
pub const ADDRESS_REGISTER_4: u32 = 4;
pub const ADDRESS_REGISTER_5: u32 = 5;

// Balance
pub const BALANCE_INPUT_REGISTER_1: u32 = 1;
pub const BALANCE_INPUT_REGISTER_2: u32 = 2;
pub const BALANCE_INPUT_REGISTER_3: u32 = 3;
pub const BALANCE_INPUT_REGISTER_4: u32 = 4;
pub const BALANCE_INPUT_REGISTER_5: u32 = 5;

pub const BALANCE_OUTPUT_REGISTER_1: u32 = 6;
pub const BALANCE_OUTPUT_REGISTER_2: u32 = 7;
pub const BALANCE_OUTPUT_REGISTER_3: u32 = 8;
pub const BALANCE_OUTPUT_REGISTER_4: u32 = 9;
pub const BALANCE_OUTPUT_REGISTER_5: u32 = 10;
pub const BALANCE_OUTPUT_REGISTER_6: u32 = 11;
pub const BALANCE_OUTPUT_REGISTER_7: u32 = 12;
pub const BALANCE_OUTPUT_REGISTER_8: u32 = 13;

// Origin
pub const ORIGIN_OUTPUT_REGISTER_1: u32 = 1;
pub const ORIGIN_OUTPUT_REGISTER_2: u32 = 2;
pub const ORIGIN_OUTPUT_REGISTER_3: u32 = 3;
pub const ORIGIN_OUTPUT_REGISTER_4: u32 = 4;
pub const ORIGIN_OUTPUT_REGISTER_5: u32 = 5;

// Origin
pub const CALLER_OUTPUT_REGISTER_1: u32 = 1;
pub const CALLER_OUTPUT_REGISTER_2: u32 = 2;
pub const CALLER_OUTPUT_REGISTER_3: u32 = 3;
pub const CALLER_OUTPUT_REGISTER_4: u32 = 4;
pub const CALLER_OUTPUT_REGISTER_5: u32 = 5;

// CallValue
pub const CALL_VALUE_OUTPUT_REGISTER_1: u32 = 1;
pub const CALL_VALUE_OUTPUT_REGISTER_2: u32 = 2;
pub const CALL_VALUE_OUTPUT_REGISTER_3: u32 = 3;
pub const CALL_VALUE_OUTPUT_REGISTER_4: u32 = 4;
pub const CALL_VALUE_OUTPUT_REGISTER_5: u32 = 5;
pub const CALL_VALUE_OUTPUT_REGISTER_6: u32 = 6;
pub const CALL_VALUE_OUTPUT_REGISTER_7: u32 = 7;
pub const CALL_VALUE_OUTPUT_REGISTER_8: u32 = 8;

// CallDataLoad
pub const CALL_DATA_LOAD_INPUT_REGISTER: u32 = 1;

pub const CALL_DATA_LOAD_OUTPUT_REGISTER_1: u32 = 2;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_2: u32 = 3;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_3: u32 = 4;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_4: u32 = 5;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_5: u32 = 6;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_6: u32 = 7;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_7: u32 = 8;
pub const CALL_DATA_LOAD_OUTPUT_REGISTER_8: u32 = 9;

// CallDataSize
pub const CALL_DATA_SIZE_OUTPUT_REGISTER: u32 = 1;

// CallDataCopy
pub const CALL_DATA_COPY_INPUT_REGISTER_1: u32 = 1;
pub const CALL_DATA_COPY_INPUT_REGISTER_2: u32 = 2;
pub const CALL_DATA_COPY_INPUT_REGISTER_3: u32 = 3;

// CodeSize
pub const CODE_SIZE_OUT_REGISTER: u32 = 1;

// CodeCopy
pub const CODE_COPY_INPUT_REGISTER_1: u32 = 1;
pub const CODE_COPY_INPUT_REGISTER_2: u32 = 2;
pub const CODE_COPY_INPUT_REGISTER_3: u32 = 3;

// GasPrice
pub const GAS_PRICE_OUTPUT_REGISTER_1: u32 = 1;
pub const GAS_PRICE_OUTPUT_REGISTER_2: u32 = 2;
pub const GAS_PRICE_OUTPUT_REGISTER_3: u32 = 3;
pub const GAS_PRICE_OUTPUT_REGISTER_4: u32 = 4;
pub const GAS_PRICE_OUTPUT_REGISTER_5: u32 = 5;
pub const GAS_PRICE_OUTPUT_REGISTER_6: u32 = 6;
pub const GAS_PRICE_OUTPUT_REGISTER_7: u32 = 7;
pub const GAS_PRICE_OUTPUT_REGISTER_8: u32 = 8;

//ExtCodeSize
pub const EXT_CODE_SIZE_INPUT_REGISTER_1: u32 = 1;
pub const EXT_CODE_SIZE_INPUT_REGISTER_2: u32 = 2;
pub const EXT_CODE_SIZE_INPUT_REGISTER_3: u32 = 3;
pub const EXT_CODE_SIZE_INPUT_REGISTER_4: u32 = 4;
pub const EXT_CODE_SIZE_INPUT_REGISTER_5: u32 = 5;
pub const EXT_CODE_SIZE_INPUT_REGISTER_6: u32 = 6;

// ExtCodeCopy
pub const EXT_CODE_COPY_INPUT_REGISTER_1: u32 = 1;
pub const EXT_CODE_COPY_INPUT_REGISTER_2: u32 = 2;
pub const EXT_CODE_COPY_INPUT_REGISTER_3: u32 = 3;
pub const EXT_CODE_COPY_INPUT_REGISTER_4: u32 = 4;
pub const EXT_CODE_COPY_INPUT_REGISTER_5: u32 = 5;
pub const EXT_CODE_COPY_INPUT_REGISTER_6: u32 = 6;
pub const EXT_CODE_COPY_INPUT_REGISTER_7: u32 = 7;
pub const EXT_CODE_COPY_INPUT_REGISTER_8: u32 = 8;

// ReturnDataSize
pub const RETURN_DATA_SIZE_OUTPUT_REGISTER: u32 = 1;

// ReturnDataCopy
pub const RETURN_DATA_COPY_INPUT_REGISTER_1: u32 = 1;
pub const RETURN_DATA_COPY_INPUT_REGISTER_2: u32 = 2;
pub const RETURN_DATA_COPY_INPUT_REGISTER_3: u32 = 3;

// ExtCodeHash
pub const EXT_CODE_HASH_INPUT_REGISTER_1: u32 = 1;
pub const EXT_CODE_HASH_INPUT_REGISTER_2: u32 = 2;
pub const EXT_CODE_HASH_INPUT_REGISTER_3: u32 = 3;
pub const EXT_CODE_HASH_INPUT_REGISTER_4: u32 = 4;
pub const EXT_CODE_HASH_INPUT_REGISTER_5: u32 = 5;

pub const EXT_CODE_HASH_OUTPUT_REGISTER_1: u32 = 6;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_2: u32 = 7;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_3: u32 = 8;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_4: u32 = 9;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_5: u32 = 10;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_6: u32 = 11;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_7: u32 = 12;
pub const EXT_CODE_HASH_OUTPUT_REGISTER_8: u32 = 13;

// BlockHash
pub const BLOCK_HASH_INPUT_REGISTER_1: u32 = 1;
pub const BLOCK_HASH_INPUT_REGISTER_2: u32 = 2;

pub const BLOCK_HASH_OUTPUT_REGISTER_1: u32 = 3;
pub const BLOCK_HASH_OUTPUT_REGISTER_2: u32 = 4;
pub const BLOCK_HASH_OUTPUT_REGISTER_3: u32 = 5;
pub const BLOCK_HASH_OUTPUT_REGISTER_4: u32 = 6;
pub const BLOCK_HASH_OUTPUT_REGISTER_5: u32 = 7;
pub const BLOCK_HASH_OUTPUT_REGISTER_6: u32 = 8;
pub const BLOCK_HASH_OUTPUT_REGISTER_7: u32 = 9;
pub const BLOCK_HASH_OUTPUT_REGISTER_8: u32 = 10;

// Coinbase
pub const COINBASE_OUTPUT_REGISTER_1: u32 = 1;
pub const COINBASE_OUTPUT_REGISTER_2: u32 = 2;
pub const COINBASE_OUTPUT_REGISTER_3: u32 = 3;
pub const COINBASE_OUTPUT_REGISTER_4: u32 = 4;
pub const COINBASE_OUTPUT_REGISTER_5: u32 = 5;

// Timestamp
pub const TIMESTAMP_OUTPUT_REGISTER_1: u32 = 1;
pub const TIMESTAMP_OUTPUT_REGISTER_2: u32 = 2;

// Number
pub const NUMBER_OUTPUT_REGISTER_1: u32 = 1;
pub const NUMBER_OUTPUT_REGISTER_2: u32 = 2;

// PrevRandao
pub const PREV_RANDAO_OUTPUT_REGISTER_1: u32 = 1;
pub const PREV_RANDAO_OUTPUT_REGISTER_2: u32 = 2;
pub const PREV_RANDAO_OUTPUT_REGISTER_3: u32 = 3;
pub const PREV_RANDAO_OUTPUT_REGISTER_4: u32 = 4;
pub const PREV_RANDAO_OUTPUT_REGISTER_5: u32 = 5;
pub const PREV_RANDAO_OUTPUT_REGISTER_6: u32 = 6;
pub const PREV_RANDAO_OUTPUT_REGISTER_7: u32 = 7;
pub const PREV_RANDAO_OUTPUT_REGISTER_8: u32 = 8;

// GasLimit
pub const GAS_LIMIT_OUTPUT_REGISTER_1: u32 = 1;
pub const GAS_LIMIT_OUTPUT_REGISTER_2: u32 = 2;
pub const GAS_LIMIT_OUTPUT_REGISTER_3: u32 = 3;
pub const GAS_LIMIT_OUTPUT_REGISTER_4: u32 = 4;
pub const GAS_LIMIT_OUTPUT_REGISTER_5: u32 = 5;
pub const GAS_LIMIT_OUTPUT_REGISTER_6: u32 = 6;
pub const GAS_LIMIT_OUTPUT_REGISTER_7: u32 = 7;
pub const GAS_LIMIT_OUTPUT_REGISTER_8: u32 = 8;

// ChainId
pub const CHAIN_ID_OUTPUT_REGISTER_1: u32 = 1;
pub const CHAIN_ID_OUTPUT_REGISTER_2: u32 = 2;

// SelfBalance
pub const SELF_BALANCE_OUTPUT_REGISTER_1: u32 = 1;
pub const SELF_BALANCE_OUTPUT_REGISTER_2: u32 = 2;
pub const SELF_BALANCE_OUTPUT_REGISTER_3: u32 = 3;
pub const SELF_BALANCE_OUTPUT_REGISTER_4: u32 = 4;
pub const SELF_BALANCE_OUTPUT_REGISTER_5: u32 = 5;
pub const SELF_BALANCE_OUTPUT_REGISTER_6: u32 = 6;
pub const SELF_BALANCE_OUTPUT_REGISTER_7: u32 = 7;
pub const SELF_BALANCE_OUTPUT_REGISTER_8: u32 = 8;

// BaseFee
pub const BASE_FEE_OUTPUT_REGISTER_1: u32 = 1;
pub const BASE_FEE_OUTPUT_REGISTER_2: u32 = 2;
pub const BASE_FEE_OUTPUT_REGISTER_3: u32 = 3;
pub const BASE_FEE_OUTPUT_REGISTER_4: u32 = 4;
pub const BASE_FEE_OUTPUT_REGISTER_5: u32 = 5;
pub const BASE_FEE_OUTPUT_REGISTER_6: u32 = 6;
pub const BASE_FEE_OUTPUT_REGISTER_7: u32 = 7;
pub const BASE_FEE_OUTPUT_REGISTER_8: u32 = 8;

// BlobHash
pub const BLOB_HASH_OUTPUT_REGISTER_1: u32 = 1; // Input for index
pub const BLOB_HASH_OUTPUT_REGISTER_2: u32 = 2;
pub const BLOB_HASH_OUTPUT_REGISTER_3: u32 = 3;
pub const BLOB_HASH_OUTPUT_REGISTER_4: u32 = 4;
pub const BLOB_HASH_OUTPUT_REGISTER_5: u32 = 5;
pub const BLOB_HASH_OUTPUT_REGISTER_6: u32 = 6;
pub const BLOB_HASH_OUTPUT_REGISTER_7: u32 = 7;
pub const BLOB_HASH_OUTPUT_REGISTER_8: u32 = 8;
pub const BLOB_HASH_OUTPUT_REGISTER_9: u32 = 9;

// BlobBaseFee
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_1: u32 = 1;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_2: u32 = 2;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_3: u32 = 3;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_4: u32 = 4;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_5: u32 = 5;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_6: u32 = 6;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_7: u32 = 7;
pub const BLOB_BASE_FEE_OUTPUT_REGISTER_8: u32 = 8;

// Gas
pub const GAS_OUTPUT_REGISTER_1: u32 = 1;
pub const GAS_OUTPUT_REGISTER_2: u32 = 2;
pub const GAS_OUTPUT_REGISTER_3: u32 = 3;
pub const GAS_OUTPUT_REGISTER_4: u32 = 4;
pub const GAS_OUTPUT_REGISTER_5: u32 = 5;
pub const GAS_OUTPUT_REGISTER_6: u32 = 6;
pub const GAS_OUTPUT_REGISTER_7: u32 = 7;
pub const GAS_OUTPUT_REGISTER_8: u32 = 8;

// Log0
pub const LOG0_INPUT_REGISTER_1: u32 = 1;
pub const LOG0_INPUT_REGISTER_2: u32 = 2;

// Log1
pub const LOG1_INPUT_REGISTER_1: u32 = 1;
pub const LOG1_INPUT_REGISTER_2: u32 = 2;
pub const LOG1_INPUT_REGISTER_3: u32 = 3;
pub const LOG1_INPUT_REGISTER_4: u32 = 4;
pub const LOG1_INPUT_REGISTER_5: u32 = 5;
pub const LOG1_INPUT_REGISTER_6: u32 = 6;
pub const LOG1_INPUT_REGISTER_7: u32 = 7;
pub const LOG1_INPUT_REGISTER_8: u32 = 8;
pub const LOG1_INPUT_REGISTER_9: u32 = 9;
pub const LOG1_INPUT_REGISTER_10: u32 = 10;

// Log2

// Log3

// Log4

// Create
pub const CREATE_INPUT_REGISTER_1: u32 = 1;
pub const CREATE_INPUT_REGISTER_2: u32 = 2;

// Call

// CallCode

// Return
pub const RETURN_INPUT_REGISTER_1: u32 = 1;
pub const RETURN_INPUT_REGISTER_2: u32 = 2;

// DelegateCall

// Create2

// StaticCall

// Revert
pub const REVERT_INPUT_REGISTER_1: u32 = 1;
pub const REVERT_INPUT_REGISTER_2: u32 = 2;

// SLoad
pub const SLOAD_INPUT_REGISTER_1: u32 = 1;
pub const SLOAD_INPUT_REGISTER_2: u32 = 2;
pub const SLOAD_INPUT_REGISTER_3: u32 = 3;
pub const SLOAD_INPUT_REGISTER_4: u32 = 4;
pub const SLOAD_INPUT_REGISTER_5: u32 = 5;
pub const SLOAD_INPUT_REGISTER_6: u32 = 6;
pub const SLOAD_INPUT_REGISTER_7: u32 = 7;
pub const SLOAD_INPUT_REGISTER_8: u32 = 8;

pub const SLOAD_OUTPUT_REGISTER_1: u32 = 9;
pub const SLOAD_OUTPUT_REGISTER_2: u32 = 10;
pub const SLOAD_OUTPUT_REGISTER_3: u32 = 11;
pub const SLOAD_OUTPUT_REGISTER_4: u32 = 12;
pub const SLOAD_OUTPUT_REGISTER_5: u32 = 13;
pub const SLOAD_OUTPUT_REGISTER_6: u32 = 14;
pub const SLOAD_OUTPUT_REGISTER_7: u32 = 15;
pub const SLOAD_OUTPUT_REGISTER_8: u32 = 16;

// SStore
pub const SSTORE_INPUT_REGISTER_1: u32 = 1;
pub const SSTORE_INPUT_REGISTER_2: u32 = 2;
pub const SSTORE_INPUT_REGISTER_3: u32 = 3;
pub const SSTORE_INPUT_REGISTER_4: u32 = 4;
pub const SSTORE_INPUT_REGISTER_5: u32 = 5;
pub const SSTORE_INPUT_REGISTER_6: u32 = 6;
pub const SSTORE_INPUT_REGISTER_7: u32 = 7;
pub const SSTORE_INPUT_REGISTER_8: u32 = 8;
pub const SSTORE_INPUT_REGISTER_9: u32 = 9;
pub const SSTORE_INPUT_REGISTER_10: u32 = 10;
pub const SSTORE_INPUT_REGISTER_11: u32 = 11;
pub const SSTORE_INPUT_REGISTER_12: u32 = 12;
pub const SSTORE_INPUT_REGISTER_13: u32 = 13;
pub const SSTORE_INPUT_REGISTER_14: u32 = 14;
pub const SSTORE_INPUT_REGISTER_15: u32 = 15;
pub const SSTORE_INPUT_REGISTER_16: u32 = 16;
