use super::exp::*;
use super::stmt::*;
use koopa::ir::TypeKind;

// ============= Function =============

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
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

// ============= Block =============

#[derive(Debug)]
pub struct Block {
    pub block_items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Box<Decl>),
    Stmt(Box<Stmt>),
}

// ============= Declaration =============

#[derive(Debug)]
pub enum Decl {
    ConstDecl(Box<ConstDecl>),
    VarDecl(Box<VarDecl>),
}

// ---- Constant Declaration ----

#[derive(Debug)]
pub struct ConstDecl {
    pub btype: BType,
    pub const_defs: Vec<ConstDef>,
}

#[derive(Debug)]
pub enum BType {
    Int,
}

#[derive(Debug)]
pub struct ConstDef {
    pub ident: String,
    pub index: Vec<ConstExp>,
    pub const_init_val: Box<ConstInitVal>,
}

#[derive(Debug)]
pub enum ConstInitVal {
    ConstExp(Box<ConstExp>),
}

// ---- Variable Declaration ----

#[derive(Debug)]
pub struct VarDecl {
    pub btype: BType,
    pub var_defs: Vec<VarDef>,
}

#[derive(Debug)]
pub enum VarDef {
    VarDef {
        ident: String,
        index: Vec<ConstExp>,
    },
    VarDefInit {
        ident: String,
        index: Vec<ConstExp>,
        init_val: Box<InitVal>,
    },
}

#[derive(Debug)]
pub enum InitVal {
    Exp(Box<Exp>),
}
