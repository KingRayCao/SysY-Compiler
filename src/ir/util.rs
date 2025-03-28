use super::*;
use koopa::ir::builder::{BasicBlockBuilder, BlockBuilder, LocalBuilder};
use koopa::ir::entities::ValueData;
use koopa::ir::{BasicBlock, FunctionData, Program, Value, ValueKind};
use std::collections::HashMap;

pub fn new_value_builder<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> LocalBuilder<'a> {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.dfg_mut().new_value()
}

pub fn new_bb_builder<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> BlockBuilder<'a> {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.dfg_mut().new_bb()
}

pub fn create_bb<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    name: &str,
) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.unwrap());
    let block = func_data
        .dfg_mut()
        .new_bb()
        .basic_block(Some(name.to_string()));
    func_data
        .layout_mut()
        .bbs_mut()
        .push_key_back(block)
        .unwrap();
    block
}

pub fn new_bb<'a>(program: &'a mut Program, context: &'a mut IrContext, name: &str) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data
        .dfg_mut()
        .new_bb()
        .basic_block(Some(name.to_string()))
}

pub fn insert_bb<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    bb: BasicBlock,
) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.layout_mut().bbs_mut().push_key_back(bb).unwrap();
    bb
}

pub fn change_current_bb<'a>(program: &'a mut Program, context: &'a mut IrContext, bb: BasicBlock) {
    context.current_bb = Some(bb);
}

pub fn get_bb_last_value<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> Option<Value> {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data
        .layout_mut()
        .bb_mut(context.current_bb.unwrap())
        .insts_mut()
        .keys()
        .last()
        .map(|v| v.clone())
}

pub fn add_value(
    program: &mut Program,
    context: &mut IrContext,
    value: Value,
) -> Result<(), String> {
    let bb = context.current_bb.unwrap();
    let bb_last_value = get_bb_last_value(program, context);
    if let Some(bb_last_value) = bb_last_value {
        if let ValueKind::Return(_) = get_valuekind(program, context, bb_last_value) {
            return Ok(());
        }
    }
    let func_data = program.func_mut(context.current_func.unwrap());
    let insert_ok = func_data
        .layout_mut()
        .bb_mut(bb)
        .insts_mut()
        .push_key_back(value);
    match insert_ok {
        Ok(_) => Ok(()),
        Err(value) => Err(format!("Failed to insert value: {:?}", value)),
    }
}

pub fn get_valuedata<'a>(
    program: &'a Program,
    context: &'a IrContext,
    value: Value,
) -> &'a ValueData {
    let func_data = program.func(context.current_func.unwrap());
    func_data.dfg().value(value)
}
pub fn set_value_name<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    value: Value,
    name: &str,
) {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data
        .dfg_mut()
        .set_value_name(value, Some(name.to_string()));
}
pub fn get_valuekind<'a>(
    program: &'a Program,
    context: &'a IrContext,
    value: Value,
) -> &'a ValueKind {
    let value_data = get_valuedata(program, context, value);
    value_data.kind()
}
pub fn get_typekind<'a>(
    program: &'a Program,
    context: &'a IrContext,
    value: Value,
) -> &'a TypeKind {
    let value_data = get_valuedata(program, context, value);
    value_data.ty().kind()
}

// Symbol Table
pub struct SymbolTableStack {
    tables: Vec<HashMap<String, SymbolTableEntry>>,
}

pub enum SymbolTableEntry {
    Const(TypeKind, i32),
    Var(TypeKind, Value),
}

impl SymbolTableStack {
    pub fn new() -> Self {
        SymbolTableStack { tables: Vec::new() }
    }
    pub fn push_table(&mut self) {
        self.tables.push(HashMap::new());
    }
    pub fn pop_table(&mut self) {
        self.tables.pop();
    }
    pub fn get_symbol(&self, name: &str) -> (Option<&SymbolTableEntry>, usize) {
        for (i, table) in self.tables.iter().rev().enumerate() {
            if let Some(entry) = table.get(name) {
                return (Some(entry), self.tables.len() - i);
            }
        }
        (None, 0)
    }
    fn add_symbol(&mut self, name: &str, entry: SymbolTableEntry) {
        self.tables
            .last_mut()
            .unwrap()
            .insert(name.to_string(), entry);
    }
    pub fn add_var(&mut self, name: &str, tk: TypeKind, value: Value) {
        self.add_symbol(name, SymbolTableEntry::Var(tk, value));
    }
    pub fn add_const(&mut self, name: &str, tk: TypeKind, value: i32) {
        self.add_symbol(name, SymbolTableEntry::Const(tk, value));
    }
    pub fn get_depth(&self) -> usize {
        self.tables.len()
    }
}

// Debug
pub fn print_value(program: &Program, context: &IrContext, value: Value) {
    let value_data = get_valuedata(program, context, value);
    println!("value: {:?}", value);
    print_value_data(value_data);
}

pub fn print_value_data(value_data: &ValueData) {
    println!("kind: {:?}", value_data.kind());
    println!("type: {:?}", value_data.ty());
    println!("size: {:?}", value_data.ty().size());
}
