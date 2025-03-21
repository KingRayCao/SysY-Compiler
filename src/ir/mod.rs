mod build_decl;
mod build_expr;
mod build_stmt;
use crate::ast::*;

pub trait IRBuildable {
    fn build_ir(&self) -> String;
}

impl IRBuildable for CompUnit {
    fn build_ir(&self) -> String {
        self.func_def.build_ir()
    }
}