use interfaces::MemoryInterface;
pub mod e_constants;
pub mod interfaces;

/// This is the size of a word in bytes for this vm
pub const WORD_SIZE: usize = 4;
/// This is the maximum memory size for this vm
pub const MAXIMUM_MEMORY_SIZE: u32 = u32::MAX;
/// This is the size of the half word of the VM
pub const HALF_WORD: usize = 2;
/// This is the size of a byte in the VM
pub const BYTE: usize = 1;

/// This defines the different chuck of memory that can be read or written to
#[derive(Debug, Clone)]
pub enum MemoryChuckSize {
    BYTE,
    HalfWord,
    WordSize,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub memory: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Registers {
    data: [u32; 32],
}

impl MemoryInterface for Memory {
    // fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> Option<u32> {
    //     // Calculate a mask and shift to apply to a 32-bit word to get the required data
    //     let (shift, mask) = match size {
    //         MemoryChuckSize::BYTE => (addr & 0x3, 0xff),
    //         MemoryChuckSize::HalfWord => (addr & 0x2, 0xffff),
    //         MemoryChuckSize::WordSize => (0, 0xffffffff),
    //     };

    //     if (addr & 0x3) != shift {
    //         panic!("Memory read must be aligned");
    //     }

    //     // Calculate vector index required data is contained in
    //     let word_addr = addr >> 2;

    //     // Read data from vector
    //     let read_data = self.memory.get(word_addr as usize).copied()?;

    //     // Apply mask and shift to extract required data from word
    //     Some((read_data >> (shift * 8)) & mask)
    // }
    fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> Option<u32> {
        // Calculate vector index for the word
        let word_addr = addr >> 2;

        // Get the offset within the word (0-3)
        let byte_offset = addr & 0x3;

        // Read the word from memory
        let word = self.memory.get(word_addr as usize).copied()?;

        match size {
            MemoryChuckSize::BYTE => {
                // For big-endian, we need to read from the opposite end of the word
                let shift = (3 - byte_offset) * 8;
                Some((word >> shift) & 0xFF)
            }
            MemoryChuckSize::HalfWord => {
                if byte_offset % 2 != 0 {
                    panic!("Half-word reads must be aligned to half-word boundaries");
                }
                let shift = (2 - byte_offset) * 8;
                Some((word >> shift) & 0xFFFF)
            }
            MemoryChuckSize::WordSize => {
                if byte_offset != 0 {
                    panic!("Word reads must be aligned to word boundaries");
                }
                Some(word)
            }
        }
    }

    // fn write_mem(&mut self, addr: u32, size: MemoryChuckSize, value: u32) -> bool {
    //     // Calculate a mask and shift needed to update 32-bit word
    //     let (shift, mask) = match size {
    //         MemoryChuckSize::BYTE => (addr & 0x3, 0xff),
    //         MemoryChuckSize::HalfWord => (addr & 0x2, 0xffff),
    //         MemoryChuckSize::WordSize => (0, 0xffffffff),
    //     };

    //     if (addr & 0x3) != shift {
    //         panic!("Memory write must be aligned");
    //     }

    //     // `mask` << (shift * 8) gives bits being updated, invert to get bits not being updated
    //     let write_mask = !(mask << (shift * 8));

    //     // Calculate vector index data to update is contained in
    //     let word_addr = (addr >> 2) as usize;

    //     if let Some(update_data) = self.memory.get(word_addr) {
    //         // Update word with store data, if it exists
    //         let new = (update_data & write_mask) | ((value & mask) << (shift * 8));
    //         self.memory[word_addr] = new;
    //         true
    //     } else {
    //         false
    //     }
    // }
    fn write_mem(&mut self, addr: u32, size: MemoryChuckSize, value: u32) -> bool {
        // Calculate vector index data to update is contained in
        let word_addr = addr >> 2;

        // Get the byte offset within the word
        let byte_offset = addr & 0x3;

        // Check if the address is within bounds
        if word_addr >= self.memory.len() as u32 {
            return false;
        }

        match size {
            MemoryChuckSize::BYTE => {
                // Calculate shift based on big-endian byte order
                let shift = (3 - byte_offset) * 8;

                // Create a mask to clear the target byte
                let clear_mask = !(0xFF << shift);

                // Read the current word value
                let current = self.memory[word_addr as usize];

                // Clear the target byte and set the new value
                let new_value = (current & clear_mask) | ((value & 0xFF) << shift);

                // Write the updated word back to memory
                self.memory[word_addr as usize] = new_value;
            }
            MemoryChuckSize::HalfWord => {
                if byte_offset % 2 != 0 {
                    panic!("Half-word writes must be aligned to half-word boundaries");
                }

                // Calculate shift based on big-endian half-word order
                let shift = (2 - byte_offset) * 8;

                // Create a mask to clear the target half-word
                let clear_mask = !(0xFFFF << shift);

                // Read the current word value
                let current = self.memory[word_addr as usize];

                // Clear the target half-word and set the new value
                let new_value = (current & clear_mask) | ((value & 0xFFFF) << shift);

                // Write the updated word back to memory
                self.memory[word_addr as usize] = new_value;
            }
            MemoryChuckSize::WordSize => {
                if byte_offset != 0 {
                    panic!("Word writes must be aligned to word boundaries");
                }

                // Write the full word directly
                self.memory[word_addr as usize] = value;
            }
        }

        true
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers { data: [0; 32] }
    }

    pub fn read_reg(&self, reg: u32) -> u32 {
        self.data[reg as usize]
    }

    pub fn write_reg(&mut self, reg: u32, value: u32) {
        if reg == 0 {
            return;
        }

        self.data[reg as usize] = value;
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; MAXIMUM_MEMORY_SIZE as usize],
        }
    }

    pub fn load_program(&mut self, program: &Vec<u32>, base_addr: u32) {
        let mut addr = (base_addr >> 2) as usize;

        for word in program {
            self.memory[addr] = *word;
            addr += 1;
        }
    }

    pub fn new_with_load_program(program: &Vec<u32>, base_addr: u32) -> Self {
        let mut memory = Memory::new();
        memory.load_program(program, base_addr);

        memory
    }
}

pub fn sign_extend_u32(x: u32) -> i64 {
    (x as i32) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read() {
        // Create a memory instance with our test values
        let mut memory = Memory::new();
        memory.memory[0] = 147;
        memory.memory[1] = 59772819;

        // Test byte-by-byte reading from first word
        assert_eq!(memory.read_mem(0, MemoryChuckSize::BYTE), Some(0));
        assert_eq!(memory.read_mem(1, MemoryChuckSize::BYTE), Some(0));
        assert_eq!(memory.read_mem(2, MemoryChuckSize::BYTE), Some(0));
        assert_eq!(memory.read_mem(3, MemoryChuckSize::BYTE), Some(147));

        // Test byte-by-byte reading from second word
        assert_eq!(memory.read_mem(4, MemoryChuckSize::BYTE), Some(3));
        assert_eq!(memory.read_mem(5, MemoryChuckSize::BYTE), Some(144));
        assert_eq!(memory.read_mem(6, MemoryChuckSize::BYTE), Some(15));
        assert_eq!(memory.read_mem(7, MemoryChuckSize::BYTE), Some(147));

        // Test half-word reading - update expected values
        assert_eq!(memory.read_mem(0, MemoryChuckSize::HalfWord), Some(0));
        assert_eq!(memory.read_mem(2, MemoryChuckSize::HalfWord), Some(147));
        assert_eq!(
            memory.read_mem(4, MemoryChuckSize::HalfWord),
            Some(3 * 256 + 144)
        );
        assert_eq!(
            memory.read_mem(6, MemoryChuckSize::HalfWord),
            Some(15 * 256 + 147)
        );

        // Test word reading - update expected value for second word
        assert_eq!(memory.read_mem(0, MemoryChuckSize::WordSize), Some(147));
        assert_eq!(
            memory.read_mem(4, MemoryChuckSize::WordSize),
            Some(59772819)
        );

        // Test out-of-bounds reading
        assert_eq!(
            memory.read_mem(MAXIMUM_MEMORY_SIZE, MemoryChuckSize::BYTE),
            Some(0)
        );
    }

    #[test]
    #[should_panic(expected = "Half-word reads must be aligned")]
    fn test_unaligned_half_word_read() {
        let memory = Memory::new();
        memory.read_mem(1, MemoryChuckSize::HalfWord);
    }

    #[test]
    #[should_panic(expected = "Word reads must be aligned")]
    fn test_unaligned_word_read() {
        let memory = Memory::new();
        memory.read_mem(1, MemoryChuckSize::WordSize);
    }

    #[test]
    fn test_write_and_read() {
        let mut memory = Memory::new();

        // Test byte writing and reading
        assert!(memory.write_mem(0, MemoryChuckSize::BYTE, 0xAA));
        assert!(memory.write_mem(1, MemoryChuckSize::BYTE, 0xBB));
        assert!(memory.write_mem(2, MemoryChuckSize::BYTE, 0xCC));
        assert!(memory.write_mem(3, MemoryChuckSize::BYTE, 0xDD));

        assert_eq!(memory.read_mem(0, MemoryChuckSize::BYTE), Some(0xAA));
        assert_eq!(memory.read_mem(1, MemoryChuckSize::BYTE), Some(0xBB));
        assert_eq!(memory.read_mem(2, MemoryChuckSize::BYTE), Some(0xCC));
        assert_eq!(memory.read_mem(3, MemoryChuckSize::BYTE), Some(0xDD));

        // Test half-word writing and reading
        assert!(memory.write_mem(4, MemoryChuckSize::HalfWord, 0x1234));
        assert!(memory.write_mem(6, MemoryChuckSize::HalfWord, 0x5678));

        assert_eq!(memory.read_mem(4, MemoryChuckSize::HalfWord), Some(0x1234));
        assert_eq!(memory.read_mem(6, MemoryChuckSize::HalfWord), Some(0x5678));

        // Test word writing and reading
        assert!(memory.write_mem(8, MemoryChuckSize::WordSize, 0x87654321));

        assert_eq!(memory.read_mem(8, MemoryChuckSize::BYTE), Some(0x87));
        assert_eq!(memory.read_mem(9, MemoryChuckSize::BYTE), Some(0x65));
        assert_eq!(memory.read_mem(10, MemoryChuckSize::BYTE), Some(0x43));
        assert_eq!(memory.read_mem(11, MemoryChuckSize::BYTE), Some(0x21));
        assert_eq!(
            memory.read_mem(8, MemoryChuckSize::WordSize),
            Some(0x87654321)
        );
    }
}
