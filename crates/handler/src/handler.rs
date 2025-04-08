//! This is a handler implementation pulled from the revm implementation and modified to accommodate this experiment
use primitives::RiscvEvmTr;
use revm::{
    Database,
    context::{
        Cfg, ContextTr, JournalOutput, JournalTr, Transaction,
        result::{
            FromStringError, HaltReasonTr, InvalidHeader, InvalidTransaction, ResultAndState,
        },
    },
    context_interface::context::ContextError,
    handler::{Frame, FrameResult, post_execution, validation},
    interpreter::{FrameInput, InitialAndFloorGas},
};

use crate::{execution, pre_execution};

pub trait EvmTrError<EVM: RiscvEvmTr>:
    From<InvalidTransaction>
    + From<InvalidHeader>
    + From<<<EVM::Context as ContextTr>::Db as Database>::Error>
    + FromStringError
{
}

impl<
    EVM: RiscvEvmTr,
    T: From<InvalidTransaction>
        + From<InvalidHeader>
        + From<<<EVM::Context as ContextTr>::Db as Database>::Error>
        + FromStringError,
> EvmTrError<EVM> for T
{
}

/// The main implementation of Ethereum Mainnet transaction execution.
pub trait Handler<CTX: ContextTr> {
    /// This is the VM (RiscvEVM)
    type RiscvEVM: RiscvEvmTr<Context: ContextTr<Journal: JournalTr<FinalOutput = JournalOutput>>>;
    /// The Frame type containing data for frame execution. Supports Call, Create and EofCreate frames.
    type Frame: Frame<
            Evm = Self::RiscvEVM,
            Error = Self::Error,
            FrameResult = FrameResult,
            FrameInit = FrameInput,
        >;
    /// The halt reason type included in the output
    type HaltReason: HaltReasonTr;
    /// This is the Error type
    type Error: EvmTrError<Self::RiscvEVM>;

    /// The main entry point for transaction execution.
    #[inline]
    fn run(
        &mut self,
        riscv_evm: &mut Self::RiscvEVM,
    ) -> Result<ResultAndState<Self::HaltReason>, Self::Error> {
        // Run inner handler and catch all errors to handle cleanup.
        match self.run_without_catch_error(riscv_evm) {
            Ok(output) => Ok(output),
            Err(e) => self.catch_error(riscv_evm, e),
        }
    }

    #[inline]
    fn run_without_catch_error(
        &mut self,
        riscv_evm: &mut Self::RiscvEVM,
    ) -> Result<ResultAndState<Self::HaltReason>, Self::Error> {
        let init_and_floor_gas = self.validate(riscv_evm)?;
        let eip7702_refund = self.pre_execution(riscv_evm)? as i64;
        let exec_result = self.execution(riscv_evm, &init_and_floor_gas)?;
        self.post_execution(riscv_evm, exec_result, init_and_floor_gas, eip7702_refund)
    }

    //=========================
    // Validatation
    // ========================
    // This stage  validates the tx/block/chain_config fields, loads caller account and validates inital gas requirements and performs a balance check

    #[inline]
    fn validate(&self, riscv_evm: &mut Self::RiscvEVM) -> Result<InitialAndFloorGas, Self::Error> {
        self.validate_env(riscv_evm)?;
        let initial_and_floor_gas = self.validate_initial_tx_gas(riscv_evm)?;
        self.validate_tx_against_state(riscv_evm)?;
        Ok(initial_and_floor_gas)
    }

    #[inline]
    fn validate_initial_tx_gas(
        &self,
        evm: &Self::RiscvEVM,
    ) -> Result<InitialAndFloorGas, Self::Error> {
        let ctx = evm.ctx_ref();
        validation::validate_initial_tx_gas(ctx.tx(), ctx.cfg().spec().into()).map_err(From::from)
    }

    #[inline]
    fn validate_env(&self, riscv_evm: &mut Self::RiscvEVM) -> Result<(), Self::Error> {
        validation::validate_env(riscv_evm.ctx())
    }

    #[inline]
    fn validate_tx_against_state(&self, riscv_evm: &mut Self::RiscvEVM) -> Result<(), Self::Error> {
        validation::validate_tx_against_state(riscv_evm.ctx())
    }

    //=========================
    // Pre-Execution
    // ========================
    // During this stage accounts are loaded and warmed and intial gas is deducted
    #[inline]
    fn pre_execution(&self, evm: &mut Self::RiscvEVM) -> Result<u64, Self::Error> {
        self.load_accounts(evm)?;
        self.deduct_caller(evm)?;
        let gas = 0; // This is zero because EIP7702 is not supported in the VM to reduce complexity
        Ok(gas)
    }

    #[inline]
    fn load_accounts(&self, evm: &mut Self::RiscvEVM) -> Result<(), Self::Error> {
        pre_execution::load_accounts(evm)
    }

    #[inline]
    fn deduct_caller(&self, evm: &mut Self::RiscvEVM) -> Result<(), Self::Error> {
        pre_execution::deduct_caller(evm.ctx()).map_err(From::from)
    }

    //=========================
    // Execution
    // ========================
    // Executes the main frame loop, delegating to Frame for sub-calls
    #[inline]
    fn execution(
        &mut self,
        evm: &mut Self::RiscvEVM,
        init_and_floor_gas: &InitialAndFloorGas,
    ) -> Result<FrameResult, Self::Error> {
        let gas_limit = evm.ctx().tx().gas_limit() - init_and_floor_gas.initial_gas;

        // Create first frame action
        let first_frame_input = self.first_frame_input(evm, gas_limit)?;
        let first_frame = self.first_frame_init(evm, first_frame_input)?;
        let mut frame_result = match first_frame {
            ItemOrResult::Item(frame) => self.run_exec_loop(evm, frame)?,
            ItemOrResult::Result(result) => result,
        };

        self.last_frame_result(evm, &mut frame_result)?;
        Ok(frame_result)
    }

    /// Creates initial frame input using transaction parameters, gas limit and configuration.
    #[inline]
    fn first_frame_input(
        &mut self,
        evm: &mut Self::RiscvEVM,
        gas_limit: u64,
    ) -> Result<FrameInput, Self::Error> {
        let ctx: &<<Self as Handler<CTX>>::RiscvEVM as RiscvEvmTr>::Context = evm.ctx_ref();
        Ok(execution::create_init_frame(
            ctx.tx(),
            ctx.cfg().spec().into(),
            gas_limit,
        ))
    }

    //=========================
    // Post-Execution
    // ========================
    // The stage calculates final refunds, validates gas floor, reimburses caller and rewards beneficary
    #[inline]
    fn post_execution(
        &self,
        evm: &mut Self::RiscvEVM,
        mut exec_result: FrameResult,
        init_and_floor_gas: InitialAndFloorGas,
        eip7702_gas_refund: i64,
    ) -> Result<ResultAndState<Self::HaltReason>, Self::Error> {
        // Calculate final refund and add EIP-7702 refund to gas.
        self.refund(evm, &mut exec_result, eip7702_gas_refund);
        // Ensure gas floor is met and minimum floor gas is spent.
        self.eip7623_check_gas_floor(evm, &mut exec_result, init_and_floor_gas);
        // Return unused gas to caller
        self.reimburse_caller(evm, &mut exec_result)?;
        // Pay transaction fees to beneficiary
        self.reward_beneficiary(evm, &mut exec_result)?;
        // Prepare transaction output
        self.output(evm, exec_result)
    }

    /// Calculates the final gas refund amount, including any EIP-7702 refunds (Which would be zero in this case).
    #[inline]
    fn refund(
        &self,
        riscv_evm: &mut Self::RiscvEVM,
        exec_result: &mut <Self::Frame as Frame>::FrameResult,
        eip7702_refund: i64,
    ) {
        let spec = riscv_evm.ctx().cfg().spec().into();
        post_execution::refund(spec, exec_result.gas_mut(), eip7702_refund)
    }

    /// Validates that the minimum gas floor requirements are satisfied.
    ///
    /// Ensures that at least the floor gas amount has been consumed during execution.
    #[inline]
    fn eip7623_check_gas_floor(
        &self,
        _riscv_evm: &mut Self::RiscvEVM,
        exec_result: &mut <Self::Frame as Frame>::FrameResult,
        init_and_floor_gas: InitialAndFloorGas,
    ) {
        post_execution::eip7623_check_gas_floor(exec_result.gas_mut(), init_and_floor_gas)
    }

    /// Returns unused gas costs to the transaction sender's account.
    #[inline]
    fn reimburse_caller(
        &self,
        riscv_evm: &mut Self::RiscvEVM,
        exec_result: &mut <Self::Frame as Frame>::FrameResult,
    ) -> Result<(), Self::Error> {
        post_execution::reimburse_caller(riscv_evm.ctx(), exec_result.gas_mut()).map_err(From::from)
    }

    /// Transfers transaction fees to the block beneficiary's account.
    #[inline]
    fn reward_beneficiary(
        &self,
        riscv_evm: &mut Self::RiscvEVM,
        exec_result: &mut <Self::Frame as Frame>::FrameResult,
    ) -> Result<(), Self::Error> {
        post_execution::reward_beneficiary(riscv_evm.ctx(), exec_result.gas_mut())
            .map_err(From::from)
    }

    /// Processes the final execution output.
    ///
    /// This method, retrieves the final state from the journal, converts internal results to the external output format.
    /// Internal state is cleared and EVM is prepared for the next transaction.
    #[inline]
    fn output(
        &self,
        riscv_evm: &mut Self::RiscvEVM,
        result: <Self::Frame as Frame>::FrameResult,
    ) -> Result<ResultAndState<Self::HaltReason>, Self::Error> {
        match core::mem::replace(riscv_evm.ctx().error(), Ok(())) {
            Err(ContextError::Db(e)) => return Err(e.into()),
            Err(ContextError::Custom(e)) => return Err(Self::Error::from_string(e)),
            Ok(_) => (),
        }

        let output = post_execution::output(riscv_evm.ctx(), result);

        // Clear journal
        riscv_evm.ctx().journal().clear();
        Ok(output)
    }

    #[inline]
    fn catch_error(
        &self,
        riscv_evm: &mut Self::RiscvEVM,
        error: Self::Error,
    ) -> Result<ResultAndState<Self::HaltReason>, Self::Error> {
        // Clean up journal state if error occurs
        riscv_evm.ctx().journal().clear();
        Err(error)
    }
}
