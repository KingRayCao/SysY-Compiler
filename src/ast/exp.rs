#[derive(Debug)]
pub enum Exp {
    UnaryExp(Box<UnaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(i32),
}
#[derive(Debug)]
pub enum UnaryExp {
    UnaryExp(UnaryOp, Box<UnaryExp>),
    PrimaryExp(Box<PrimaryExp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

impl ToString for UnaryOp {
    fn to_string(&self) -> String {
        match self {
            UnaryOp::Plus => "+".to_string(),
            UnaryOp::Minus => "-".to_string(),
            UnaryOp::Not => "!".to_string(),
        }
    }
}
