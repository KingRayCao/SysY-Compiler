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
            Stmt::ExpStmt(exp) => {
                if let Some(exp) = exp.as_ref() {
                    let exp_val = exp.build_ir(program, context)?;
                }
                Ok(())
            }
            Stmt::BlockStmt(block) => {
                block.build_ir(program, context)?;
                Ok(())
            }
            Stmt::IfStmt(exp, then_stmt, else_stmt) => {
                let exp_val = exp.build_ir(program, context)?;
                match else_stmt {
                    Some(else_stmt) => {
                        // if else pair
                        let then_bb = new_bb(program, context, "%then");
                        let else_bb = new_bb(program, context, "%else");
                        let end_bb = new_bb(program, context, "%end");
                        let if_stmt_value =
                            new_value_builder(program, context).branch(exp_val, then_bb, else_bb);
                        add_value(program, context, if_stmt_value)?;
                        // build then stmt
                        let then_bb = insert_bb(program, context, then_bb);
                        change_current_bb(program, context, then_bb);
                        let then_stmt_val = then_stmt.build_ir(program, context)?;
                        let then_jump = new_value_builder(program, context).jump(end_bb);
                        add_value(program, context, then_jump)?;
                        // build else stmt
                        let else_bb = insert_bb(program, context, else_bb);
                        change_current_bb(program, context, else_bb);
                        let else_stmt_val = else_stmt.build_ir(program, context)?;
                        let else_jump = new_value_builder(program, context).jump(end_bb);
                        add_value(program, context, else_jump)?;
                        // build end stmt
                        let end_bb = insert_bb(program, context, end_bb);
                        change_current_bb(program, context, end_bb);
                    }
                    None => {
                        // single if
                        let then_bb = new_bb(program, context, "%then");
                        let end_bb = new_bb(program, context, "%end");
                        let if_stmt_value =
                            new_value_builder(program, context).branch(exp_val, then_bb, end_bb);
                        add_value(program, context, if_stmt_value)?;
                        // build then stmt
                        let then_bb = insert_bb(program, context, then_bb);
                        change_current_bb(program, context, then_bb);
                        let then_stmt_val = then_stmt.build_ir(program, context)?;
                        let then_jump = new_value_builder(program, context).jump(end_bb);
                        add_value(program, context, then_jump)?;
                        // build end stmt
                        let end_bb = insert_bb(program, context, end_bb);
                        change_current_bb(program, context, end_bb);
                    }
                }
                Ok(())
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
