//! # ELF Parser
//! The primary purpose of this crate is to take in a binary file in the ELF format and parse it
//! return the binary instructions and the memory image. The memory image is useful for global constant
//! this code was copied from SP1 codebase's implementation of the ELF parser.
//! see: [code](https://github.com/succinctlabs/sp1/blob/dev/crates/core/executor/src/disassembler/elf.rs)

use elf::{
    ElfBytes,
    abi::{EM_RISCV, ET_EXEC, PF_X, PT_LOAD},
    endian::LittleEndian,
    file::Class,
};
use hashbrown::HashMap;
use riscv_evm_core::{MAXIMUM_MEMORY_SIZE, WORD_SIZE};
use std::cmp::min;

/// RISC-V 32IM ELF (Executable and Linkable Format) File.
///
/// This file represents a binary in the ELF format, specifically the RISC-V 32IM architecture
/// with the following extensions:
///
/// - Base Integer Instruction Set (I)
/// - Integer Multiplication and Division (M)
///
/// This format is commonly used in embedded systems and is supported by many compilers.
#[derive(Debug, Clone)]
pub struct Elf {
    /// The instructions of the program encoded as 32-bits.
    pub instructions: Vec<u32>,
    /// The start address of the program.
    pub pc_start: u32,
    /// The base address of the program. (this is the memory address where the program is loaded)
    pub pc_base: u32,
    /// The initial memory image, useful for global constants.
    pub memory_image: HashMap<u32, u32>,
}

impl Elf {
    /// Create a new [Elf].
    pub fn new(
        instructions: Vec<u32>,
        pc_start: u32,
        pc_base: u32,
        memory_image: HashMap<u32, u32>,
    ) -> Self {
        Self {
            instructions,
            pc_start,
            pc_base,
            memory_image,
        }
    }

    /// Parse the ELF file into a vector of 32-bit encoded instructions and the first memory
    /// address.
    ///
    /// # Errors
    ///
    /// This function may return an error if the ELF is not valid.
    ///
    /// Reference: [Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
    pub fn decode(input: &[u8]) -> anyhow::Result<Self> {
        let mut image: HashMap<u32, u32> = HashMap::new();
        // Parse the ELF file assuming that it is little-endian..
        let elf = ElfBytes::<LittleEndian>::minimal_parse(input)?;
        // Some sanity checks to make sure that the ELF file is valid.
        if elf.ehdr.class != Class::ELF32 {
            anyhow::bail!("must be a 32-bit elf");
        } else if elf.ehdr.e_machine != EM_RISCV {
            anyhow::bail!("must be a riscv machine");
        } else if elf.ehdr.e_type != ET_EXEC {
            anyhow::bail!("must be executable");
        }

        // Get the entrypoint of the ELF file as an u32.
        let entry: u32 = elf.ehdr.e_entry.try_into()?;

        // Make sure the entrypoint is valid.
        if entry == MAXIMUM_MEMORY_SIZE || entry % WORD_SIZE as u32 != 0 {
            anyhow::bail!("invalid entrypoint");
        }

        // Get the segments of the ELF file.
        let segments = elf
            .segments()
            .ok_or_else(|| anyhow::anyhow!("failed to get segments"))?;
        if segments.len() > 256 {
            anyhow::bail!("too many program headers");
        }

        let mut instructions: Vec<u32> = Vec::new();
        let mut base_address = u32::MAX;

        // Only read segments that are executable instructions that are also PT_LOAD.
        for segment in segments.iter().filter(|x| x.p_type == PT_LOAD) {
            // Get the file size of the segment as an u32.
            let file_size: u32 = segment.p_filesz.try_into()?;
            if file_size == MAXIMUM_MEMORY_SIZE {
                anyhow::bail!("invalid segment file_size");
            }

            // Get the memory size of the segment as an u32.
            let mem_size: u32 = segment.p_memsz.try_into()?;
            if mem_size == MAXIMUM_MEMORY_SIZE {
                anyhow::bail!("Invalid segment mem_size");
            }

            // Get the virtual address of the segment as an u32.
            let vaddr: u32 = segment.p_vaddr.try_into()?;
            if vaddr % WORD_SIZE as u32 != 0 {
                anyhow::bail!("vaddr {vaddr:08x} is unaligned");
            }

            // If the virtual address is less than the first memory address, then update the first
            // memory address.
            if (segment.p_flags & PF_X) != 0 && base_address > vaddr {
                base_address = vaddr;
            }

            // Get the offset to the segment.
            let offset: u32 = segment.p_offset.try_into()?;

            // Read the segment and decode each word as an instruction.
            for i in (0..mem_size).step_by(WORD_SIZE) {
                let addr = vaddr
                    .checked_add(i)
                    .ok_or_else(|| anyhow::anyhow!("vaddr overflow"))?;
                if addr == MAXIMUM_MEMORY_SIZE {
                    anyhow::bail!(
                        "address [0x{addr:08x}] exceeds maximum address for guest programs [0x{MAXIMUM_MEMORY_SIZE:08x}]"
                    );
                }

                // If we are reading past the end of the file, then break.
                if i >= file_size {
                    image.insert(addr, 0);
                    continue;
                }

                // Get the word as an u32 but make sure we don't read past the end of the file.
                let mut word = 0;
                let len = min(file_size - i, WORD_SIZE as u32);
                for j in 0..len {
                    let offset = (offset + i + j) as usize;
                    let byte = input
                        .get(offset)
                        .ok_or_else(|| anyhow::anyhow!("failed to read segment offset"))?;
                    word |= u32::from(*byte) << (j * 8);
                }
                image.insert(addr, word);
                if (segment.p_flags & PF_X) != 0 {
                    instructions.push(word);
                }
            }
        }

        Ok(Elf::new(instructions, entry, base_address, image))
    }
}
