use super::exp::*;
use super::stmt::*;
use koopa::ir::Type;
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
pub enum FuncFParam {
    Var(BType, String),
    Array(BType, String, Vec<ConstExp>),
}

impl FuncFParam {
    pub fn to_typekind(&self) -> TypeKind {
        match self {
            FuncFParam::Var(btype, _) => btype.to_typekind(),
            FuncFParam::Array(btype, _, _) => todo!(),
        }
    }
    pub fn to_type(&self) -> Type {
        Type::get(self.to_typekind())
    }
    pub fn get_ident(&self) -> &String {
        match self {
            FuncFParam::Var(_, ident) => ident,
            FuncFParam::Array(_, ident, _) => ident,
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
    Void,
}

impl BType {
    pub fn to_typekind(&self) -> TypeKind {
        match self {
            BType::Int => TypeKind::Int32,
            BType::Void => TypeKind::Unit,
        }
    }
    pub fn to_type(&self) -> Type {
        Type::get(self.to_typekind())
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
    ConstArray(Vec<ConstInitVal>),
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
    Array(Vec<InitVal>),
}
