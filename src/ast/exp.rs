#[derive(Debug)]
pub enum Exp {
    LOrExp(Box<LOrExp>),
}

#[derive(Debug)]
pub struct LVal {
    pub ident: String,
    pub index: Vec<Exp>,
}

#[derive(Debug)]
pub enum PrimaryExp {
    BracketExp(Box<Exp>),
    LVal(Box<LVal>),
    Number(i32),
}

#[derive(Debug)]
pub enum UnaryExp {
    UnaryExp(UnaryOp, Box<UnaryExp>),
    FuncCallExp(String, Vec<Exp>),
    PrimaryExp(Box<PrimaryExp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(Box<UnaryExp>),
    MulExp(Box<MulExp>, MulOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

impl ToString for MulOp {
    fn to_string(&self) -> String {
        match self {
            MulOp::Mul => "*".to_string(),
            MulOp::Div => "/".to_string(),
            MulOp::Mod => "%".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AddExp {
    MulExp(Box<MulExp>),
    AddExp(Box<AddExp>, AddOp, Box<MulExp>),
}

#[derive(Debug)]
pub enum AddOp {
    Add,
    Sub,
}

impl ToString for AddOp {
    fn to_string(&self) -> String {
        match self {
            AddOp::Add => "+".to_string(),
            AddOp::Sub => "-".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum RelExp {
    AddExp(Box<AddExp>),
    RelExp(Box<RelExp>, RelOp, Box<AddExp>),
}

#[derive(Debug)]
pub enum RelOp {
    Lt,
    Le,
    Gt,
    Ge,
}

impl ToString for RelOp {
    fn to_string(&self) -> String {
        match self {
            RelOp::Lt => "<".to_string(),
            RelOp::Le => "<=".to_string(),
            RelOp::Gt => ">".to_string(),
            RelOp::Ge => ">=".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(Box<RelExp>),
    EqExp(Box<EqExp>, EqOp, Box<RelExp>),
}

#[derive(Debug)]
pub enum EqOp {
    Eq,
    Ne,
}

impl ToString for EqOp {
    fn to_string(&self) -> String {
        match self {
            EqOp::Eq => "==".to_string(),
            EqOp::Ne => "!=".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(Box<EqExp>),
    LAndExp(Box<LAndExp>, Box<EqExp>),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(Box<LAndExp>),
    LOrExp(Box<LOrExp>, Box<LAndExp>),
}

#[derive(Debug)]
pub struct ConstExp {
    pub exp: Box<Exp>,
}
