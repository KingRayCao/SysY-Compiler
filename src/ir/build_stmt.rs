use super::*;
use crate::ast::stmt::*;

impl IRBuildable for Stmt {
    fn build_ir(&self) -> String {
        format!("  ret {}", self.num)
    }
}
