use primitives::RiscvEvmTr;
use revm::{
    Database,
    context::{
        Block, Cfg, ContextTr, JournalTr, Transaction, TransactionType,
        transaction::AccessListItemTr,
    },
    handler::PrecompileProvider,
    primitives::{U256, hardfork::SpecId},
};

/// This function warms and loads Precompile addresses, coinbase, and access list
pub fn load_accounts<
    EVM: RiscvEvmTr,
    ERROR: From<<<EVM::Context as ContextTr>::Db as Database>::Error>,
>(
    riscv_evm: &mut EVM,
) -> Result<(), ERROR> {
    let (context, precompiles) = riscv_evm.ctx_precompiles();

    let gen_spec = context.cfg().spec();
    let spec = gen_spec.clone().into();
    // sets eth spec id in journal
    context.journal().set_spec_id(spec);
    let precompiles_changed = precompiles.set_spec(gen_spec);
    let empty_warmed_precompiles = context.journal().precompile_addresses().is_empty();

    if precompiles_changed || empty_warmed_precompiles {
        // load new precompile addresses into journal.
        // When precompiles addresses are changed we reset the warmed hashmap to those new addresses.
        context
            .journal()
            .warm_precompiles(precompiles.warm_addresses().collect());
    }

    // Load coinbase
    // EIP-3651: Warm COINBASE. Starts the `COINBASE` address warm
    if spec.is_enabled_in(SpecId::SHANGHAI) {
        let coinbase = context.block().beneficiary();
        context.journal().warm_account(coinbase);
    }

    // Load access list
    let (tx, journal) = context.tx_journal();
    if let Some(access_list) = tx.access_list() {
        for item in access_list {
            let address = item.address();
            let mut storage = item.storage_slots().peekable();
            if storage.peek().is_none() {
                journal.warm_account(*address);
            } else {
                journal.warm_account_and_storage(
                    *address,
                    storage.map(|i| U256::from_be_bytes(i.0)),
                )?;
            }
        }
    }

    Ok(())
}

/// Handles initial gas accounting, removing the higtest possible gas this user can pay...change would be refunded later
#[inline]
pub fn deduct_caller<CTX: ContextTr>(
    context: &mut CTX,
) -> Result<(), <CTX::Db as Database>::Error> {
    let basefee = context.block().basefee();
    let blob_price = context.block().blob_gasprice().unwrap_or_default();
    let effective_gas_price = context.tx().effective_gas_price(basefee as u128);
    let is_balance_check_disabled = context.cfg().is_balance_check_disabled();
    let value = context.tx().value();

    // Subtract gas costs from the caller's account.
    // We need to saturate the gas cost to prevent underflow in case that `disable_balance_check` is enabled.
    let mut gas_cost = (context.tx().gas_limit() as u128).saturating_mul(effective_gas_price);

    // EIP-4844
    if context.tx().tx_type() == TransactionType::Eip4844 {
        let blob_gas = context.tx().total_blob_gas() as u128;
        gas_cost = gas_cost.saturating_add(blob_price.saturating_mul(blob_gas));
    }

    let is_call = context.tx().kind().is_call();
    let caller = context.tx().caller();

    // Load caller's account.
    let caller_account = context.journal().load_account(caller)?.data;
    // Set new caller account balance.
    caller_account.info.balance = caller_account
        .info
        .balance
        .saturating_sub(U256::from(gas_cost));

    if is_balance_check_disabled {
        // Make sure the caller's balance is at least the value of the transaction.
        caller_account.info.balance = value.max(caller_account.info.balance);
    }

    // Bump the nonce for calls. Nonce for CREATE will be bumped in `handle_create`.
    if is_call {
        // Nonce is already checked
        caller_account.info.nonce = caller_account.info.nonce.saturating_add(1);
    }

    // Touch account so we know it is changed.
    caller_account.mark_touch();
    Ok(())
}
