use crate::{
    context::Context,
    utils::{bytes_to_u32, u32_vec_to_address},
    vm::{VMErrors, Vm},
};
use revm::{
    interpreter::Host,
    primitives::{Address, keccak256},
};
use riscv_evm_core::{
    MemoryChuckSize,
    e_constants::{
        ADDRESS_REGISTER_1, ADDRESS_REGISTER_2, ADDRESS_REGISTER_3, ADDRESS_REGISTER_4,
        ADDRESS_REGISTER_5, BALANCE_INPUT_REGISTER_1, BALANCE_INPUT_REGISTER_2,
        BALANCE_INPUT_REGISTER_3, BALANCE_INPUT_REGISTER_4, BALANCE_INPUT_REGISTER_5,
        BALANCE_OUTPUT_REGISTER_1, BALANCE_OUTPUT_REGISTER_2, BALANCE_OUTPUT_REGISTER_3,
        BALANCE_OUTPUT_REGISTER_4, BALANCE_OUTPUT_REGISTER_5, BALANCE_OUTPUT_REGISTER_6,
        BALANCE_OUTPUT_REGISTER_7, BALANCE_OUTPUT_REGISTER_8, CALLER_OUTPUT_REGISTER_1,
        CALLER_OUTPUT_REGISTER_2, CALLER_OUTPUT_REGISTER_3, CALLER_OUTPUT_REGISTER_4,
        CALLER_OUTPUT_REGISTER_5, ECALL_CODE_REG, KECCAK256_OFFSET_REGISTER,
        KECCAK256_OUTPUT_REGITER_1, KECCAK256_OUTPUT_REGITER_2, KECCAK256_OUTPUT_REGITER_3,
        KECCAK256_OUTPUT_REGITER_4, KECCAK256_OUTPUT_REGITER_5, KECCAK256_OUTPUT_REGITER_6,
        KECCAK256_OUTPUT_REGITER_7, KECCAK256_OUTPUT_REGITER_8, KECCAK256_SIZE_REGISTER,
        ORIGIN_OUTPUT_REGISTER_1, ORIGIN_OUTPUT_REGISTER_2, ORIGIN_OUTPUT_REGISTER_3,
        ORIGIN_OUTPUT_REGISTER_4, ORIGIN_OUTPUT_REGISTER_5, RiscvEVMECalls,
    },
    interfaces::MemoryInterface,
};

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
            RiscvEVMECalls::CallValue => todo!(),
            RiscvEVMECalls::CallDataLoad => todo!(),
            RiscvEVMECalls::CallDataSize => todo!(),
            RiscvEVMECalls::CallDataCopy => todo!(),
            RiscvEVMECalls::CodeSize => todo!(),
            RiscvEVMECalls::CodeCopy => todo!(),
            RiscvEVMECalls::GasPrice => todo!(),
            RiscvEVMECalls::ExtCodeSize => todo!(),
            RiscvEVMECalls::ExtCodeCopy => todo!(),
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
