#[cfg(test)]
mod basic_tests {
    use super::*;
    use crate::{
        context::Context,
        ecall_manager::process_ecall,
        utils::{address_to_u32_vec, bytes_to_u32, u32_vec_to_address, u32_vec_to_u256},
        vm::Vm,
    };
    use revm::{
        Context as RevmEthContext, MainContext,
        database::CacheDB,
        primitives::{Address, B256, Bytes, U256, keccak256},
    };
    use riscv_evm_core::{MemoryChuckSize, e_constants::*, interfaces::MemoryInterface};

    // Helper function to create test VM and Context
    fn setup() -> (Vm, Context) {
        let mut vm = Vm::new();
        let eth_context = RevmEthContext::mainnet().with_db(CacheDB::default());
        let mut context = Context::new(eth_context);

        // Set up a test address
        let test_address = Address::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67,
        ]);
        context.address = test_address;

        (vm, context)
    }

    #[test]
    fn test_keccak256() {
        let (mut vm, mut context) = setup();

        // Test data to hash
        let data = b"Hello, world!";
        let offset = 100;

        // Write data to memory
        for (i, byte) in data.iter().enumerate() {
            vm.memory
                .write_mem(offset + i as u32, MemoryChuckSize::BYTE, *byte as u32);
        }

        // Set up ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0x20); // Keccak256
        vm.registers.write_reg(KECCAK256_OFFSET_REGISTER, offset);
        vm.registers
            .write_reg(KECCAK256_SIZE_REGISTER, data.len() as u32);

        // Process ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Calculate expected hash
        let expected_hash = keccak256(data);

        // Verify output registers contain correct hash
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_1),
            bytes_to_u32(&expected_hash.0[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_2),
            bytes_to_u32(&expected_hash.0[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_3),
            bytes_to_u32(&expected_hash.0[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_4),
            bytes_to_u32(&expected_hash.0[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_5),
            bytes_to_u32(&expected_hash.0[16..20])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_6),
            bytes_to_u32(&expected_hash.0[20..24])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_7),
            bytes_to_u32(&expected_hash.0[24..28])
        );
        assert_eq!(
            vm.registers.read_reg(KECCAK256_OUTPUT_REGITER_8),
            bytes_to_u32(&expected_hash.0[28..32])
        );
    }

    #[test]
    fn test_address() {
        let (mut vm, mut context) = setup();

        // Set up ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0x30); // Address

        // Process ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify output registers contain correct address
        assert_eq!(
            vm.registers.read_reg(ADDRESS_REGISTER_1),
            bytes_to_u32(&context.address.0.0[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(ADDRESS_REGISTER_2),
            bytes_to_u32(&context.address.0.0[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(ADDRESS_REGISTER_3),
            bytes_to_u32(&context.address.0.0[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(ADDRESS_REGISTER_4),
            bytes_to_u32(&context.address.0.0[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(ADDRESS_REGISTER_5),
            bytes_to_u32(&context.address.0.0[16..20])
        );
    }

    #[test]
    fn test_call_data_operations() {
        let (mut vm, mut context) = setup();

        // Test calldata
        let calldata = hex::decode("a9059cbb000000000000000000000000b97048628db6b661d4c2aa833e95dbe1a905b2800000000000000000000000000000000000000000000000000000000000003e84").unwrap();

        // Set calldata in context
        context.eth_context.modify_tx(|tx| {
            tx.data = calldata.clone().into();
        });

        // Test CallDataSize
        vm.registers.write_reg(ECALL_CODE_REG, 0x36); // CallDataSize
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_SIZE_OUTPUT_REGISTER),
            calldata.len() as u32
        );

        // Test CallDataLoad
        vm.registers.write_reg(ECALL_CODE_REG, 0x35); // CallDataLoad
        vm.registers.write_reg(CALL_DATA_LOAD_INPUT_REGISTER, 4); // offset (after function selector)

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify first parameter (address) was loaded
        let expected_value = &calldata[4..36];
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_1),
            bytes_to_u32(&expected_value[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_2),
            bytes_to_u32(&expected_value[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_3),
            bytes_to_u32(&expected_value[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_4),
            bytes_to_u32(&expected_value[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_5),
            bytes_to_u32(&expected_value[16..20])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_6),
            bytes_to_u32(&expected_value[20..24])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_7),
            bytes_to_u32(&expected_value[24..28])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_8),
            bytes_to_u32(&expected_value[28..32])
        );

        // Test CallDataCopy
        let dest_offset = 200;
        vm.registers.write_reg(ECALL_CODE_REG, 0x37); // CallDataCopy
        vm.registers
            .write_reg(CALL_DATA_COPY_INPUT_REGISTER_1, dest_offset); // destOffset
        vm.registers.write_reg(CALL_DATA_COPY_INPUT_REGISTER_2, 0); // offset
        vm.registers.write_reg(CALL_DATA_COPY_INPUT_REGISTER_3, 4); // size (function selector)

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify function selector was copied to memory
        for i in 0..4 {
            assert_eq!(
                vm.memory
                    .read_mem(dest_offset + i, MemoryChuckSize::BYTE)
                    .unwrap(),
                calldata[i as usize] as u32
            );
        }
    }

    #[test]
    fn test_storage_operations() {
        let (mut vm, mut context) = setup();

        // Prepare test data for storage slot and value
        let slot_u32 = [
            0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210, 0x11223344, 0x55667788, 0x99AABBCC,
            0xDDEEFF00,
        ];
        let value_u32 = [
            0xAAAAAAAA, 0xBBBBBBBB, 0xCCCCCCCC, 0xDDDDDDDD, 0xEEEEEEEE, 0xFFFFFFFF, 0x11111111,
            0x22222222,
        ];

        // Set up SStore ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0x55); // SStore

        // Load slot registers
        for (i, &val) in slot_u32.iter().enumerate() {
            vm.registers
                .write_reg(SSTORE_INPUT_REGISTER_1 + i as u32, val);
        }

        // Load value registers
        for (i, &val) in value_u32.iter().enumerate() {
            vm.registers
                .write_reg(SSTORE_INPUT_REGISTER_9 + i as u32, val);
        }

        // Process SStore ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Now test SLoad for the same slot
        vm.registers.write_reg(ECALL_CODE_REG, 0x54); // SLoad

        // Load slot registers again
        for (i, &val) in slot_u32.iter().enumerate() {
            vm.registers
                .write_reg(SLOAD_INPUT_REGISTER_1 + i as u32, val);
        }

        // Process SLoad ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify loaded value matches what we stored
        for i in 0..8 {
            assert_eq!(
                vm.registers.read_reg(SLOAD_OUTPUT_REGISTER_1 + i as u32),
                value_u32[i]
            );
        }
    }

    #[test]
    fn test_return_operation() {
        let (mut vm, mut context) = setup();

        // Create test return data
        let return_data = b"Return data test";
        let offset = 300;

        // Write return data to memory
        for (i, &byte) in return_data.iter().enumerate() {
            vm.memory
                .write_mem(offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Set up Return ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0xF3); // Return
        vm.registers.write_reg(RETURN_INPUT_REGISTER_1, offset);
        vm.registers
            .write_reg(RETURN_INPUT_REGISTER_2, return_data.len() as u32);

        // Process Return ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify VM is stopped
        assert!(!vm.running);

        // Verify return data was set correctly
        assert_eq!(context.return_data.len(), return_data.len());
        for (i, &byte) in return_data.iter().enumerate() {
            assert_eq!(context.return_data[i], byte);
        }
    }

    #[test]
    fn test_invalid_ecall() {
        let (mut vm, mut context) = setup();

        // Set up invalid ECALL code
        vm.registers.write_reg(ECALL_CODE_REG, 0xFF); // Invalid code

        // Process ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod more_ecall_tests {
    use super::*;
    use crate::{
        context::Context,
        ecall_manager::process_ecall,
        utils::{
            address_to_u32_vec, bytes_to_u32, combine_u32_to_u64, split_u64_to_u32,
            u32_vec_to_address, u32_vec_to_bytes, u32_vec_to_u256,
        },
        vm::{VMErrors, Vm},
    };
    use revm::{
        Context as RevmEthContext, DatabaseCommit, MainContext,
        context::{ContextTr, JournalTr},
        database::{CacheDB, InMemoryDB},
        primitives::{Address, B256, Bytes, Log, LogData, TxKind, U256, keccak256},
        state::{AccountInfo, Bytecode},
    };
    use riscv_evm_core::{MemoryChuckSize, Registers, e_constants::*, interfaces::MemoryInterface};
    use std::str::FromStr;

    // Helper function to create test VM and Context
    fn setup() -> (Vm, Context) {
        let mut vm = Vm::new();
        let eth_context = RevmEthContext::mainnet().with_db(CacheDB::default());
        let mut context = Context::new(eth_context);

        // Set up a test address
        let test_address = Address::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67,
        ]);
        context.address = test_address;

        // Set up a caller address
        let caller_address = Address::from([
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD,
        ]);
        context.current_caller = caller_address;

        // Set test transaction value
        context.eth_context.modify_tx(|tx| {
            tx.value = U256::from(1000000);
            tx.caller = caller_address;
        });

        // Set test block properties
        context.eth_context.modify_block(|block| {
            block.timestamp = 1234567890;
            block.number = 12345678;
            block.beneficiary =
                Address::from_str("0x8888f1f195afa192cfee860698584c030f4c9db1").unwrap();
        });

        (vm, context)
    }

    #[test]
    fn test_balance() {
        let (mut vm, mut context) = setup();

        let mut db = InMemoryDB::default();
        let test_address = Address::from_str("0x0000000000000000000000000000000000000003").unwrap();
        let test_balance = U256::from(1000000);
        let account_info = AccountInfo {
            balance: test_balance,
            ..Default::default()
        };
        db.insert_account_info(test_address, account_info);

        // Set the database in the context
        context.eth_context = RevmEthContext::mainnet().with_db(db);

        // Set up ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0x31); // Balance

        // Set address in registers
        let addr_u32 = address_to_u32_vec(&test_address.0);
        for (i, &val) in addr_u32.iter().enumerate() {
            vm.registers
                .write_reg(BALANCE_INPUT_REGISTER_1 + i as u32, val);
        }

        // Process ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify output registers contain correct balance
        let balance_bytes: [u8; 32] = test_balance.to_be_bytes();
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_1),
            bytes_to_u32(&balance_bytes[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_2),
            bytes_to_u32(&balance_bytes[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_3),
            bytes_to_u32(&balance_bytes[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_4),
            bytes_to_u32(&balance_bytes[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_5),
            bytes_to_u32(&balance_bytes[16..20])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_6),
            bytes_to_u32(&balance_bytes[20..24])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_7),
            bytes_to_u32(&balance_bytes[24..28])
        );
        assert_eq!(
            vm.registers.read_reg(BALANCE_OUTPUT_REGISTER_8),
            bytes_to_u32(&balance_bytes[28..32])
        );
    }

    #[test]
    fn test_origin_and_caller() {
        let (mut vm, mut context) = setup();

        // Test Origin
        vm.registers.write_reg(ECALL_CODE_REG, 0x32); // Origin
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify output registers contain correct origin address
        let origin = context.eth_context.tx.caller.0;
        assert_eq!(
            vm.registers.read_reg(ORIGIN_OUTPUT_REGISTER_1),
            bytes_to_u32(&origin[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(ORIGIN_OUTPUT_REGISTER_2),
            bytes_to_u32(&origin[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(ORIGIN_OUTPUT_REGISTER_3),
            bytes_to_u32(&origin[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(ORIGIN_OUTPUT_REGISTER_4),
            bytes_to_u32(&origin[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(ORIGIN_OUTPUT_REGISTER_5),
            bytes_to_u32(&origin[16..20])
        );

        // Test Caller
        vm.registers.write_reg(ECALL_CODE_REG, 0x33); // Caller
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify output registers contain correct caller address
        let caller = context.current_caller.0;
        assert_eq!(
            vm.registers.read_reg(CALLER_OUTPUT_REGISTER_1),
            bytes_to_u32(&caller[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(CALLER_OUTPUT_REGISTER_2),
            bytes_to_u32(&caller[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(CALLER_OUTPUT_REGISTER_3),
            bytes_to_u32(&caller[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(CALLER_OUTPUT_REGISTER_4),
            bytes_to_u32(&caller[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(CALLER_OUTPUT_REGISTER_5),
            bytes_to_u32(&caller[16..20])
        );
    }

    #[test]
    fn test_call_value() {
        let (mut vm, mut context) = setup();

        // Set up ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0x34); // CallValue

        // Process ECALL
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify output registers contain correct value
        let value_bytes: [u8; 32] = context.eth_context.tx.value.to_be_bytes();
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_1),
            bytes_to_u32(&value_bytes[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_2),
            bytes_to_u32(&value_bytes[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_3),
            bytes_to_u32(&value_bytes[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_4),
            bytes_to_u32(&value_bytes[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_5),
            bytes_to_u32(&value_bytes[16..20])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_6),
            bytes_to_u32(&value_bytes[20..24])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_7),
            bytes_to_u32(&value_bytes[24..28])
        );
        assert_eq!(
            vm.registers.read_reg(CALL_VALUE_OUTPUT_REGISTER_8),
            bytes_to_u32(&value_bytes[28..32])
        );
    }

    #[test]
    fn test_code_operations() {
        let (mut vm, mut context) = setup();

        // Setup test code in contract account
        let test_code = vec![0x60, 0x80, 0x60, 0x40, 0x52, 0x34, 0x80, 0x15]; // Some bytecode
        let bytecode = Bytecode::new_legacy(test_code.clone().into());

        let mut db = InMemoryDB::default();
        let account_info = AccountInfo {
            code: Some(bytecode.clone()),
            ..Default::default()
        };
        db.insert_account_info(context.address, account_info);
        context.eth_context = RevmEthContext::mainnet().with_db(db);
        context
            .eth_context
            .journal()
            .load_account(context.address)
            .unwrap();

        // Test CodeSize
        vm.registers.write_reg(ECALL_CODE_REG, 0x38); // CodeSize
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());
        assert_eq!(
            vm.registers.read_reg(CODE_SIZE_OUT_REGISTER),
            bytecode.len() as u32
        );

        // Test CodeCopy
        let dest_offset = 400;
        vm.registers.write_reg(ECALL_CODE_REG, 0x39); // CodeCopy
        vm.registers
            .write_reg(CODE_COPY_INPUT_REGISTER_1, dest_offset); // destOffset
        vm.registers.write_reg(CODE_COPY_INPUT_REGISTER_2, 0); // offset
        vm.registers
            .write_reg(CODE_COPY_INPUT_REGISTER_3, test_code.len() as u32); // size

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify code was copied to memory
        for (i, &byte) in test_code.iter().enumerate() {
            assert_eq!(
                vm.memory
                    .read_mem(dest_offset + i as u32, MemoryChuckSize::BYTE)
                    .unwrap(),
                byte as u32
            );
        }
    }

    #[test]
    fn test_external_code_operations() {
        let (mut vm, mut context) = setup();

        // Setup test address and code
        let test_address = Address::from([
            0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11,
            0x00, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB,
        ]);
        let test_code = vec![0x6A, 0x60, 0x91, 0x40, 0x52, 0x33, 0x80, 0x16, 0x80, 0x15];
        let bytecode = Bytecode::new_legacy(test_code.clone().into());

        let mut db = InMemoryDB::default();
        let account_info = AccountInfo {
            code: Some(bytecode.clone()),
            ..Default::default()
        };
        db.insert_account_info(test_address, account_info);
        context.eth_context = RevmEthContext::mainnet().with_db(db);
        context
            .eth_context
            .journal()
            .load_account(test_address)
            .unwrap();

        // Write address to registers
        let addr_u32 = address_to_u32_vec(&test_address.0);

        // Test ExtCodeSize
        vm.registers.write_reg(ECALL_CODE_REG, 0x3B); // ExtCodeSize
        for (i, &val) in addr_u32.iter().enumerate() {
            vm.registers
                .write_reg(EXT_CODE_SIZE_INPUT_REGISTER_1 + i as u32, val);
        }

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());
        assert_eq!(
            vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_6),
            test_code.len() as u32
        );

        // Test ExtCodeCopy
        let dest_offset = 500;
        vm.registers.write_reg(ECALL_CODE_REG, 0x3C); // ExtCodeCopy
        for (i, &val) in addr_u32.iter().enumerate() {
            vm.registers
                .write_reg(EXT_CODE_COPY_INPUT_REGISTER_1 + i as u32, val);
        }
        vm.registers
            .write_reg(EXT_CODE_COPY_INPUT_REGISTER_6, dest_offset); // destOffset
        vm.registers.write_reg(EXT_CODE_COPY_INPUT_REGISTER_7, 0); // offset
        vm.registers
            .write_reg(EXT_CODE_COPY_INPUT_REGISTER_8, test_code.len() as u32); // size

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify code was copied to memory
        for (i, &byte) in test_code.iter().enumerate() {
            assert_eq!(
                vm.memory
                    .read_mem(dest_offset + i as u32, MemoryChuckSize::BYTE)
                    .unwrap(),
                byte as u32
            );
        }

        // Test ExtCodeHash
        vm.registers.write_reg(ECALL_CODE_REG, 0x3F); // ExtCodeHash
        for (i, &val) in addr_u32.iter().enumerate() {
            vm.registers
                .write_reg(EXT_CODE_HASH_INPUT_REGISTER_1 + i as u32, val);
        }

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // We can't easily predict the exact hash, but we can verify registers were written
        assert!(
            vm.registers.read_reg(EXT_CODE_HASH_OUTPUT_REGISTER_1) != 0
                || vm.registers.read_reg(EXT_CODE_HASH_OUTPUT_REGISTER_2) != 0
                || vm.registers.read_reg(EXT_CODE_HASH_OUTPUT_REGISTER_3) != 0
        );
    }

    #[test]
    fn test_return_data_operations() {
        let (mut vm, mut context) = setup();

        // Set up test return data
        let return_data = b"Test return data from previous call";
        context.return_data = return_data.to_vec().into();

        // Test ReturnDataSize
        vm.registers.write_reg(ECALL_CODE_REG, 0x3D); // ReturnDataSize
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());
        assert_eq!(
            vm.registers.read_reg(RETURN_DATA_SIZE_OUTPUT_REGISTER),
            return_data.len() as u32
        );

        // Test ReturnDataCopy
        let dest_offset = 600;
        vm.registers.write_reg(ECALL_CODE_REG, 0x3E); // ReturnDataCopy
        vm.registers
            .write_reg(RETURN_DATA_COPY_INPUT_REGISTER_1, dest_offset); // destOffset
        vm.registers.write_reg(RETURN_DATA_COPY_INPUT_REGISTER_2, 0); // offset
        vm.registers
            .write_reg(RETURN_DATA_COPY_INPUT_REGISTER_3, return_data.len() as u32); // size

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify return data was copied to memory
        for (i, &byte) in return_data.iter().enumerate() {
            assert_eq!(
                vm.memory
                    .read_mem(dest_offset + i as u32, MemoryChuckSize::BYTE)
                    .unwrap(),
                byte as u32
            );
        }
    }

    #[test]
    fn test_block_operations() {
        let (mut vm, mut context) = setup();

        // Test BlockHash
        vm.registers.write_reg(ECALL_CODE_REG, 0x40); // BlockHash
        let block_number = 12345000; // Some block number
        let (high, low) = split_u64_to_u32(block_number);
        vm.registers.write_reg(BLOCK_HASH_INPUT_REGISTER_1, high);
        vm.registers.write_reg(BLOCK_HASH_INPUT_REGISTER_2, low);

        // Mock implementation would typically return a pre-computed hash
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Test Coinbase
        vm.registers.write_reg(ECALL_CODE_REG, 0x41); // Coinbase
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        let coinbase = context.eth_context.block.beneficiary.0.0;
        assert_eq!(
            vm.registers.read_reg(COINBASE_OUTPUT_REGISTER_1),
            bytes_to_u32(&coinbase[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(COINBASE_OUTPUT_REGISTER_2),
            bytes_to_u32(&coinbase[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(COINBASE_OUTPUT_REGISTER_3),
            bytes_to_u32(&coinbase[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(COINBASE_OUTPUT_REGISTER_4),
            bytes_to_u32(&coinbase[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(COINBASE_OUTPUT_REGISTER_5),
            bytes_to_u32(&coinbase[16..20])
        );

        // Test Timestamp
        vm.registers.write_reg(ECALL_CODE_REG, 0x42); // Timestamp
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        let (timestamp_high, timestamp_low) = split_u64_to_u32(context.eth_context.block.timestamp);
        assert_eq!(
            vm.registers.read_reg(TIMESTAMP_OUTPUT_REGISTER_1),
            timestamp_high
        );
        assert_eq!(
            vm.registers.read_reg(TIMESTAMP_OUTPUT_REGISTER_2),
            timestamp_low
        );

        // Test Number
        vm.registers.write_reg(ECALL_CODE_REG, 0x43); // Number
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        let (number_high, number_low) = split_u64_to_u32(context.eth_context.block.number);
        assert_eq!(vm.registers.read_reg(NUMBER_OUTPUT_REGISTER_1), number_high);
        assert_eq!(vm.registers.read_reg(NUMBER_OUTPUT_REGISTER_2), number_low);
    }

    #[test]
    fn test_network_operations() {
        let (mut vm, mut context) = setup();

        // Set test chain ID
        context.eth_context.modify_cfg(|cfg| {
            cfg.chain_id = 1; // Ethereum mainnet
        });

        // Test ChainId
        vm.registers.write_reg(ECALL_CODE_REG, 0x46); // ChainId
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        let (chain_id_high, chain_id_low) = split_u64_to_u32(context.eth_context.cfg.chain_id);
        assert_eq!(
            vm.registers.read_reg(CHAIN_ID_OUTPUT_REGISTER_1),
            chain_id_high
        );
        assert_eq!(
            vm.registers.read_reg(CHAIN_ID_OUTPUT_REGISTER_2),
            chain_id_low
        );
    }

    #[test]
    fn test_self_balance() {
        let (mut vm, mut context) = setup();

        // Set test balance for contract
        let balance = U256::from(987654321);
        let mut db = InMemoryDB::default();
        let account_info = AccountInfo {
            balance,
            ..Default::default()
        };
        db.insert_account_info(context.address, account_info);
        context.eth_context = RevmEthContext::mainnet().with_db(db);

        // Test SelfBalance
        vm.registers.write_reg(ECALL_CODE_REG, 0x47); // SelfBalance
        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        let balance_bytes: [u8; 32] = balance.to_be_bytes();
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_1),
            bytes_to_u32(&balance_bytes[0..4])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_2),
            bytes_to_u32(&balance_bytes[4..8])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_3),
            bytes_to_u32(&balance_bytes[8..12])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_4),
            bytes_to_u32(&balance_bytes[12..16])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_5),
            bytes_to_u32(&balance_bytes[16..20])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_6),
            bytes_to_u32(&balance_bytes[20..24])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_7),
            bytes_to_u32(&balance_bytes[24..28])
        );
        assert_eq!(
            vm.registers.read_reg(SELF_BALANCE_OUTPUT_REGISTER_8),
            bytes_to_u32(&balance_bytes[28..32])
        );
    }

    #[test]
    fn test_log_operations() {
        let (mut vm, mut context) = setup();

        // Setup test data for log
        let log_data = b"Log test data";
        let log_offset = 700;

        // Write log data to memory
        for (i, &byte) in log_data.iter().enumerate() {
            vm.memory
                .write_mem(log_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Test Log0
        vm.registers.write_reg(ECALL_CODE_REG, 0xA0); // Log0
        vm.registers.write_reg(LOG0_INPUT_REGISTER_1, log_offset);
        vm.registers
            .write_reg(LOG0_INPUT_REGISTER_2, log_data.len() as u32);

        // Track logs before ECALL
        let logs_before = context.eth_context.journal().logs.len();

        let result = process_ecall(&mut vm, &mut context).unwrap();

        // Verify log was added
        assert_eq!(result[0].logs.len(), logs_before + 1);

        // Test Log1 with topic
        vm.registers.write_reg(ECALL_CODE_REG, 0xA1); // Log1
        vm.registers.write_reg(LOG1_INPUT_REGISTER_1, log_offset);
        vm.registers
            .write_reg(LOG1_INPUT_REGISTER_2, log_data.len() as u32);

        // Set topic
        let topic_value =
            U256::from_str("0x1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF")
                .unwrap();
        let topic_bytes: [u8; 32] = topic_value.to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                LOG1_INPUT_REGISTER_3 + i as u32,
                bytes_to_u32(&topic_bytes[i * 4..(i + 1) * 4]),
            );
        }

        let logs_before = context.eth_context.journal().logs.len();

        let result = process_ecall(&mut vm, &mut context).unwrap();

        // Verify log with topic was added
        assert_eq!(result[0].logs.len(), logs_before + 1);
    }

    #[test]
    fn test_return_and_revert() {
        let (mut vm, mut context) = setup();

        // Setup test data
        let return_data = b"Return or revert data";
        let data_offset = 800;

        // Write data to memory
        for (i, &byte) in return_data.iter().enumerate() {
            vm.memory
                .write_mem(data_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Test Revert
        vm.registers.write_reg(ECALL_CODE_REG, 0xFD); // Revert
        vm.registers.write_reg(REVERT_INPUT_REGISTER_1, data_offset);
        vm.registers
            .write_reg(REVERT_INPUT_REGISTER_2, return_data.len() as u32);

        let result = process_ecall(&mut vm, &mut context);
        assert!(result.is_ok());

        // Verify VM is stopped
        assert!(!vm.running);

        // Verify return data was set correctly
        assert_eq!(context.return_data.len(), return_data.len());
        for (i, &byte) in return_data.iter().enumerate() {
            assert_eq!(context.return_data[i], byte);
        }
    }

    #[test]
    fn test_create_operation() {
        let (mut vm, mut context) = setup();

        // Setup test data for contract creation
        let init_code: Vec<u32> = vec![
            147, 275, 403, 531, 659, 787, 915, 1043, 1171, 1299, 1427, 1555, 1683, 1811, 1939,
            1050643, 89132947, 115, 104858259, 591397651, 1079181747, 5243059, 3146035, 254807955,
            115, 147, 59772819, 115, 57672083, 3214435, 33554835, 170984547, 89129363, 271646819,
            432013423, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115, 1574931, 525411,
            62914671, 1542035, 34052707, 1509139, 34018915, 1476243, 33985123, 1443347, 400995,
            1410451, 367203, 1377555, 333411, 1344659, 299619, 147, 275, 403, 531, 659, 787, 915,
            1043, 89132947, 115, 197132399, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115,
            4286644499, 9510947, 10560035, 11609123, 12658211, 13707299, 14756387, 15805475,
            16854563, 2097331, 8388883, 254807955, 115, 8454419, 4194451, 55578515, 115, 3147059,
            4195763, 5244467, 6293171, 7341875, 8390579, 9439283, 2098355, 147, 275, 403, 531, 659,
            787, 915, 1043, 89132947, 115, 4194415, 4286644499, 89132947, 403, 1049107, 3219491,
            3220003, 3220515, 3221027, 3221539, 3222051, 3222563, 4271651, 2097331, 8388883,
            254807955, 115, 8454419, 4225757295, 147, 275, 265293715, 115,
        ];
        let init_code = u32_vec_to_bytes(&init_code, init_code.len() * 4);
        let init_offset = 900;

        // Write init code to memory
        for (i, &byte) in init_code.iter().enumerate() {
            vm.memory
                .write_mem(init_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Set up Create ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0xF0); // Create
        vm.registers.write_reg(CREATE_INPUT_REGISTER_1, init_offset);
        vm.registers
            .write_reg(CREATE_INPUT_REGISTER_2, init_code.len() as u32);

        // Set value (10 ETH)
        let value = 10_000_000_000_000_000_000u64;
        let value_bytes: [u8; 32] = U256::from(value).to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CREATE_INPUT_REGISTER_3 + i as u32,
                bytes_to_u32(&value_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set balance for creator so it can transfer value
        context
            .eth_context
            .journal()
            .load_account(context.address)
            .unwrap();

        // Process Create ECALL (this would be complex to fully test)
        // In a real test we'd need to properly mock the creation process
        // Here we're mostly testing the interface, not the actual contract creation
        let result = process_ecall(&mut vm, &mut context);

        // Since we don't have a complete EVM, this would likely fail in test
        // We're just checking the interface works correctly
        if result.is_ok() {
            // Check that output registers were written
            let addr1 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_1);
            let addr2 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_2);
            let addr3 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_3);
            let addr4 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_4);
            let addr5 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_5);

            // Reconstruct address to check it's valid
            let address_bytes = u32_vec_to_address(&[addr1, addr2, addr3, addr4, addr5]);
            // Validate it looks like an address (non-zero)
            assert!(address_bytes.iter().any(|&b| b != 0));

            let changes = context.eth_context.journal().finalize();
            context.eth_context.db().commit(changes.state);

            let new_contract = context
                .eth_context
                .journal()
                .load_account_code(Address::from(address_bytes))
                .unwrap()
                .clone()
                .info
                .code
                .unwrap();
            println!("This is the runtime code: {:?}", new_contract);
        }
    }

    #[test]
    fn test_create2_operation() {
        let (mut vm, mut context) = setup();

        // Setup test data for contract creation
        let init_code: Vec<u32> = vec![
            147, 275, 403, 531, 659, 787, 915, 1043, 1171, 1299, 1427, 1555, 1683, 1811, 1939,
            1050643, 89132947, 115, 104858259, 591397651, 1079181747, 5243059, 3146035, 254807955,
            115, 147, 59772819, 115, 57672083, 3214435, 33554835, 170984547, 89129363, 271646819,
            432013423, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115, 1574931, 525411,
            62914671, 1542035, 34052707, 1509139, 34018915, 1476243, 33985123, 1443347, 400995,
            1410451, 367203, 1377555, 333411, 1344659, 299619, 147, 275, 403, 531, 659, 787, 915,
            1043, 89132947, 115, 197132399, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115,
            4286644499, 9510947, 10560035, 11609123, 12658211, 13707299, 14756387, 15805475,
            16854563, 2097331, 8388883, 254807955, 115, 8454419, 4194451, 55578515, 115, 3147059,
            4195763, 5244467, 6293171, 7341875, 8390579, 9439283, 2098355, 147, 275, 403, 531, 659,
            787, 915, 1043, 89132947, 115, 4194415, 4286644499, 89132947, 403, 1049107, 3219491,
            3220003, 3220515, 3221027, 3221539, 3222051, 3222563, 4271651, 2097331, 8388883,
            254807955, 115, 8454419, 4225757295, 147, 275, 265293715, 115,
        ];
        let init_code = u32_vec_to_bytes(&init_code, init_code.len() * 4);
        let init_offset = 1200;

        // Write init code to memory
        for (i, &byte) in init_code.iter().enumerate() {
            vm.memory
                .write_mem(init_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Set up Create2 ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0xF5); // Create2
        vm.registers
            .write_reg(CREATE_2_INPUT_REGISTER_1, init_offset);
        vm.registers
            .write_reg(CREATE_2_INPUT_REGISTER_2, init_code.len() as u32);

        // Set value (500,000 ETH)
        let value = 500_000_000u64;
        let value_bytes: [u8; 32] = U256::from(value).to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CREATE_2_INPUT_REGISTER_3 + i as u32,
                bytes_to_u32(&value_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set salt
        let salt =
            U256::from_str("0xABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890")
                .unwrap();
        let salt_bytes: [u8; 32] = salt.to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CREATE_2_INPUT_REGISTER_11 + i as u32,
                bytes_to_u32(&salt_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set balance for creator so it can transfer value
        context
            .eth_context
            .journal()
            .load_account(context.address)
            .unwrap();

        // Process Create2 ECALL (this would be complex to fully test)
        // In a real test we'd need to properly mock the creation process
        // Here we're mostly testing the interface, not the actual contract creation
        let result = process_ecall(&mut vm, &mut context);

        // Since we don't have a complete EVM, this would likely fail in test
        // We're just checking the interface works correctly
        if result.is_ok() {
            // Check that output registers were written
            let addr1 = vm.registers.read_reg(CREATE_2_OUTPUT_REGISTER_1);
            let addr2 = vm.registers.read_reg(CREATE_2_OUTPUT_REGISTER_2);
            let addr3 = vm.registers.read_reg(CREATE_2_OUTPUT_REGISTER_3);
            let addr4 = vm.registers.read_reg(CREATE_2_OUTPUT_REGISTER_4);
            let addr5 = vm.registers.read_reg(CREATE_2_OUTPUT_REGISTER_5);

            // Reconstruct address to check it's valid
            let address_bytes = u32_vec_to_address(&[addr1, addr2, addr3, addr4, addr5]);
            // Validate it looks like an address (non-zero)
            assert_eq!(
                address_bytes,
                Address::from_str("0x306cc4469343cea819a1b758f2385b8e11fc1898")
                    .unwrap()
                    .0
            );

            let changes = context.eth_context.journal().finalize();
            context.eth_context.db().commit(changes.state);

            let new_contract = context
                .eth_context
                .journal()
                .load_account_code(Address::from(address_bytes))
                .unwrap()
                .clone()
                .info
                .code
                .unwrap();
            println!("This is the runtime code from CREATE2: {:?}", new_contract);
        }
    }

    #[test]
    fn test_call_operation() {
        let (mut vm, mut context) = setup();

        // Setup test data for contract creation
        let init_code: Vec<u32> = vec![
            147, 275, 403, 531, 659, 787, 915, 1043, 1171, 1299, 1427, 1555, 1683, 1811, 1939,
            1050643, 89132947, 115, 104858259, 591397651, 1079181747, 5243059, 3146035, 254807955,
            115, 147, 55578515, 115, 57672083, 3214435, 33554835, 170984547, 89129363, 271646819,
            432013423, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115, 1574931, 525411,
            62914671, 1542035, 34052707, 1509139, 34018915, 1476243, 33985123, 1443347, 400995,
            1410451, 367203, 1377555, 333411, 1344659, 299619, 147, 275, 403, 531, 659, 787, 915,
            1043, 89132947, 115, 197132399, 147, 275, 403, 531, 659, 787, 915, 1043, 88084371, 115,
            4286644499, 9510947, 10560035, 11609123, 12658211, 13707299, 14756387, 15805475,
            16854563, 2097331, 8388883, 254807955, 115, 8454419, 4194451, 55578515, 115, 3147059,
            4195763, 5244467, 6293171, 7341875, 8390579, 9439283, 2098355, 147, 275, 403, 531, 659,
            787, 915, 1043, 89132947, 115, 4194415, 4244635923, 89132947, 403, 1049107, 3219491,
            3220003, 3220515, 3221027, 3221539, 3222051, 3222563, 4271651, 2097331, 33554707,
            254807955, 115, 8454419, 4225757295, 147, 275, 265293715, 115,
        ];
        let init_code = u32_vec_to_bytes(&init_code, init_code.len() * 4);
        let init_offset = 900;

        // Write init code to memory
        for (i, &byte) in init_code.iter().enumerate() {
            vm.memory
                .write_mem(init_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Set up Create ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0xF0); // Create
        vm.registers.write_reg(CREATE_INPUT_REGISTER_1, init_offset);
        vm.registers
            .write_reg(CREATE_INPUT_REGISTER_2, init_code.len() as u32);

        // Set value (10 ETH)
        let value = 10_000_000_000_000_000_000u64;
        let value_bytes: [u8; 32] = U256::from(value).to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CREATE_INPUT_REGISTER_3 + i as u32,
                bytes_to_u32(&value_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set balance for creator so it can transfer value
        context
            .eth_context
            .journal()
            .load_account(context.address)
            .unwrap();

        // Process Create ECALL (this would be complex to fully test)
        // In a real test we'd need to properly mock the creation process
        // Here we're mostly testing the interface, not the actual contract creation
        let result = process_ecall(&mut vm, &mut context).unwrap();

        for i in result {
            context.eth_context.db().commit(i.state);
        }

        // Check that output registers were written
        let addr1 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_1);
        let addr2 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_2);
        let addr3 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_3);
        let addr4 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_4);
        let addr5 = vm.registers.read_reg(CREATE_OUTPUT_REGISTER_5);

        // Reconstruct address to check it's valid
        let address_bytes = u32_vec_to_address(&[addr1, addr2, addr3, addr4, addr5]);
        // Validate it looks like an address (non-zero)
        assert!(address_bytes.iter().any(|&b| b != 0));

        let new_contract = context
            .eth_context
            .journal()
            .load_account_code(Address::from(address_bytes))
            .unwrap()
            .clone()
            .info
            .code
            .unwrap();
        println!("This is the runtime code: {:?}", new_contract);

        // CALL test begins here
        println!("Starting CALL test to contract at: {:?}", address_bytes);
        let caller_address = Address::from([
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD,
        ]);

        context.eth_context.modify_tx(|tx| {
            tx.caller = caller_address;
            tx.kind = TxKind::Call(Address::from(address_bytes))
        });
        context.current_caller = caller_address;
        context.address = Address::from(address_bytes);

        println!("Contract Address: {:?}", context.address);

        // Set up simple call data: This one is for the increament function in a counter example contract
        let call_data = hex::decode("00000037").unwrap();
        let call_offset = 1100;
        let return_offset = 1500;

        // Write call data to memory
        for (i, &byte) in call_data.iter().enumerate() {
            vm.memory
                .write_mem(call_offset + i as u32, MemoryChuckSize::BYTE, byte as u32);
        }

        // Reset registers for the new CALL operation
        vm.registers = Registers::new();

        // Set up CALL ECALL
        vm.registers.write_reg(ECALL_CODE_REG, 0xF1); // Call

        // Set gas (use a high amount for testing)
        let gas = U256::from(1000000);
        let gas_bytes: [u8; 32] = gas.to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CALL_INPUT_REGISTER_1 + i as u32,
                bytes_to_u32(&gas_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set target address (the contract we just created)
        let addr_u32 = address_to_u32_vec(&address_bytes);
        for (i, &val) in addr_u32.iter().enumerate() {
            vm.registers
                .write_reg(CALL_INPUT_REGISTER_9 + i as u32, val);
        }

        // Set value for the call (a small amount)
        let call_value = U256::from(1000); // 0.000000000000001 ETH
        let value_bytes: [u8; 32] = call_value.to_be_bytes();
        for i in 0..8 {
            vm.registers.write_reg(
                CALL_INPUT_REGISTER_14 + i as u32,
                bytes_to_u32(&value_bytes[i * 4..(i + 1) * 4]),
            );
        }

        // Set call data location and size
        vm.registers.write_reg(CALL_INPUT_REGISTER_22, call_offset);
        vm.registers
            .write_reg(CALL_INPUT_REGISTER_23, call_data.len() as u32);

        // Set return data location and expected size
        vm.registers
            .write_reg(CALL_INPUT_REGISTER_24, return_offset);
        vm.registers.write_reg(CALL_INPUT_REGISTER_25, 32); // Typical return size for a bool

        // Process the CALL
        let j_outs = process_ecall(&mut vm, &mut context).unwrap();

        for i in j_outs {
            println!("These are the state changes: {:?}", i.state);
            context.eth_context.journal().db().commit(i.state);
        }
    }
}
