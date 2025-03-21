use super::*;
use crate::ast::decl::*;
impl IRBuildable for FuncDef {
    fn build_ir(&self) -> String {
        format!(
            "fun @{}(): {} {{\n{}\n}}",
            self.ident,
            self.func_type.to_string(),
            self.block.build_ir()
        )
    }
}

impl IRBuildable for Block {
    fn build_ir(&self) -> String {
        format!("%entry: \n{}", self.stmt.build_ir())
    }
}
