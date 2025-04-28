use super::{IrContext, SymbolTableEntry};
use crate::ast::{decl::ConstInitVal, decl::InitVal, exp::*};

pub trait ConstI32Eval {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String>;
}

impl ConstI32Eval for ConstInitVal {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            ConstInitVal::ConstExp(e) => e.get_const_i32(context),
            ConstInitVal::ConstArray(_) => unreachable!(),
        }
    }
}

impl ConstI32Eval for InitVal {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            InitVal::Exp(e) => e.get_const_i32(context),
            InitVal::Array(_) => unreachable!(),
        }
    }
}

impl ConstI32Eval for Exp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            Exp::LOrExp(e) => e.get_const_i32(context),
        }
    }
}

impl ConstI32Eval for PrimaryExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            PrimaryExp::Number(n) => Ok(*n),
            PrimaryExp::BracketExp(e) => e.get_const_i32(context),
            PrimaryExp::LVal(lval) => lval.get_const_i32(context),
        }
    }
}

impl ConstI32Eval for UnaryExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            UnaryExp::PrimaryExp(p) => p.get_const_i32(context),
            UnaryExp::UnaryExp(op, e) => {
                let val = e.get_const_i32(context).unwrap();
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

impl ConstI32Eval for MulExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            MulExp::UnaryExp(e) => e.get_const_i32(context),
            MulExp::MulExp(e, op, u) => {
                let val = e.get_const_i32(context).unwrap();
                let uval = u.get_const_i32(context).unwrap();
                match op {
                    MulOp::Mul => Ok(val * uval),
                    MulOp::Div => Ok(val / uval),
                    MulOp::Mod => Ok(val % uval),
                }
            }
        }
    }
}

impl ConstI32Eval for AddExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            AddExp::MulExp(e) => e.get_const_i32(context),
            AddExp::AddExp(e, op, m) => {
                let val = e.get_const_i32(context).unwrap();
                let mval = m.get_const_i32(context).unwrap();
                match op {
                    AddOp::Add => Ok(val + mval),
                    AddOp::Sub => Ok(val - mval),
                }
            }
        }
    }
}

impl ConstI32Eval for RelExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            RelExp::AddExp(e) => e.get_const_i32(context),
            RelExp::RelExp(e, op, a) => {
                let val = e.get_const_i32(context).unwrap();
                let aval = a.get_const_i32(context).unwrap();
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

impl ConstI32Eval for EqExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            EqExp::RelExp(e) => e.get_const_i32(context),
            EqExp::EqExp(e, op, r) => {
                let val = e.get_const_i32(context).unwrap();
                let rval = r.get_const_i32(context).unwrap();
                match op {
                    EqOp::Eq => Ok(if val == rval { 1 } else { 0 }),
                    EqOp::Ne => Ok(if val != rval { 1 } else { 0 }),
                }
            }
        }
    }
}

impl ConstI32Eval for LAndExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            LAndExp::EqExp(e) => e.get_const_i32(context),
            LAndExp::LAndExp(e, eq) => {
                let val = e.get_const_i32(context).unwrap();
                let eqval = eq.get_const_i32(context).unwrap();
                if val != 0 && eqval != 0 {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

impl ConstI32Eval for LOrExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        match self {
            LOrExp::LAndExp(e) => e.get_const_i32(context),
            LOrExp::LOrExp(e, land) => {
                let val = e.get_const_i32(context).unwrap();
                let landval = land.get_const_i32(context).unwrap();
                if val != 0 || landval != 0 {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
        }
    }
}

impl ConstI32Eval for LVal {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        let (symbol, _) = context.symbol_tables.get_symbol(&self.ident);
        if let Some(symbol) = symbol {
            match symbol {
                SymbolTableEntry::Const(_, value) => Ok(value),
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

impl ConstI32Eval for ConstExp {
    fn get_const_i32(&self, context: &IrContext) -> Result<i32, String> {
        self.exp.get_const_i32(context)
    }
}
