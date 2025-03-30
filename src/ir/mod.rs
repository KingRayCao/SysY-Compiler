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

impl IrGenerator for CompUnit {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        self.func_def.build_ir(program, context).unwrap();
        Ok(())
    }
}

pub fn compile(ast: &CompUnit) -> koopa::ir::Program {
    let mut program = Program::new();
    let mut context = IrContext::new();
    ast.build_ir(&mut program, &mut context).unwrap();
    program
}
