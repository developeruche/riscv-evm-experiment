use crate::{
    context::Context,
    utils::{bytes_to_u32, bytes_to_u32_vec, u32_vec_to_address},
    vm::{VMErrors, Vm},
};
use revm::{
    context::ContextTr,
    interpreter::Host,
    primitives::{Address, keccak256},
};
use riscv_evm_core::{MemoryChuckSize, e_constants::*, interfaces::MemoryInterface};

pub fn process_ecall(vm: &mut Vm, context: &mut Context) -> Result<(), VMErrors> {
    let e_call_code = vm.registers.read_reg(ECALL_CODE_REG);

    match RiscvEVMECalls::from_u32(e_call_code) {
        Some(rv_ec) => match rv_ec {
            RiscvEVMECalls::Keccak256 => {
                // This would load from memory data from `offset` and `size`,
                // the `offest` is an address in memroy where the read should start from
                // `size` is the number of bytes has the data that is to be hashed.
                // after the hashing is done, it would be stored in 8 registers
                let offset = vm.registers.read_reg(KECCAK256_OFFSET_REGISTER);
                let size = vm.registers.read_reg(KECCAK256_SIZE_REGISTER);
                let mut data = vec![0u8; size as usize];
                for i in 0..size {
                    data[i as usize] =
                        vm.memory.read_mem(offset, MemoryChuckSize::BYTE).unwrap() as u8;
                }
                let hash = keccak256(&data);

                // writing 256 bits to 8 regiters
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_1, bytes_to_u32(&hash.0[0..4]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_2, bytes_to_u32(&hash.0[4..8]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_3, bytes_to_u32(&hash.0[8..12]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_4, bytes_to_u32(&hash.0[12..16]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_5, bytes_to_u32(&hash.0[16..20]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_6, bytes_to_u32(&hash.0[20..24]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_7, bytes_to_u32(&hash.0[24..28]));
                vm.registers
                    .write_reg(KECCAK256_OUTPUT_REGITER_8, bytes_to_u32(&hash.0[28..32]));

                Ok(())
            }
            RiscvEVMECalls::Address => {
                // This branch would load the address of this current running contract from context
                // to 5 regiters
                let address = context.address.0.0;

                // writing 160 bits (20 bytes) to register
                vm.registers
                    .write_reg(ADDRESS_REGISTER_1, bytes_to_u32(&address[0..4]));
                vm.registers
                    .write_reg(ADDRESS_REGISTER_2, bytes_to_u32(&address[4..8]));
                vm.registers
                    .write_reg(ADDRESS_REGISTER_3, bytes_to_u32(&address[8..12]));
                vm.registers
                    .write_reg(ADDRESS_REGISTER_4, bytes_to_u32(&address[12..16]));
                vm.registers
                    .write_reg(ADDRESS_REGISTER_5, bytes_to_u32(&address[16..20]));

                Ok(())
            }
            RiscvEVMECalls::Balance => {
                // Construct the address that is to be read by reading 5 registers, reconstruct the address
                // query the balance from context, write this balance to 8 new registers
                let address_u32_1 = vm.registers.read_reg(BALANCE_INPUT_REGISTER_1);
                let address_u32_2 = vm.registers.read_reg(BALANCE_INPUT_REGISTER_2);
                let address_u32_3 = vm.registers.read_reg(BALANCE_INPUT_REGISTER_3);
                let address_u32_4 = vm.registers.read_reg(BALANCE_INPUT_REGISTER_4);
                let address_u32_5 = vm.registers.read_reg(BALANCE_INPUT_REGISTER_5);

                let address = u32_vec_to_address(&vec![
                    address_u32_1,
                    address_u32_2,
                    address_u32_3,
                    address_u32_4,
                    address_u32_5,
                ]);

                let balance: [u8; 32] = context
                    .eth_context
                    .balance(Address::new(address))
                    .unwrap_or_default()
                    .data
                    .to_be_bytes();

                // writing 256 bits to 8 regiters
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_1, bytes_to_u32(&balance[0..4]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_2, bytes_to_u32(&balance[4..8]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_3, bytes_to_u32(&balance[8..12]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_4, bytes_to_u32(&balance[12..16]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_5, bytes_to_u32(&balance[16..20]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_6, bytes_to_u32(&balance[20..24]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_7, bytes_to_u32(&balance[24..28]));
                vm.registers
                    .write_reg(BALANCE_OUTPUT_REGISTER_8, bytes_to_u32(&balance[28..32]));

                Ok(())
            }
            RiscvEVMECalls::Origin => {
                let origin = context.eth_context.tx.caller.0;

                // Writing this origin to five registers
                vm.registers
                    .write_reg(ORIGIN_OUTPUT_REGISTER_1, bytes_to_u32(&origin[0..4]));
                vm.registers
                    .write_reg(ORIGIN_OUTPUT_REGISTER_2, bytes_to_u32(&origin[4..8]));
                vm.registers
                    .write_reg(ORIGIN_OUTPUT_REGISTER_3, bytes_to_u32(&origin[8..12]));
                vm.registers
                    .write_reg(ORIGIN_OUTPUT_REGISTER_4, bytes_to_u32(&origin[12..16]));
                vm.registers
                    .write_reg(ORIGIN_OUTPUT_REGISTER_5, bytes_to_u32(&origin[16..20]));

                Ok(())
            }
            RiscvEVMECalls::Caller => {
                let origin = context.current_caller.0;

                // Writing this origin to five registers
                vm.registers
                    .write_reg(CALLER_OUTPUT_REGISTER_1, bytes_to_u32(&origin[0..4]));
                vm.registers
                    .write_reg(CALLER_OUTPUT_REGISTER_2, bytes_to_u32(&origin[4..8]));
                vm.registers
                    .write_reg(CALLER_OUTPUT_REGISTER_3, bytes_to_u32(&origin[8..12]));
                vm.registers
                    .write_reg(CALLER_OUTPUT_REGISTER_4, bytes_to_u32(&origin[12..16]));
                vm.registers
                    .write_reg(CALLER_OUTPUT_REGISTER_5, bytes_to_u32(&origin[16..20]));

                Ok(())
            }
            RiscvEVMECalls::CallValue => {
                // Load the vaule from context into a 8 registers
                let value: [u8; 32] = context.eth_context.tx.value.to_be_bytes();

                // writing 256 bits to 8 regiters
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_1, bytes_to_u32(&value[0..4]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_2, bytes_to_u32(&value[4..8]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_3, bytes_to_u32(&value[8..12]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_4, bytes_to_u32(&value[12..16]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_5, bytes_to_u32(&value[16..20]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_6, bytes_to_u32(&value[20..24]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_7, bytes_to_u32(&value[24..28]));
                vm.registers
                    .write_reg(CALL_VALUE_OUTPUT_REGISTER_8, bytes_to_u32(&value[28..32]));

                Ok(())
            }
            RiscvEVMECalls::CallDataLoad => {
                // This would load 32bytes of the call data to 8 registers
                // The offset this 32bytes should come from is gotten from a register.
                let offset = vm.registers.read_reg(CALL_DATA_LOAD_INPUT_REGISTER);
                let mut data = Vec::new();
                for i in offset as usize..(offset + 32) as usize {
                    data.push(*context.eth_context.tx.data.get(i).unwrap_or(&0u8));
                }
                // writing 256 bits to 8 regiters
                vm.registers
                    .write_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_1, bytes_to_u32(&data[0..4]));
                vm.registers
                    .write_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_2, bytes_to_u32(&data[4..8]));
                vm.registers
                    .write_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_3, bytes_to_u32(&data[8..12]));
                vm.registers.write_reg(
                    CALL_DATA_LOAD_OUTPUT_REGISTER_4,
                    bytes_to_u32(&data[12..16]),
                );
                vm.registers.write_reg(
                    CALL_DATA_LOAD_OUTPUT_REGISTER_5,
                    bytes_to_u32(&data[16..20]),
                );
                vm.registers.write_reg(
                    CALL_DATA_LOAD_OUTPUT_REGISTER_6,
                    bytes_to_u32(&data[20..24]),
                );
                vm.registers.write_reg(
                    CALL_DATA_LOAD_OUTPUT_REGISTER_7,
                    bytes_to_u32(&data[24..28]),
                );
                vm.registers.write_reg(
                    CALL_DATA_LOAD_OUTPUT_REGISTER_8,
                    bytes_to_u32(&data[28..32]),
                );

                Ok(())
            }
            RiscvEVMECalls::CallDataSize => {
                // This load to a register the number of bytes present in the calldata
                // into a register
                let size = context.eth_context.tx.data.len() as u32;

                vm.registers
                    .write_reg(CALL_DATA_LOAD_OUTPUT_REGISTER_1, size);

                Ok(())
            }
            RiscvEVMECalls::CallDataCopy => {
                // Loads the dest_offset, offset and size from registers
                let dest_offset = vm.registers.read_reg(CALL_DATA_COPY_INPUT_REGISTER_1);
                let offset = vm.registers.read_reg(CALL_DATA_COPY_INPUT_REGISTER_2);
                let size = vm.registers.read_reg(CALL_DATA_COPY_INPUT_REGISTER_3);

                let mut data = Vec::new();
                for i in offset as usize..(offset + size) as usize {
                    data.push(*context.eth_context.tx.data.get(i).unwrap_or(&0u8));
                }

                // writing to memory
                for (i, byte) in data.iter().enumerate() {
                    let byte_addr = dest_offset + i as u32;
                    vm.memory
                        .write_mem(byte_addr as u32, MemoryChuckSize::BYTE, *byte as u32);
                }

                Ok(())
            }
            RiscvEVMECalls::CodeSize => {
                // This function retruns the code of the currently excecuting contract
                let contract_accout = context
                    .eth_context
                    .journaled_state
                    .account(context.address)
                    .clone();
                let code_len = contract_accout
                    .info
                    .code
                    .unwrap_or_default()
                    .bytecode()
                    .len() as u32;

                vm.registers.write_reg(CODE_SIZE_OUT_REGISTER, code_len);

                Ok(())
            }
            RiscvEVMECalls::CodeCopy => {
                // This copies the code of the current running contract to memory
                // the dest_offest, offset and size is gotten from the register
                // Loads the dest_offset, offset and size from registers
                let dest_offset = vm.registers.read_reg(CODE_COPY_INPUT_REGISTER_1);
                let offset = vm.registers.read_reg(CODE_COPY_INPUT_REGISTER_2);
                let size = vm.registers.read_reg(CODE_COPY_INPUT_REGISTER_3);

                let contract_accout = context
                    .eth_context
                    .journaled_state
                    .account(context.address)
                    .clone();
                let code = contract_accout
                    .info
                    .code
                    .unwrap_or_default()
                    .bytecode()
                    .0
                    .clone()
                    .to_vec();

                let mut data = Vec::new();
                for i in offset as usize..(offset + size) as usize {
                    data.push(*code.get(i).unwrap_or(&0u8));
                }

                // writing to memory
                for (i, byte) in data.iter().enumerate() {
                    let byte_addr = dest_offset + i as u32;
                    vm.memory
                        .write_mem(byte_addr as u32, MemoryChuckSize::BYTE, *byte as u32);
                }

                Ok(())
            }
            RiscvEVMECalls::GasPrice => {
                // This returns the gas price in the current enviroment
                let gas_price: [u8; 32] = context.eth_context.effective_gas_price().to_be_bytes();

                // writing 256 bits to 8 regiters
                vm.registers
                    .write_reg(GAS_PRICE_OUTPUT_REGISTER_1, bytes_to_u32(&gas_price[0..4]));
                vm.registers
                    .write_reg(GAS_PRICE_OUTPUT_REGISTER_2, bytes_to_u32(&gas_price[4..8]));
                vm.registers
                    .write_reg(GAS_PRICE_OUTPUT_REGISTER_3, bytes_to_u32(&gas_price[8..12]));
                vm.registers.write_reg(
                    GAS_PRICE_OUTPUT_REGISTER_4,
                    bytes_to_u32(&gas_price[12..16]),
                );
                vm.registers.write_reg(
                    GAS_PRICE_OUTPUT_REGISTER_5,
                    bytes_to_u32(&gas_price[16..20]),
                );
                vm.registers.write_reg(
                    GAS_PRICE_OUTPUT_REGISTER_6,
                    bytes_to_u32(&gas_price[20..24]),
                );
                vm.registers.write_reg(
                    GAS_PRICE_OUTPUT_REGISTER_7,
                    bytes_to_u32(&gas_price[24..28]),
                );
                vm.registers.write_reg(
                    GAS_PRICE_OUTPUT_REGISTER_8,
                    bytes_to_u32(&gas_price[28..32]),
                );

                Ok(())
            }
            RiscvEVMECalls::ExtCodeSize => {
                // This would copy the code of a given address to memory
                // This function retruns the code of the currently excecuting contract
                let address_u32_1 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_1);
                let address_u32_2 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_2);
                let address_u32_3 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_3);
                let address_u32_4 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_4);
                let address_u32_5 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_5);

                let address = u32_vec_to_address(&vec![
                    address_u32_1,
                    address_u32_2,
                    address_u32_3,
                    address_u32_4,
                    address_u32_5,
                ]);

                let contract_accout = context
                    .eth_context
                    .journaled_state
                    .account(Address::from(address))
                    .clone();
                let code_len = contract_accout
                    .info
                    .code
                    .unwrap_or_default()
                    .bytecode()
                    .len() as u32;

                vm.registers.write_reg(CODE_SIZE_OUT_REGISTER, code_len);

                Ok(())
            }
            RiscvEVMECalls::ExtCodeCopy => {
                let address_u32_1 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_1);
                let address_u32_2 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_2);
                let address_u32_3 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_3);
                let address_u32_4 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_4);
                let address_u32_5 = vm.registers.read_reg(EXT_CODE_SIZE_INPUT_REGISTER_5);

                let dest_offset = vm.registers.read_reg(EXT_CODE_COPY_INPUT_REGISTER_6);
                let offset = vm.registers.read_reg(EXT_CODE_COPY_INPUT_REGISTER_7);
                let size = vm.registers.read_reg(EXT_CODE_COPY_INPUT_REGISTER_8);

                let address = u32_vec_to_address(&vec![
                    address_u32_1,
                    address_u32_2,
                    address_u32_3,
                    address_u32_4,
                    address_u32_5,
                ]);

                let contract_accout = context
                    .eth_context
                    .journaled_state
                    .account(Address::from(address))
                    .clone();
                let code = contract_accout
                    .info
                    .code
                    .unwrap_or_default()
                    .bytecode()
                    .0
                    .clone()
                    .to_vec();

                let mut data = Vec::new();
                for i in offset as usize..(offset + size) as usize {
                    data.push(*code.get(i).unwrap_or(&0u8));
                }

                // writing to memory
                for (i, byte) in data.iter().enumerate() {
                    let byte_addr = dest_offset + i as u32;
                    vm.memory
                        .write_mem(byte_addr as u32, MemoryChuckSize::BYTE, *byte as u32);
                }

                Ok(())
            }
            RiscvEVMECalls::ReturnDataSize => todo!(),
            RiscvEVMECalls::ReturnDataCopy => todo!(),
            RiscvEVMECalls::ExtCodeHash => todo!(),
            RiscvEVMECalls::BlockHash => todo!(),
            RiscvEVMECalls::Coinbase => todo!(),
            RiscvEVMECalls::Timestamp => todo!(),
            RiscvEVMECalls::Number => todo!(),
            RiscvEVMECalls::PrevRandao => todo!(),
            RiscvEVMECalls::GasLimit => todo!(),
            RiscvEVMECalls::ChainId => todo!(),
            RiscvEVMECalls::SelfBalance => todo!(),
            RiscvEVMECalls::BaseFee => todo!(),
            RiscvEVMECalls::BlobHash => todo!(),
            RiscvEVMECalls::BlobBaseFee => todo!(),
            RiscvEVMECalls::Gas => todo!(),
            RiscvEVMECalls::Log0 => todo!(),
            RiscvEVMECalls::Log1 => todo!(),
            RiscvEVMECalls::Log2 => todo!(),
            RiscvEVMECalls::Log3 => todo!(),
            RiscvEVMECalls::Log4 => todo!(),
            RiscvEVMECalls::Create => todo!(),
            RiscvEVMECalls::Call => todo!(),
            RiscvEVMECalls::CallCode => todo!(),
            RiscvEVMECalls::Return => todo!(),
            RiscvEVMECalls::DelegateCall => todo!(),
            RiscvEVMECalls::Create2 => todo!(),
            RiscvEVMECalls::StaticCall => todo!(),
            RiscvEVMECalls::Revert => todo!(),
        },
        None => Err(VMErrors::EnvironmentError),
    }
}
