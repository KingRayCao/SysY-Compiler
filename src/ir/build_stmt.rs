use super::util::*;
use super::*;
use crate::ast::stmt::*;
use crate::ir::build_expr::LValValue;
use koopa::ir::builder::LocalInstBuilder;

impl IrGenerator for Stmt {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Stmt::AssignStmt(lval, exp) => {
                let lval_val = lval.build_ir(program, context)?;
                match lval_val {
                    LValValue::Var(value) => {
                        let exp_val = exp.build_ir(program, context)?;
                        let store = new_value_builder(program, context).store(exp_val, value);
                        add_value(program, context, store)?;
                        Ok(())
                    }
                    LValValue::Const(_) => Err("Assign to constant".to_string()),
                }
            }
            Stmt::ReturnStmt(exp) => {
                let ret_val = exp.build_ir(program, context)?;
                let ret = new_value_builder(program, context).ret(Some(ret_val));
                add_value(program, context, ret)?;
                Ok(())
            }
        }
    }
}
