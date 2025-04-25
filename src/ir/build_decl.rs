use super::const_eval::*;
use super::*;
use crate::ast::decl::*;
use crate::ast::stmt::*;
use koopa::ir::builder::GlobalInstBuilder;
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
            let const_init_val = self.const_init_val.get_const_i32(context).unwrap();
            context
                .symbol_tables
                .add_const(&self.ident, Type::get_i32(), const_init_val);
        } else {
            let size = Array::const_exp2size(&self.index, context);
            let array_type = Array::size2type(&size);
            let const_init_array =
                Array::get_const_init_array(program, context, &self.const_init_val, &size);
            if !context.is_global {
                // Local Variable
                let alloc = new_value_builder(program, context).alloc(array_type.clone());
                add_value(program, context, alloc).unwrap();
                context
                    .symbol_tables
                    .add_array(&self.ident, array_type, alloc, size);
                // assign
                const_init_array.init_assign_to_array(program, context, alloc);
            } else {
                // Global Variable
                let array_value = const_init_array.to_value(program, context);
                let alloc = program.new_value().global_alloc(array_value);
                context
                    .symbol_tables
                    .add_array(&self.ident, array_type, alloc, size);
            }
        }
        Ok(())
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
        if !context.is_global {
            // Local Variable
            match self {
                VarDef::VarDef { ident, index } => {
                    if index.is_empty() {
                        // Single Variable
                        let alloc = new_value_builder(program, context).alloc(Type::get_i32());
                        add_value(program, context, alloc).unwrap();
                        context
                            .symbol_tables
                            .add_var(&ident, Type::get_i32(), alloc);
                    } else {
                        // Array Variable
                        let size = Array::const_exp2size(index, context);
                        let array_kind = Array::size2type(&size);
                        let alloc = new_value_builder(program, context).alloc(array_kind.clone());
                        add_value(program, context, alloc).unwrap();
                        context
                            .symbol_tables
                            .add_array(&ident, array_kind, alloc, size);
                    }
                    return Ok(());
                }
                VarDef::VarDefInit {
                    ident,
                    index,
                    init_val,
                } => {
                    if index.is_empty() {
                        let alloc = new_value_builder(program, context).alloc(Type::get_i32());
                        add_value(program, context, alloc).unwrap();
                        context
                            .symbol_tables
                            .add_var(&ident, Type::get_i32(), alloc);
                        match init_val.as_ref() {
                            InitVal::Exp(exp) => {
                                let exp_val = exp.build_ir(program, context).unwrap();
                                let store =
                                    new_value_builder(program, context).store(exp_val, alloc);
                                add_value(program, context, store).unwrap();
                                Ok(())
                            }
                            InitVal::Array(init_val_vec) => unreachable!(),
                        }
                    } else {
                        // Array Variable
                        let size = Array::const_exp2size(index, context);
                        let array_kind = Array::size2type(&size);
                        let alloc = new_value_builder(program, context).alloc(array_kind.clone());
                        add_value(program, context, alloc).unwrap();
                        let init_array = Array::get_init_array(program, context, &init_val, &size);
                        init_array.init_assign_to_array(program, context, alloc);
                        context
                            .symbol_tables
                            .add_array(&ident, array_kind, alloc, size);

                        Ok(())
                    }
                }
            }
        } else {
            // Global Variable
            match self {
                VarDef::VarDef { ident, index } => {
                    if index.is_empty() {
                        // Single Variable
                        let val_0 = const_int_value(program, context, 0);
                        let alloc = program.new_value().global_alloc(val_0);
                        context
                            .symbol_tables
                            .add_var(&ident, Type::get_i32(), alloc);
                    } else {
                        // Array Variable
                        let size = Array::const_exp2size(index, context);
                        let array_type = Array::size2type(&size);
                        let init_0_array =
                            Array::new(program, context, &size).to_value(program, context);
                        let alloc = program.new_value().global_alloc(init_0_array);
                        context
                            .symbol_tables
                            .add_array(&ident, array_type, alloc, size);
                    }
                    Ok(())
                }
                VarDef::VarDefInit {
                    ident,
                    index,
                    init_val,
                } => {
                    if index.is_empty() {
                        // Single Variable
                        let const_init_val = init_val.get_const_i32(context).unwrap();
                        let val = const_int_value(program, context, const_init_val);
                        let alloc = program.new_value().global_alloc(val);
                        context
                            .symbol_tables
                            .add_var(&ident, Type::get_i32(), alloc);
                    } else {
                        // Array Variable
                        let size = Array::const_exp2size(index, context);
                        let array_type = Array::size2type(&size);
                        let init_array = Array::get_init_array(program, context, &init_val, &size);
                        let init_array_value = init_array.to_value(program, context);
                        let alloc = program.new_value().global_alloc(init_array_value);
                        context
                            .symbol_tables
                            .add_array(&ident, array_type, alloc, size);
                    }
                    Ok(())
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
        let params_vec = self
            .func_f_params
            .iter()
            .map(|func_param| func_param.to_type(program, context))
            .collect();
        let func_data = FunctionData::new(
            format!("@{}", self.ident),
            params_vec,
            self.return_type.to_type(),
        );
        let func = program.new_func(func_data);
        context.func_table.insert(self.ident.clone(), func.clone());
        context.change_current_func(func);
        // create entry block
        let entry_bb = create_bb(program, context, "%entry");
        change_current_bb(program, context, entry_bb);
        // allocate function params to stack
        context.symbol_tables.push_table();

        let params: Vec<_> = get_func_data(program, context, func)
            .params()
            .iter()
            .map(|&param| {
                let ty = get_type(program, context, param).clone();
                (param, ty)
            })
            .collect();

        for i in 0..params.len() {
            let (param, param_ty) = params[i].clone();
            let alloc_value = new_value_builder(program, context).alloc(param_ty.clone());
            add_value(program, context, alloc_value).unwrap();
            if let FuncFParam::Var(_, _) = &self.func_f_params[i] {
                context.symbol_tables.add_var(
                    &self.func_f_params[i].get_ident(),
                    param_ty,
                    alloc_value,
                );
            } else {
                let size =
                    Array::const_exp2size(&self.func_f_params[i].get_size().unwrap(), context);
                context.symbol_tables.add_array_param(
                    &self.func_f_params[i].get_ident(),
                    param_ty,
                    alloc_value,
                    size,
                );
            }
            let store_value = new_value_builder(program, context).store(param, alloc_value);
            add_value(program, context, store_value).unwrap();
        }
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
            match self.return_type.to_typekind() {
                TypeKind::Unit => {
                    let ret = new_value_builder(program, context).ret(None);
                    add_value(program, context, ret).unwrap();
                }
                TypeKind::Int32 => {
                    let val_0 = const_int_value(program, context, 0);
                    let ret = new_value_builder(program, context).ret(Some(val_0));
                    add_value(program, context, ret).unwrap();
                }
                _ => unreachable!(),
            }
        }
        context.symbol_tables.pop_table();
        Ok(())
    }
}

// ============= Block =============

impl IrGenerator for Block {
    type Output = Result<(), String>;
    // 确保调用前使用了push_table，调用后使用了pop_table
    fn build_ir(&self, program: &mut Program, context: &mut IrContext) -> Self::Output {
        for item in self.block_items.iter() {
            match item {
                BlockItem::Decl(decl) => {
                    decl.build_ir(program, context)?;
                }
                BlockItem::Stmt(stmt) => {
                    stmt.build_ir(program, context)?;
                }
            }
        }
        Ok(())
    }
}
