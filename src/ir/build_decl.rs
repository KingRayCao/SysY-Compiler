use super::const_eval::*;
use super::*;
use crate::ast::decl::*;
use koopa::ir::{builder::BasicBlockBuilder, FunctionData, Type, TypeKind};
impl FuncType {
    fn to_koopa_kind(&self) -> TypeKind {
        match self {
            FuncType::Void => TypeKind::Unit,
            FuncType::Int => TypeKind::Int32,
        }
    }
}
impl IrGenerator for FuncDef {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        // add function_data to program
        let func_data = FunctionData::new(
            format!("@{}", self.ident),
            Vec::new(),
            Type::get(self.func_type.to_koopa_kind()),
        );
        let func = program.new_func(func_data);
        context.current_func = Some(func);
        // create blocks
        self.block.build_ir(program, context)?;
        Ok(())
    }
}

impl IrGenerator for Block {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        let func_data = program.func_mut(context.current_func.unwrap());
        let entry_block = func_data
            .dfg_mut()
            .new_bb()
            .basic_block(Some("%entry".into()));
        func_data
            .layout_mut()
            .bbs_mut()
            .push_key_back(entry_block)
            .unwrap();
        context.current_block = Some(entry_block);
        for item in self.block_items.iter() {
            match item {
                BlockItem::Decl(decl) => decl.build_ir(program, context)?,
                BlockItem::Stmt(stmt) => stmt.build_ir(program, context)?,
            }
        }
        Ok(())
    }
}

impl IrGenerator for Decl {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Decl::ConstDecl(decl) => decl.build_ir(program, context),
        }
    }
}

impl IrGenerator for ConstDecl {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self.btype {
            BType::Int => {
                for def in self.const_defs.iter() {
                    def.build_ir(program, context)?;
                }
                Ok(())
            }
        }
    }
}

impl IrGenerator for ConstDef {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        if self.index.is_empty() {
            match self.const_init_val.as_ref() {
                ConstInitVal::ConstExp(exp) => {
                    let const_init_val = exp.get_const_value(context).unwrap();
                    context
                        .symbol_tables
                        .add_const(&self.ident, TypeKind::Int32, const_init_val);
                }
            }
            Ok(())
        } else {
            todo!()
        }
    }
}
