use super::build_value::value_to_asm;
use super::gen_riscv::*;
use super::util::ValueTable;
use super::util::*;
use super::GenerateAsm;
use super::{Asm, Reg};
use koopa::ir::entities::ValueData;
use koopa::ir::{FunctionData, Value, ValueKind};
use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;

type ValueAddr = HashMap<Value, i32>;
pub struct FuncContext<'a> {
    pub func_data: &'a FunctionData,
    pub stack_size: usize,
    pub current_offset: usize,
    pub value_table: ValueTable,
    pub has_call: bool,
    pub max_param_num: i32,
}

impl GenerateAsm for FunctionData {
    fn to_asm(&self) -> Asm {
        let mut asm = String::new();

        let mut func_context = FuncContext::new(self);
        // prologue
        asm.push_str(&format!("{}:\n", &self.name()[1..]));
        riscv_bin_op_imm(
            "add",
            "sp",
            "sp",
            -(func_context.stack_size as i32),
            &mut asm,
            &mut func_context.value_table,
        );
        // body
        for (&bb, node) in self.layout().bbs() {
            let bb_name = get_bb_name(self, bb);
            if !bb_name.starts_with("entry") {
                asm.push_str(&format!("\n{}:\n", bb_name));
            }
            // println!("bb: {}", bb_name);
            for &inst in node.insts().keys() {
                value_to_asm(inst, &mut asm, &mut func_context);
            }
        }
        return asm;
    }
}

impl<'a> FuncContext<'a> {
    pub fn new(func_data: &'a FunctionData) -> Self {
        let mut func_context = FuncContext {
            func_data: func_data,
            stack_size: 0,
            current_offset: 0,
            value_table: ValueTable::new(),
            has_call: false,
            max_param_num: 0,
        };
        let stack_size = Self::get_stack_size(func_data, &mut func_context);
        func_context.stack_size = stack_size;
        // TODO: 初始化value_table为param
        func_context
    }

    pub fn get_value_data(&self, value: Value) -> &ValueData {
        get_value_data(self.func_data, value)
    }

    fn get_stack_size(func_data: &FunctionData, func_context: &mut FuncContext) -> usize {
        let mut stack_size = 0;
        for (&bb, node) in func_data.layout().bbs() {
            for &inst in node.insts().keys() {
                stack_size += Self::get_value_stack_size(func_data, inst, func_context);
            }
        }
        if func_context.has_call {
            stack_size += 4;
        }
        stack_size += (max(func_context.max_param_num - 8, 0) * 4) as usize;
        // align to 16
        stack_size = (stack_size + 15) / 16 * 16;
        return stack_size;
    }

    pub fn get_value_stack_size(
        func_data: &FunctionData,
        value: Value,
        func_context: &mut FuncContext,
    ) -> usize {
        let valuedata = func_data.dfg().value(value);
        // 注意区分TypeKind和ValueKind
        // match valuedata.kind() {
        //     ValueKind::Alloc(alloc) => valuedata.ty().size(),
        //     ValueKind::Load(_) => valuedata.ty().size(),
        //     ValueKind::Integer(_) => valuedata.ty().size(),
        //     ValueKind::Binary(bin) => valuedata.ty().size(),
        //     ValueKind::Store(store) => valuedata.ty().size(),
        //     ValueKind::Call(call) => {
        //         func_context.has_call = true;
        //         func_context.max_param_num = max(func_context.max_param_num, call.args().len() as i32);
        //         valuedata.ty().size()
        //     }
        //     _ => valuedata.ty().size(),
        // }
        valuedata.ty().size()
    }

    // For Debug
    pub fn print_value(&self, value: Value) {
        let value_data = self.func_data.dfg().value(value);
        println!("value: {:?}", value);
        print_value_data(value_data);
    }
}
