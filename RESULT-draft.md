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


# Some Note (not-formal, just for me)
1. I am not doing this like warming storage and other stuffs because I am currently not handle gas metering in this Draft Experiment, it is Kinda out of scope for. In the Final Result all these and others would be included the final result goal is to be Identical to the original EVM, this is to be fare on compute benchs.

2. Ethereum going the route of using Riscv this way (that is keeping the initial design philosophy of the EVM) would carry alot of overhead cost, computationally and developer's experience. I am of the opinion to ditch the initial design philosophy of the EVM completely and use a more modern approach (e.g., RISC-V...), we would need to redefine the blockchain storage, addresses, signature schemes and so on. This is not to undermine the technological exploits birthed in the EVM, but a transistion of this nature would be a significant undertaking, so much is changing, so much could break. Leaving this big questions, how does the future look like? are we (developers) every going to see a future where these innovations will be implemented? Are we (Ethereum decision makers) going to off-load this innovation burden to L2 teams (which would have less resources, and  this could lead to more fagmentations we current trying to solve)... This are really hard questions which need to be answered. it quite funny how these questions gets some persons angry at me, it needs to be addressed either way.

some ethereum opcodes almost consumes all the registers of the RISV VM (SSTORE, LOG2, CALL), sometimes, it actually comsumes all needed more registers for it operation (LOG3, LOG4).

3. It would be nice to have my concluding on the result placed in the beginning and end of the document.

4. I can assure with high certainty that this experimental code is buggy and should not be used for anything looking like a production environment.

5. What would ethereum look like in the next 20 years? Are we stucked with the EVM forever? For real, why do we need a VM other than the EVM?
