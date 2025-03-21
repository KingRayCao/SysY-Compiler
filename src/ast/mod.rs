pub mod decl;
pub mod exp;
pub mod stmt;
use decl::*;
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}
