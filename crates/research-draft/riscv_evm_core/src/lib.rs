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
    fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> Option<u32> {
        // Calculate a mask and shift to apply to a 32-bit word to get the required data
        let (shift, mask) = match size {
            MemoryChuckSize::BYTE => (addr & 0x3, 0xff),
            MemoryChuckSize::HalfWord => (addr & 0x2, 0xffff),
            MemoryChuckSize::WordSize => (0, 0xffffffff),
        };

        if (addr & 0x3) != shift {
            panic!("Memory read must be aligned");
        }

        // Calculate vector index required data is contained in
        let word_addr = addr >> 2;

        // Read data from vector
        let read_data = self.memory.get(word_addr as usize).copied()?;

        // Apply mask and shift to extract required data from word
        Some((read_data >> (shift * 8)) & mask)
    }

    fn write_mem(&mut self, addr: u32, size: MemoryChuckSize, value: u32) -> bool {
        // Calculate a mask and shift needed to update 32-bit word
        let (shift, mask) = match size {
            MemoryChuckSize::BYTE => (addr & 0x3, 0xff),
            MemoryChuckSize::HalfWord => (addr & 0x2, 0xffff),
            MemoryChuckSize::WordSize => (0, 0xffffffff),
        };

        if (addr & 0x3) != shift {
            panic!("Memory write must be aligned");
        }

        // `mask` << (shift * 8) gives bits being updated, invert to get bits not being updated
        let write_mask = !(mask << (shift * 8));

        // Calculate vector index data to update is contained in
        let word_addr = (addr >> 2) as usize;

        if let Some(update_data) = self.memory.get(word_addr) {
            // Update word with store data, if it exists
            let new = (update_data & write_mask) | ((value & mask) << (shift * 8));
            self.memory[word_addr] = new;
            true
        } else {
            false
        }
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
