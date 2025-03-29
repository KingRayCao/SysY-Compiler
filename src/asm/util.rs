use super::build_func::FuncContext;
use super::gen_riscv::*;
use super::{Asm, Reg};
use koopa::ir::entities::{BasicBlockData, ValueData};
use koopa::ir::{BasicBlock, FunctionData, Value, ValueKind};
use std::collections::HashMap;

pub fn get_value_data(func_data: &FunctionData, value: Value) -> &ValueData {
    func_data.dfg().value(value)
}
pub fn get_bb_data(func_data: &FunctionData, bb: BasicBlock) -> &BasicBlockData {
    func_data.dfg().bb(bb)
}
pub fn get_bb_name(func_data: &FunctionData, bb: BasicBlock) -> &str {
    &get_bb_data(func_data, bb).name().as_ref().unwrap()[1..]
}
// Reg-Value Table
pub struct RegValueTable {
    reg_value: HashMap<Reg, Option<Option<Value>>>,
    // Some(Some(value)) means the value is in the register
    // Some(None) means the reg is in temporary use
    // None means the reg is free
    value_reg: HashMap<Value, Reg>,
}

impl RegValueTable {
    pub fn new() -> Self {
        let mut reg_value: HashMap<Reg, Option<Option<Value>>> = HashMap::new();
        let value_reg: HashMap<Value, Reg> = HashMap::new();
        reg_value.insert("a0", None);
        reg_value.insert("a1", None);
        reg_value.insert("a2", None);
        reg_value.insert("a3", None);
        reg_value.insert("a4", None);
        reg_value.insert("a5", None);
        reg_value.insert("a6", None);
        reg_value.insert("a7", None);
        reg_value.insert("t0", None);
        reg_value.insert("t1", None);
        reg_value.insert("t2", None);
        reg_value.insert("t3", None);
        reg_value.insert("t4", None);
        reg_value.insert("t5", None);
        reg_value.insert("t6", None);
        RegValueTable {
            reg_value,
            value_reg,
        }
    }

    pub fn get_value(&self, reg: Reg) -> Option<Option<Value>> {
        self.reg_value[&reg]
    }

    pub fn get_reg(&self, value: Value) -> Option<Reg> {
        self.value_reg.get(&value).cloned()
    }

    pub fn alloc_reg(&mut self, reg: Reg, value: Option<Value>) {
        if self.reg_is_free(reg) {
            *self.reg_value.get_mut(&reg).unwrap() = value.map(|v| Some(v));
            if let Some(value) = value {
                self.value_reg.insert(value, reg);
            }
        } else {
            panic!("register {} is not free", reg);
        }
    }

    pub fn alloc_value(&mut self, value: Value) -> Reg {
        for (reg, v) in self.reg_value.iter_mut() {
            if v.is_none() {
                *v = Some(Some(value));
                self.value_reg.insert(value, reg);
                return reg;
            }
        }
        panic!("no available register");
    }

    pub fn free_value(&mut self, value: Value) {
        let reg = self.value_reg[&value];
        if let Some(v) = self.reg_value.get_mut(&reg) {
            if let Some(Some(value)) = v.take() {
                self.value_reg.remove(&value);
            }
        }
    }

    pub fn free_reg(&mut self, reg: Reg) {
        if let Some(v) = self.reg_value.get_mut(&reg) {
            if let Some(Some(value)) = v.take() {
                self.value_reg.remove(&value);
            }
        }
    }

    pub fn reg_is_free(&self, reg: Reg) -> bool {
        self.reg_value[&reg].is_none()
    }

    pub fn value_in_reg(&self, value: Value) -> bool {
        self.value_reg.contains_key(&value)
    }

    pub fn alloc_temp_reg(&mut self) -> Reg {
        for (reg, v) in self.reg_value.iter_mut() {
            if v.is_none() {
                *v = Some(None);
                return reg;
            }
        }
        panic!("no available register");
    }

    pub fn reg_all_free(&mut self) -> bool {
        for (_, v) in self.reg_value.iter_mut() {
            if v.is_some() {
                return false;
            }
        }
        true
    }
}

pub fn print_value_data(value_data: &ValueData) {
    println!("kind: {:?}", value_data.kind());
    println!("type: {:?}", value_data.ty());
    println!("size: {:?}", value_data.ty().size());
}

pub fn print_value(func_data: &FunctionData, func_ctx: &FuncContext, value: Value) {
    let value_data = get_value_data(func_data, value);
    print_value_data(value_data);
}
