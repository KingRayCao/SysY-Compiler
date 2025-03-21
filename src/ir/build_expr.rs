use super::util::*;
use super::*;
use crate::ast::exp::*;
use koopa::ir::builder::{LocalInstBuilder, ValueBuilder};
use koopa::ir::{BinaryOp, Value};

impl IrGenerator for Exp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Exp::UnaryExp(exp) => exp.build_ir(program, context),
        }
    }
}

impl IrGenerator for PrimaryExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            PrimaryExp::Exp(exp) => exp.build_ir(program, context),
            PrimaryExp::Number(num) => Ok(new_value(program, context).integer(*num)),
        }
    }
}

impl IrGenerator for UnaryExp {
    type Output = Result<Value, String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            UnaryExp::UnaryExp(op, exp) => {
                let exp_val = exp.build_ir(program, context)?;
                let value = match op {
                    UnaryOp::Plus => exp_val,
                    UnaryOp::Minus => {
                        let value_0 = new_value(program, context).integer(0);
                        new_value(program, context).binary(BinaryOp::Sub, value_0, exp_val)
                    }
                    UnaryOp::Not => {
                        let value_0 = new_value(program, context).integer(0);
                        new_value(program, context).binary(BinaryOp::Eq, value_0, exp_val)
                    }
                };
                add_value(program, context, value)?;
                Ok(value)
            }
            UnaryExp::PrimaryExp(exp) => exp.build_ir(program, context),
        }
    }
}
