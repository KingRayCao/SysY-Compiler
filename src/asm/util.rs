use super::gen_riscv::*;
use super::{Addr, Asm, Reg, REG_LIST};
use koopa::ir::entities::{BasicBlockData, ValueData};
use koopa::ir::types::TypeKind;
use koopa::ir::values::Aggregate;
use koopa::ir::{BasicBlock, FunctionData, Program, Value, ValueKind};
use std::collections::HashMap;

pub fn get_value_data<'a>(func_data: &'a FunctionData, value: Value) -> &'a ValueData {
    func_data.dfg().value(value)
}

pub fn get_bb_data(func_data: &FunctionData, bb: BasicBlock) -> &BasicBlockData {
    func_data.dfg().bb(bb)
}
pub fn get_bb_name(func_data: &FunctionData, bb: BasicBlock) -> &str {
    &get_bb_data(func_data, bb).name().as_ref().unwrap()[1..]
}
// ============ Value Table =================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegStatus {
    Free,
    Used(Value),
    Temp,
}

pub const PARAM_ADDR: i32 = -1;
pub const GLOBL_ADDR: i32 = -2;

pub struct ValueTable {
    value_addr: HashMap<Value, Addr>,
    reg_status: HashMap<Reg, RegStatus>,
    value_reg: HashMap<Value, Option<Reg>>,
    reg_locked: HashMap<Reg, bool>,
}

impl ValueTable {
    pub fn new() -> Self {
        let value_addr: HashMap<Value, Addr> = HashMap::new();
        let mut reg_status: HashMap<Reg, RegStatus> = HashMap::new();
        let addr_reg = HashMap::new();
        let mut reg_locked: HashMap<Reg, bool> = HashMap::new();
        for reg in REG_LIST.iter() {
            reg_status.insert(*reg, RegStatus::Free);
            reg_locked.insert(*reg, false);
        }
        ValueTable {
            value_addr,
            reg_status,
            value_reg: addr_reg,
            reg_locked,
        }
    }

    pub fn get_value_addr(&self, value: &Value) -> Option<Addr> {
        self.value_addr.get(value).map(|v| *v)
    }

    pub fn get_reg_status(&self, reg: &Reg) -> RegStatus {
        self.reg_status[reg]
    }

    pub fn get_value_reg(&self, value: &Value) -> Option<Reg> {
        self.value_reg.get(&value).unwrap().clone()
    }

    // lock reg, so that it cannot be used by other values
    // be sure to lock reg before generating riscv
    pub fn lock_reg(&mut self, reg: &Reg) {
        if let Some(v) = self.reg_locked.get_mut(reg) {
            *v = true;
        }
    }

    pub fn reg_is_locked(&self, reg: &Reg) -> bool {
        *self.reg_locked.get(reg).unwrap()
    }

    pub fn reg_all_unlocked(&self) -> bool {
        for (_, v) in self.reg_locked.iter() {
            if *v {
                return false;
            }
        }
        true
    }

    pub fn unlock_reg(&mut self, reg: &Reg) {
        if let Some(v) = self.reg_locked.get_mut(reg) {
            *v = false;
        }
    }

    pub fn get_free_reg(&mut self, asm: &mut Asm) -> Reg {
        for (reg, v) in self.reg_status.iter() {
            if *v == RegStatus::Free && !self.reg_is_locked(reg) {
                return reg;
            }
        }
        let mut reg_to_free = None;
        for (reg, v) in self.reg_status.iter() {
            if !self.reg_is_locked(reg) {
                if let RegStatus::Temp = v {
                    reg_to_free = Some(*reg);
                    break;
                }
            }
        }

        if let None = reg_to_free {
            for (reg, _) in self.reg_status.iter() {
                if !self.reg_is_locked(reg) {
                    reg_to_free = Some(*reg);
                    break;
                }
            }
        }
        if let Some(reg) = reg_to_free {
            self.free_reg(&reg, asm);
            return reg;
        } else {
            panic!("all regs are locked");
        }
    }

    pub fn allocate_value_to_reg(&mut self, value: &Value, asm: &mut Asm) -> Reg {
        // if value is already in reg, return reg
        if let Some(reg) = self.get_value_reg(value) {
            self.lock_reg(&reg);
            return reg;
        }
        // value in stack
        let addr = self.get_value_addr(value);
        if let Some(_offset) = addr {
            let reg = self.get_free_reg(asm);

            self.value_reg.insert(*value, Some(reg));
            self.reg_status.insert(reg, RegStatus::Used(*value));
            self.lock_reg(&reg);

            return reg;
        } else {
            panic!("value is not in stack");
        }
    }

    pub fn assign_value_to_reg(
        &mut self,
        value: &Value,
        value_data: &ValueData,
        asm: &mut Asm,
    ) -> Reg {
        match value_data.kind() {
            ValueKind::Integer(num) => self.assign_temp_to_reg(num.value(), asm),
            _ => {
                // if value is already in reg, return reg
                if let Some(reg) = self.get_value_reg(value) {
                    self.lock_reg(&reg);
                    return reg;
                }
                // value in stack
                let addr = self.get_value_addr(value);
                if let Some(offset) = addr {
                    let reg = self.get_free_reg(asm);

                    self.value_reg.insert(*value, Some(reg));
                    self.reg_status.insert(reg, RegStatus::Used(*value));
                    self.lock_reg(&reg);

                    riscv_lw(reg, "sp", offset, asm, self);
                    return reg;
                } else {
                    panic!("value is not in stack");
                }
            }
        }
    }

    pub fn set_value_to_reg(&mut self, value: &Value, _value_data: &ValueData, reg: &Reg) {
        self.value_reg.insert(*value, Some(reg));
        self.reg_status.insert(reg, RegStatus::Used(*value));
        self.lock_reg(reg);
    }

    pub fn assign_value_to_specified_reg(
        &mut self,
        value: &Value,
        value_data: &ValueData,
        reg: &Reg,
        asm: &mut Asm,
    ) -> Reg {
        match value_data.kind() {
            ValueKind::Integer(num) => {
                self.assign_temp_to_specified_reg(num.value(), reg, asm);
                return *reg;
            }
            _ => {
                if let Some(value_reg) = self.get_value_reg(value) {
                    if value_reg == *reg {
                        self.lock_reg(reg);
                        return *reg;
                    }
                }
                // free specified reg
                self.free_reg(reg, asm);
                // load value to specified reg
                if let Some(value_reg) = self.get_value_reg(value) {
                    riscv_mv(reg, value_reg, asm);
                    self.value_reg.insert(*value, Some(reg));
                    self.reg_status.insert(reg, RegStatus::Used(*value));
                    self.reg_status.insert(value_reg, RegStatus::Free);
                    self.lock_reg(reg);
                    return reg;
                } else {
                    let addr = self.get_value_addr(value);
                    if let Some(offset) = addr {
                        self.value_reg.insert(*value, Some(reg));
                        self.reg_status.insert(reg, RegStatus::Used(*value));
                        self.lock_reg(&reg);
                        riscv_lw(reg, "sp", offset, asm, self);
                        return reg;
                    } else {
                        panic!("value is not in stack");
                    }
                }
            }
        }
    }

    pub fn assign_temp_to_reg(&mut self, temp: i32, asm: &mut Asm) -> Reg {
        if temp == 0 {
            return "x0";
        }
        // find a free reg
        let reg = self.get_free_reg(asm);
        self.reg_status.insert(reg, RegStatus::Temp);
        self.lock_reg(&reg);
        asm.push_str(&format!("  li {}, {}\n", reg, temp));
        reg
    }

    pub fn assign_temp_to_specified_reg(&mut self, temp: i32, reg: &Reg, asm: &mut Asm) {
        // free specified reg
        self.free_reg(reg, asm);
        // assign temp to specified reg
        self.reg_status.insert(*reg, RegStatus::Temp);
        self.lock_reg(reg);
        asm.push_str(&format!("  li {}, {}\n", reg, temp));
    }

    pub fn alloc_value(&mut self, value: Value, offset: i32) {
        self.value_addr.insert(value, offset);
        self.value_reg.insert(value, None);
    }

    pub fn free_reg(&mut self, reg: &Reg, asm: &mut Asm) {
        // println!("free reg: {:?}", reg);
        if *reg == "x0" {
            return;
        }
        if self.reg_is_locked(reg) {
            panic!("reg is locked");
        }
        self.lock_reg(reg);
        if let Some(v) = self.reg_status.get_mut(reg) {
            match *v {
                RegStatus::Used(value) => {
                    *v = RegStatus::Free;
                    self.value_reg.insert(value, None);
                    let addr = self.get_value_addr(&value).unwrap();
                    if addr != PARAM_ADDR && addr != GLOBL_ADDR {
                        riscv_sw(reg, "sp", addr, asm, self);
                    }
                }

                RegStatus::Temp => {
                    *v = RegStatus::Free;
                }
                RegStatus::Free => {}
            }
        }
        self.unlock_reg(reg);
    }
    pub fn free_regs(&mut self, regs: &Vec<Reg>, asm: &mut Asm) {
        for reg in regs {
            self.free_reg(&reg, asm);
        }
    }
}

// ================== Array ====================

pub fn aggregate_to_asm(prog: &Program, aggregate: &Aggregate, asm: &mut Asm) {
    for elem in aggregate.elems().iter() {
        let elem_data = prog.borrow_value(*elem);
        match elem_data.kind() {
            ValueKind::Integer(num) => {
                asm.push_str(&format!("  .word {}\n", num.value()));
            }
            ValueKind::ZeroInit(zeroinit) => {
                let size = elem_data.ty().size();
                asm.push_str(&format!("  .zero {}\n", size));
            }
            ValueKind::Aggregate(aggr) => {
                aggregate_to_asm(prog, aggr, asm);
            }
            _ => unreachable!(),
        }
    }
}

pub fn get_elem_ptr_step(array_data: &ValueData) -> usize {
    match array_data.ty().kind() {
        TypeKind::Pointer(ty) => match ty.kind() {
            TypeKind::Array(ty, _size) => ty.size(),
            TypeKind::Pointer(ty) => ty.size(),
            _ => {
                panic!("array step error");
            }
        },
        _ => {
            panic!("array step error");
        }
    }
}

pub fn get_ptr_step(array_data: &ValueData) -> usize {
    match array_data.ty().kind() {
        TypeKind::Pointer(ty) => ty.size(),
        _ => {
            panic!("array step error");
        }
    }
}

// ================= Debug ==================

pub fn print_value_data(value_data: &ValueData) {
    println!("kind: {:?}", value_data.kind());
    println!("type: {:?}", value_data.ty());
    println!("size: {:?}", value_data.ty().size());
}

pub fn print_value(func_data: &FunctionData, value: Value) {
    let value_data = get_value_data(func_data, value);
    print_value_data(value_data);
}
