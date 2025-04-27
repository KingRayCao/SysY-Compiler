use super::util::*;
use super::GenerateAsm;
use koopa::ir::{Program, ValueKind};

pub fn prog_to_asm(prog: &Program) -> String {
    let mut result = String::new();

    result += "  .data\n";

    for &globl_var in prog.inst_layout() {
        let globl_var_data = prog.borrow_value(globl_var);
        let globl_name = &globl_var_data.name().as_ref().unwrap()[1..];
        result += &format!("  .global {}\n{}:\n", globl_name, globl_name);
        match globl_var_data.kind() {
            ValueKind::GlobalAlloc(globl_alloc) => {
                let init = globl_alloc.init();
                let init_data = prog.borrow_value(init);
                match init_data.kind() {
                    ValueKind::Integer(num) => {
                        result += &format!("  .word {}\n", num.value());
                    }
                    _ => todo!(),
                }
            }
            _ => unreachable!(),
        }
    }

    result += "\n  .text\n";
    result += "  .globl main\n";
    for &func in prog.func_layout() {
        result = result + &prog.func(func).to_asm(prog);
    }
    return result;
}
