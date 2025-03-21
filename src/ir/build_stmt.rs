use super::util::*;
use super::*;
use crate::ast::stmt::*;
use koopa::ir::builder::LocalInstBuilder;

impl IrGenerator for Stmt {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Stmt::ReturnStmt(exp) => {
                let ret_val = exp.build_ir(program, context)?;
                let ret = new_value(program, context).ret(Some(ret_val));
                add_value(program, context, ret)?;
                Ok(())
            }
        }
    }
}
