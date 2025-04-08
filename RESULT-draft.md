# RISC-V EVM Experiment Results - Draft

## Pre-Experiment Insights

One of the goals of the experiment is to see how multiple other languages targeting RISC-V can be used to write smart contracts for blockchains like Ethereum.

This is awesome and exciting! But we need to keep in mind that if the goal is to attract more developers from various ecosystems utilizing these languages, that mission is somewhat compromised. Here is my reasoning:

Blockchains like Ethereum, Polygon, Solana, and others have value operations that interact with the blockchain context (storage, configs, and so on). These would have to be implemented using environment calls custom to the blockchain. To reduce the complexity of these calls, we can use a library that provides a set of functions that abstract away the complexity. For example, we can use a library like `riscv-evm-utils` that provides functions abstracting these calls:

```rust 
use riscv_evm_utils::{env, storage};

fn main() {
    let block_number = env::block_number();
    let account_balance = env::balance();
    let storage_value = storage::get(0);

    println!("Block Number: {}", block_number);
    println!("Account Balance: {}", account_balance);
    println!("Storage Value: {}", storage_value);
}
```

In my opinion, developers attracted through this campaign would still need to learn and understand the blockchain context and environment calls custom to the blockchain. This is because the blockchain context and environment calls are complex and require a deep understanding of both the blockchain and its environment.

**My point: It is not going to be all roses. While it would certainly reduce the learning curve—with no need to pick up a new language like Solidity—we shouldn't forget there is still significant work to be done.**

# DRAFT RESULT