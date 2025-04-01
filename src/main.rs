#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, non_snake_case)
)]

use crate::asm::*;
use crate::ir::*;
use koopa::ir::Type;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use std::io::Write;

pub mod asm;
pub mod ast;
pub mod ir;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
    // INIT
    Type::set_ptr_size(4);
    // 解析命令行参数
    let mut args = args();
    args.next();
    let mode = args.next().unwrap();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
    let koopa_program = compile(&ast);

    // 输出解析得到的 AST 到输出文件
    match mode.as_str() {
        "-koopa" => {
            use koopa::back::KoopaGenerator;
            let mut generator = KoopaGenerator::new(Vec::new());
            generator.generate_on(&koopa_program).unwrap();
            let koopa_str = String::from_utf8(generator.writer().to_vec()).unwrap();
            let mut output = std::fs::File::create(output)?;
            write!(output, "{}", koopa_str)?;
        }
        "-riscv" => {
            let mut output = std::fs::File::create(output)?;
            let asm_str = koopa_to_asm(&koopa_program);
            write!(output, "{}", asm_str)?;
        }
        _ => panic!("Unknown mode: {}", mode),
    }
    Ok(())
}
