use super::exp::*;
#[derive(Debug)]
pub enum Stmt {
    AssignStmt(Box<LVal>, Box<Exp>),
    ReturnStmt(Box<Exp>),
}
