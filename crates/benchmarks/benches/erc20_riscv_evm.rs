//! ERC20 RISC-V EVM Benchmark
//! Performing benches on tasks such as deployment, transfer and balanceOf.
use criterion::{Criterion, criterion_group, criterion_main};
use handler::{api::ExecuteCommitEvm, main_builder::MainBuilder};
use revm::{
    Context, MainContext,
    database::{CacheDB, EmptyDB},
    primitives::{Address, Bytes, TxKind, U256, address, hex},
};

const FROM: Address = address!("5B38Da6a701c568545dCfcB03FcB875f56beddC4");
const TO: Address = address!("Ab8483F64d9C6d1EcF9b849Ae677dD3315835cb2");

fn erc20_bytescode_and_initcode() -> Bytes {
    // Containing the InitCode and Runtime Code
    hex::decode("0x00").unwrap().into()
}

fn bench_erc20_deployment(c: &mut Criterion) {
    c.bench_function("RiscvEVM: Bench ERC20 Deployment", |b| {
        b.iter(|| {
            let bytecode: Bytes = erc20_bytescode_and_initcode();
            let ctx = Context::mainnet()
                .modify_tx_chained(|tx| {
                    tx.caller = FROM;
                    tx.kind = TxKind::Create;
                    tx.data = bytecode.clone();
                    tx.value = U256::from(0);
                })
                .with_db(CacheDB::<EmptyDB>::default());

            let mut evm = ctx.build_mainnet_with_riscv_evm();
            let ref_tx = evm.replay_commit().unwrap();
            // let ExecutionResult::Success {
            //     output: Output::Create(_, Some(_address)),
            //     ..
            // } = ref_tx
            // else {
            //     panic!("Failed to create contract: {ref_tx:#?}");
            // };
        })
    });
}
