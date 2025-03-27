use super::gen_riscv::*;
use super::util::*;
use super::{Asm, Reg};
use crate::asm::build_func::FuncContext;
use koopa::ir::{BinaryOp, Value, ValueKind};

pub fn value_to_asm(value: Value, func_ctx: &mut FuncContext) -> Asm {
    let func_data = func_ctx.func_data;
    let value_data = func_data.dfg().value(value);
    let mut asm = String::new();
    func_ctx.print_value(value);
    match value_data.kind() {
        ValueKind::Integer(int) => {
            // let int_reg = func_ctx.reg_value_table.alloc_value(value);
            // asm = format!("  li {}, {}\n", int_reg, int.value());
            // asm
            unreachable!()
        }
        ValueKind::Return(ret) => {
            let ret_value = ret.value().unwrap();
            // compile return value
            asm += &child_value_to_asm(ret_value, func_ctx);
            match func_ctx.reg_value_table.get_reg(ret_value) {
                Some(ret_reg) => {
                    // load return value from reg to a0
                    if ret_reg != "a0" {
                        asm += &riscv_mv("a0", ret_reg);
                        func_ctx.reg_value_table.free_reg(ret_reg);
                        func_ctx.reg_value_table.alloc_reg("a0", Some(ret_value));
                    }
                }
                _ => {
                    // load return value from stack to a0
                    asm += &riscv_lw("a0", "sp", func_ctx.value_addr[&ret_value], func_ctx);
                }
            }
            // epilogue
            asm += &riscv_bin_op_imm("add", "sp", "sp", func_ctx.stack_size as i32, func_ctx);
            func_ctx.reg_value_table.free_reg("a0");
            // return
            asm += "  ret\n";
            asm
        }
        ValueKind::Alloc(_) => {
            let size = 4;
            let offset = func_ctx.current_offset as i32;
            func_ctx.current_offset += size;
            func_ctx.value_addr.insert(value, offset);
            asm
        }
        ValueKind::Load(load) => {
            let load_value = load.src();
            let load_value_addr = func_ctx.value_addr[&load_value];
            let load_reg = func_ctx.reg_value_table.alloc_value(value);
            asm += &riscv_lw(load_reg, "sp", load_value_addr, func_ctx);
            asm
        }
        ValueKind::Binary(bin) => {
            let op = bin.op();
            let lhs_value = bin.lhs();
            let rhs_value = bin.rhs();
            let lhs_asm = child_value_to_asm(lhs_value, func_ctx);
            let rhs_asm = child_value_to_asm(rhs_value, func_ctx);
            asm += &lhs_asm;
            asm += &rhs_asm;
            let (lhs_load_asm, lhs_reg) = func_ctx.load_value_to_reg(lhs_value);
            let (rhs_load_asm, rhs_reg) = func_ctx.load_value_to_reg(rhs_value);
            asm += &lhs_load_asm;
            asm += &rhs_load_asm;
            let dest_reg = func_ctx.reg_value_table.alloc_value(value);
            match op {
                BinaryOp::Add => {
                    asm += &riscv_bin_op("add", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Sub => {
                    asm += &riscv_bin_op("sub", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Mul => {
                    asm += &riscv_bin_op("mul", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Div => {
                    asm += &riscv_bin_op("div", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Mod => {
                    asm += &riscv_bin_op("rem", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::And => {
                    // bitwise and
                    asm += &riscv_bin_op("and", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Or => {
                    // bitwise or
                    asm += &riscv_bin_op("or", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Eq => {
                    asm += &riscv_bin_op("xor", dest_reg, lhs_reg, rhs_reg);
                    asm += &riscv_unary_op("seqz", dest_reg, dest_reg);
                }
                BinaryOp::NotEq => {
                    asm += &riscv_bin_op("xor", dest_reg, lhs_reg, rhs_reg);
                    asm += &riscv_unary_op("snez", dest_reg, dest_reg);
                }
                BinaryOp::Gt => {
                    asm += &riscv_bin_op("slt", dest_reg, rhs_reg, lhs_reg);
                }
                BinaryOp::Lt => {
                    asm += &riscv_bin_op("slt", dest_reg, lhs_reg, rhs_reg);
                }
                BinaryOp::Ge => {
                    asm += &riscv_bin_op("slt", dest_reg, lhs_reg, rhs_reg);
                    asm += &riscv_bin_op_imm("xor", dest_reg, dest_reg, 1, func_ctx);
                }
                BinaryOp::Le => {
                    asm += &riscv_bin_op("slt", dest_reg, rhs_reg, lhs_reg);
                    asm += &riscv_bin_op_imm("xor", dest_reg, dest_reg, 1, func_ctx);
                }
                BinaryOp::Xor => {
                    asm += &riscv_bin_op("xor", dest_reg, lhs_reg, rhs_reg);
                }
                _ => todo!(),
            };
            func_ctx.reg_value_table.free_reg(lhs_reg);
            func_ctx.reg_value_table.free_reg(rhs_reg);
            asm
        }
        ValueKind::Store(store) => {
            let store_value = store.value();
            let store_dest = store.dest();
            let store_value_asm = child_value_to_asm(store_value, func_ctx);
            let (store_reg_asm, store_reg) = func_ctx.load_value_to_reg(store_value);
            let store_dest_addr = func_ctx.value_addr[&store_dest];
            asm += &store_value_asm;
            asm += &store_reg_asm;
            asm += &riscv_sw(store_reg, "sp", store_dest_addr, func_ctx);
            func_ctx.reg_value_table.free_reg(store_reg);
            asm
        }
        _ => {
            panic!("unsupported value kind: {:?}", value_data.kind());
        }
    }
}

pub fn child_value_to_asm(value: Value, func_ctx: &mut FuncContext) -> Asm {
    let func_data = func_ctx.func_data;
    let value_data = func_data.dfg().value(value);
    func_ctx.print_value(value);
    match value_data.kind() {
        ValueKind::Integer(int) => {
            let int_reg = func_ctx.reg_value_table.alloc_value(value);
            format!("  li {}, {}\n", int_reg, int.value())
        }
        ValueKind::Binary(_) | ValueKind::Load(_) => String::new(),
        _ => {
            unreachable!()
        }
    }
}
