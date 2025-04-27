use super::util::*;
use super::GenerateAsm;
use koopa::ir::Program;

pub fn prog_to_asm(prog: &Program) -> String {
    let mut result = String::new();
    result += "  .text\n";
    result += "  .globl main\n";
    for &func in prog.func_layout() {
        result = result + &prog.func(func).to_asm(prog);
    }
    return result;
}
