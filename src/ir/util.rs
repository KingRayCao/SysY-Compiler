use super::const_eval::ConstI32Eval;
use super::*;
use crate::ast::decl::*;
use crate::ast::exp::*;
use crate::ast::stmt::*;
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
    context
        .func_table
        .insert("getint".to_string(), func.clone());

    // getch
    let func_data =
        FunctionData::new_decl("@getch".to_string(), Vec::new(), Type::get(TypeKind::Int32));
    let func = program.new_func(func_data);
    context.func_table.insert("getch".to_string(), func.clone());

    // getarray
    let func_data = FunctionData::new_decl(
        "@getarray".to_string(),
        vec![Type::get_pointer(Type::get(TypeKind::Int32))],
        Type::get(TypeKind::Int32),
    );
    let func = program.new_func(func_data);
    context
        .func_table
        .insert("getarray".to_string(), func.clone());

    // putint
    let func_data = FunctionData::new_decl(
        "@putint".to_string(),
        vec![Type::get(TypeKind::Int32)],
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);
    context
        .func_table
        .insert("putint".to_string(), func.clone());

    // putch
    let func_data = FunctionData::new_decl(
        "@putch".to_string(),
        vec![Type::get(TypeKind::Int32)],
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);
    context.func_table.insert("putch".to_string(), func.clone());

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
    context
        .func_table
        .insert("putarray".to_string(), func.clone());

    // starttime
    let func_data = FunctionData::new_decl(
        "@starttime".to_string(),
        Vec::new(),
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);
    context
        .func_table
        .insert("starttime".to_string(), func.clone());

    // stoptime
    let func_data = FunctionData::new_decl(
        "@stoptime".to_string(),
        Vec::new(),
        Type::get(TypeKind::Unit),
    );
    let func = program.new_func(func_data);
    context
        .func_table
        .insert("stoptime".to_string(), func.clone());
}

// ============ Basic Block utils ============

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
    let block = new_bb(program, context, name);
    insert_bb(program, context, block)
}

pub fn new_bb<'a>(program: &'a mut Program, context: &'a mut IrContext, name: &str) -> BasicBlock {
    let func_data = program.func_mut(context.current_func.unwrap());
    let name = context.name_manager.get_name(name);
    func_data.dfg_mut().new_bb().basic_block(Some(name))
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
    // 检查前一个bb是否closed，如果没有close，则需要跳转到新bb
    if context.current_bb.is_some() && !bb_closed(program, context, context.current_bb.unwrap()) {
        let jump_val = new_value_builder(program, context).jump(bb);
        add_value(program, context, jump_val).unwrap();
    }
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

pub fn bb_closed(program: &Program, context: &IrContext, bb: BasicBlock) -> bool {
    let func_data = program.func(context.current_func.unwrap());
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
    let func_data = program.func_mut(context.current_func.unwrap());
    func_data.dfg_mut().new_value()
}

pub fn const_int_value(program: &mut Program, context: &mut IrContext, value: i32) -> Value {
    if context.is_global {
        program.new_value().integer(value)
    } else {
        new_value_builder(program, context).integer(value)
    }
}

pub fn add_value(
    program: &mut Program,
    context: &mut IrContext,
    value: Value,
) -> Result<(), String> {
    let mut bb = context.current_bb.unwrap();
    let bb_last_value = get_bb_last_value(program, context);
    // 如果当前bb已经closed，则新建bb
    if bb_closed(program, context, bb) {
        let new_bb = new_bb(program, context, "%new_bb");
        bb = insert_bb(program, context, new_bb);
        change_current_bb(program, context, bb);
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
    context.func_table.get(ident).unwrap().clone()
}

pub fn get_func_data<'a>(
    program: &'a Program,
    context: &'a IrContext,
    func: Function,
) -> &'a FunctionData {
    program.func(func)
}

// =========== Array utils ============

// used for load
pub fn get_array_elem(
    program: &mut Program,
    context: &mut IrContext,
    array: Value,
    size: &Vec<usize>,
    index: &Vec<Value>,
) -> Value {
    let mut elem = array;
    for i in index.iter() {
        elem = new_value_builder(program, context).get_elem_ptr(elem, *i);
        add_value(program, context, elem).unwrap();
    }
    if size.len() == index.len() {
        elem = new_value_builder(program, context).load(elem);
        add_value(program, context, elem).unwrap();
    } else {
        let val_0 = const_int_value(program, context, 0);
        elem = new_value_builder(program, context).get_elem_ptr(elem, val_0);
        add_value(program, context, elem).unwrap();
    }
    elem
}

pub fn get_array_param_elem(
    program: &mut Program,
    context: &mut IrContext,
    array: Value,
    size: &Vec<usize>,
    index: &Vec<Value>,
) -> Value {
    let mut elem = array;
    if !index.is_empty() {
        elem = new_value_builder(program, context).get_ptr(elem, index[0]);
        add_value(program, context, elem).unwrap();
        for i in index.iter().skip(1) {
            elem = new_value_builder(program, context).get_elem_ptr(elem, *i);
            add_value(program, context, elem).unwrap();
        }
    }
    if size.len() + 1 == index.len() {
        elem = new_value_builder(program, context).load(elem);
        add_value(program, context, elem).unwrap();
    } else {
        if index.len() != 0 {
            // elem point to an array
            let val_0 = const_int_value(program, context, 0);
            elem = new_value_builder(program, context).get_elem_ptr(elem, val_0);
            add_value(program, context, elem).unwrap();
        }
    }
    elem
}

// used for store
pub fn get_array_elem_addr(
    program: &mut Program,
    context: &mut IrContext,
    array: Value,
    size: &Vec<usize>,
    index: &Vec<Value>,
) -> Value {
    let mut elem = array;
    for i in index.iter() {
        elem = new_value_builder(program, context).get_elem_ptr(elem, *i);
        add_value(program, context, elem).unwrap();
    }
    elem
}

pub fn get_array_param_elem_addr(
    program: &mut Program,
    context: &mut IrContext,
    array: Value,
    size: &Vec<usize>,
    index: &Vec<Value>,
) -> Value {
    let mut elem = array;
    if !index.is_empty() {
        elem = new_value_builder(program, context).get_ptr(elem, index[0]);
        add_value(program, context, elem).unwrap();
        for i in index.iter().skip(1) {
            elem = new_value_builder(program, context).get_elem_ptr(elem, *i);
            add_value(program, context, elem).unwrap();
        }
    }
    elem
}

#[derive(Clone)]
pub struct Array {
    data: Vec<Value>,
    size: Vec<usize>,
}

impl Array {
    pub fn new(program: &mut Program, context: &mut IrContext, size: &Vec<usize>) -> Self {
        let len = Self::size2len(size);
        let val_0 = const_int_value(program, context, 0);
        let data = vec![val_0; len];
        Self {
            data,
            size: size.clone(),
        }
    }

    pub fn get_mut(&mut self, idx: Vec<usize>) -> &mut Value {
        let pos = self.index2pos(idx);
        self.data.get_mut(pos).unwrap()
    }
    // -------- utils --------
    fn pos2index(&self, pos: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut pos = pos;
        for i in self.size.iter().rev() {
            result.push(pos % i);
            pos /= i;
        }
        result.reverse();
        result
    }
    fn index2pos(&self, idx: Vec<usize>) -> usize {
        let mut result = 0;
        let mut factor = 1;
        for (i, &p) in idx.iter().enumerate().rev() {
            result += p as usize * factor;
            factor *= self.size[i] as usize;
        }
        result
    }

    pub fn const_exp2size(index: &Vec<ConstExp>, context: &IrContext) -> Vec<usize> {
        index
            .iter()
            .map(|i| i.get_const_i32(context).unwrap() as usize)
            .collect()
    }

    pub fn size2len(size: &Vec<usize>) -> usize {
        size.iter().product::<usize>()
    }

    pub fn size2type(size: &Vec<usize>) -> Type {
        let mut ty = Type::get_i32();
        for i in size.iter().rev() {
            ty = Type::get_array(ty, *i as usize);
        }
        ty
    }
    // -------- init --------
    pub fn const_init_to_array(
        &mut self,
        program: &mut Program,
        context: &mut IrContext,
        init_val: &ConstInitVal,
        size: &Vec<usize>,
        start_pos: &mut usize,
    ) {
        match init_val {
            ConstInitVal::ConstExp(_) => unreachable!(),
            ConstInitVal::ConstArray(a) => {
                let init_start_pos = start_pos.clone();
                for v in a.iter() {
                    match v {
                        ConstInitVal::ConstExp(e) => {
                            let val = e.get_const_i32(context).unwrap();
                            let val = const_int_value(program, context, val);
                            *self.data.get_mut(*start_pos as usize).unwrap() = val;
                            *start_pos = *start_pos + 1;
                        }
                        ConstInitVal::ConstArray(_) => {
                            // check current len
                            let mut len = *start_pos - init_start_pos;
                            let mut new_size: Vec<usize> = Vec::new();
                            for dim in size.iter().skip(1).rev() {
                                if len % *dim == 0 {
                                    new_size.insert(0, *dim);
                                    len = len / *dim;
                                } else {
                                    break;
                                }
                            }
                            self.const_init_to_array(program, context, v, &new_size, start_pos);
                        }
                    }
                }
                // fill the rest with 0
                let val_0 = const_int_value(program, context, 0);
                while *start_pos < init_start_pos + size.iter().product::<usize>() {
                    *self.data.get_mut(*start_pos as usize).unwrap() = val_0;
                    *start_pos = *start_pos + 1;
                }
            }
        }
    }
    pub fn init_to_array(
        &mut self,
        program: &mut Program,
        context: &mut IrContext,
        init_val: &InitVal,
        size: &Vec<usize>,
        start_pos: &mut usize,
    ) {
        match init_val {
            InitVal::Exp(_) => unreachable!(),
            InitVal::Array(a) => {
                let init_start_pos = start_pos.clone();
                for v in a.iter() {
                    match v {
                        InitVal::Exp(e) => {
                            let val = e.build_ir(program, context).unwrap();
                            *self.data.get_mut(*start_pos as usize).unwrap() = val;
                            *start_pos = *start_pos + 1;
                        }
                        InitVal::Array(_) => {
                            // check current len
                            let mut len = *start_pos - init_start_pos;
                            let mut new_size: Vec<usize> = Vec::new();
                            for dim in size.iter().skip(1).rev() {
                                if len % *dim == 0 {
                                    new_size.insert(0, *dim);
                                    len = len / *dim;
                                } else {
                                    break;
                                }
                            }
                            self.init_to_array(program, context, v, &new_size, start_pos);
                        }
                    }
                }
                // fill the rest with 0
                let val_0 = const_int_value(program, context, 0);
                while *start_pos < init_start_pos + size.iter().product::<usize>() {
                    *self.data.get_mut(*start_pos as usize).unwrap() = val_0;
                    *start_pos = *start_pos + 1;
                }
            }
        }
    }
    pub fn to_value(&self, program: &mut Program, context: &mut IrContext) -> Value {
        let mut values = Vec::new();
        for &v in self.data.iter() {
            values.push(v);
        }
        for i in (0..self.size.len()).rev() {
            let mut values_new = Vec::new();
            let dim = self.size[i] as usize;
            for j in (0..values.len()).step_by(dim) {
                let val = if context.is_global {
                    program.new_value().aggregate(values[j..j + dim].to_vec())
                } else {
                    new_value_builder(program, context).aggregate(values[j..j + dim].to_vec())
                };
                values_new.push(val);
            }
            values = values_new;
        }
        values[0]
    }

    pub fn init_assign_to_array(
        &self,
        program: &mut Program,
        context: &mut IrContext,
        array: Value,
    ) {
        for i in 0..self.data.len() {
            let index = self
                .pos2index(i)
                .into_iter()
                .map(|v| const_int_value(program, context, v as i32))
                .collect();
            let elem = get_array_elem_addr(program, context, array, &self.size, &index);
            let init_val = self.data[i];
            let store = new_value_builder(program, context).store(init_val, elem);
            add_value(program, context, store).unwrap();
        }
    }

    pub fn get_const_init_array(
        program: &mut Program,
        context: &mut IrContext,
        init_val: &ConstInitVal,
        size: &Vec<usize>,
    ) -> Array {
        let mut start_pos = 0;
        let mut const_init_array = Array::new(program, context, size);
        const_init_array.const_init_to_array(program, context, init_val, size, &mut start_pos);
        const_init_array
    }
    pub fn get_init_array(
        program: &mut Program,
        context: &mut IrContext,
        init_val: &InitVal,
        size: &Vec<usize>,
    ) -> Array {
        let mut start_pos = 0;
        let mut init_array = Array::new(program, context, size);
        init_array.init_to_array(program, context, init_val, size, &mut start_pos);
        init_array
    }
}

// ============ Function utils ============
impl FuncFParam {
    pub fn to_type(&self, program: &mut Program, context: &mut IrContext) -> Type {
        match self {
            FuncFParam::Var(btype, _) => btype.to_type(),
            FuncFParam::Array(btype, _, size) => {
                let size_val = size
                    .iter()
                    .map(|exp| exp.get_const_i32(context).unwrap() as usize)
                    .collect();
                Type::get_pointer(Array::size2type(&size_val))
            }
        }
    }
}
// ============ Symbol Table ============

pub struct SymbolTableStack {
    tables: Vec<HashMap<String, SymbolTableEntry>>,
}

#[derive(Clone)]
pub enum SymbolTableEntry {
    Const(Type, i32),
    Var(Type, Value),
    Array(Type, Value, Vec<usize>),
    ArrayParam(Type, Value, Vec<usize>),
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
    pub fn get_symbol(&self, name: &str) -> (Option<SymbolTableEntry>, usize) {
        for (i, table) in self.tables.iter().rev().enumerate() {
            if let Some(entry) = table.get(name).cloned() {
                return (Some(entry), self.tables.len() - i - 1);
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
    pub fn add_var(&mut self, name: &str, ty: Type, value: Value) {
        self.add_symbol(name, SymbolTableEntry::Var(ty, value));
    }
    pub fn add_const(&mut self, name: &str, ty: Type, value: i32) {
        self.add_symbol(name, SymbolTableEntry::Const(ty, value));
    }
    pub fn add_array(&mut self, name: &str, ty: Type, value: Value, size: Vec<usize>) {
        self.add_symbol(name, SymbolTableEntry::Array(ty, value, size));
    }
    pub fn add_array_param(&mut self, name: &str, ty: Type, value: Value, size: Vec<usize>) {
        self.add_symbol(name, SymbolTableEntry::ArrayParam(ty, value, size));
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
        let count = self.name_map.get_mut(name).unwrap();
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
    pub func_table: HashMap<String, Function>,
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
            func_table: HashMap::new(),
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
