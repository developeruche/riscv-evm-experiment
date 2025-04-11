use crate::{
    context::Context,
    vm::{VMErrors, Vm},
};

pub fn process_ecall(vm: &mut Vm, context: &mut Context) -> Result<(), VMErrors> {
    Ok(())
}
