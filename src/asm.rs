pub fn koopa_to_asm(koopa_str: &str) -> String {
    let driver = koopa::front::Driver::from(koopa_str);
    let program = driver.generate_program().unwrap();
    program.to_asm()
}

trait GenerateAsm {
    fn to_asm(&self) -> String;
}

impl GenerateAsm for koopa::ir::Program {
    fn to_asm(&self) -> String {
        let mut result = String::new();
        result += "  .text\n";
        result += "  .globl main\n";
        for &func in self.func_layout() {
            result = result + &self.func(func).to_asm();
        }
        return result;
    }
}

impl GenerateAsm for koopa::ir::FunctionData {
    fn to_asm(&self) -> String {
        let mut result = String::new();
        result += &format!("{}:\n", &self.name()[1..]);
        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                result += &value_to_asm(inst, self);
                // let mut value_asm = String::new();
            }
        }
        return result;
    }
}

fn value_to_asm(value: koopa::ir::Value, func_data: &koopa::ir::FunctionData) -> String {
    use koopa::ir::ValueKind;
    let value_data = func_data.dfg().value(value);
    match value_data.kind() {
        ValueKind::Integer(int) => {
            return format!("  li a0, {}\n", int.value());
        }
        ValueKind::Return(ret) => {
            return value_to_asm(ret.value().unwrap(), func_data)
                + "  ret\n";
        }
        _ => unreachable!(),
    }
}
