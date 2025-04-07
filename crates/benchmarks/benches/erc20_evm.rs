//! This benchmark tests the performance of the ERC20 contract on the EVM.
//! Performing benches on tasks such as deployment, transfer, approval and balanceOf.
use criterion::{Criterion, black_box, criterion_group, criterion_main};

use revm::{
    ExecuteCommitEvm, ExecuteEvm, MainBuilder, MainContext,
    bytecode::opcode,
    context::Context,
    context_interface::result::{ExecutionResult, Output},
    database::CacheDB,
    database_interface::EmptyDB,
    handler::EvmTr,
    primitives::{Bytes, TxKind, U256, hex},
};

fn bench_erc20_deployment(c: &mut Criterion) {
    c.bench_function("RISCVEVM: Bench ERC20 Deployment", |b| {
        b.iter(|| {
            // Code goes here
        })
    });
}

criterion_group!(benches, bench_erc20_deployment,);
criterion_main!(benches);
