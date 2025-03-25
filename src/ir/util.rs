use super::*;
use koopa::ir::entities::ValueData;
use koopa::ir::{builder::LocalBuilder, BasicBlock, Program, Value};
use std::collections::HashMap;

pub fn new_value_builder<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> LocalBuilder<'a> {
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.dfg_mut().new_value()
}

pub fn add_value(
    program: &mut Program,
    context: &mut IrContext,
    value: Value,
) -> Result<(), String> {
    let func_data = program.func_mut(context.current_func.unwrap());
    let bb = context.current_block.unwrap();
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
pub fn get_valuedata_kind<'a>(
    program: &'a Program,
    context: &'a IrContext,
    value: Value,
) -> &'a TypeKind {
    let func_data = program.func(context.current_func.unwrap());
    func_data.dfg().value(value).ty().kind()
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
    pub fn get_symbol(&self, name: &str) -> Option<&SymbolTableEntry> {
        for table in self.tables.iter().rev() {
            if let Some(entry) = table.get(name) {
                return Some(entry);
            }
        }
        None
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
}
