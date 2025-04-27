mod build_func;
mod build_prog;
mod build_value;
mod gen_riscv;
mod util;
use koopa::ir::Program;

/*
    t0 ~ t6, a0 ~ a7 available
*/
static REG_LIST: [&str; 15] = [
    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
];

pub trait GenerateAsm {
    fn to_asm(&self, prog: &Program) -> String;
}

pub type Asm = String;

pub type Reg = &'static str;

type Addr = i32;

pub fn koopa_to_asm(koopa_program: &Program) -> String {
    build_prog::prog_to_asm(koopa_program)
}
