use super::gen_riscv::*;
use super::util::*;
use super::{Asm, Reg};
use crate::asm::build_func::FuncContext;
use koopa::ir::{BinaryOp, Value, ValueKind};

pub fn value_to_asm(value: Value, asm: &mut Asm, func_ctx: &mut FuncContext) {
    let func_data = func_ctx.func_data;
    let value_data = func_data.dfg().value(value);
    // func_ctx.print_value(value);
    match value_data.kind() {
        ValueKind::Integer(int) => {
            unreachable!()
        }
        ValueKind::Return(ret) => {
            let ret_value = ret.value();
            // compile return value
            if let Some(ret_value) = ret_value {
                let ret_value_data = get_value_data(func_data, ret_value);
                let ret_value_reg = func_ctx.value_table.load_value_to_specified_reg(
                    &ret_value,
                    ret_value_data,
                    &"a0",
                    asm,
                );
                // epilogue
                todo!();
                riscv_bin_op_imm(
                    "add",
                    "sp",
                    "sp",
                    func_ctx.stack_size as i32,
                    asm,
                    &mut func_ctx.value_table,
                );
                // return
                asm.push_str("  ret\n");
            }
        }
        ValueKind::Alloc(alloc) => {
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset, true);
            func_ctx.current_offset += size;
        }
        ValueKind::Load(load) => {
            // let load_value = load.src();
            // let load_value_addr = func_ctx.value_addr[&load_value];
            // let load_reg = func_ctx.reg_value_table.alloc_value(value);
            // asm += &riscv_lw(load_reg, "sp", load_value_addr, func_ctx);
            // if value_data.used_by().is_empty() {
            //     func_ctx.reg_value_table.free_reg(load_reg);
            // }
            // asm
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset, false);
            func_ctx.current_offset += size;

            let src_value = load.src();
            let src_value_data = get_value_data(func_data, src_value);
            let load_reg = func_ctx.value_table.assign_value_to_reg(&value, asm);
            if func_ctx.value_table.is_alloc(&src_value) {
                // indicate this value is in alloc list; we can determine the offset directly
                let src_reg = func_ctx.value_table.load_value_to_reg(&src_value, src_value_data, asm);
                asm.push_str()
                func_ctx.value_table.unlock_reg(&load_reg);

            } else {
                panic!("value is not in stack");
            }
        }
        ValueKind::Binary(bin) => {
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;

            let op = bin.op();
            let lhs_value = bin.lhs();
            let lhs_value_data = get_value_data(func_data, lhs_value);
            let rhs_value = bin.rhs();
            let rhs_value_data = get_value_data(func_data, rhs_value);
            let lhs_reg = func_ctx.value_table.load_value_to_reg(&lhs_value,lhs_value_data, asm);
            let rhs_reg = func_ctx.value_table.load_value_to_reg(&rhs_value,rhs_value_data, asm);
            let dest_reg = func_ctx.value_table.assign_value_to_reg(&value, asm);
            match op {
                BinaryOp::Add => {
                    riscv_bin_op(
                        "add",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Sub => {
                    riscv_bin_op(
                        "sub",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Mul => {
                    riscv_bin_op(
                        "mul",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Div => {
                    riscv_bin_op(
                        "div",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Mod => {
                    riscv_bin_op(
                        "rem",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::And => {
                    // bitwise and
                    riscv_bin_op(
                        "and",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Or => {
                    // bitwise or
                    riscv_bin_op(
                        "or",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Eq => {
                    riscv_bin_op(
                        "xor",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    riscv_unary_op("seqz", dest_reg, dest_reg, asm, &mut func_ctx.value_table);
                }
                BinaryOp::NotEq => {
                    riscv_bin_op(
                        "xor",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    riscv_unary_op("snez", dest_reg, dest_reg, asm, &mut func_ctx.value_table);
                }
                BinaryOp::Gt => {
                    riscv_bin_op(
                        "slt",
                        dest_reg,
                        rhs_reg,
                        lhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Lt => {
                    riscv_bin_op(
                        "slt",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                BinaryOp::Ge => {
                    riscv_bin_op(
                        "slt",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    riscv_bin_op_imm("xor", dest_reg, dest_reg, 1, asm, &mut func_ctx.value_table);
                }
                BinaryOp::Le => {
                    riscv_bin_op(
                        "slt",
                        dest_reg,
                        rhs_reg,
                        lhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    riscv_bin_op_imm("xor", dest_reg, dest_reg, 1, asm, &mut func_ctx.value_table);
                }
                BinaryOp::Xor => {
                    riscv_bin_op(
                        "xor",
                        dest_reg,
                        lhs_reg,
                        rhs_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                }
                _ => todo!(),
            };
            func_ctx.value_table.unlock_reg(&lhs_reg);
            func_ctx.value_table.unlock_reg(&rhs_reg);
            func_ctx.value_table.unlock_reg(&dest_reg);
        }
        ValueKind::Store(store) => {
            let store_value = store.value();
            let store_dest = store.dest();
            let store_value_reg = func_ctx.value_table.load_value_to_reg(&value, asm);
            let store_dest_addr = func_ctx.value_addr[&store_dest];
            asm += &store_value_asm;
            asm += &store_reg_asm;
            asm += &riscv_sw(store_reg, "sp", store_dest_addr, func_ctx);
            func_ctx.reg_value_table.free_reg(store_reg);
            asm
        }
        ValueKind::Jump(jump) => {
            let jump_bb = jump.target();
            let jump_bb_name = get_bb_name(func_ctx.func_data, jump_bb);
            asm += &format!("  j {}\n", jump_bb_name);
            asm
        }
        ValueKind::Branch(branch) => {
            let cond_value = branch.cond();
            let cond_asm = child_value_to_asm(cond_value, func_ctx);
            asm += &cond_asm;
            let (cond_reg_asm, cond_reg) = func_ctx.load_value_to_reg(cond_value);
            asm += &cond_reg_asm;
            let true_bb = branch.true_bb();
            let false_bb = branch.false_bb();
            let true_bb_name = get_bb_name(func_ctx.func_data, true_bb);
            let false_bb_name = get_bb_name(func_ctx.func_data, false_bb);
            asm += &format!("  bnez {}, {}\n", cond_reg, true_bb_name);
            asm += &format!("  j {}\n", false_bb_name);
            func_ctx.reg_value_table.free_reg(cond_reg);
            asm
        }
        _ => {
            panic!("unsupported value kind: {:?}", value_data.kind());
        }
    }
    assert!(func_ctx.value_table.reg_all_unlocked());

}

// pub fn child_value_to_asm(value: Value, func_ctx: &mut FuncContext) -> Asm {
//     let func_data = func_ctx.func_data;
//     let value_data = func_data.dfg().value(value);
//     // func_ctx.print_value(value);
//     match value_data.kind() {
//         ValueKind::Integer(int) => {
//             let int_reg = func_ctx.reg_value_table.alloc_value(value);
//             format!("  li {}, {}\n", int_reg, int.value())
//         }
//         ValueKind::Binary(_) | ValueKind::Load(_) => String::new(),
//         _ => {
//             unreachable!()
//         }
//     }
// }
