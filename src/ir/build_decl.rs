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
            match self.const_init_val.as_ref() {
                ConstInitVal::ConstExp(exp) => {
                    let const_init_val = exp.get_const_value(context).get_or_exit(11);
                    context
                        .symbol_tables
                        .add_const(&self.ident, TypeKind::Int32, const_init_val);
                }
                ConstInitVal::ConstArray(init_val_vec) => {
                    todo!()
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
        if !context.is_global {
            // Local Variable
            match self {
                VarDef::VarDef { ident, index } => {
                    if index.is_empty() {
                        // Single Variable
                        let alloc = new_value_builder(program, context).alloc(Type::get_i32());
                        add_value(program, context, alloc).get_or_exit(12);
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
                        // Array Variable
                        todo!()
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
                        add_value(program, context, alloc).get_or_exit(13);
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
                                let exp_val = exp.build_ir(program, context).get_or_exit(14);
                                let store =
                                    new_value_builder(program, context).store(exp_val, alloc);
                                add_value(program, context, store).get_or_exit(15);
                                return Ok(());
                            }
                            InitVal::Array(init_val_vec) => {
                                unreachable!()
                            }
                        }
                    } else {
                        // Array Variable
                        todo!()
                    }
                }
            }
        } else {
            // Global Variable
            match self {
                VarDef::VarDef { ident, index } => {
                    if index.is_empty() {
                        // Single Variable
                        let val_0 = program.new_value().integer(0);
                        let alloc = program.new_value().global_alloc(val_0);
                        program.set_value_name(alloc, Some(format!("@{}_0", ident)));
                        context
                            .symbol_tables
                            .add_var(&ident, TypeKind::Int32, alloc);
                    } else {
                        // Array Variable
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
                        // Single Variable
                        match init_val.as_ref() {
                            InitVal::Exp(exp) => {
                                let const_init_val = exp.get_const_value(context).get_or_exit(16);
                                let val = program.new_value().integer(const_init_val);
                                let alloc = program.new_value().global_alloc(val);
                                program.set_value_name(alloc, Some(format!("@{}_0", ident)));
                                context
                                    .symbol_tables
                                    .add_var(&ident, TypeKind::Int32, alloc);
                            }
                            InitVal::Array(init_val_vec) => {
                                unreachable!()
                            }
                        }
                    } else {
                        // Array Variable
                        todo!()
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
            .map(|func_param| {
                (
                    Some(format!("@{}", func_param.get_ident())),
                    func_param.to_type(),
                )
            })
            .collect();
        let func_data = FunctionData::with_param_names(
            format!("@{}", self.ident),
            params_vec,
            self.return_type.to_type(),
        );

        let func = program.new_func(func_data);
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
                let data = get_valuedata(program, context, param);
                let name = data.name().as_ref().get_or_exit(17).clone();
                let tk = get_typekind(program, context, param).clone();
                let ty = get_type(program, context, param).clone();
                (param, name, tk, ty)
            })
            .collect();

        for (param, param_name, param_tk, param_ty) in params {
            let alloc_value = new_value_builder(program, context).alloc(param_ty);
            add_value(program, context, alloc_value).get_or_exit(18);
            set_value_name(
                program,
                context,
                alloc_value,
                &format!(
                    "%{}_{}",
                    &param_name[1..],
                    context.symbol_tables.get_depth()
                ),
            );
            context
                .symbol_tables
                .add_var(&param_name[1..], param_tk, alloc_value);
            let store_value = new_value_builder(program, context).store(param, alloc_value);
            add_value(program, context, store_value).get_or_exit(19);
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
                    add_value(program, context, ret).get_or_exit(20);
                }
                TypeKind::Int32 => {
                    let val_0 = new_value_builder(program, context).integer(0);
                    let ret = new_value_builder(program, context).ret(Some(val_0));
                    add_value(program, context, ret).get_or_exit(21);
                }
                _ => todo!(),
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
