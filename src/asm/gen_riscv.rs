use super::{build_func::FuncContext, Asm, Reg};

/*
    Notice:
    1. dest & src may be the same register
*/

pub fn riscv_bin_op_imm(
    op: &str,
    dest: Reg,
    src: Reg,
    imm_data: i32,
    func_ctx: &mut FuncContext,
) -> Asm {
    if imm_data >= -2048 && imm_data <= 2047 {
        return format!("  {}i {}, {}, {}\n", op, dest, src, imm_data);
    } else {
        let tmp_reg = func_ctx.reg_value_table.alloc_temp_reg();
        let mut ret = format!("  li {}, {}\n", tmp_reg, imm_data);
        ret += &format!("  {} {}, {}, {}\n", op, dest, src, tmp_reg);
        func_ctx.reg_value_table.free_reg(tmp_reg);
        return ret;
    }
}

pub fn riscv_bin_op(op: &str, dest: Reg, src0: Reg, src1: Reg) -> Asm {
    return format!("  {} {}, {}, {}\n", op, dest, src0, src1);
}

pub fn riscv_unary_op(op: &str, dest: Reg, src: Reg) -> Asm {
    return format!("  {} {}, {}\n", op, dest, src);
}

pub fn riscv_lw(dest: Reg, src: Reg, imm_offset: i32, func_ctx: &mut FuncContext) -> Asm {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        return format!("  lw {}, {}({})\n", dest, imm_offset, src);
    } else {
        let tmp_reg = func_ctx.reg_value_table.alloc_temp_reg();
        let mut ret = format!("  li {}, {}\n", tmp_reg, imm_offset);
        ret += &format!("  add {}, {}, {}\n", tmp_reg, tmp_reg, src);
        ret += &format!("  lw {}, 0({})\n", dest, tmp_reg);
        func_ctx.reg_value_table.free_reg(tmp_reg);
        return ret;
    }
}

pub fn riscv_sw(data: Reg, dest: Reg, imm_offset: i32, func_ctx: &mut FuncContext) -> Asm {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        return format!("  sw {}, {}({})\n", data, imm_offset, dest);
    } else {
        let tmp_reg = func_ctx.reg_value_table.alloc_temp_reg();
        let mut ret = format!("  li {}, {}\n", tmp_reg, imm_offset);
        ret += &format!("  add {}, {}, {}\n", tmp_reg, tmp_reg, dest);
        ret += &format!("  sw {}, 0({})\n", data, tmp_reg);
        func_ctx.reg_value_table.free_reg(tmp_reg);
        return ret;
    }
}

pub fn riscv_swi(imm_data: i32, imm_offset: i32, dest: Reg, func_ctx: &mut FuncContext) -> Asm {
    let tmp_reg = func_ctx.reg_value_table.alloc_temp_reg();
    let mut ret = format!("  li {}, {}\n", tmp_reg, imm_data);
    ret += &riscv_sw(tmp_reg, dest, imm_offset, func_ctx);
    func_ctx.reg_value_table.free_reg(tmp_reg);
    ret
}

pub fn riscv_mv(dest: Reg, src: Reg) -> Asm {
    return format!("  mv {}, {}\n", dest, src);
}
