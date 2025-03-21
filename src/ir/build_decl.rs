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
        self.stmt.build_ir(program, context)?;
        Ok(())
    }
}
