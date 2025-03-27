mod build_func;
mod build_prog;
mod build_value;
mod gen_riscv;
mod util;
use koopa::ir::Program;

/*
    t0 ~ t6, a0 ~ a7 available
*/

pub trait GenerateAsm {
    fn to_asm(&self) -> String;
}

pub type Asm = String;

pub type Reg = &'static str;

pub fn koopa_to_asm(koopa_program: &Program) -> String {
    koopa_program.to_asm()
}
