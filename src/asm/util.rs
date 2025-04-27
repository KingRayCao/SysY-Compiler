use super::build_func::FuncContext;
use super::gen_riscv::*;
use super::{Addr, Asm, Reg, REG_LIST};
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
// ============ Value Table =================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegStatus {
    Free,
    Used(Addr),
    Temp,
}

pub struct ValueTable {
    pub value_addr: HashMap<Value, Addr>,
    reg_status: HashMap<Reg, RegStatus>,
    addr_reg: HashMap<Addr, Option<Reg>>,
    pub reg_locked: HashMap<Reg, bool>,
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
            addr_reg,
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
        let addr = self.get_value_addr(value).unwrap();
        self.addr_reg.get(&addr).unwrap().clone()
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
            if *v == RegStatus::Free {
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
            for (reg, v) in self.reg_status.iter() {
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
        if let Some(offset) = addr {
            let reg = self.get_free_reg(asm);

            self.addr_reg.insert(offset, Some(reg));
            self.reg_status.insert(reg, RegStatus::Used(offset));
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

                    self.addr_reg.insert(offset, Some(reg));
                    self.reg_status.insert(reg, RegStatus::Used(offset));
                    self.lock_reg(&reg);

                    riscv_lw(reg, "sp", offset, asm, self);
                    return reg;
                } else {
                    panic!("value is not in stack");
                }
            }
        }
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
                        return *reg;
                    }
                }
                // free specified reg
                self.free_reg(reg, asm);
                // load value to specified reg
                if let Some(value_reg) = self.get_value_reg(value) {
                    riscv_mv(reg, value_reg, asm);
                    return reg;
                } else {
                    let addr = self.get_value_addr(value);
                    if let Some(offset) = addr {
                        self.addr_reg.insert(offset, Some(reg));
                        self.reg_status.insert(reg, RegStatus::Used(offset));
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
        self.addr_reg.insert(offset, None);
    }

    pub fn free_reg(&mut self, reg: &Reg, asm: &mut Asm) {
        if *reg == "x0" {
            return;
        }
        if self.reg_is_locked(reg) {
            panic!("reg is locked");
        }
        if let Some(v) = self.reg_status.get_mut(reg) {
            match *v {
                RegStatus::Used(addr) => {
                    *v = RegStatus::Free;
                    self.addr_reg.insert(addr, None);
                    riscv_sw(reg, "sp", addr, asm, self);
                }

                RegStatus::Temp => {
                    *v = RegStatus::Free;
                }
                RegStatus::Free => {}
            }
        }
    }
    pub fn free_regs(&mut self, regs: &Vec<Reg>, asm: &mut Asm) {
        for reg in regs {
            self.free_reg(&reg, asm);
        }
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
