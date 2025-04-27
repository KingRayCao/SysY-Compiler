use super::build_value::value_to_asm;
use super::gen_riscv::*;
use super::util::*;
use super::GenerateAsm;
use super::REG_LIST;
use super::{Asm, Reg};
use koopa::ir::entities::ValueData;
use koopa::ir::values::Call;
use koopa::ir::{FunctionData, Program, Value, ValueKind};
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
    pub program: &'a Program,
}

impl GenerateAsm for FunctionData {
    fn to_asm(&self, prog: &Program) -> Asm {
        if self.layout().bbs().len() == 0 {
            return Asm::new();
        }
        let mut asm = String::new();

        let mut func_context = FuncContext::new(self, prog);
        // ------------- prologue --------------
        // update sp
        asm.push_str(&format!("{}:\n", &self.name()[1..]));
        riscv_bin_op_imm(
            "add",
            "sp",
            "sp",
            -(func_context.stack_size as i32),
            &mut asm,
            &mut func_context.value_table,
        );
        if func_context.has_call {
            riscv_sw(
                "ra",
                "sp",
                func_context.stack_size as i32 - 4,
                &mut asm,
                &mut func_context.value_table,
            );
        }
        func_context.current_offset = max(func_context.max_param_num - 8, 0) as usize * 4;
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
    pub fn new(func_data: &'a FunctionData, prog: &'a Program) -> Self {
        let mut func_context = FuncContext {
            func_data: func_data,
            stack_size: 0,
            current_offset: 0,
            value_table: ValueTable::new(),
            has_call: false,
            max_param_num: 0,
            program: prog,
        };
        let stack_size = Self::get_stack_size(func_data, &mut func_context);
        func_context.stack_size = stack_size;
        // 初始化value_table为param
        for (i, param) in func_data.params().iter().enumerate() {
            let value = *param;
            let value_data = func_data.dfg().value(value);
            if i < 8 {
                func_context.value_table.alloc_value(value, PARAM_ADDR);
                func_context
                    .value_table
                    .set_value_to_reg(&value, value_data, &REG_LIST[i]);
                func_context.value_table.unlock_reg(&REG_LIST[i]);
            } else {
                func_context
                    .value_table
                    .alloc_value(value, (stack_size + (i - 8) * 4) as i32);
            }
        }
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
                let valuedata = func_data.dfg().value(inst);
                if let ValueKind::Call(call) = valuedata.kind() {
                    func_context.has_call = true;
                    let callee = call.callee();
                    func_context.max_param_num = max(
                        func_context.max_param_num,
                        func_context.program.func(callee).params().len() as i32,
                    );
                }
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
        valuedata.ty().size()
    }

    // For Debug
    pub fn print_value(&self, value: Value) {
        let value_data = self.func_data.dfg().value(value);
        println!("value: {:?}", value);
        print_value_data(value_data);
    }
}
