use primitives::RiscvEVM;
use revm::{
    Database, DatabaseCommit,
    context::{
        Block, ContextSetters, ContextTr, JournalOutput, JournalTr, Transaction,
        result::{EVMError, ExecutionResult, HaltReason, InvalidTransaction, ResultAndState},
    },
};

/// Execute EVM transactions. Main trait for transaction execution.
pub trait ExecuteEvm {
    /// Output of transaction execution.
    type Output;
    /// Transaction type.
    type Tx: Transaction;
    /// Block type.
    type Block: Block;

    /// Set the transaction.
    fn set_tx(&mut self, tx: Self::Tx);

    /// Set the block.
    fn set_block(&mut self, block: Self::Block);

    /// Transact the transaction that is set in the context.
    fn replay(&mut self) -> Self::Output;

    /// Transact the given transaction.
    ///
    /// Internally sets transaction in context and use `replay` to execute the transaction.
    fn transact(&mut self, tx: Self::Tx) -> Self::Output {
        self.set_tx(tx);
        self.replay()
    }
}

/// Extension of the [`ExecuteEvm`] trait that adds a method that commits the state after execution.
pub trait ExecuteCommitEvm: ExecuteEvm {
    /// Commit output of transaction execution.
    type CommitOutput;

    /// Transact the transaction and commit to the state.
    fn replay_commit(&mut self) -> Self::CommitOutput;

    /// Transact the transaction and commit to the state.
    fn transact_commit(&mut self, tx: Self::Tx) -> Self::CommitOutput {
        self.set_tx(tx);
        self.replay_commit()
    }
}

impl<CTX, P> ExecuteEvm for RiscvEVM<CTX, P>
where
    CTX: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>> + ContextSetters,
{
    type Output = Result<
        ResultAndState<HaltReason>,
        EVMError<<CTX::Db as Database>::Error, InvalidTransaction>,
    >;

    type Tx = <CTX as ContextTr>::Tx;

    type Block = <CTX as ContextTr>::Block;

    fn replay(&mut self) -> Self::Output {
        // let mut t = MainnetHandler::<_, _, EthFrame<_, _, _>>::default();
        // t.run(self)
        todo!()
    }

    fn set_tx(&mut self, tx: Self::Tx) {
        self.context.set_tx(tx);
    }

    fn set_block(&mut self, block: Self::Block) {
        self.context.set_block(block);
    }
}

impl<CTX, P> ExecuteCommitEvm for RiscvEVM<CTX, P>
where
    CTX: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>, Db: DatabaseCommit>
        + ContextSetters,
{
    type CommitOutput = Result<
        ExecutionResult<HaltReason>,
        EVMError<<CTX::Db as Database>::Error, InvalidTransaction>,
    >;

    fn replay_commit(&mut self) -> Self::CommitOutput {
        self.replay().map(|r| {
            self.context.db().commit(r.state);
            r.result
        })
    }
}
