use std::{fmt::Debug, mem::transmute};

use bytemuck::cast_ref;
use itertools::Itertools;

const WASM_PAGE_SIZE: usize = 65536;

use crate::{parser::{
    self,
    module::DecodedBytecode,
    op::{Blocktype, Memarg, Op},
    types::{TypeId, ValueType},
}, validation::ctrl::JumpTable};

use super::stack::StackValue;

pub struct ActivationFrame {
    locals_offset: usize,
    func_id: usize,
    arity: usize,
}

pub struct Label {
    stack_height: usize    
}

pub struct Function {
    t: TypeId,
    
    locals: Vec<ValueType>,
    code_offset: usize,
    
}

pub struct Code {
    instructions: Vec<Op>,
    functions: Vec<Function>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum RuntimeError {
    MemoryAddressOutOfScope, 
    UnreachableReached,
}

impl Code {
    pub fn from_module(module: &DecodedBytecode) -> Option<Self> {
        let mut offset: usize = 0;
        let funcs = &module.code.as_ref()?.0;
        let mut code: Vec<Op> = Vec::new();
        let mut functions: Vec<Function> = Vec::new();

        for (i, (func, _)) in funcs.iter().enumerate() {
            let locals = func.iter_local_types().collect::<Vec<_>>();
            let ops = func.iter_ops().collect::<Vec<_>>();
            code.extend(ops);

            let entry = Function {
                t: i as u32,
                locals,
                code_offset: offset,
            };
            offset = code.len();
            functions.push(entry);
        }

        Some(Self {
            instructions: code,
            functions,
        })
    }

}
pub trait PopFromValueStack {
    unsafe fn pop(vm: &mut Vm) -> Self;
}

impl PopFromValueStack for u32 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_u32() }
    }
}
impl PopFromValueStack for u64 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_u64() }
    }
}

impl PopFromValueStack for i64 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_i64() }
    }
}

impl PopFromValueStack for i32 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_i32() }
    }
}

impl PopFromValueStack for f32 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_f32() }
    }
}
impl PopFromValueStack for f64 {
    unsafe fn pop(vm: &mut Vm) -> Self {
        unsafe { vm.pop_f64() }
    }
}

impl PopFromValueStack for bool {
    unsafe fn pop(vm: &mut Vm) -> Self {
        let val = unsafe {vm.pop_u32()};
        val != 0
    }
}
#[derive(Debug, Copy, Clone)]
pub enum LocalValue {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

impl From<LocalValue> for StackValue {
    fn from(value: LocalValue) -> Self {
        match value {
            LocalValue::I32(val) => Self {i32: val},
            LocalValue::I64(val) => Self {i64: val},
            LocalValue::F32(val) => Self {f32: val},
            LocalValue::F64(val) => Self {f64: val},
        }
    }
}
impl LocalValue {
    pub unsafe fn set_inner_from_stack_val(&mut self, val: StackValue) {
        match self {
            LocalValue::I32(v) => *v = unsafe { val.i32 },
            LocalValue::I64(v) => *v = unsafe { val.i64 },
            LocalValue::F32(v) => *v = unsafe { val.f32 },
            LocalValue::F64(v) => *v = unsafe { val.f64 },
        };
    }

    pub fn init_from_type(t: ValueType) -> Self {
        match t {
            ValueType::I32 => Self::I32(0),
            ValueType::I64 => Self::I64(0),
            ValueType::F32 => Self::F32(0.0),
            ValueType::F64 => Self::F64(0.0),
            ValueType::Funcref => todo!(),
            ValueType::Externref => todo!(),
            ValueType::Vectype => todo!(),
        }
    }
}
/*
macro_rules! do_load_op {
    ($t: tt, $n: literal, $arg: ident) => {
        let val = self.pop
    }
}
*/
macro_rules! trap_vm {
    () => {
        self.trap = true;
        self.running = false;
        break;
    };
}

pub struct Type {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl From<parser::types::Type> for Type {
    fn from(value: parser::types::Type) -> Self {
        let params = value.params.iter().cloned().map(|(t, _)| t).collect(); 
        let results = value.params.iter().cloned().map(|(t, _)| t).collect(); 

        Self {
            params,
            results,
        }
    }
}

pub struct Vm {
    value_stack: Vec<StackValue>,
    activation_stack: Vec<ActivationFrame>,
    labels: Vec<Label>,
    types: Vec<Type>,
    ip: usize,
    func_id: Option<usize>,
    local_offset: usize,
    code: Code,
    locals: Vec<LocalValue>,
    memory: Vec<u8>,  
    jump_table: Vec<JumpTable>,

    start_func_id: Option<usize>,
}

impl Vm {
    //NOTE: (joh): Vielleicht sollten wir ownership uebernehmen?
    pub fn init_from_bytecode(bytecode: &DecodedBytecode, jump_table: Vec<JumpTable>) -> Option<Self> {
        //NOTE: (joh): Sollte es moeglich sein ein Modul ohne Code zu erstellen?  
        let code = Code::from_module(bytecode)?;
        //TODO: (joh): Checke imports/exports

        let inital_memory_pages = bytecode.inital_memory_size(0).unwrap_or(0); 
        let memory = Vec::with_capacity(inital_memory_pages * WASM_PAGE_SIZE);
        let locals = Vec::new();
        let start_func_id = bytecode.start.map(|i| i as usize);
        let types = bytecode.iter_types()?.cloned().map_into::<Type>().collect();
                     
        let vm = Self {
            jump_table,
            labels: Vec::new(),
            value_stack: Vec::new(),
            types,
            activation_stack: Vec::new(),
            ip: 0,
            func_id: None,
            code,
            locals,
            memory,
            start_func_id,
            local_offset: 0
        };

        Some(vm)
    }

    pub fn init_function(&mut self, func_id: usize) {
        let func = &self.code.functions[func_id];
        let t = &self.types[func.t as usize];
        let locals_offset = self.locals.len();

        let new_locals = t.params
            .iter()
            .cloned()
            .chain(func.locals.iter().cloned()) 
            .map(|t| LocalValue::init_from_type(t));

        self.locals.extend(new_locals);
        let activ = ActivationFrame {
            locals_offset,
            func_id,
            arity: 0,
        };

        self.activation_stack.push(activ);  
        self.func_id = Some(func_id);
    }
    
    pub fn get_local(&self, id: usize) -> LocalValue {
        self.locals[id + self.local_offset]
    }

    pub fn push_value(&mut self, val: impl Into<StackValue> + Debug) {
        println!("Pushing value: {:?}", val);
        self.value_stack.push(val.into());
    }
    pub fn pop_any(&mut self) -> StackValue {
        println!("pop any");
        self.ip += 1;
        self.value_stack.pop().unwrap()

    }
    pub unsafe fn pop_u32(&mut self) -> u32 {
        unsafe { self.value_stack.pop().unwrap().i32 }
    }
    pub unsafe fn pop_i32(&mut self) -> i32 {
        let val = unsafe { self.value_stack.pop().unwrap().i32 };
        bytemuck::cast(val)
    }
    pub unsafe fn pop_u64(&mut self) -> u64 {
        unsafe { self.value_stack.pop().unwrap().i64 }
    }

    pub unsafe fn pop_i64(&mut self) -> i64 {
        let val = unsafe { self.value_stack.pop().unwrap().i64 };
        bytemuck::cast(val)
    }

    pub unsafe fn pop_f32(&mut self) -> f32 {
        let val = unsafe { self.value_stack.pop().unwrap().f32 };
        bytemuck::cast(val)
    }

    pub unsafe fn pop_f64(&mut self) -> f64 {
        let val = unsafe { self.value_stack.pop().unwrap().f64 };
        bytemuck::cast(val)
    }

    pub unsafe fn pop_value<T: PopFromValueStack + Debug>(&mut self) -> T {
        unsafe { 
            let val = T::pop(self); 
            println!("Popping: {:?}", val);
            val
        }
    }
    
    pub fn fetch_instruction(&self) -> &Op {
        &self.code.instructions[self.ip]
    }

    pub fn push_label(&mut self, label: Label)  {
        self.labels.push(label);
    }

    pub fn exec_local_get(&mut self, id: usize) {
        let local_val = self.locals[self.local_offset + id];
        println!("local get: {:?}", local_val);
        self.push_value(local_val);
        self.ip += 1;
    }

    pub fn is_mem_index_valid(&self, n: usize, offset: usize, addr: usize) -> bool {
        self.memory.len() > offset + addr + n
    }


    pub fn try_mem_load_n<const BYTES: usize>(&mut self, arg: Memarg) -> Result<[u8; BYTES], RuntimeError> {
        debug_assert!(self.memory.len() > 0); 

        let addr = unsafe { self.pop_value::<u32>() as usize}; 
        let addr_start = addr + arg.offset as usize; 
        let range = addr_start .. addr + BYTES;
        
        //Laaangsam...
        Ok(self.memory.get(range).ok_or(RuntimeError::MemoryAddressOutOfScope)?.try_into().unwrap())
    }
        
    pub fn exec_i32_load<const BYTES: usize>(&mut self, arg: Memarg) -> Result<(), RuntimeError>{
        match BYTES {
            1 => {
                let addr = unsafe { self.pop_value::<u32>() as usize}; 
                let data = *self.memory.get(addr + arg.offset as usize).ok_or(RuntimeError::MemoryAddressOutOfScope)? as u32;
                self.push_value(data);
            }
            2 => {
                let data = self.try_mem_load_n::<2>(arg)?;
                self.push_value(u16::from_le_bytes(data));
            }
            4 => {
                let data = self.try_mem_load_n::<4>(arg)?;
                self.push_value(u32::from_le_bytes(data));
            }
            _ => panic!("Unable to load this type!") 
        }
        Ok(())
    }

    pub fn exec_local_set(&mut self, id: usize) {
        let val = self.value_stack.pop().unwrap(); 
        let local_val = &mut self.locals[self.local_offset + id];

        unsafe {local_val.set_inner_from_stack_val(val)};
        println!("local set: {:?}", local_val);
        self.ip += 1;
    }

    pub fn exec_local_tee(&mut self, id: usize) {
        let val = self.value_stack.last().unwrap(); 
        let local_val = &mut self.locals[self.local_offset + id];
        unsafe {local_val.set_inner_from_stack_val(*val)};
        self.ip += 1;
    }
    
    pub fn exec_unop_push<T, F, R>(&mut self, op: F)
    where
        T: PopFromValueStack + Debug,
        R: Into<StackValue> + Debug,
        F: FnOnce(T) -> R,
    {
        debug_assert!(self.value_stack.len() >= 1);
        let res = op(unsafe { self.pop_value::<T>() });
        self.push_value(res);
        self.ip += 1
    }

    pub fn exec_binop_push<T, F, R>(&mut self, op: F)
    where
        T: PopFromValueStack + Debug,
        R: Into<StackValue> + Debug,
        F: FnOnce(T, T) -> R,
    {
        unsafe {
            debug_assert!(self.value_stack.len() >= 1);
            let res = op(self.pop_value::<T>(), self.pop_value::<T>());
            self.push_value(res);
            self.ip += 1;
        }
    }

    pub fn exec_push(&mut self, value: impl Into<StackValue> + Debug) {
        self.push_value(value);
        self.ip += 1;
    }

    pub fn exec_end(&mut self) -> bool {
        if self.activation_stack.len() > 1 {
            todo!();
            false
        } else {
            true 
        }

    }
    /*
    pub fn exec_if(&mut self, blocktype: &Blocktype, table_entry_id: usize) {
        let cond = unsafe { self.pop_value::<bool>() };
        if cond {
        }
    }
    */
    
    pub fn label_from_blocktype(&self, blocktype: &Blocktype) -> Label {
        match blocktype {
            Blocktype::TypeIndex(t_id) => {
                        let t = &self.types[*t_id as usize];
                        let in_count = t.params.len(); 
                        let stack_height = self.ip - in_count;      
                        Label {
                            stack_height,
                        }
                    },
            _ => Label {stack_height: self.ip}
        }

    }

    pub fn exec_block(&mut self, blocktype: Blocktype) {
        self.push_label(self.label_from_blocktype(&blocktype));
        self.ip += 1;
    }
    
    pub fn exec_if(&mut self, blocktype: Blocktype, table_index: usize) {
        let cond = unsafe { self.pop_value::<bool>() };
        let label = self.label_from_blocktype(&blocktype); 
        if cond {
            self.ip += 1
        } else {
            //NOTE: (joh): Laaaaangsam
            let jte = &self.jump_table[self.func_id.unwrap()];
            self.ip = (self.ip as isize + jte.0[table_index].delta_ip) as usize;
        }
        self.push_label(label);
    }


    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.fetch_instruction() {
                Op::Unreachable => {
                    println!("Reached unreachable!");
                    return Err(RuntimeError::UnreachableReached)
                }
                Op::Nop => {}
                Op::Block(blocktype) => self.exec_block(blocktype.clone()),
                Op::Loop(blocktype) => todo!(),
                Op::If(blocktype, table_entry_id) => self.exec_if(blocktype.clone(), *table_entry_id),
                Op::Else => todo!(),
                Op::End => if self.exec_end() {break;},
                Op::Br(_, _) => todo!(),
                Op::BrIf(_, _) => todo!(),
                Op::Return => todo!(),
                Op::Call(_) => todo!(),
                Op::CallIndirect(_, _) => todo!(),
                Op::Drop => _ = self.pop_any(),
                Op::Select(value_type) => todo!(),
                Op::LocalGet(id) => self.exec_local_get(*id as usize),
                Op::LocalSet(id) => self.exec_local_set(*id as usize),
                Op::LocalTee(id) => self.exec_local_tee(*id as usize),
                Op::GlobalGet(_) => todo!(),
                Op::GlobalSet(_) => todo!(),
                Op::I32Load(memarg) => todo!(),
                Op::I64Load(memarg) => todo!(),
                Op::F32Load(memarg) => todo!(),
                Op::F64Load(memarg) => todo!(),
                Op::I32Load8s(memarg) => todo!(),
                Op::I32Load8u(memarg) => todo!(),
                Op::I32Load16s(memarg) => todo!(),
                Op::I32Load16u(memarg) => todo!(),
                Op::I64Load8s(memarg) => todo!(),
                Op::I64Load8u(memarg) => todo!(),
                Op::I64Load16s(memarg) => todo!(),
                Op::I64Load16u(memarg) => todo!(),
                Op::I64Load32s(memarg) => todo!(),
                Op::I64Load32u(memarg) => todo!(),
                Op::I32Store(memarg) => todo!(),
                Op::I64Store(memarg) => todo!(),
                Op::F32Store(memarg) => todo!(),
                Op::F64Store(memarg) => todo!(),
                Op::I32Store8(memarg) => todo!(),
                Op::I32Store16(memarg) => todo!(),
                Op::I64Store8(memarg) => todo!(),
                Op::I64Store16(memarg) => todo!(),
                Op::I64Store32(memarg) => todo!(),
                Op::I32Const(val) => self.exec_push(val.clone()) ,
                Op::I64Const(val) => self.exec_push(val.clone()),
                Op::F32Const(val) => self.exec_push(val.clone()),
                Op::F64Const(val) => self.exec_push(val.clone()),
                Op::I32Eqz => self.exec_unop_push(|val: u32| val == 0),
                Op::I32Eq => self.exec_binop_push(|a: u32, b: u32| a == b),
                Op::I32Ne => self.exec_binop_push(|a: u32, b: u32| a != b),
                Op::I32Lts => self.exec_binop_push(|a: i32, b: i32| a < b),
                Op::I32Ltu => self.exec_binop_push(|a: u32, b: u32| a < b),
                Op::I32Gts => self.exec_binop_push(|a: i32, b: i32| a > b),
                Op::I32Gtu => self.exec_binop_push(|a: u32, b: u32| a > b),
                Op::I32Leu => self.exec_binop_push(|a: u32, b: u32| a <= b),
                Op::I32Les => self.exec_binop_push(|a: i32, b: i32| a <= b),
                Op::I32Geu => self.exec_binop_push(|a: u32, b: u32| a >= b),
                Op::I32Ges => self.exec_binop_push(|a: i32, b: i32| a >= b),
                Op::I64Eqz => self.exec_unop_push(|val: u32| val == 0),
                Op::I64Eq => self.exec_binop_push(|a: u32, b: u32| a == b),
                Op::I64Ne => self.exec_binop_push(|a: u64, b: u64| a != b),
                Op::I64Lts => self.exec_binop_push(|a: i64, b: i64| a < b),
                Op::I64Ltu => self.exec_binop_push(|a: u64, b: u64| a < b),
                Op::I64Gts => self.exec_binop_push(|a: i64, b: i64| a > b),
                Op::I64Gtu => self.exec_binop_push(|a: u64, b: u64| a > b),
                Op::I64Les => self.exec_binop_push(|a: u64, b: u64| a <= b),
                Op::I64Leu => self.exec_binop_push(|a: i64, b: i64| a <= b),
                Op::I64Geu =>   self.exec_binop_push(|a: u64, b: u64| a >= b),
                Op::I64Ges =>   self.exec_binop_push(|a: i64, b: i64| a >= b),
                Op::I32Add =>   self.exec_binop_push(|a: u32, b: u32| a + b),
                Op::I32Sub =>   self.exec_binop_push(|a: u32, b: u32| a - b),
                Op::I32Mul =>   self.exec_binop_push(|a: u32, b: u32| a * b),
                Op::I32Divs =>  self.exec_binop_push(|a: i32, b: i32| a / b),
                Op::I32Divu =>  self.exec_binop_push(|a: u32, b: u32| a / b),
                Op::I32Rems =>  self.exec_binop_push(|a: i32, b: i32| a % b),
                Op::I32Remu =>  self.exec_binop_push(|a: u32, b: u32| a % b),
                Op::I32And =>   self.exec_binop_push(|a: u32, b: u32| a & b),
                Op::I32Or =>    self.exec_binop_push(|a: u32, b: u32| a | b),
                Op::I32Xor =>   self.exec_binop_push(|a: u32, b: u32| a ^ b),
                Op::I32Shl =>   self.exec_binop_push(|a: u32, b: u32| a << b),
                Op::I32Shrs =>  self.exec_binop_push(|a: i32, b: i32| a >> b),
                Op::I32Shru =>  self.exec_binop_push(|a: u32, b: u32| a >> b),
                Op::I32Rotl => todo!(),
                Op::I32Rotr => todo!(),
                Op::I64Add => self.exec_binop_push(|a: u64, b: u64| a + b),
                Op::I64Sub => self.exec_binop_push(|a: u64, b: u64| a - b),
                Op::I64Mul => self.exec_binop_push(|a: u64, b: u64| a * b),
                Op::I64Divs =>self.exec_binop_push(|a: i64, b: i64| a / b),
                Op::I64Divu =>self.exec_binop_push(|a: u64, b: u64| a / b),
                Op::I64Rems =>self.exec_binop_push(|a: i64, b: i64| a % b),
                Op::I64Remu =>self.exec_binop_push(|a: u64, b: u64| a % b),
                Op::I64And => self.exec_binop_push(|a: u64, b: u64| a & b),
                Op::I64Or => self.exec_binop_push(|a:  u64, b: u64| a | b),
                Op::I64Xor => self.exec_binop_push(|a: u64, b: u64| a ^ b),
                Op::I64Shl => self.exec_binop_push(|a: u64, b: u64| a << b),
                Op::I64Shrs =>self.exec_binop_push(|a: i64, b: i64| a >> b),
                Op::I64Shru =>self.exec_binop_push(|a: u64, b: u64| a >> b),
                Op::I64Rotl => todo!(),
                Op::I64Rotr => todo!(),
                Op::MemoryCopy => todo!(),
                Op::MemoryFill => todo!(),
            }
        }
        Ok(())
    }
}


mod tests {
    use crate::{
        interpreter::vm::RuntimeError, parser::{error::ReaderError, module::DecodedBytecode, op::Op, reader::Reader, types::ValueType}, validation::{
            error::ValidationError,
            validator::{Context, Validator},
        }
    };

    use super::{Code, Vm};

    #[derive(Debug)]
    enum InterpreterTestError {
        Validation(ValidationError),
        Parsing(ReaderError),
    }
    impl From<ReaderError> for InterpreterTestError {
        fn from(value: ReaderError) -> Self {
            Self::Parsing(value)
        }
    }

    impl From<ValidationError> for InterpreterTestError {
        fn from(value: ValidationError) -> Self {
            Self::Validation(value)
        }
    }

    #[test]
    fn linear_code_mult_funcs() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (import "console" "log" (func $log (param i32))) 
                (func (param i32) (local i32 i32)
                    i32.const 0
                    call $log
                )
                (func (param i32) (local i32)
                    i32.const 1
                    call $log
                )
                (func (param i32) (local i32 f32 i64)
                    i32.const 2
                    call $log
                )
                (func (param i32) 
                    i32.const 3
                    call $log
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let _ = Validator::validate_all(&context)?;
        let code = Code::from_module(&module).unwrap();

        assert_eq!(
            code.instructions[code.functions[0].code_offset],
            Op::I32Const(0)
        );
        assert_eq!(
            code.functions[0].locals,
            vec![ValueType::I32,ValueType::I32]
        );

        assert_eq!(
            code.instructions[code.functions[1].code_offset],
            Op::I32Const(1)
        );

        assert_eq!(
            code.functions[1].locals,
            vec![ValueType::I32]
        );

        assert_eq!(
            code.instructions[code.functions[2].code_offset],
            Op::I32Const(2)
        );
        assert_eq!(
            code.functions[2].locals,
            vec![ValueType::I32, ValueType::F32, ValueType::I64]
        );

        assert_eq!(
            code.instructions[code.functions[3].code_offset],
            Op::I32Const(3)
        );
        assert_eq!(
            code.functions[3].locals,
            vec![]
        );
         
        Ok(())
    }
    #[test]
    fn run_add_two_numbers() -> Result<(), InterpreterTestError> {
       let src = r#"
            (module
                (func 
                    i32.const 5
                    i32.const 1
                    i32.add
                    drop
                )
            )
        "#; 
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        let mut vm = Vm::init_from_bytecode(&module, jumps).unwrap();
        vm.init_function(0);  
        vm.run().unwrap();
        Ok(())
    }

    #[test]
    fn run_add_locals() -> Result<(), InterpreterTestError> {
       let src = r#"
            (module
                (func (local i32 i32)
                    i32.const 1
                    i32.const 2
                    i32.add
                    local.set 0

                    local.get 0
                    i32.const 1
                    i32.add
                    drop
                )
            )
        "#; 
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        let mut vm = Vm::init_from_bytecode(&module, jumps).unwrap();
        vm.init_function(0);  
        vm.run().unwrap();
        Ok(())
    }

    #[test]
    fn run_test_if() -> Result<(), InterpreterTestError> {
       let src = r#"
            (module
                (func (local i32 i32)
                    i32.const 1
                    i32.const 0
                    i32.add
                    local.set 0

                    local.get 0
                    (if 
                        (then
                            unreachable
                        )
                    )
                )
            )
        "#; 
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        let mut vm = Vm::init_from_bytecode(&module, jumps).unwrap();
        vm.init_function(0);  
        assert_eq!(vm.run().unwrap_err(), RuntimeError::UnreachableReached);
        Ok(())
    }

    #[test]
    fn run_test_if_else() -> Result<(), InterpreterTestError> {
       let src = r#"
            (module
                (func (local i32 i32)
                    i32.const 0
                    i32.const 0
                    i32.add
                    local.set 0

                    local.get 0
                    (if 
                        (then
                              
                        )
                        (else
                            unreachable
                        )
                    )
                )
            )
        "#; 
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        let mut vm = Vm::init_from_bytecode(&module, jumps).unwrap();
        vm.init_function(0);  
        assert_eq!(vm.run().unwrap_err(), RuntimeError::UnreachableReached);
        Ok(())
    }
}
    
