//! This mod holds all the necessary structs and functions to emulate a RISC-V CPU.
use crate::{
    context::Context,
    ecall_manager::process_ecall,
    elf_parser::Elf,
    instructions::InstructionDecoder,
    utils::{process_load_to_reg, process_store_to_memory},
};
use riscv_evm_core::{
    Memory, MemoryChuckSize, Registers, interfaces::MemoryInterface, sign_extend_u32,
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, Clone)]
pub enum VMErrors {
    InvalidInstruction,
    InvalidMemoryAccess,
    EnvironmentError,
    InvalidOpcode(u32),
    MemoryError,
    MemoryLoadError,
    MemoryStoreError,
    InvalidFunct7(u32),
    InvalidFunct3(u32),
    EnvirmentCallErrorWithDetail(String),
    VMAccountLoadFailed,
    VMCreateError(u32),
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub registers: Registers,
    pub memory: Memory,
    pub pc: u32,
    pub running: bool,
    pub exit_code: u32,
}

impl Vm {
    /// Create a new Vm.
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            pc: 0,
            running: false,
            exit_code: 0,
        }
    }

    /// Create a new Vm from a binary ELF file.
    /// # Errors
    /// This function may return an error if the ELF is not valid.
    pub fn from_bin_elf(path: String) -> Result<Self, anyhow::Error> {
        let mut file = BufReader::new(File::open(path)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();

        let program_elf_decoded = Elf::decode(&buf)?;

        Ok(Self {
            registers: Registers::new(),
            memory: Memory::new_with_load_program(
                &program_elf_decoded.instructions,
                program_elf_decoded.pc_base,
            ),
            pc: program_elf_decoded.pc_start,
            running: false,
            exit_code: 0,
        })
    }

    pub fn from_bin(instructions: Vec<u32>) -> Result<Self, anyhow::Error> {
        Ok(Self {
            registers: Registers::new(),
            memory: Memory::new_with_load_program(&instructions, 0),
            pc: 0,
            running: false,
            exit_code: 0,
        })
    }

    /// Step the Vm.
    /// This function will execute the instruction at the current program counter.
    /// If the instruction is a branch, the program counter will be updated accordingly.
    /// If the instruction is a jump, the program counter will be updated accordingly.
    /// If the instruction is a syscall, the program will be halted.
    /// If the instruction is a halt, the program will be halted.
    pub fn step(&mut self, debug_mode: bool, context: &mut Context) -> Result<bool, VMErrors> {
        // Fetch the instruction from memory
        let instruction = self
            .memory
            .read_mem(self.pc, MemoryChuckSize::WordSize)
            .ok_or(VMErrors::InvalidMemoryAccess)?;

        // Decode the instruction
        let decoded_instruction = InstructionDecoder::decode(&instruction)?;

        if debug_mode {
            println!("{}", decoded_instruction.to_string());
        }

        // Execute the instruction
        match decoded_instruction.decoded_instruction {
            crate::instructions::DecodedInstruction::RType(rtype) => {
                match rtype.funct3 {
                    0b000 => {
                        // Funct3 for add, sub, mul
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for add
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_add(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0100000 => {
                                // Funct7 for sub
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_sub(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mul
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_mul(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b001 => {
                        // Funct3 for sll, mulh
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sll
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_shl(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulh
                                let rs1 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs1 as u32));
                                let rs2 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs2 as u32));
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b010 => {
                        // Funct3 for slt, mulhsu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for slt
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs1 < rs2 { 1 } else { 0 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulhsu
                                let rs1 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs1 as u32));
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i64;
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b011 => {
                        // Funct3 for sltu, mulhu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sltu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs1 < rs2 { 1 } else { 0 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulhu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as u64;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as u64;
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b100 => {
                        // Funct3 for xor, div
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for xor
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 ^ rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for div
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs2 != 0 {
                                    rs1.wrapping_div(rs2) as u32
                                } else {
                                    u32::MAX
                                };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b101 => {
                        // Funct3 for srl, sra, divu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for srl
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_shr(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for divu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs2 != 0 { rs1 / rs2 } else { u32::MAX };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0100000 => {
                                // Funct7 for sra
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as u32;
                                let rd = rs1.wrapping_shr(rs2) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b110 => {
                        // Funct3 for or, rem
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for or
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 | rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for rem
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs2 != 0 {
                                    rs1.wrapping_rem(rs2) as u32
                                } else {
                                    rs1 as u32
                                };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    0b111 => {
                        // Funct3 for and, remu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for and
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 & rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for remu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs2 != 0 { rs1 % rs2 } else { rs1 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct7(rtype.funct7)),
                        }
                    }
                    _ => return Err(VMErrors::InvalidFunct3(rtype.funct3)),
                }
            }
            crate::instructions::DecodedInstruction::IType(itype) => {
                match decoded_instruction.opcode {
                    0b0010011 => {
                        // Funct3 for addi, slti, sltiu, xori, ori, andi
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for addi
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let rd = rs1.wrapping_add(imm);
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b001 => {
                                // Funct3 for slli
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.metadata.imm_shift_amt;
                                let rd = rs1.wrapping_shl(imm);
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b010 => {
                                // Funct3 for slti
                                let rs1 = self.registers.read_reg(itype.rs1 as u32) as i32;
                                let imm = itype.imm as i32;
                                let rd = if rs1 < imm { 1 } else { 0 };
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b011 => {
                                // Funct3 for sltiu
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let rd = if rs1 < imm { 1 } else { 0 };
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b100 => {
                                // Funct3 for xori
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let rd = rs1 ^ imm;
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b101 => {
                                // Funct3 for srli, srai
                                match itype.metadata.funct7 {
                                    0b0000000 => {
                                        // Funct7 for srli
                                        let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                        let imm = itype.metadata.imm_shift_amt;
                                        let rd = rs1.wrapping_shr(imm);
                                        self.registers.write_reg(itype.rd as u32, rd);
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    0b0100000 => {
                                        // Funct7 for srai
                                        let rs1 = self.registers.read_reg(itype.rs1 as u32) as i32;
                                        let imm = itype.metadata.imm_shift_amt;
                                        let rd = rs1.wrapping_shr(imm) as u32;
                                        self.registers.write_reg(itype.rd as u32, rd);
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    _ => {
                                        return Err(VMErrors::InvalidFunct7(itype.metadata.funct7));
                                    }
                                }
                            }
                            0b110 => {
                                // Funct3 for ori
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let rd = rs1 | imm;
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b111 => {
                                // Funct3 for andi
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let rd = rs1 & imm;
                                self.registers.write_reg(itype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct3(itype.funct3)),
                        }
                    }
                    0b0000011 => {
                        // Funct3 for lb, lh, lw, lbu, lhu
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for lb
                                match process_load_to_reg(self, &itype, MemoryChuckSize::BYTE, true)
                                {
                                    Ok(_) => {
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            0b001 => {
                                // Funct3 for lh
                                match process_load_to_reg(
                                    self,
                                    &itype,
                                    MemoryChuckSize::HalfWord,
                                    true,
                                ) {
                                    Ok(_) => {
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            0b010 => {
                                // Funct3 for lw
                                match process_load_to_reg(
                                    self,
                                    &itype,
                                    MemoryChuckSize::WordSize,
                                    false,
                                ) {
                                    Ok(_) => {
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            0b100 => {
                                // Funct3 for lbu
                                match process_load_to_reg(
                                    self,
                                    &itype,
                                    MemoryChuckSize::BYTE,
                                    false,
                                ) {
                                    Ok(_) => {
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            0b101 => {
                                // Funct3 for lhu
                                match process_load_to_reg(
                                    self,
                                    &itype,
                                    MemoryChuckSize::HalfWord,
                                    false,
                                ) {
                                    Ok(_) => {
                                        self.pc += 4;
                                        Ok(true)
                                    }
                                    Err(e) => Err(e),
                                }
                            }
                            _ => return Err(VMErrors::InvalidFunct3(itype.funct3)),
                        }
                    }
                    0b1100111 => {
                        // Funct3 for jalr
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for jalr
                                let rs1 = self.registers.read_reg(itype.rs1 as u32);
                                let imm = itype.imm as u32;
                                let mut dest_addr = rs1.wrapping_add(imm);

                                // see that dest_addr is even
                                dest_addr &= 0xfffffffe;
                                self.registers.write_reg(itype.rd as u32, self.pc + 4);
                                self.pc = dest_addr;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidFunct3(itype.funct3)),
                        }
                    }
                    0b1110011 => {
                        process_ecall(self, context)?;
                        self.running = false;
                        Ok(true)
                    }
                    _ => return Err(VMErrors::InvalidOpcode(decoded_instruction.opcode)),
                }
            }
            crate::instructions::DecodedInstruction::SType(stype) => {
                match stype.funct3 {
                    0b000 => {
                        // Funct3 for sb
                        match process_store_to_memory(self, &stype, MemoryChuckSize::BYTE) {
                            Ok(_) => {
                                self.pc += 4;
                                Ok(true)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    0b001 => {
                        // Funct3 for sh
                        match process_store_to_memory(self, &stype, MemoryChuckSize::HalfWord) {
                            Ok(_) => {
                                self.pc += 4;
                                Ok(true)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    0b010 => {
                        // Funct3 for sw
                        match process_store_to_memory(self, &stype, MemoryChuckSize::WordSize) {
                            Ok(_) => {
                                self.pc += 4;
                                Ok(true)
                            }
                            Err(e) => Err(e),
                        }
                    }
                    _ => return Err(VMErrors::InvalidFunct3(stype.funct3)),
                }
            }
            crate::instructions::DecodedInstruction::BType(btype) => {
                match btype.funct3 {
                    0b000 => {
                        // Funct3 for beq
                        let rs1 = self.registers.read_reg(btype.rs1 as u32);
                        let rs2 = self.registers.read_reg(btype.rs2 as u32);

                        if rs1 == rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    0b001 => {
                        // Funct3 for bne
                        let rs1 = self.registers.read_reg(btype.rs1 as u32);
                        let rs2 = self.registers.read_reg(btype.rs2 as u32);

                        if rs1 != rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    0b100 => {
                        // Funct3 for blt
                        let rs1 = self.registers.read_reg(btype.rs1 as u32) as i32;
                        let rs2 = self.registers.read_reg(btype.rs2 as u32) as i32;

                        if rs1 < rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    0b101 => {
                        // Funct3 for bge
                        let rs1 = self.registers.read_reg(btype.rs1 as u32) as i32;
                        let rs2 = self.registers.read_reg(btype.rs2 as u32) as i32;

                        if rs1 >= rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    0b110 => {
                        // Funct3 for bltu
                        let rs1 = self.registers.read_reg(btype.rs1 as u32);
                        let rs2 = self.registers.read_reg(btype.rs2 as u32);

                        if rs1 < rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    0b111 => {
                        // Funct3 for bgeu
                        let rs1 = self.registers.read_reg(btype.rs1 as u32);
                        let rs2 = self.registers.read_reg(btype.rs2 as u32);

                        if rs1 >= rs2 {
                            let target = self.pc.wrapping_add(btype.imm as u32);
                            self.pc = target;
                        } else {
                            self.pc += 4;
                        }

                        Ok(true)
                    }
                    _ => return Err(VMErrors::InvalidFunct3(btype.funct3)),
                }
            }
            crate::instructions::DecodedInstruction::UType(utype) => {
                match decoded_instruction.opcode {
                    0b0110111 => {
                        // Funct3 for lui
                        let imm = utype.imm as u32;
                        self.registers.write_reg(utype.rd as u32, imm);
                        self.pc += 4;
                        Ok(true)
                    }
                    0b0010111 => {
                        // Funct3 for auipc
                        let imm = utype.imm as u32;
                        let pc = self.pc;
                        self.registers
                            .write_reg(utype.rd as u32, pc.wrapping_add(imm));
                        self.pc += 4;
                        Ok(true)
                    }
                    _ => return Err(VMErrors::InvalidOpcode(decoded_instruction.opcode)),
                }
            }
            crate::instructions::DecodedInstruction::JType(jtype) => {
                match decoded_instruction.opcode {
                    0b1101111 => {
                        // Funct3 for jal
                        self.pc += 4;
                        self.registers.write_reg(jtype.rd as u32, self.pc);
                        self.pc += jtype.imm as u32;
                        Ok(true)
                    }
                    _ => return Err(VMErrors::InvalidOpcode(decoded_instruction.opcode)),
                }
            }
        }
    }

    /// Run the Vm.
    /// This function will run the Vm until it halts.
    /// The Vm will halt if the program counter is out of bounds or if the instruction is a halt.
    pub fn run(&mut self, debug_mode: bool, context: &mut Context) {
        self.running = true;
        while self.running {
            match self.step(debug_mode, context) {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    match e {
                        VMErrors::EnvironmentError => {} // would just be halting the program, sysytem calls are not allowed on the VM
                        _ => {
                            eprintln!("Error at pc: {:x} - error: {:?}", self.pc, e);
                        }
                    }
                    self.running = false;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vm;
    use crate::context::Context;
    use revm::{Context as EthContext, MainContext, database::EmptyDB};

    #[test]
    fn test_vm_run() {
        let code: Vec<u32> = vec![
            4278255891, 1123875, 5244179, 10487187, 11863139, 11863475, 16777455, 12656771,
            16843027, 115, 1410451, 32871,
        ];
        let eth_context = EthContext::mainnet().with_db(EmptyDB::default());
        let mut context = Context::new(eth_context);
        let mut vm = Vm::from_bin(code).unwrap();
        vm.run(true, &mut context);
    }
}
