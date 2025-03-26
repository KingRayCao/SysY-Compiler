use super::build_value::value_to_asm;
use super::gen_riscv::*;
use super::util::RegValueTable;
use super::util::*;
use super::GenerateAsm;
use super::{Reg, ASM};
use koopa::ir::entities::ValueData;
use koopa::ir::{FunctionData, Value, ValueKind};
use std::collections::HashMap;

type ValueAddr = HashMap<Value, i32>;
pub struct FuncContext<'a> {
    pub func_data: &'a FunctionData,
    pub stack_size: usize,
    pub value_addr: ValueAddr,
    pub current_offset: usize,
    pub reg_value_table: RegValueTable,
}

impl GenerateAsm for FunctionData {
    fn to_asm(&self) -> ASM {
        let mut asm = String::new();

        let mut func_context = FuncContext::new(self);
        // prologue
        asm += &format!("{}:\n", &self.name()[1..]);
        asm += &riscv_bin_op_imm("add", "sp", "sp", -(func_context.stack_size as i32));
        // body
        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                asm += &value_to_asm(inst, &mut func_context);
            }
        }
        return asm;
    }
}

impl<'a> FuncContext<'a> {
    pub fn new(func_data: &'a FunctionData) -> Self {
        let func_context = FuncContext {
            func_data: func_data,
            stack_size: Self::get_stack_size(func_data),
            value_addr: ValueAddr::new(),
            current_offset: 0,
            reg_value_table: RegValueTable::new(),
        };
        func_context
    }

    pub fn get_value_data(&self, value: Value) -> &ValueData {
        get_value_data(self.func_data, value)
    }

    fn get_stack_size(func_data: &FunctionData) -> usize {
        let mut stack_size = 0;
        for (&bb, node) in func_data.layout().bbs() {
            for &inst in node.insts().keys() {
                stack_size += Self::get_value_stack_size(func_data, inst);
            }
        }
        // align to 16
        stack_size = (stack_size + 15) / 16 * 16;
        return stack_size;
    }

    pub fn get_value_stack_size(func_data: &FunctionData, value: Value) -> usize {
        let valuedata = func_data.dfg().value(value);
        // 注意区分TypeKind和ValueKind
        match valuedata.kind() {
            ValueKind::Alloc(_) => 4,
            ValueKind::Load(_) => 0,
            ValueKind::Integer(_) => 0,
            ValueKind::Binary(bin) => {
                valuedata.ty().size()
                    + Self::get_value_stack_size(func_data, bin.lhs())
                    + Self::get_value_stack_size(func_data, bin.rhs())
            }
            ValueKind::Store(store) => {
                valuedata.ty().size() + Self::get_value_stack_size(func_data, store.value())
            }
            _ => valuedata.ty().size(),
        }
    }

    pub fn load_value_to_reg(&mut self, value: Value) -> (ASM, Reg) {
        match self.reg_value_table.get_reg(value) {
            Some(reg) => {
                // already in reg
                (String::new(), reg)
            }
            _ => {
                // not in reg
                let reg = self.reg_value_table.alloc_value(value);
                (riscv_lw(reg, "sp", self.value_addr[&value]), reg)
            }
        }
    }

    // For Debug
    pub fn print_value(&self, value: Value) {
        let value_data = self.func_data.dfg().value(value);
        println!("value: {:?}", value);
        print_value_data(value_data);
    }
}
