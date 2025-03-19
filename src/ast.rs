pub trait AstNode {
    fn to_ir(&self) -> String;
}

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}
// 在 Rust 中，实现 trait 使用 `impl` 关键字，而不是 `implement`。下面是修正后的代码：
impl AstNode for CompUnit {
    fn to_ir(&self) -> String {
        self.func_def.to_ir()
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}
impl AstNode for FuncDef {
    fn to_ir(&self) -> String {
        format!(
            "fun @{}(): {} {{\n{}\n}}",
            self.ident,
            self.func_type.to_string(),
            self.block.to_ir()
        )
    }
}

#[derive(Debug)]
pub enum FuncType {
    Void,
    Int,
}
impl ToString for FuncType {
    fn to_string(&self) -> String {
        match self {
            FuncType::Void => "void".to_string(),
            FuncType::Int => "i32".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}
impl AstNode for Block {
    fn to_ir(&self) -> String {
        format!("%entry: \n{}", self.stmt.to_ir())
    }
}
#[derive(Debug)]
pub struct Stmt {
    pub num: i32,
}
impl AstNode for Stmt {
    fn to_ir(&self) -> String {
        format!("  ret {}", self.num)
    }
}
