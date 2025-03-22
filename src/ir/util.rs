use super::*;
use koopa::ir::{builder::LocalBuilder, BasicBlock, Program, Value};

pub fn new_value<'a>(program: &'a mut Program, context: &'a mut IrContext) -> LocalBuilder<'a> {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.dfg_mut().new_value()
}

pub fn add_value(
    program: &mut Program,
    context: &mut IrContext,
    value: Value,
) -> Result<(), String> {
    let func_data = program.func_mut(context.current_func.unwrap());
    let bb = context.current_block.unwrap();
    let insert_ok = func_data
        .layout_mut()
        .bb_mut(bb)
        .insts_mut()
        .push_key_back(value);
    match insert_ok {
        Ok(_) => Ok(()),
        Err(value) => Err(format!("Failed to insert value: {:?}", value)),
    }
}
