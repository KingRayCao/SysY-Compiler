mod build_decl;
mod build_expr;
mod build_stmt;
mod const_eval;
mod util;
use crate::ast::*;
use koopa::ir::{BasicBlock, Function, FunctionData, Program, TypeKind, Value};
use util::*;

pub trait IrGenerator {
    type Output;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output;
}

impl IrGenerator for CompUnit {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        init_lib_decl(program, context);
        for item in &self.items {
            item.build_ir(program, context).unwrap();
        }
        Ok(())
    }
}

impl IrGenerator for CompItem {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            CompItem::Decl(decl) => {
                context.is_global = true;
                decl.build_ir(program, context)
            }
            CompItem::FuncDef(func_def) => {
                context.is_global = false;
                func_def.build_ir(program, context)
            }
        }
    }
}
pub fn compile(ast: &CompUnit) -> koopa::ir::Program {
    let mut program = Program::new();
    let mut context = IrContext::new();
    ast.build_ir(&mut program, &mut context).unwrap();
    program
}
