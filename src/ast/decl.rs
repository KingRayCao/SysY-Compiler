use super::exp::*;
use super::stmt::*;
use koopa::ir::TypeKind;

// ============= Function =============

#[derive(Debug)]
pub struct FuncDef {
    pub return_type: BType,
    pub ident: String,
    pub func_f_params: Vec<FuncFParam>,
    pub block: Block,
}

#[derive(Debug)]
pub struct FuncFParam {
    pub btype: BType,
    pub ident: String,
    pub index: Option<Vec<ConstExp>>,
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
    Void,
}

impl BType {
    pub fn to_koopa_kind(&self) -> TypeKind {
        match self {
            BType::Int => TypeKind::Int32,
            BType::Void => TypeKind::Unit,
        }
    }
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
