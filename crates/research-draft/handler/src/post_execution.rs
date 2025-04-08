use revm::{
    interpreter::{Gas, InitialAndFloorGas},
    primitives::hardfork::SpecId,
};

pub fn refund(spec: SpecId, gas: &mut Gas, eip7702_refund: i64) {
    gas.record_refund(eip7702_refund);
    // Calculate gas refund for transaction.
    // If spec is set to london, it will decrease the maximum refund amount to 5th part of
    // gas spend. (Before london it was 2th part of gas spend)
    gas.set_final_refund(spec.is_enabled_in(SpecId::LONDON));
}

pub fn eip7623_check_gas_floor(gas: &mut Gas, init_and_floor_gas: InitialAndFloorGas) {
    // EIP-7623: Increase calldata cost
    // spend at least a gas_floor amount of gas.
    if gas.spent_sub_refunded() < init_and_floor_gas.floor_gas {
        gas.set_spent(init_and_floor_gas.floor_gas);
        // clear refund
        gas.set_refund(0);
    }
}
