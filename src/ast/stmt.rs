use super::{decl::Block, exp::*};
#[derive(Debug)]
pub enum Stmt {
    AssignStmt(Box<LVal>, Box<Exp>),
    ExpStmt(Box<Option<Exp>>),
    BlockStmt(Box<Block>),
    ReturnStmt(Box<Exp>),
}
