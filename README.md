# RISC-V EVM: A Novel Approach to Blockchain Virtual Machine Architecture

> **Research Project**: Exploring the integration of RISC-V instruction set architecture with Ethereum Virtual Machine to create a more versatile blockchain execution environment.

## Overview

This repository contains experimental implementation of a RISC-V based Ethereum Virtual Machine (EVM). The project explores the technical feasibility and performance implications of using the RISC-V instruction set architecture as an alternative execution environment for blockchain smart contracts.

By leveraging RISC-V's open architecture, this project aims to enable multi-language smart contract development while investigating potential improvements in execution efficiency and security compared to the traditional stack-based EVM.

## 🔍 Research Context

This implementation serves as the foundation for research exploring whether blockchain platforms could benefit from the advantages of modern CPU architecture while maintaining compatibility with existing smart contract paradigms. Key aspects investigated include:

- The feasibility of mapping EVM semantics to RISC-V instructions
- Performance characteristics of register-based vs stack-based execution
- Developer experience implications for smart contract authors
- Technical challenges in preserving EVM security guarantees

The complete research paper draft is available here.

## 🏗️ Project Structure

```
crates/
├── research-draft/                  # Initial experimental implementation
│   ├── counter_riscvim32_smart_contract_asm/ # Sample smart contract in RISC-V assembly
│   ├── riscv_evm/                   # Core RISC-V EVM implementation
│   └── riscv_evm_core/              # Core RISC-V VM primitives
├── research-final/                  # Final implementation with benchmarks (WIP)
│   ├── benchmarks/                  # Performance benchmarking
│   ├── handler/                     # Custom handler implementation
│   └── primitives/                  # Primitive types and utilities
└── riscv_smart_contracts/           # Example RISC-V smart contracts
```

## ✨ Key Features

- **RISC-V Instruction Set**: Implementation of the RV32IM subset of RISC-V
- **Blockchain Integration**: Support for key blockchain operations (storage, calls, logs, etc.)
- **Environment Calls System**: 44 environment calls including Keccak256, Address, Call, Create, etc.
- **Smart Contract Compatibility**: Ability to deploy and execute RISC-V assembly smart contracts
- **REVM Integration**: Compatible with Rust Ethereum Virtual Machine (REVM) API

## 🚀 Getting Started

### Prerequisites

- Rust 1.70.0 or higher
- Cargo package manager

### Building the Project

```bash
# Clone the repository
git clone https://github.com/developeruche/riscv-evm-experiment.git
cd riscv-evm-experiment

# Build the project
cargo build
```

### Running the Examples

To deploy and execute the sample counter smart contract:

```bash
cd crates/research-draft/counter_riscvim32_smart_contract_asm
cargo run
```

## 📝 Sample Smart Contract

This project includes a sample counter contract implemented in RISC-V assembly:

```assembly
# Simple Counter Contract in RISC-V RV32IM Assembly
.equ SLOT_COUNTER_1, 0
# ... other definitions ...

.text
_start:
    # Initialize Counter Value to 0
    addi x1, zero, SLOT_COUNTER_1 
    # ... contract implementation ...
    
    # Store value to storage
    addi x31, zero, ECALL_SSTORE
    ecall
    
    # Return success
    jal x0, _return_true
```

For the complete smart contract, see contract.rs.

## 🔬 Technical Approach

The project follows a two-phase approach:

### Phase One: Custom RISC-V EVM Architecture (Completed)
- Designing the core RISC-V EVM virtual machine components
- Implementation of a RISC-V IM32 assembler for blockchain smart contracts
- Implementation of blockchain operations as environment calls
- Testing with sample smart contracts

### Phase Two: Integration with Existing Ethereum Runtime (In Progress)
- Adaptation of custom RISC-V EVM to the REVM API
- Implementation of comparative benchmarking
- Analysis of performance characteristics
- Optimization opportunities

## 🧪 Current Findings

Initial implementation has revealed several interesting insights:

1. **Instruction Mapping**: Simple arithmetic, logic, and memory operations map efficiently to RISC-V
2. **Register Pressure**: Some Ethereum opcodes (like SSTORE, LOG2, CALL) consume most or all available registers
3. **Environment Interaction**: Blockchain state access can be effectively abstracted through environment calls
4. **REVM Compatibility**: The existing Ethereum context model works well with the RISC-V execution model

Full performance benchmarks will be available in the final research report.

## 📚 Related Projects

- [RISC-V Assembler](https://github.com/developeruche/riscv-assembler) - Assembler used for RISC-V smart contracts
- [REVM](https://github.com/bluealloy/revm) - Rust EVM implementation used for integration

## 👥 Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them
4. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📊 Research Status

This is an ongoing research project. The draft implementation is complete, and work is underway on benchmarking and final analysis. 

[draft-report](https://hackmd.io/@0xdeveloperuche/Hk18BWxkxl)

**Future work includes**:
- Implementing gas metering
- Additional EVM optimizations
- Comprehensive benchmarking
- Exploring custom RISC-V extensions for blockchain operations

## 📧 Contact

For questions or feedback about this research, please contact the author:
- Twitter: [@developeruche](https://x.com/developeruche)