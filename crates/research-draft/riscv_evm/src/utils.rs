use crate::vm::{VMErrors, Vm};
use riscv_evm_core::{MemoryChuckSize, interfaces::MemoryInterface};

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

// Function to convert a slice of 4 bytes to a u32 (big-endian)
pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | (bytes[3] as u32)
}

/// Converts a 20-byte Ethereum address into a vector of 5 u32 values (big-endian format)
pub fn address_to_u32_vec(address: &[u8; 20]) -> Vec<u32> {
    let mut result = Vec::with_capacity(5);

    // Process 4 bytes at a time
    for chunk_idx in 0..5 {
        let offset = chunk_idx * 4;
        let value = u32::from_be_bytes([
            address[offset],
            address[offset + 1],
            address[offset + 2],
            address[offset + 3],
        ]);
        result.push(value);
    }

    result
}

/// Converts a vector of 5 u32 values back to a 20-byte Ethereum address
pub fn u32_vec_to_address(u32_values: &[u32]) -> [u8; 20] {
    assert_eq!(u32_values.len(), 5, "Expected exactly 5 u32 values");

    let mut result = [0u8; 20];

    // Process each u32 and extract its 4 bytes
    for (idx, &value) in u32_values.iter().enumerate() {
        let bytes = value.to_be_bytes();
        let offset = idx * 4;

        // Copy the 4 bytes to the appropriate position
        result[offset..offset + 4].copy_from_slice(&bytes);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address_conversion() {
        let address = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
            0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
        ];

        // Convert address to u32 vector
        let u32_values = address_to_u32_vec(&address);
        println!("u32 values: {:?}", u32_values);

        // Convert back to address
        let reconstructed = u32_vec_to_address(&u32_values);
        println!("Reconstructed address: {:?}", reconstructed);

        // Verify the conversion is correct
        assert_eq!(address, reconstructed);
    }

    #[test]
    fn test_bytes_to_u32() {
        // Test case 1: Basic conversion
        let bytes = [0x12, 0x34, 0x56, 0x78];
        assert_eq!(bytes_to_u32(&bytes), 0x12345678);

        // Test case 2: Zero values
        let bytes = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(bytes_to_u32(&bytes), 0x00000000);

        // Test case 3: Maximum values
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(bytes_to_u32(&bytes), 0xFFFFFFFF);

        // Test case 4: First byte significant
        let bytes = [0xFF, 0x00, 0x00, 0x00];
        assert_eq!(bytes_to_u32(&bytes), 0xFF000000);

        // Test case 5: Last byte significant
        let bytes = [0x00, 0x00, 0x00, 0xFF];
        assert_eq!(bytes_to_u32(&bytes), 0x000000FF);

        // Test case 6: Mixed values
        let bytes = [0xA1, 0xB2, 0xC3, 0xD4];
        assert_eq!(bytes_to_u32(&bytes), 0xA1B2C3D4);

        // Test case 7: Equivalent to from_be_bytes standard function
        let bytes = [0x11, 0x22, 0x33, 0x44];
        assert_eq!(bytes_to_u32(&bytes), u32::from_be_bytes(bytes));
    }

    #[test]
    fn test_bytes_to_u32_with_longer_slice() {
        // Test with a longer slice - should only use first 4 bytes
        let bytes = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
        assert_eq!(bytes_to_u32(&bytes), 0x12345678);
    }

    #[test]
    #[should_panic]
    fn test_bytes_to_u32_with_shorter_slice() {
        // Test with a slice that's too short - should panic
        let bytes = [0x12, 0x34, 0x56];
        bytes_to_u32(&bytes); // This should panic
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test converting from u32 to bytes and back
        let original: u32 = 0x01234567;
        let bytes = original.to_be_bytes();
        let result = bytes_to_u32(&bytes);
        assert_eq!(result, original);
    }
}
