use super::{IrContext, SymbolTableEntry};
use crate::ast::exp::*;

pub trait ConstEval {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String>;
}

impl ConstEval for Exp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            Exp::LOrExp(e) => e.get_const_value(context),
        }
    }
}

impl ConstEval for PrimaryExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            PrimaryExp::Number(n) => Ok(*n),
            PrimaryExp::BracketExp(e) => e.get_const_value(context),
            PrimaryExp::LVal(lval) => lval.get_const_value(context),
        }
    }
}

impl ConstEval for UnaryExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            UnaryExp::PrimaryExp(p) => p.get_const_value(context),
            UnaryExp::UnaryExp(op, e) => {
                let val = e.get_const_value(context).unwrap();
                match op {
                    UnaryOp::Plus => Ok(val),
                    UnaryOp::Minus => Ok(-val),
                    UnaryOp::Not => Ok(if val != 0 { 0 } else { 1 }),
                }
            }
            _ => unreachable!(),
        }
    }
}

impl ConstEval for MulExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            MulExp::UnaryExp(e) => e.get_const_value(context),
            MulExp::MulExp(e, op, u) => {
                let val = e.get_const_value(context).unwrap();
                let uval = u.get_const_value(context).unwrap();
                match op {
                    MulOp::Mul => Ok(val * uval),
                    MulOp::Div => Ok(val / uval),
                    MulOp::Mod => Ok(val % uval),
                }
            }
        }
    }
}

impl ConstEval for AddExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            AddExp::MulExp(e) => e.get_const_value(context),
            AddExp::AddExp(e, op, m) => {
                let val = e.get_const_value(context).unwrap();
                let mval = m.get_const_value(context).unwrap();
                match op {
                    AddOp::Add => Ok(val + mval),
                    AddOp::Sub => Ok(val - mval),
                }
            }
        }
    }
}

impl ConstEval for RelExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            RelExp::AddExp(e) => e.get_const_value(context),
            RelExp::RelExp(e, op, a) => {
                let val = e.get_const_value(context).unwrap();
                let aval = a.get_const_value(context).unwrap();
                match op {
                    RelOp::Lt => Ok(if val < aval { 1 } else { 0 }),
                    RelOp::Le => Ok(if val <= aval { 1 } else { 0 }),
                    RelOp::Gt => Ok(if val > aval { 1 } else { 0 }),
                    RelOp::Ge => Ok(if val >= aval { 1 } else { 0 }),
                }
            }
        }
    }
}

impl ConstEval for EqExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            EqExp::RelExp(e) => e.get_const_value(context),
            EqExp::EqExp(e, op, r) => {
                let val = e.get_const_value(context).unwrap();
                let rval = r.get_const_value(context).unwrap();
                match op {
                    EqOp::Eq => Ok(if val == rval { 1 } else { 0 }),
                    EqOp::Ne => Ok(if val != rval { 1 } else { 0 }),
                }
            }
        }
    }
}

impl ConstEval for LAndExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            LAndExp::EqExp(e) => e.get_const_value(context),
            LAndExp::LAndExp(e, eq) => {
                let val = e.get_const_value(context).unwrap();
                let eqval = eq.get_const_value(context).unwrap();
                if val != 0 && eqval != 0 {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

impl ConstEval for LOrExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            LOrExp::LAndExp(e) => e.get_const_value(context),
            LOrExp::LOrExp(e, land) => {
                let val = e.get_const_value(context).unwrap();
                let landval = land.get_const_value(context).unwrap();
                if val != 0 || landval != 0 {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

impl ConstEval for LVal {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        let (symbol, _) = context.symbol_tables.get_symbol(&self.ident);
        if let Some(symbol) = symbol {
            match symbol {
                SymbolTableEntry::Const(_, value) => Ok(*value),
                _ => Err(format!(
                    "{} cannot be evaluated during compilation",
                    self.ident
                )),
            }
        } else {
            Err(format!("{} is not defined", self.ident))
        }
    }
}

impl ConstEval for ConstExp {
    fn get_const_value(&self, context: &IrContext) -> Result<i32, String> {
        self.exp.get_const_value(context)
    }
}
