use super::util::*;
use super::*;
use crate::ast::exp::*;
use koopa::ir::builder::{LocalInstBuilder, ValueBuilder};
use koopa::ir::{BinaryOp, Value};

impl IrGenerator for Exp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Exp::LOrExp(exp) => exp.build_ir(program, context),
        }
    }
}

impl IrGenerator for PrimaryExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            PrimaryExp::BracketExp(exp) => exp.build_ir(program, context),
            PrimaryExp::LVal(lval) => lval.build_ir(program, context),
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
            LAndExp::LAndExp(exp1, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let val_0 = new_value_builder(program, context).integer(0);
                let exp1_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, exp1_val, val_0);
                add_value(program, context, exp1_not_0_val)?;
                let exp2_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, exp2_val, val_0);
                add_value(program, context, exp2_not_0_val)?;
                let value = new_value_builder(program, context).binary(
                    BinaryOp::And,
                    exp1_not_0_val,
                    exp2_not_0_val,
                );
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for LOrExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            LOrExp::LAndExp(exp) => exp.build_ir(program, context),
            LOrExp::LOrExp(exp1, exp2) => {
                let exp1_val = exp1.build_ir(program, context)?;
                let exp2_val = exp2.build_ir(program, context)?;
                let val_0 = new_value_builder(program, context).integer(0);
                let exp1_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, exp1_val, val_0);
                add_value(program, context, exp1_not_0_val)?;
                let exp2_not_0_val =
                    new_value_builder(program, context).binary(BinaryOp::NotEq, exp2_val, val_0);
                add_value(program, context, exp2_not_0_val)?;
                let value = new_value_builder(program, context).binary(
                    BinaryOp::Or,
                    exp1_not_0_val,
                    exp2_not_0_val,
                );
                add_value(program, context, value)?;
                Ok(value)
            }
        }
    }
}

impl IrGenerator for LVal {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        if self.index.is_empty() {
            let entry = context.symbol_tables.get_symbol(&self.ident).unwrap();
            match entry {
                SymbolTableEntry::Var(_, value) => Ok(*value),
                SymbolTableEntry::Const(_, value) => {
                    let val = *value;
                    Ok(new_value_builder(program, context).integer(val))
                }
            }
        } else {
            todo!()
        }
    }
}
