use super::{Reg, ASM};

pub fn riscv_bin_op_imm(op: &str, dest: Reg, src: Reg, imm_data: i32) -> ASM {
    if imm_data >= -2048 && imm_data <= 2047 {
        return format!("  {}i {}, {}, {}\n", op, dest, src, imm_data);
    } else {
        let mut ret = format!("  li {}, {}\n", dest, imm_data);
        ret += &format!("  {} {}, {}, {}", op, dest, src, dest).as_str();
        return ret;
    }
}

pub fn riscv_bin_op(op: &str, dest: Reg, src0: Reg, src1: Reg) -> ASM {
    return format!("  {} {}, {}, {}\n", op, dest, src0, src1);
}

pub fn riscv_unary_op(op: &str, dest: Reg, src: Reg) -> ASM {
    return format!("  {} {}, {}\n", op, dest, src);
}

pub fn riscv_lw(dest: Reg, src: Reg, imm_offset: i32) -> ASM {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        return format!("  lw {}, {}({})\n", dest, imm_offset, src);
    } else {
        let mut ret = format!("  li {}, {}\n", dest, imm_offset);
        ret += &format!("  add {}, {}, {}\n", dest, dest, src);
        ret += &format!("  lw {}, 0({})\n", dest, dest);
        return ret;
    }
}

pub fn riscv_sw(data: Reg, dest: Reg, imm_offset: i32, tmp_reg: Reg) -> ASM {
    if imm_offset >= -2048 && imm_offset <= 2047 {
        return format!("  sw {}, {}({})\n", data, imm_offset, dest);
    } else {
        let mut ret = format!("  li {}, {}\n", tmp_reg, imm_offset);
        ret += &format!("  add {}, {}, {}\n", tmp_reg, tmp_reg, dest);
        ret += &format!("  sw {}, 0({})\n", data, tmp_reg);
        return ret;
    }
}

pub fn riscv_swi(imm_data: i32, imm_offset: i32, dest: Reg, tmp_reg0: Reg, tmp_reg1: Reg) -> ASM {
    let mut ret = String::new();
    ret = format!("  li {}, {}\n", tmp_reg0, imm_data);
    ret += &riscv_sw(tmp_reg0, dest, imm_offset, tmp_reg1);
    ret
}

pub fn riscv_mv(dest: Reg, src: Reg) -> ASM {
    return format!("  mv {}, {}\n", dest, src);
}
