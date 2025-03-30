pub mod decl;
pub mod exp;
pub mod stmt;
use decl::*;
#[derive(Debug)]
pub struct CompUnit {
    pub items: Vec<CompItem>,
}

#[derive(Debug)]
pub enum CompItem {
    Decl(Box<Decl>),
    FuncDef(Box<FuncDef>),
}
