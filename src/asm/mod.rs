mod build_asm;
use build_asm::GenerateAsm;
use koopa::ir::Program;
pub fn koopa_to_asm(koopa_program: &Program) -> String {
    koopa_program.to_asm()
}
