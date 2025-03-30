use super::const_eval::*;
use super::*;
use crate::ast::decl::*;
use crate::ast::stmt::*;
use koopa::ir::builder::ValueBuilder;
use koopa::ir::builder::{BasicBlockBuilder, LocalInstBuilder};
use koopa::ir::{FunctionData, Type, TypeKind, ValueKind};

// ============= Declaration =============

impl IrGenerator for Decl {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            Decl::ConstDecl(decl) => decl.build_ir(program, context),
            Decl::VarDecl(decl) => decl.build_ir(program, context),
        }
    }
}

// ---- Constant Declaration ----

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
            _ => unreachable!(),
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

// ---- Variable Declaration ----

impl IrGenerator for VarDecl {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        for def in self.var_defs.iter() {
            def.build_ir(program, context)?;
        }
        Ok(())
    }
}

impl IrGenerator for VarDef {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        match self {
            VarDef::VarDef { ident, index } => {
                if index.is_empty() {
                    let alloc = new_value_builder(program, context).alloc(Type::get_i32());
                    add_value(program, context, alloc).unwrap();
                    set_value_name(
                        program,
                        context,
                        alloc,
                        format!("@{}_{}", ident, context.symbol_tables.get_depth()).as_str(),
                    );
                    context
                        .symbol_tables
                        .add_var(&ident, TypeKind::Int32, alloc);
                } else {
                    todo!()
                }
                Ok(())
            }
            VarDef::VarDefInit {
                ident,
                index,
                init_val,
            } => {
                if index.is_empty() {
                    let alloc = new_value_builder(program, context).alloc(Type::get_i32());
                    add_value(program, context, alloc).unwrap();
                    set_value_name(
                        program,
                        context,
                        alloc,
                        format!("@{}_{}", ident, context.symbol_tables.get_depth()).as_str(),
                    );
                    context
                        .symbol_tables
                        .add_var(&ident, TypeKind::Int32, alloc);
                    match init_val.as_ref() {
                        InitVal::Exp(exp) => {
                            let exp_val = exp.build_ir(program, context).unwrap();
                            let store = new_value_builder(program, context).store(exp_val, alloc);
                            add_value(program, context, store).unwrap();
                            Ok(())
                        }
                    }
                } else {
                    todo!()
                }
            }
        }
    }
}

// ---- Function Declaration ----

impl IrGenerator for FuncDef {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        // add function_data to program
        let func_data = FunctionData::new(
            format!("@{}", self.ident),
            Vec::new(),
            Type::get(self.return_type.to_koopa_kind()),
        );
        let func = program.new_func(func_data);
        context.current_func = Some(func);
        // create entry block
        let entry_bb = create_bb(program, context, "%entry");
        change_current_bb(program, context, entry_bb);
        // compile block
        // 注意 BasicBlock和Block的区别
        self.block.build_ir(program, context)?;
        // 检查目前Block的最后一个语句是否为ret
        let bb_last_value = get_bb_last_value(program, context);
        let mut need_ret = false;
        if let Some(bb_last_value) = bb_last_value {
            if !matches!(
                get_valuekind(program, context, bb_last_value),
                ValueKind::Return(_)
            ) {
                need_ret = true;
            }
        } else {
            need_ret = true;
        }
        if need_ret {
            match self.return_type.to_koopa_kind() {
                TypeKind::Unit => {
                    let ret = new_value_builder(program, context).ret(None);
                    add_value(program, context, ret).unwrap();
                }
                TypeKind::Int32 => {
                    let val_0 = new_value_builder(program, context).integer(0);
                    let ret = new_value_builder(program, context).ret(Some(val_0));
                    add_value(program, context, ret).unwrap();
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
}

// ============= Block =============

impl IrGenerator for Block {
    type Output = Result<(), String>;
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        context.symbol_tables.push_table();
        for item in self.block_items.iter() {
            match item {
                BlockItem::Decl(decl) => decl.build_ir(program, context)?,
                BlockItem::Stmt(stmt) => {
                    stmt.build_ir(program, context)?;
                }
            }
        }
        context.symbol_tables.pop_table();
        Ok(())
    }
}
