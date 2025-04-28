use super::{Asm, Reg};

/*
    Notice:
    1. dest & src may be the same register
*/

pub fn riscv_bin_op_imm(op: &str, dest: Reg, src: Reg, imm_data: i32, asm: &mut Asm) {
    if imm_data >= -2048 && imm_data <= 2047 {
        asm.push_str(&format!("  {}i {}, {}, {}\n", op, dest, src, imm_data));
    } else {
        asm.push_str(&format!("  li t6, {}\n", imm_data));
        asm.push_str(&format!("  {} {}, {}, t6\n", op, dest, src));
    }
}

pub fn riscv_bin_op(op: &str, dest: Reg, src0: Reg, src1: Reg, asm: &mut Asm) {
    asm.push_str(&format!("  {} {}, {}, {}\n", op, dest, src0, src1));
}

pub fn riscv_unary_op(op: &str, dest: Reg, src: Reg, asm: &mut Asm) {
    asm.push_str(&format!("  {} {}, {}\n", op, dest, src));
}

pub fn riscv_lw(dest: Reg, src: Reg, imm_offset: i32, asm: &mut Asm) {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        asm.push_str(&format!("  lw {}, {}({})\n", dest, imm_offset, src));
    } else {
        asm.push_str(&format!("  li t6, {}\n", imm_offset));
        asm.push_str(&format!("  add t6, t6, {}\n", src));
        asm.push_str(&format!("  lw {}, 0(t6)\n", dest));
    }
}

pub fn riscv_sw(data: Reg, dest: Reg, imm_offset: i32, asm: &mut Asm) {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        asm.push_str(&format!("  sw {}, {}({})\n", data, imm_offset, dest));
    } else {
        asm.push_str(&format!("  li t6, {}\n", imm_offset));
        asm.push_str(&format!("  add t6, t6, {}\n", dest));
        asm.push_str(&format!("  sw {}, 0(t6)\n", data));
    }
}

pub fn riscv_mv(dest: Reg, src: Reg, asm: &mut Asm) {
    asm.push_str(&format!("  mv {}, {}\n", dest, src));
}

pub fn riscv_la(dest: Reg, label: &str, asm: &mut Asm) {
    asm.push_str(&format!("  la {}, {}\n", dest, label));
}
