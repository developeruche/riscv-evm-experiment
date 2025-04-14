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

/// Converts a vector of bytes into a vector of u32 values (big-endian format)
/// Pads with zeros if necessary to complete the last u32
pub fn bytes_to_u32_vec(bytes: &[u8]) -> Vec<u32> {
    // Calculate how many u32 values we'll need
    let count = (bytes.len() + 3) / 4; // Ceiling division by 4
    let mut result = Vec::with_capacity(count);

    // Process each 4-byte chunk
    for chunk_idx in 0..count {
        let chunk_start = chunk_idx * 4;
        let chunk_end = std::cmp::min(chunk_start + 4, bytes.len());

        // Create a 4-byte array, padded with zeros if needed
        let mut chunk = [0u8; 4];
        for i in chunk_start..chunk_end {
            chunk[i - chunk_start] = bytes[i];
        }

        // Convert to u32 using big-endian format
        let value = ((chunk[0] as u32) << 24)
            | ((chunk[1] as u32) << 16)
            | ((chunk[2] as u32) << 8)
            | (chunk[3] as u32);

        result.push(value);
    }

    result
}

/// Converts a vector of u32 values back to a vector of bytes (big-endian format)
/// Returns exactly `byte_len` bytes, truncating if the u32 vector would produce more
pub fn u32_vec_to_bytes(u32_values: &[u32], byte_len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(u32_values.len() * 4);

    // Process each u32 value
    for &value in u32_values {
        // Extract bytes in big-endian order
        result.push(((value >> 24) & 0xFF) as u8);
        result.push(((value >> 16) & 0xFF) as u8);
        result.push(((value >> 8) & 0xFF) as u8);
        result.push((value & 0xFF) as u8);
    }

    // Truncate to requested length (in case we generated too many bytes)
    result.truncate(byte_len);

    result
}

/// Combines two u32 values into a single u64
///
/// # Arguments
/// * `high` - The high 32 bits of the resulting u64
/// * `low` - The low 32 bits of the resulting u64
///
/// # Returns
/// A u64 where the high 32 bits are from `high` and the low 32 bits are from `low`
pub fn combine_u32_to_u64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

/// Splits a u64 into two u32 values
///
/// # Arguments
/// * `value` - The u64 value to split
///
/// # Returns
/// A tuple (high, low) where:
/// * `high` contains the high 32 bits of the input
/// * `low` contains the low 32 bits of the input
pub fn split_u64_to_u32(value: u64) -> (u32, u32) {
    let high = (value >> 32) as u32;
    let low = value as u32;
    (high, low)
}

/// Converts a 32-byte value (U256) into a vector of 8 u32 values (big-endian format)
pub fn u256_to_u32_vec(value: &[u8; 32]) -> Vec<u32> {
    let mut result = Vec::with_capacity(8);

    // Process 4 bytes at a time
    for chunk_idx in 0..8 {
        let offset = chunk_idx * 4;
        let value = u32::from_be_bytes([
            value[offset],
            value[offset + 1],
            value[offset + 2],
            value[offset + 3],
        ]);
        result.push(value);
    }

    result
}

/// Converts a vector of 8 u32 values back to a 32-byte value (U256)
pub fn u32_vec_to_u256(u32_values: &[u32]) -> [u8; 32] {
    assert_eq!(u32_values.len(), 8, "Expected exactly 8 u32 values");

    let mut result = [0u8; 32];

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

    #[test]
    fn test_bytes_to_u32_vec_exact_multiple() {
        // Test with exactly 8 bytes (2 u32s)
        let bytes = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let result = bytes_to_u32_vec(&bytes);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 0x12345678);
        assert_eq!(result[1], 0x9ABCDEF0);
    }

    #[test]
    fn test_bytes_to_u32_vec_partial() {
        // Test with 6 bytes (not a multiple of 4)
        let bytes = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC];
        let result = bytes_to_u32_vec(&bytes);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 0x12345678);
        assert_eq!(result[1], 0x9ABC0000); // Last 2 bytes should be zero-padded
    }

    #[test]
    fn test_bytes_to_u32_vec_empty() {
        // Test with empty input
        let bytes: Vec<u8> = vec![];
        let result = bytes_to_u32_vec(&bytes);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_bytes_to_u32_vec_single_byte() {
        // Test with a single byte
        let bytes = vec![0xFF];
        let result = bytes_to_u32_vec(&bytes);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 0xFF000000);
    }

    #[test]
    fn test_u32_vec_to_bytes_exact_multiple() {
        // Test with 2 u32s to make 8 bytes
        let u32_values = vec![0x12345678, 0x9ABCDEF0];
        let result = u32_vec_to_bytes(&u32_values, 8);

        assert_eq!(result.len(), 8);
        assert_eq!(result, vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
    }

    #[test]
    fn test_u32_vec_to_bytes_truncate() {
        // Test with 2 u32s but only want 6 bytes
        let u32_values = vec![0x12345678, 0x9ABCDEF0];
        let result = u32_vec_to_bytes(&u32_values, 6);

        assert_eq!(result.len(), 6);
        assert_eq!(result, vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]);
    }

    #[test]
    fn test_u32_vec_to_bytes_empty() {
        // Test with empty input
        let u32_values: Vec<u32> = vec![];
        let result = u32_vec_to_bytes(&u32_values, 0);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_u32_vec_to_bytes_larger_size() {
        // Test requesting more bytes than the u32 vector can provide
        // It should still only return the number of bytes the u32 vector can provide
        let u32_values = vec![0x12345678];
        let result = u32_vec_to_bytes(&u32_values, 10);

        assert_eq!(result.len(), 4); // Should only return 4 bytes
        assert_eq!(result, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_round_trip_exact() {
        // Test round trip conversion with exact multiple of 4
        let original = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        let u32_values = bytes_to_u32_vec(&original);
        let result = u32_vec_to_bytes(&u32_values, original.len());

        assert_eq!(result, original);
    }

    #[test]
    fn test_round_trip_partial() {
        // Test round trip conversion with non-multiple of 4
        let original = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let u32_values = bytes_to_u32_vec(&original);
        let result = u32_vec_to_bytes(&u32_values, original.len());

        assert_eq!(result, original);
    }

    #[test]
    fn test_real_world_example() {
        // A realistic test case with EVM calldata
        let calldata = hex::decode("a9059cbb000000000000000000000000b97048628db6b661d4c2aa833e95dbe1a905b2800000000000000000000000000000000000000000000000000000000000003e84").unwrap();

        // Convert to u32s and back
        let u32_values = bytes_to_u32_vec(&calldata);
        let result = u32_vec_to_bytes(&u32_values, calldata.len());

        assert_eq!(result, calldata);
    }

    #[test]
    fn test_combine_u32_to_u64() {
        // Test case 1: Basic combination
        let high = 0x12345678;
        let low = 0x9ABCDEF0;
        assert_eq!(combine_u32_to_u64(high, low), 0x123456789ABCDEF0);

        // Test case 2: With zeros
        assert_eq!(combine_u32_to_u64(0, 0), 0);

        // Test case 3: Only high bits
        assert_eq!(combine_u32_to_u64(0x12345678, 0), 0x1234567800000000);

        // Test case 4: Only low bits
        assert_eq!(combine_u32_to_u64(0, 0x9ABCDEF0), 0x00000000_9ABCDEF0);

        // Test case 5: Maximum values
        assert_eq!(combine_u32_to_u64(u32::MAX, u32::MAX), u64::MAX);
    }

    #[test]
    fn test_split_u64_to_u32() {
        // Test case 1: Basic split
        let value = 0x123456789ABCDEF0;
        let (high, low) = split_u64_to_u32(value);
        assert_eq!(high, 0x12345678);
        assert_eq!(low, 0x9ABCDEF0);

        // Test case 2: With zeros
        let (high, low) = split_u64_to_u32(0);
        assert_eq!(high, 0);
        assert_eq!(low, 0);

        // Test case 3: Only high bits
        let (high, low) = split_u64_to_u32(0x1234567800000000);
        assert_eq!(high, 0x12345678);
        assert_eq!(low, 0);

        // Test case 4: Only low bits
        let (high, low) = split_u64_to_u32(0x00000000_9ABCDEF0);
        assert_eq!(high, 0);
        assert_eq!(low, 0x9ABCDEF0);

        // Test case 5: Maximum values
        let (high, low) = split_u64_to_u32(u64::MAX);
        assert_eq!(high, u32::MAX);
        assert_eq!(low, u32::MAX);
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test case: Round-trip conversion
        let original_high = 0x12345678;
        let original_low = 0x9ABCDEF0;

        // Combine
        let combined = combine_u32_to_u64(original_high, original_low);

        // Split
        let (result_high, result_low) = split_u64_to_u32(combined);

        // Verify
        assert_eq!(result_high, original_high);
        assert_eq!(result_low, original_low);
    }

    #[test]
    fn test_address_conversion_2() {
        // Example Ethereum address (20 bytes)
        let address = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0, 0x12, 0x34, 0x56, 0x78,
        ];

        // Convert to u32 vector
        let u32_vec = address_to_u32_vec(&address);

        // Check length and values
        assert_eq!(u32_vec.len(), 5);
        assert_eq!(u32_vec[0], 0x12345678);
        assert_eq!(u32_vec[1], 0x9abcdef0);
        assert_eq!(u32_vec[2], 0x12345678);
        assert_eq!(u32_vec[3], 0x9abcdef0);
        assert_eq!(u32_vec[4], 0x12345678);

        // Convert back to address
        let converted_back = u32_vec_to_address(&u32_vec);

        // Check round trip conversion
        assert_eq!(converted_back, address);
    }

    #[test]
    #[should_panic(expected = "Expected exactly 5 u32 values")]
    fn test_incorrect_length() {
        // Try to convert a vector with incorrect length
        let invalid_vec = vec![0x12345678, 0x9abcdef0]; // Only 2 values
        u32_vec_to_address(&invalid_vec); // Should panic
    }

    #[test]
    fn test_u256_conversion() {
        // Example 32-byte value (U256)
        let u256_value = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98,
            0x76, 0x54, 0x32, 0x10,
        ];

        // Convert to u32 vector
        let u32_vec = u256_to_u32_vec(&u256_value);

        // Check length and values
        assert_eq!(u32_vec.len(), 8);
        assert_eq!(u32_vec[0], 0x00112233);
        assert_eq!(u32_vec[1], 0x44556677);
        assert_eq!(u32_vec[2], 0x8899aabb);
        assert_eq!(u32_vec[3], 0xccddeeff);
        assert_eq!(u32_vec[4], 0x01234567);
        assert_eq!(u32_vec[5], 0x89abcdef);
        assert_eq!(u32_vec[6], 0xfedcba98);
        assert_eq!(u32_vec[7], 0x76543210);

        // Convert back to U256
        let converted_back = u32_vec_to_u256(&u32_vec);

        // Check round trip conversion
        assert_eq!(converted_back, u256_value);
    }

    #[test]
    #[should_panic(expected = "Expected exactly 8 u32 values")]
    fn test_incorrect_length_u256() {
        // Try to convert a vector with incorrect length
        let invalid_vec = vec![0x12345678, 0x9abcdef0, 0x11223344]; // Only 3 values
        u32_vec_to_u256(&invalid_vec); // Should panic
    }

    #[test]
    fn test_u256_zero() {
        // Test with all zeros
        let zero_u256 = [0u8; 32];
        let u32_vec = u256_to_u32_vec(&zero_u256);

        assert_eq!(u32_vec, vec![0u32; 8]);

        let converted_back = u32_vec_to_u256(&u32_vec);
        assert_eq!(converted_back, zero_u256);
    }

    #[test]
    fn test_u256_max() {
        // Test with all ones (maximum value)
        let max_u256 = [0xff; 32];
        let u32_vec = u256_to_u32_vec(&max_u256);

        assert_eq!(u32_vec, vec![0xffffffff; 8]);

        let converted_back = u32_vec_to_u256(&u32_vec);
        assert_eq!(converted_back, max_u256);
    }
}
