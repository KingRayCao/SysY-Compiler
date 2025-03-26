use super::util::*;
use super::GenerateAsm;
use koopa::ir::Program;

impl GenerateAsm for Program {
    fn to_asm(&self) -> String {
        let mut result = String::new();
        result += "  .text\n";
        result += "  .globl main\n";
        for &func in self.func_layout() {
            result = result + &self.func(func).to_asm();
        }
        return result;
    }
}
