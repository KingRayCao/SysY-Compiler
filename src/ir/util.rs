use super::*;
use key_node_list::Node;
use koopa::ir::builder::{
    BasicBlockBuilder, BlockBuilder, GlobalBuilder, LocalBuilder, LocalInstBuilder, ValueBuilder,
};
use koopa::ir::entities::ValueData;
use koopa::ir::{BasicBlock, FunctionData, Program, Type, Value, ValueKind};
use std::collections::HashMap;

// ============ Library Functions ============

pub fn init_lib_decl(program: &mut Program, context: &mut IrContext) {
    /*
       Library Functions
       decl @getint(): i32
       decl @getch(): i32
       decl @getarray(*i32): i32
       decl @putint(i32)
       decl @putch(i32)
       decl @putarray(i32, *i32)
       decl @starttime()
       decl @stoptime()
    */

    // getint
    let func_data = FunctionData::new_decl(
        "@getint".to_string(),
        Vec::new(),
        Type::get(TypeKind::Int32),
    );
    let func = program.new_func(func_data);

    // getch
    let func_data =
        FunctionData::new_decl("@getch".to_string(), Vec::new(), Type::get(TypeKind::Int32));
    let func = program.new_func(func_data);

    // getarray
    let func_data = FunctionData::new_decl(
        "@getarray".to_string(),
        vec![Type::get_pointer(Type::get(TypeKind::Int32))],
        Type::get(TypeKind::Int32),
    );
    let func = program.new_func(func_data);

    // putint
    let func_data = FunctionData::new_decl(
        "@putint".to_string(),
        vec![Type::get(TypeKind::Int32)],
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);

    // putch
    let func_data = FunctionData::new_decl(
        "@putch".to_string(),
        vec![Type::get(TypeKind::Int32)],
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);

    // putarray
    let func_data = FunctionData::new_decl(
        "@putarray".to_string(),
        vec![
            Type::get(TypeKind::Int32),
            Type::get_pointer(Type::get(TypeKind::Int32)),
        ],
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);

    // starttime
    let func_data = FunctionData::new_decl(
        "@starttime".to_string(),
        Vec::new(),
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);

    // stoptime
    let func_data = FunctionData::new_decl(
        "@stoptime".to_string(),
        Vec::new(),
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);
}

// ============ Basic Block utils ============

pub fn new_bb_builder<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> BlockBuilder<'a> {
    let func_data = program.func_mut(context.current_func.get_or_exit(111));
    func_data.dfg_mut().new_bb()
}

pub fn create_bb<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    name: &str,
) -> BasicBlock {
    let block = new_bb(program, context, name);
    insert_bb(program, context, block)
}

pub fn new_bb<'a>(program: &'a mut Program, context: &'a mut IrContext, name: &str) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.get_or_exit(112));
    let name = if name == "%entry" {
        name.to_string()
    } else {
        context.name_manager.get_name(name)
    };
    func_data.dfg_mut().new_bb().basic_block(Some(name))
}

pub fn insert_bb<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    bb: BasicBlock,
) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.get_or_exit(113));
    func_data
        .layout_mut()
        .bbs_mut()
        .push_key_back(bb)
        .get_or_exit(114);
    bb
}

pub fn change_current_bb<'a>(program: &'a mut Program, context: &'a mut IrContext, bb: BasicBlock) {
    // 检查前一个bb是否closed，如果没有close，则需要跳转到新bb
    if context.current_bb.is_some()
        && !bb_closed(program, context, context.current_bb.get_or_exit(115))
    {
        let jump_val = new_value_builder(program, context).jump(bb);
        add_value(program, context, jump_val).get_or_exit(116);
    }
    context.current_bb = Some(bb);
}

pub fn get_bb_last_value<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> Option<Value> {
    let func_data = program.func_mut(context.current_func.get_or_exit(117));
    func_data
        .layout_mut()
        .bb_mut(context.current_bb.get_or_exit(118))
        .insts_mut()
        .keys()
        .last()
        .map(|v| v.clone())
}

pub fn bb_closed(program: &Program, context: &IrContext, bb: BasicBlock) -> bool {
    let func_data = program.func(context.current_func.get_or_exit(119));
    let last_value = func_data.layout().bbs()[&bb]
        .insts()
        .keys()
        .last()
        .map(|v| v.clone());
    if let Some(last_value) = last_value {
        if let ValueKind::Return(_) = get_valuekind(program, context, last_value) {
            return true;
        }
        if let ValueKind::Jump(_) = get_valuekind(program, context, last_value) {
            return true;
        }
        if let ValueKind::Branch(_) = get_valuekind(program, context, last_value) {
            return true;
        }
    }
    false
}

// ============ Value utils ============

pub fn new_value_builder<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
) -> LocalBuilder<'a> {
    let func_data = program.func_mut(context.current_func.get_or_exit(120));
    func_data.dfg_mut().new_value()
}

pub fn add_value(
    program: &mut Program,
    context: &mut IrContext,
    value: Value,
) -> Result<(), String> {
    let mut bb = context.current_bb.get_or_exit(121);
    let bb_last_value = get_bb_last_value(program, context);
    // 如果当前bb已经closed，则新建bb
    if bb_closed(program, context, bb) {
        let new_bb = new_bb(program, context, "%new_bb");
        bb = insert_bb(program, context, new_bb);
        change_current_bb(program, context, bb);
    }
    let func_data = program.func_mut(context.current_func.get_or_exit(122));
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
    let func_data = program.func(context.current_func.get_or_exit(123));
    func_data.dfg().value(value)
}
pub fn set_value_name<'a>(
    program: &'a mut Program,
    context: &'a mut IrContext,
    value: Value,
    name: &str,
) {
    let func_data = program.func_mut(context.current_func.get_or_exit(124));
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

pub fn get_type(program: &Program, context: &IrContext, value: Value) -> Type {
    Type::get(get_typekind(program, context, value).clone())
}
// ============ Function utils ============

pub fn get_func(program: &Program, context: &IrContext, ident: &str) -> Function {
    for (func, func_data) in program.funcs().iter() {
        if func_data.name() == ident {
            return func.clone();
        }
    }
    // panic!("Function not found: {}", ident);
    std::process::exit(171);
}

pub fn get_func_data<'a>(
    program: &'a Program,
    context: &'a IrContext,
    func: Function,
) -> &'a FunctionData {
    program.func(func)
}

// ============ Symbol Table ============

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
                return (Some(entry), self.tables.len() - i - 1);
            }
        }
        (None, 0)
    }
    fn add_symbol(&mut self, name: &str, entry: SymbolTableEntry) {
        self.tables
            .last_mut()
            .get_or_exit(125)
            .insert(name.to_string(), entry);
    }
    pub fn add_var(&mut self, name: &str, tk: TypeKind, value: Value) {
        self.add_symbol(name, SymbolTableEntry::Var(tk, value));
    }
    pub fn add_const(&mut self, name: &str, tk: TypeKind, value: i32) {
        self.add_symbol(name, SymbolTableEntry::Const(tk, value));
    }
    pub fn get_depth(&self) -> usize {
        self.tables.len() - 1
    }
}

// ============ NameManager ============
// Generate unique name for input str

pub struct NameManager {
    name_map: HashMap<String, i32>,
}

impl NameManager {
    pub fn new() -> Self {
        NameManager {
            name_map: HashMap::new(),
        }
    }
    pub fn get_name(&mut self, name: &str) -> String {
        if !self.name_map.contains_key(name) {
            self.name_map.insert(name.to_string(), 0);
        }
        let count = self.name_map.get_mut(name).get_or_exit(126);
        *count += 1;
        let new_name = format!("{}_{}", name, count);
        new_name
    }
    pub fn reset(&mut self) {
        self.name_map.clear();
    }
}

// ============ WhileStack ============
// Store current while loop info

pub struct WhileStack {
    stack: Vec<(BasicBlock, BasicBlock)>,
}

impl WhileStack {
    pub fn new() -> Self {
        WhileStack { stack: Vec::new() }
    }
    pub fn push(&mut self, while_bb: BasicBlock, end_bb: BasicBlock) {
        self.stack.push((while_bb, end_bb));
    }
    pub fn pop(&mut self) -> Option<(BasicBlock, BasicBlock)> {
        self.stack.pop()
    }
    pub fn get_top(&self) -> Option<(BasicBlock, BasicBlock)> {
        self.stack.last().cloned()
    }
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
// ============ IrContext ============

pub struct IrContext {
    pub current_func: Option<Function>,
    pub current_bb: Option<BasicBlock>,
    pub symbol_tables: SymbolTableStack,
    pub name_manager: NameManager,
    pub while_stack: WhileStack,
    pub is_global: bool,
}

impl IrContext {
    pub fn new() -> Self {
        let mut ret = IrContext {
            current_func: None,
            current_bb: None,
            symbol_tables: SymbolTableStack::new(),
            name_manager: NameManager::new(),
            while_stack: WhileStack::new(),
            is_global: true,
        };
        ret.symbol_tables.push_table(); // 全局变量表
        ret
    }
    pub fn change_current_func(&mut self, func: Function) {
        self.current_func = Some(func);
        self.current_bb = None;
        self.while_stack.clear();
    }
}

// ============ Debug ============
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

pub trait ExitOnError {
    type Output;
    fn get_or_exit(self, code: i32) -> Self::Output;
}

impl<M, N> ExitOnError for Result<M, N> {
    type Output = M;
    fn get_or_exit(self, code: i32) -> Self::Output {
        self.unwrap_or_else(|_| std::process::exit(code))
    }
}

impl<T> ExitOnError for Option<T> {
    type Output = T;
    fn get_or_exit(self, code: i32) -> Self::Output {
        self.unwrap_or_else(|| std::process::exit(code))
    }
}
