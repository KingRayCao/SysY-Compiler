mod build_asm;
use build_asm::GenerateAsm;
pub fn koopa_to_asm(koopa_str: &str) -> String {
    let driver = koopa::front::Driver::from(koopa_str);
    let program = driver.generate_program().unwrap();
    program.to_asm()
}