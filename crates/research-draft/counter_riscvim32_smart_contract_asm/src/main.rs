use revm::{
    Context as RevmEthContext, DatabaseCommit, MainContext,
    context::{ContextTr, JournalTr},
    database::CacheDB,
    primitives::{Address, U256},
};
use riscv_assembler::assembler::assemble;
use riscv_evm::{
    context::Context,
    ecall_manager::process_ecall,
    riscv_evm_core::{MemoryChuckSize, e_constants::*, interfaces::MemoryInterface},
    utils::{bytes_to_u32, u32_vec_to_address, u32_vec_to_bytes},
    vm::Vm,
};
mod contract;

fn main() {
    let from_addr = Address::from([
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        0x00, 0xAA, 0xBB, 0xCC, 0xDD,
    ]);

    let mut vm = Vm::new();
    let eth_context = RevmEthContext::mainnet().with_db(CacheDB::default());
    let mut context = Context::new(eth_context);

    context.current_caller = from_addr;

    // ===================================
    // deploying the counter contract
    // ===================================
    let init_code = assemble(contract::ASSEMBLE_CODE).unwrap().code;
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

    // Set value (0.0..01ETH)
    let value = 10_000_000_000_000_000_000u64;
    let value_bytes: [u8; 32] = U256::from(value).to_be_bytes();
    for i in 0..8 {
        vm.registers.write_reg(
            CREATE_INPUT_REGISTER_3 + i as u32,
            bytes_to_u32(&value_bytes[i * 4..(i + 1) * 4]),
        );
    }

    // Load account of the creator so it can transfer value
    context
        .eth_context
        .journal()
        .load_account(context.address)
        .unwrap();

    // Going ahead to excecute this ecall
    let result = process_ecall(&mut vm, &mut context).unwrap();

    // Commit all states changes to database
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
    println!("Address: {:?}", Address::from(address_bytes));

    context
        .eth_context
        .journal()
        .load_account(Address::from(address_bytes))
        .unwrap();
    let new_contract = context
        .eth_context
        .journal()
        .load_account_code(Address::from(address_bytes))
        .unwrap()
        .clone()
        .info
        .code
        .unwrap();
    println!("This is the runtime code: {:?}", new_contract); //NOTE!!!: this runtime code is a RISCV bytecode not the native EVM bytecode
}
