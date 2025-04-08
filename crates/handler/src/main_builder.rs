use primitives::RiscvEVM;
use revm::{
    Context, Database,
    context::{Block, Cfg, JournalTr, Transaction},
};

pub trait MainBuilder: Sized {
    type Context;

    fn build_mainnet_with_riscv_evm(self) -> RiscvEVM<Self::Context>;
}

impl<BLOCK, TX, CFG, DB, JOURNAL, CHAIN> MainBuilder for Context<BLOCK, TX, CFG, DB, JOURNAL, CHAIN>
where
    BLOCK: Block,
    TX: Transaction,
    CFG: Cfg,
    DB: Database,
    JOURNAL: JournalTr<Database = DB>,
{
    type Context = Self;

    fn build_mainnet_with_riscv_evm(self) -> RiscvEVM<Self::Context> {
        RiscvEVM { context: self }
    }
}
