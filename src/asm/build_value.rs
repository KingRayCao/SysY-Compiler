use core::panic;

use super::gen_riscv::*;
use super::util::*;
use super::{Asm, REG_LIST};
use crate::asm::build_func::get_alloc_size;
use crate::asm::build_func::FuncContext;
use koopa::ir::{BinaryOp, TypeKind, Value, ValueKind};

pub fn value_to_asm(value: Value, asm: &mut Asm, func_ctx: &mut FuncContext) {
    let func_data = func_ctx.func_data;
    let value_data = func_data.dfg().value(value);
    // func_ctx.print_value(value);
    match value_data.kind() {
        ValueKind::Integer(_) => {
            unreachable!()
        }
        ValueKind::Return(ret) => {
            // asm.push_str("  # return\n");
            let ret_value = ret.value();
            // compile return value
            if let Some(ret_value) = ret_value {
                let ret_value_data = get_value_data(func_data, ret_value);
                func_ctx.value_table.assign_value_to_specified_reg(
                    &ret_value,
                    ret_value_data,
                    &"a0",
                    asm,
                );
            }
            // epilogue
            // return
            if func_ctx.has_call {
                riscv_lw(
                    "ra",
                    "sp",
                    func_ctx.stack_size as i32 - 4,
                    asm,
                    &mut func_ctx.value_table,
                );
            }
            riscv_bin_op_imm(
                "add",
                "sp",
                "sp",
                func_ctx.stack_size as i32,
                asm,
                &mut func_ctx.value_table,
            );
            func_ctx.value_table.unlock_reg(&"a0");
            asm.push_str("  ret\n");
        }
        ValueKind::Alloc(alloc) => {
            let size = get_alloc_size(value_data);
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;
        }
        ValueKind::Load(load) => {
            // asm.push_str("  # load\n");
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;

            let src_value = load.src();
            let (src_value_data, src_is_global) =
                if func_ctx.func_data.dfg().values().contains_key(&src_value) {
                    (func_ctx.func_data.dfg().value(src_value).clone(), false)
                } else {
                    let value_data = func_ctx.program.borrow_value(src_value);
                    (value_data.clone(), true)
                };
            let load_reg = func_ctx.value_table.allocate_value_to_reg(&value, asm);

            if src_is_global {
                let globl_name = &src_value_data.name().as_ref().unwrap()[1..];
                let src_ptr_reg = func_ctx.value_table.allocate_value_to_reg(&src_value, asm);
                riscv_la(src_ptr_reg, globl_name, asm);
                riscv_lw(load_reg, src_ptr_reg, 0, asm, &mut func_ctx.value_table);
                func_ctx.value_table.unlock_reg(&src_ptr_reg);
            } else {
                if let ValueKind::Alloc(_) = src_value_data.kind() {
                    // indicate this value is in alloc list; we can determine the offset directly
                    let addr = func_ctx.value_table.get_value_addr(&src_value);
                    if let Some(offset) = addr {
                        riscv_lw(load_reg, "sp", offset, asm, &mut func_ctx.value_table);
                    } else {
                        panic!("value is not in stack");
                    }
                } else {
                    // indicate this value is a pointer; we need to load the value from the stack
                    let src_ptr_reg =
                        func_ctx
                            .value_table
                            .assign_value_to_reg(&src_value, &src_value_data, asm);
                    riscv_lw(load_reg, src_ptr_reg, 0, asm, &mut func_ctx.value_table);
                    func_ctx.value_table.unlock_reg(&src_ptr_reg);
                }
            }
            func_ctx.value_table.unlock_reg(&load_reg);
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
            let lhs_reg = func_ctx
                .value_table
                .assign_value_to_reg(&lhs_value, lhs_value_data, asm);
            let rhs_reg = func_ctx
                .value_table
                .assign_value_to_reg(&rhs_value, rhs_value_data, asm);
            let dest_reg = func_ctx.value_table.allocate_value_to_reg(&value, asm);
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
                _ => unreachable!(),
            };
            func_ctx.value_table.unlock_reg(&lhs_reg);
            func_ctx.value_table.unlock_reg(&rhs_reg);
            func_ctx.value_table.unlock_reg(&dest_reg);
        }
        ValueKind::Store(store) => {
            let store_value = store.value();
            let store_value_data = get_value_data(func_data, store_value);
            let store_value_reg =
                func_ctx
                    .value_table
                    .assign_value_to_reg(&store_value, store_value_data, asm);

            let store_dest = store.dest();
            let (store_dest_data, dest_is_global) = {
                let func_data = func_ctx.func_data;
                let program = func_ctx.program;
                if func_data.dfg().values().contains_key(&store_dest) {
                    (func_data.dfg().value(store_dest).clone(), false)
                } else {
                    (program.borrow_value(store_dest).clone(), true)
                }
            };

            if dest_is_global {
                let globl_name = &store_dest_data.name().as_ref().unwrap()[1..];
                let dest_ptr_reg = func_ctx.value_table.allocate_value_to_reg(&store_dest, asm);
                riscv_la(dest_ptr_reg, globl_name, asm);
                riscv_sw(
                    store_value_reg,
                    dest_ptr_reg,
                    0,
                    asm,
                    &mut func_ctx.value_table,
                );
                func_ctx.value_table.unlock_reg(&dest_ptr_reg);
            } else {
                if let ValueKind::Alloc(_) = store_dest_data.kind() {
                    // indicate this value is in alloc list; we can determine the offset directly
                    let addr = func_ctx.value_table.get_value_addr(&store_dest);
                    if let Some(offset) = addr {
                        riscv_sw(
                            store_value_reg,
                            "sp",
                            offset,
                            asm,
                            &mut func_ctx.value_table,
                        );
                        // println!("store {:?} to {}", store_value, offset);
                    } else {
                        panic!("value is not in stack");
                    }
                } else {
                    // indicate this value is a pointer; we need to load the value from the stack
                    let dest_ptr_reg = func_ctx.value_table.assign_value_to_reg(
                        &store_dest,
                        &store_dest_data,
                        asm,
                    );
                    riscv_sw(
                        store_value_reg,
                        dest_ptr_reg,
                        0,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    func_ctx.value_table.unlock_reg(&dest_ptr_reg);
                }
            }
            func_ctx.value_table.unlock_reg(&store_value_reg);
        }
        ValueKind::Jump(jump) => {
            let jump_bb = jump.target();
            let jump_bb_name = get_bb_name(func_ctx.func_data, jump_bb);
            func_ctx.value_table.free_regs(&REG_LIST.to_vec(), asm);
            asm.push_str(&format!("  j {}\n", jump_bb_name));
        }
        ValueKind::Branch(branch) => {
            let cond_value = branch.cond();
            let cond_value_data = get_value_data(func_data, cond_value);
            let cond_reg =
                func_ctx
                    .value_table
                    .assign_value_to_reg(&cond_value, cond_value_data, asm);
            let true_bb = branch.true_bb();
            let false_bb = branch.false_bb();
            let true_bb_name = get_bb_name(func_ctx.func_data, true_bb);
            let false_bb_name = get_bb_name(func_ctx.func_data, false_bb);
            func_ctx.value_table.unlock_reg(&cond_reg);
            func_ctx.value_table.free_regs(
                &REG_LIST
                    .iter()
                    .filter(|&&r| r != cond_reg)
                    .cloned()
                    .collect::<Vec<_>>(),
                asm,
            );
            func_ctx.value_table.free_reg(&cond_reg, asm);
            asm.push_str(&format!("  bnez {}, {}\n", cond_reg, true_bb_name));
            asm.push_str(&format!("  j {}\n", false_bb_name));
            func_ctx.value_table.unlock_reg(&cond_reg);
        }
        ValueKind::Call(call) => {
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;

            let callee = call.callee();
            let args = call.args();
            for (i, arg_value) in args.iter().enumerate() {
                let arg_value_data = get_value_data(func_data, *arg_value);
                if i < 8 {
                    func_ctx.value_table.assign_value_to_specified_reg(
                        arg_value,
                        arg_value_data,
                        &REG_LIST[i],
                        asm,
                    );
                } else {
                    let arg_reg =
                        func_ctx
                            .value_table
                            .assign_value_to_reg(arg_value, arg_value_data, asm);
                    riscv_sw(
                        arg_reg,
                        "sp",
                        (i - 8) as i32 * 4,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    func_ctx.value_table.unlock_reg(&arg_reg);
                }
            }
            let callee_data = func_ctx.program.func(callee);
            asm.push_str(&format!("  call {}\n", &callee_data.name()[1..]));
            // println!("call {}", &callee_data.name()[1..]);
            if let TypeKind::Function(_, ret_type) = callee_data.ty().kind() {
                if !ret_type.is_unit() {
                    // return value now in a0
                    func_ctx
                        .value_table
                        .set_value_to_reg(&value, value_data, &"a0");
                }
            }
            for i in 0..args.len() {
                if i < 8 {
                    let arg_reg = REG_LIST[i];
                    func_ctx.value_table.unlock_reg(&arg_reg);
                }
            }
            func_ctx.value_table.unlock_reg(&"a0");
        }

        ValueKind::GetElemPtr(get_elem_ptr) => {
            // asm.push_str("  # get_elem_ptr\n");
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;

            let index_value = get_elem_ptr.index();
            let index_value_data = get_value_data(func_data, index_value);

            let src_value = get_elem_ptr.src();
            let (src_value_data, src_is_global) =
                if func_ctx.func_data.dfg().values().contains_key(&src_value) {
                    (func_ctx.func_data.dfg().value(src_value).clone(), false)
                } else {
                    let value_data = func_ctx.program.borrow_value(src_value);
                    (value_data.clone(), true)
                };

            let index_reg =
                func_ctx
                    .value_table
                    .assign_value_to_reg(&index_value, index_value_data, asm);

            let dest_reg = func_ctx.value_table.allocate_value_to_reg(&value, asm);

            let step = get_elem_ptr_step(&src_value_data) as i32;

            if src_is_global {
                let globl_name = &src_value_data.name().as_ref().unwrap()[1..];
                let src_ptr_reg = func_ctx.value_table.allocate_value_to_reg(&src_value, asm);
                riscv_la(src_ptr_reg, globl_name, asm);
                asm.push_str(&format!("  li {}, {}\n", dest_reg, step));
                riscv_bin_op(
                    "mul",
                    dest_reg,
                    index_reg,
                    dest_reg,
                    asm,
                    &mut func_ctx.value_table,
                );
                riscv_bin_op(
                    "add",
                    dest_reg,
                    dest_reg,
                    src_ptr_reg,
                    asm,
                    &mut func_ctx.value_table,
                );
                func_ctx.value_table.unlock_reg(&src_ptr_reg);
            } else {
                if let ValueKind::Alloc(_) = src_value_data.kind() {
                    // indicate this value is in alloc list; we can determine the offset directly
                    let addr = func_ctx.value_table.get_value_addr(&src_value);
                    if let Some(src_value_offset) = addr {
                        asm.push_str(&format!("  li {}, {}\n", dest_reg, step));
                        riscv_bin_op(
                            "mul",
                            dest_reg,
                            index_reg,
                            dest_reg,
                            asm,
                            &mut func_ctx.value_table,
                        );
                        riscv_bin_op(
                            "add",
                            dest_reg,
                            dest_reg,
                            "sp",
                            asm,
                            &mut func_ctx.value_table,
                        );
                        riscv_bin_op_imm(
                            "add",
                            dest_reg,
                            dest_reg,
                            src_value_offset,
                            asm,
                            &mut func_ctx.value_table,
                        );
                    } else {
                        panic!("value is not in stack");
                    }
                } else {
                    // indicate this value is a pointer; we need to load the value from the stack
                    let src_ptr_reg =
                        func_ctx
                            .value_table
                            .assign_value_to_reg(&src_value, &src_value_data, asm);
                    asm.push_str(&format!("  li {}, {}\n", dest_reg, step));
                    riscv_bin_op(
                        "mul",
                        dest_reg,
                        index_reg,
                        dest_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    riscv_bin_op(
                        "add",
                        dest_reg,
                        dest_reg,
                        src_ptr_reg,
                        asm,
                        &mut func_ctx.value_table,
                    );
                    func_ctx.value_table.unlock_reg(&src_ptr_reg);
                }
            }

            func_ctx.value_table.unlock_reg(&index_reg);
            func_ctx.value_table.unlock_reg(&dest_reg);
        }
        ValueKind::GetPtr(get_ptr) => {
            let size = value_data.ty().size();
            let offset = func_ctx.current_offset as i32;
            func_ctx.value_table.alloc_value(value, offset);
            func_ctx.current_offset += size;

            let index_value = get_ptr.index();
            let index_value_data = get_value_data(func_data, index_value);

            let src_value = get_ptr.src(); // must be local
            let src_value_data = func_ctx.func_data.dfg().value(src_value);
            let index_reg =
                func_ctx
                    .value_table
                    .assign_value_to_reg(&index_value, index_value_data, asm);
            let dest_reg = func_ctx.value_table.allocate_value_to_reg(&value, asm);
            let step = get_ptr_step(&src_value_data) as i32;

            let src_reg = func_ctx
                .value_table
                .assign_value_to_reg(&src_value, src_value_data, asm);
            asm.push_str(&format!("  li {}, {}\n", dest_reg, step));
            riscv_bin_op(
                "mul",
                dest_reg,
                index_reg,
                dest_reg,
                asm,
                &mut func_ctx.value_table,
            );
            riscv_bin_op(
                "add",
                dest_reg,
                dest_reg,
                src_reg,
                asm,
                &mut func_ctx.value_table,
            );

            func_ctx.value_table.unlock_reg(&src_reg);
            func_ctx.value_table.unlock_reg(&index_reg);
            func_ctx.value_table.unlock_reg(&dest_reg);
        }
        _ => {
            panic!("unsupported value kind: {:?}", value_data.kind());
        }
    }
    assert!(func_ctx.value_table.reg_all_unlocked());
}
