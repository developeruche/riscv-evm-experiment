use crate::vm::{VMErrors, Vm};
use riscv_evm_core::{interfaces::MemoryInterface, MemoryChuckSize};

pub fn process_load_to_reg(
    vm: &mut Vm,
    decoded_instruction: &crate::instructions::IType,
    mem_chuck_size: MemoryChuckSize,
    is_signed: bool,
) -> Result<(), VMErrors> {
    let addr = vm
        .registers
        .read_reg(decoded_instruction.rs1 as u32)
        .wrapping_add(decoded_instruction.imm as u32);

    let align_mask = match mem_chuck_size {
        MemoryChuckSize::BYTE => 0x0,
        MemoryChuckSize::HalfWord => 0x1,
        MemoryChuckSize::WordSize => 0x3,
    };

    if (addr & align_mask) != 0x0 {
        return Err(VMErrors::MemoryError);
    }

    let mut load_data = match vm.memory.read_mem(addr, mem_chuck_size.clone()) {
        Some(d) => d,
        None => {
            return Err(VMErrors::MemoryLoadError);
        }
    };

    if is_signed {
        load_data = (match mem_chuck_size {
            MemoryChuckSize::BYTE => (load_data as i8) as i32,
            MemoryChuckSize::HalfWord => (load_data as i16) as i32,
            MemoryChuckSize::WordSize => load_data as i32,
        }) as u32;
    }

    vm.registers
        .write_reg(decoded_instruction.rd as u32, load_data);

    Ok(())
}

pub fn process_store_to_memory(
    vm: &mut Vm,
    decoded_instruction: &crate::instructions::SType,
    mem_chuck_size: MemoryChuckSize,
) -> Result<(), VMErrors> {
    let addr = vm
        .registers
        .read_reg(decoded_instruction.rs1 as u32)
        .wrapping_add(decoded_instruction.imm as u32);
    let data_to_store = vm.registers.read_reg(decoded_instruction.rs2 as u32);

    let align_mask = match mem_chuck_size {
        MemoryChuckSize::BYTE => 0x0,
        MemoryChuckSize::HalfWord => 0x1,
        MemoryChuckSize::WordSize => 0x3,
    };

    if (addr & align_mask) != 0x0 {
        return Err(VMErrors::MemoryError);
    }

    if !vm
        .memory
        .write_mem(addr, mem_chuck_size.clone(), data_to_store)
    {
        return Err(VMErrors::MemoryStoreError);
    }

    Ok(())
}
