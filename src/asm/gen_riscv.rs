use super::{build_func::FuncContext, util::ValueTable, Asm, Reg};

/*
    Notice:
    1. dest & src may be the same register
*/

pub fn riscv_bin_op_imm(
    op: &str,
    dest: Reg,
    src: Reg,
    imm_data: i32,
    asm: &mut Asm,
    value_table: &mut ValueTable,
) {
    if imm_data >= -2048 && imm_data <= 2047 {
        asm.push_str(&format!("  {}i {}, {}, {}\n", op, dest, src, imm_data));
    } else {
        let tmp_reg = value_table.assign_temp_to_reg(imm_data, asm);
        asm.push_str(&format!("  {} {}, {}, {}\n", op, dest, src, tmp_reg));
        value_table.unlock_reg(&tmp_reg);
    }
}

pub fn riscv_bin_op(
    op: &str,
    dest: Reg,
    src0: Reg,
    src1: Reg,
    asm: &mut Asm,
    value_table: &mut ValueTable,
) {
    asm.push_str(&format!("  {} {}, {}, {}\n", op, dest, src0, src1));
}

pub fn riscv_unary_op(op: &str, dest: Reg, src: Reg, asm: &mut Asm, value_table: &mut ValueTable) {
    asm.push_str(&format!("  {} {}, {}\n", op, dest, src));
}

pub fn riscv_lw(dest: Reg, src: Reg, imm_offset: i32, asm: &mut Asm, value_table: &mut ValueTable) {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        asm.push_str(&format!("  lw {}, {}({})\n", dest, imm_offset, src));
    } else {
        let tmp_reg = value_table.assign_temp_to_reg(imm_offset, asm);
        asm.push_str(&format!("  add {}, {}, {}\n", tmp_reg, tmp_reg, src));
        asm.push_str(&format!("  lw {}, 0({})\n", dest, tmp_reg));
        value_table.unlock_reg(&tmp_reg);
    }
}

pub fn riscv_sw(
    data: Reg,
    dest: Reg,
    imm_offset: i32,
    asm: &mut Asm,
    value_table: &mut ValueTable,
) {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        asm.push_str(&format!("  sw {}, {}({})\n", data, imm_offset, dest));
    } else {
        let tmp_reg = value_table.assign_temp_to_reg(imm_offset, asm);
        asm.push_str(&format!("  add {}, {}, {}\n", tmp_reg, tmp_reg, dest));
        asm.push_str(&format!("  sw {}, 0({})\n", data, tmp_reg));
        value_table.unlock_reg(&tmp_reg);
    }
}

pub fn riscv_swi(
    imm_data: i32,
    imm_offset: i32,
    dest: Reg,
    asm: &mut Asm,
    value_table: &mut ValueTable,
) {
    let tmp_reg = value_table.assign_temp_to_reg(imm_data, asm);
    riscv_sw(tmp_reg, dest, imm_offset, asm, value_table);
    value_table.unlock_reg(&tmp_reg);
}

pub fn riscv_mv(dest: Reg, src: Reg, asm: &mut Asm) {
    asm.push_str(&format!("  mv {}, {}\n", dest, src));
}
