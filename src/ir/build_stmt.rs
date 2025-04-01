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
                context.symbol_tables.push_table();
                block.build_ir(program, context)?;
                context.symbol_tables.pop_table();
                Ok(())
            }
            Stmt::IfStmt(exp, then_stmt, else_stmt) => {
                let if_bb = new_bb(program, context, "%if");
                let if_bb = insert_bb(program, context, if_bb);
                change_current_bb(program, context, if_bb);
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
            Stmt::WhileStmt(exp, stmt) => {
                let while_bb = new_bb(program, context, "%while");
                let while_bb = insert_bb(program, context, while_bb);
                change_current_bb(program, context, while_bb);
                let exp_val = exp.build_ir(program, context)?;
                let loop_bb = new_bb(program, context, "%loop");
                let end_bb = new_bb(program, context, "%end");
                let while_br_value =
                    new_value_builder(program, context).branch(exp_val, loop_bb, end_bb);
                add_value(program, context, while_br_value)?;
                // build loop stmt
                context.while_stack.push(while_bb, end_bb);
                let loop_bb = insert_bb(program, context, loop_bb);
                change_current_bb(program, context, loop_bb);
                let loop_stmt_val = stmt.build_ir(program, context)?;
                let loop_jump = new_value_builder(program, context).jump(while_bb);
                add_value(program, context, loop_jump)?;
                context.while_stack.pop();
                // build end stmt
                let end_bb = insert_bb(program, context, end_bb);
                change_current_bb(program, context, end_bb);
                Ok(())
            }
            Stmt::BreakStmt => {
                let (while_bb, end_bb) = context.while_stack.get_top().unwrap();
                let break_jump = new_value_builder(program, context).jump(end_bb);
                add_value(program, context, break_jump)?;
                Ok(())
            }
            Stmt::ContinueStmt => {
                let (while_bb, end_bb) = context.while_stack.get_top().unwrap();
                let continue_jump = new_value_builder(program, context).jump(while_bb);
                add_value(program, context, continue_jump)?;
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
