#![cfg(test)]

use crate::{
    context::Context,
    ecall_manager::process_ecall,
    utils::{address_to_u32_vec, u32_vec_to_address, u32_vec_to_u256, u256_to_u32_vec},
    vm::Vm,
};
use revm::{
    Context as EthContext, MainContext,
    context::ContextTr,
    database::{CacheDB, InMemoryDB},
    primitives::{Address, U256, keccak256},
    state::AccountInfo,
};
use riscv_evm_core::{MemoryChuckSize, e_constants::*, interfaces::MemoryInterface};
use std::str::FromStr;

// Helper function to create a context with a specific setup
fn setup_test_context() -> Context {
    let eth_context = EthContext::mainnet().with_db(CacheDB::default());
    let mut context = Context::new(eth_context);
    // Set a known address for testing
    context.address = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    context.current_caller =
        Address::from_str("0x0000000000000000000000000000000000000002").unwrap();
    context
}

// Helper function to initialize VM with necessary setup
fn setup_test_vm() -> Vm {
    let mut vm = Vm::new();
    // Set up memory with some test data
    vm.memory.write_mem(100, MemoryChuckSize::BYTE, 0x01);
    vm.memory.write_mem(101, MemoryChuckSize::BYTE, 0x02);
    vm.memory.write_mem(102, MemoryChuckSize::BYTE, 0x03);
    vm.memory.write_mem(103, MemoryChuckSize::BYTE, 0x04);

    vm.memory
        .write_mem(200, MemoryChuckSize::WordSize, 0x11223344);
    vm
}

// Test for Keccak256 ECALL
#[test]
fn test_ecall_keccak256() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Set up the VM registers for the Keccak256 call
    vm.registers.write_reg(ECALL_CODE_REG, 0x20); // Keccak256
    vm.registers.write_reg(KECCAK256_OFFSET_REGISTER, 100); // data offset
    vm.registers.write_reg(KECCAK256_SIZE_REGISTER, 4); // data size

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Calculate expected hash to verify
    let data = vec![0x01, 0x02, 0x03, 0x04];
    let expected_hash = keccak256(&data);

    // Extract hash from registers
    let h1 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_1);
    let h2 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_2);
    let h3 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_3);
    let h4 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_4);
    let h5 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_5);
    let h6 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_6);
    let h7 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_7);
    let h8 = vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_8);

    // Verify hash matches expected
    let actual_hash_bytes = u32_vec_to_u256(&vec![h1, h2, h3, h4, h5, h6, h7, h8]);

    assert_eq!(actual_hash_bytes.to_vec(), expected_hash.0.to_vec());
}

// Test for Address ECALL
#[test]
fn test_ecall_address() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Set up the VM registers for the Address call
    vm.registers.write_reg(ECALL_CODE_REG, 0x30); // Address

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Extract address from registers
    let a1 = vm.registers.read_reg(ADDRESS_REGISTER_1);
    let a2 = vm.registers.read_reg(ADDRESS_REGISTER_2);
    let a3 = vm.registers.read_reg(ADDRESS_REGISTER_3);
    let a4 = vm.registers.read_reg(ADDRESS_REGISTER_4);
    let a5 = vm.registers.read_reg(ADDRESS_REGISTER_5);

    // Recreate address from registers
    let address_bytes = u32_vec_to_address(&vec![a1, a2, a3, a4, a5]);

    // Verify address matches expected
    assert_eq!(address_bytes, context.address.0.0);
}

// Test for Balance ECALL
#[test]
fn test_ecall_balance() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Create a database with account balance
    let mut db = InMemoryDB::default();
    let test_address = Address::from_str("0x0000000000000000000000000000000000000003").unwrap();
    let test_balance = U256::from(1000000);
    let account_info = AccountInfo {
        balance: test_balance,
        ..Default::default()
    };
    db.insert_account_info(test_address, account_info);

    // Set the database in the context
    context.eth_context = EthContext::mainnet().with_db(db);

    // Convert address to u32 components
    let addr_components = address_to_u32_vec(&test_address.0.0);

    // Set up the VM registers for the Balance call
    vm.registers.write_reg(ECALL_CODE_REG, 0x31); // Balance
    vm.registers
        .write_reg(BALANCE_INPUT_REGISTER_1, addr_components[0]);
    vm.registers
        .write_reg(BALANCE_INPUT_REGISTER_2, addr_components[1]);
    vm.registers
        .write_reg(BALANCE_INPUT_REGISTER_3, addr_components[2]);
    vm.registers
        .write_reg(BALANCE_INPUT_REGISTER_4, addr_components[3]);
    vm.registers
        .write_reg(BALANCE_INPUT_REGISTER_5, addr_components[4]);

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Extract balance from registers
    let b1 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_1);
    let b2 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_2);
    let b3 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_3);
    let b4 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_4);
    let b5 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_5);
    let b6 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_6);
    let b7 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_7);
    let b8 = vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_8);

    // Convert to U256
    let balance_bytes = u32_vec_to_u256(&vec![b1, b2, b3, b4, b5, b6, b7, b8]);
    let balance = U256::from_be_bytes(balance_bytes);

    // Verify balance matches expected
    assert_eq!(balance, test_balance);
}

// Test for CallDataLoad ECALL
#[test]
fn test_ecall_calldataload() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Set calldata in context
    let calldata = hex::decode("a9059cbb000000000000000000000000b97048628db6b661d4c2aa833e95dbe1a905b2800000000000000000000000000000000000000000000000000000000000003e84").unwrap();
    context.eth_context.modify_tx(|tx| {
        tx.data = calldata.clone().into();
    });

    // Set up the VM registers for the CallDataLoad call
    vm.registers.write_reg(ECALL_CODE_REG, 0x35); // CallDataLoad
    vm.registers.write_reg(CALL_DATA_LOAD_INPUT_REGISTER, 0); // offset 0

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Extract data from registers
    let d1 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_1);
    let d2 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_2);
    let d3 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_3);
    let d4 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_4);
    let d5 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_5);
    let d6 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_6);
    let d7 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_7);
    let d8 = vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_8);

    // Verify first 32 bytes match expected
    let expected_data = &calldata[0..32];
    let actual_data = u32_vec_to_u256(&vec![d1, d2, d3, d4, d5, d6, d7, d8]);

    assert_eq!(actual_data.to_vec(), expected_data.to_vec());
}

// Test for CallDataSize ECALL
#[test]
fn test_ecall_calldatasize() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Set calldata in context
    let calldata = hex::decode("a9059cbb000000000000000000000000b97048628db6b661d4c2aa833e95dbe1a905b2800000000000000000000000000000000000000000000000000000000000003e84").unwrap();
    context.eth_context.modify_tx(|tx| {
        tx.data = calldata.clone().into();
    });

    // Set up the VM registers for the CallDataSize call
    vm.registers.write_reg(ECALL_CODE_REG, 0x36); // CallDataSize

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Check the size
    let size = vm.registers.read_reg(CALL_DATA_SIZE_OUTPUT_REGISTER);
    assert_eq!(size, calldata.len() as u32);
}

// Test for CallDataCopy ECALL
#[test]
fn test_ecall_calldatacopy() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Set calldata in context
    let calldata = hex::decode("a9059cbb000000000000000000000000b97048628db6b661d4c2aa833e95dbe1a905b2800000000000000000000000000000000000000000000000000000000000003e84").unwrap();
    context.eth_context.modify_tx(|tx| {
        tx.data = calldata.clone().into();
    });

    // Memory destination
    let dest_offset = 500;
    let data_offset = 4; // Skip function selector
    let copy_size = 32; // One parameter

    // Set up the VM registers for the CallDataCopy call
    vm.registers.write_reg(ECALL_CODE_REG, 0x37); // CallDataCopy
    vm.registers
        .write_reg(CALL_DATA_COPY_INPUT_REGISTER_1, dest_offset);
    vm.registers
        .write_reg(CALL_DATA_COPY_INPUT_REGISTER_2, data_offset);
    vm.registers
        .write_reg(CALL_DATA_COPY_INPUT_REGISTER_3, copy_size);

    // Process the ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Verify data was copied correctly
    for i in 0..copy_size {
        let expected_byte = calldata[(data_offset + i) as usize];
        let actual_byte = vm
            .memory
            .read_mem((dest_offset + i) as u32, MemoryChuckSize::BYTE)
            .unwrap() as u8;
        assert_eq!(
            actual_byte, expected_byte,
            "Byte at index {} doesn't match",
            i
        );
    }
}

// Test for SLoad and SStore ECALLs
#[test]
fn test_ecall_storage() {
    let mut vm = setup_test_vm();
    let mut context = setup_test_context();

    // Create a database with account balance
    let mut db = InMemoryDB::default();
    let test_address = Address::from_str("0x0000000000000000000000000000000000000003").unwrap();
    let test_balance = U256::from(1000000);
    let account_info = AccountInfo {
        balance: test_balance,
        ..Default::default()
    };
    db.insert_account_info(test_address, account_info);

    // Set the database in the context
    context.eth_context = EthContext::mainnet().with_db(db);

    println!(
        "This is the state: {:?}",
        context.eth_context.journal().state()
    );

    // Test key and value for storage
    let key = U256::from(42);
    let value = U256::from(1234567890);

    // Convert to u32 arrays
    let key_bytes: [u8; 32] = key.to_be_bytes();
    let key_u32 = u256_to_u32_vec(&key_bytes);

    let value_bytes: [u8; 32] = value.to_be_bytes();
    let value_u32 = u256_to_u32_vec(&value_bytes);

    // Set up the VM registers for SStore
    vm.registers.write_reg(ECALL_CODE_REG, 0x55); // SStore

    // Set the key
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_1, key_u32[0]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_2, key_u32[1]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_3, key_u32[2]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_4, key_u32[3]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_5, key_u32[4]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_6, key_u32[5]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_7, key_u32[6]);
    vm.registers.write_reg(SSTORE_INPUT_REGISTER_8, key_u32[7]);

    // Set the value
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_9, value_u32[0]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_10, value_u32[1]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_11, value_u32[2]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_12, value_u32[3]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_13, value_u32[4]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_14, value_u32[5]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_15, value_u32[6]);
    vm.registers
        .write_reg(SSTORE_INPUT_REGISTER_16, value_u32[7]);

    // Process the SStore ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Now test SLoad
    // Set up the VM registers for SLoad
    vm.registers.write_reg(ECALL_CODE_REG, 0x54); // SLoad

    // Set the key
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_1, key_u32[0]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_2, key_u32[1]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_3, key_u32[2]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_4, key_u32[3]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_5, key_u32[4]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_6, key_u32[5]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_7, key_u32[6]);
    vm.registers.write_reg(SLOAD_INPUT_REGISTER_8, key_u32[7]);

    // Process the SLoad ECALL
    let result = process_ecall(&mut vm, &mut context);
    assert!(result.is_ok());

    // Check the loaded value
    let loaded_1 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_1);
    let loaded_2 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_2);
    let loaded_3 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_3);
    let loaded_4 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_4);
    let loaded_5 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_5);
    let loaded_6 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_6);
    let loaded_7 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_7);
    let loaded_8 = vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_8);

    assert_eq!(loaded_1, value_u32[0]);
    assert_eq!(loaded_2, value_u32[1]);
    assert_eq!(loaded_3, value_u32[2]);
    assert_eq!(loaded_4, value_u32[3]);
    assert_eq!(loaded_5, value_u32[4]);
    assert_eq!(loaded_6, value_u32[5]);
    assert_eq!(loaded_7, value_u32[6]);
    assert_eq!(loaded_8, value_u32[7]);
}

// Test for Log0 ECALL
#[test]
fn test_ecall_log0() {
    // let mut vm = setup_test_vm();
    // let mut context = setup_test_context();

    // // Prepare data for logging
    // let log_offset = 100; // Where our test data is
    // let log_size = 4;     // 4 bytes

    // // Setup the VM registers for Log0
    // vm.registers.write_reg(ECALL_CODE_REG, 0xA0); // Log0
    // vm.registers.write_reg(LOG0_INPUT_REGISTER_1, log_offset);
    // vm.registers.write_reg(LOG0_INPUT_REGISTER_2, log_size);

    // // Create a test database to check journal
    // let db = EmptyDB::default();
    // context.eth_context = EthContext::mainnet().with_db(db);

    // // Process the Log0 ECALL
    // let result = process_ecall(&mut vm, &mut context);
    // assert!(result.is_ok());

    // // Verify log was emitted correctly
    // let logs = context.eth_context.tx.;
    // assert_eq!(logs.len(), 1);

    // let log = &logs[0];
    // assert_eq!(log.address, context.address);
    // assert_eq!(log.data.data(), &[0x01, 0x02, 0x03, 0x04]);
    // assert_eq!(log.data.topics().len(), 0);
}
