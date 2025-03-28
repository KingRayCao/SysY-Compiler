mod build_decl;
mod build_expr;
mod build_stmt;
mod const_eval;
mod util;
use crate::ast::*;
use koopa::ir::{BasicBlock, Function, Program, TypeKind, Value};
use util::*;

pub trait IrGenerator {
    type Output;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output;
}
pub struct IrContext {
    current_func: Option<Function>,
    current_bb: Option<BasicBlock>,
    symbol_tables: SymbolTableStack,
    prev_bb_end: bool,
}
impl IrGenerator for CompUnit {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        self.func_def.build_ir(program, context).unwrap();
        Ok(())
    }
}

pub fn compile(ast: &CompUnit) -> koopa::ir::Program {
    let mut program = Program::new();
    let mut context = IrContext {
        current_func: None,
        current_bb: None,
        symbol_tables: SymbolTableStack::new(),
        prev_bb_end: false,
    };
    ast.build_ir(&mut program, &mut context).unwrap();
    program
}
