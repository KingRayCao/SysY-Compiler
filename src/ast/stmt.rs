use super::{decl::Block, exp::*};
#[derive(Debug)]
pub enum Stmt {
    AssignStmt(Box<LVal>, Box<Exp>),
    ExpStmt(Box<Option<Exp>>),
    BlockStmt(Box<Block>),
    IfStmt(Box<Exp>, Box<Stmt>, Option<Box<Stmt>>),
    WhileStmt(Box<Exp>, Box<Stmt>),
    BreakStmt,
    ContinueStmt,
    ReturnStmt(Box<Option<Exp>>),
}
