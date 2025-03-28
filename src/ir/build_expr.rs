use super::util::*;
use super::*;
use crate::ast::exp::*;
use koopa::ir::builder::{LocalInstBuilder, ValueBuilder};
use koopa::ir::{BinaryOp, Type, Value};

impl IrGenerator for Exp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Exp::LOrExp(exp) => exp.build_ir(program, context),
        }
    }
}

pub enum LValValue {
    Var(Value),
    Const(Value),
}

impl IrGenerator for LVal {
    type Output = Result<LValValue, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        if self.index.is_empty() {
            let (entry, _) = context.symbol_tables.get_symbol(&self.ident);
            let entry = entry.unwrap();
            match entry {
                SymbolTableEntry::Var(_, value) => Ok(LValValue::Var(*value)),
                SymbolTableEntry::Const(_, value) => {
                    let val = *value;
                    let const_val = new_value_builder(program, context).integer(val);
                    Ok(LValValue::Const(const_val))
                }
            }
        } else {
            todo!()
        }
    }
}

impl IrGenerator for PrimaryExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            PrimaryExp::BracketExp(exp) => exp.build_ir(program, context),
            PrimaryExp::LVal(lval) => {
                let lval_val = lval.build_ir(program, context)?;
                match lval_val {
                    LValValue::Var(value) => {
                        let load = new_value_builder(program, context).load(value);
                        add_value(program, context, load)?;
                        Ok(load)
                    }
                    LValValue::Const(value) => Ok(value),
                }
            }
            PrimaryExp::Number(num) => Ok(new_value_builder(program, context).integer(*num)),
        }
    }
}

impl IrGenerator for UnaryExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            UnaryExp::UnaryExp(op, exp) => {
                let exp_val = exp.build_ir(program, context).unwrap();
                let value = match op {
                    UnaryOp::Plus => exp_val,
                    UnaryOp::Minus => {
                        let value_0 = new_value_builder(program, context).integer(0);
                        let neg_val = new_value_builder(program, context).binary(
                            BinaryOp::Sub,
                            value_0,
                            exp_val,
                        );
                        add_value(program, context, neg_val)?;
                        neg_val
                    }
                    UnaryOp::Not => {
                        let value_0 = new_value_builder(program, context).integer(0);
                        let not_val = new_value_builder(program, context).binary(
                            BinaryOp::Eq,
                            value_0,
                            exp_val,
                        );
                        add_value(program, context, not_val)?;
                        not_val
                    }
                };
                Ok(value)
            }
            UnaryExp::PrimaryExp(exp) => exp.build_ir(program, context),
        }
    }
}

impl IrGenerator for MulExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            MulExp::UnaryExp(exp) => exp.build_ir(program, context),
            MulExp::MulExp(exp1, op, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let value = match op {
                    MulOp::Mul => new_value_builder(program, context).binary(
                        BinaryOp::Mul,
                        exp1_val,
                        exp2_val,
                    ),
                    MulOp::Div => new_value_builder(program, context).binary(
                        BinaryOp::Div,
                        exp1_val,
                        exp2_val,
                    ),
                    MulOp::Mod => new_value_builder(program, context).binary(
                        BinaryOp::Mod,
                        exp1_val,
                        exp2_val,
                    ),
                };
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for AddExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            AddExp::MulExp(exp) => exp.build_ir(program, context),
            AddExp::AddExp(exp1, op, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let value = match op {
                    AddOp::Add => new_value_builder(program, context).binary(
                        BinaryOp::Add,
                        exp1_val,
                        exp2_val,
                    ),
                    AddOp::Sub => new_value_builder(program, context).binary(
                        BinaryOp::Sub,
                        exp1_val,
                        exp2_val,
                    ),
                };
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for RelExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            RelExp::AddExp(exp) => exp.build_ir(program, context),
            RelExp::RelExp(exp1, op, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let value = match op {
                    RelOp::Lt => {
                        new_value_builder(program, context).binary(BinaryOp::Lt, exp1_val, exp2_val)
                    }
                    RelOp::Le => {
                        new_value_builder(program, context).binary(BinaryOp::Le, exp1_val, exp2_val)
                    }
                    RelOp::Gt => {
                        new_value_builder(program, context).binary(BinaryOp::Gt, exp1_val, exp2_val)
                    }
                    RelOp::Ge => {
                        new_value_builder(program, context).binary(BinaryOp::Ge, exp1_val, exp2_val)
                    }
                };
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for EqExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            EqExp::RelExp(exp) => exp.build_ir(program, context),
            EqExp::EqExp(exp1, op, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let value = match op {
                    EqOp::Eq => {
                        new_value_builder(program, context).binary(BinaryOp::Eq, exp1_val, exp2_val)
                    }
                    EqOp::Ne => new_value_builder(program, context).binary(
                        BinaryOp::NotEq,
                        exp1_val,
                        exp2_val,
                    ),
                };
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for LAndExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            LAndExp::EqExp(exp) => exp.build_ir(program, context),
            LAndExp::LAndExp(lhs, rhs) => {
                /*
                   int result = 0;
                   if (exp1 != 0) {
                       result = exp2 != 0;
                   }
                */
                let res_val = new_value_builder(program, context).alloc(Type::get_i32());
                add_value(program, context, res_val)?;
                let val_0 = new_value_builder(program, context).integer(0);
                let res_store_0 = new_value_builder(program, context).store(val_0, res_val);
                add_value(program, context, res_store_0)?;
                let lhs_val = lhs.build_ir(program, context)?;
                let lhs_true_bb = new_bb(program, context, "%lhs_true");
                let lhs_false_bb = new_bb(program, context, "%lhs_false");
                let and_stmt_val =
                    new_value_builder(program, context).branch(lhs_val, lhs_true_bb, lhs_false_bb);
                add_value(program, context, and_stmt_val)?;
                // build lhs_true_bb
                let lhs_true_bb = insert_bb(program, context, lhs_true_bb);
                change_current_bb(program, context, lhs_true_bb);
                let rhs_val = rhs.build_ir(program, context)?;
                let rhs_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, rhs_val, val_0);
                add_value(program, context, rhs_not_0_val)?;
                let res_store_rhs =
                    new_value_builder(program, context).store(rhs_not_0_val, res_val);
                add_value(program, context, res_store_rhs)?;
                let lhs_true_jump = new_value_builder(program, context).jump(lhs_false_bb);
                add_value(program, context, lhs_true_jump)?;
                // build lhs_false_bb
                let lhs_false_bb = insert_bb(program, context, lhs_false_bb);
                change_current_bb(program, context, lhs_false_bb);
                let load_val = new_value_builder(program, context).load(res_val);
                add_value(program, context, load_val)?;
                Ok(load_val)
            }
        }
    }
}

impl IrGenerator for LOrExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            LOrExp::LAndExp(exp) => exp.build_ir(program, context),
            LOrExp::LOrExp(lhs, rhs) => {
                /*
                   int result = 1;
                   if (lhs == 0) {
                       result = rhs != 0;
                   }
                */
                let res_val = new_value_builder(program, context).alloc(Type::get_i32());
                add_value(program, context, res_val)?;
                let val_1 = new_value_builder(program, context).integer(1);
                let val_0 = new_value_builder(program, context).integer(0);
                let res_store_1 = new_value_builder(program, context).store(val_1, res_val);
                add_value(program, context, res_store_1)?;
                let lhs_val = lhs.build_ir(program, context)?;
                let lhs_false_bb = new_bb(program, context, "%lhs_false");
                let lhs_true_bb = new_bb(program, context, "%lhs_true");
                let or_stmt_val =
                    new_value_builder(program, context).branch(lhs_val, lhs_true_bb, lhs_false_bb);
                add_value(program, context, or_stmt_val)?;
                // build lhs_false_bb
                let lhs_false_bb = insert_bb(program, context, lhs_false_bb);
                change_current_bb(program, context, lhs_false_bb);
                let rhs_val = rhs.build_ir(program, context)?;
                let rhs_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, rhs_val, val_0);
                add_value(program, context, rhs_not_0_val)?;
                let res_store_rhs =
                    new_value_builder(program, context).store(rhs_not_0_val, res_val);
                add_value(program, context, res_store_rhs)?;
                let lhs_false_jump = new_value_builder(program, context).jump(lhs_true_bb);
                add_value(program, context, lhs_false_jump)?;
                // build lhs_true_bb
                let lhs_true_bb = insert_bb(program, context, lhs_true_bb);
                change_current_bb(program, context, lhs_true_bb);
                let load_val = new_value_builder(program, context).load(res_val);
                add_value(program, context, load_val)?;
                Ok(load_val)
            }
        }
    }
}
