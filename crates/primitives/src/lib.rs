use auto_impl::auto_impl;
use revm::{context::ContextTr, handler::PrecompileProvider};

#[auto_impl(&mut, Box)]
pub trait RiscvEvmTr {
    /// The context type that implements ContextTr to provide access to execution state
    type Context: ContextTr;
    /// The type containing the available precompiled contracts
    type Precompiles: PrecompileProvider<Self::Context>;

    /// Returns a mutable reference to the execution context
    fn ctx(&mut self) -> &mut Self::Context;

    /// Returns an immutable reference to the execution context
    fn ctx_ref(&self) -> &Self::Context;

    /// Returns mutable references to both the context and precompiles.
    /// This enables atomic access to both components when needed.
    fn ctx_precompiles(&mut self) -> (&mut Self::Context, &mut Self::Precompiles);
}

#[derive(Debug)]
pub struct RiscvEVM<Context, P> {
    pub context: Context,
    pub precompiles: P,
}

impl<CTX, P> RiscvEvmTr for RiscvEVM<CTX, P>
where
    CTX: ContextTr,
    P: PrecompileProvider<CTX>,
{
    type Context = CTX;
    type Precompiles = P;

    fn ctx(&mut self) -> &mut Self::Context {
        &mut self.context
    }

    fn ctx_ref(&self) -> &Self::Context {
        &self.context
    }

    #[inline]
    fn ctx_precompiles(&mut self) -> (&mut Self::Context, &mut Self::Precompiles) {
        (&mut self.context, &mut self.precompiles)
    }
}
