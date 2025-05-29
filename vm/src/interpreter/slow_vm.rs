use std::{fmt::Debug, mem::transmute};

use bytemuck::{AnyBitPattern, cast_ref};
use itertools::Itertools;
use smallvec::SmallVec;

const WASM_PAGE_SIZE: usize = 65536;

use crate::{
    parser::{
        self,
        module::DecodedBytecode,
        op::{Blocktype, Memarg, Op},
        types::{ImportDesc, TypeId, ValueType},
    },
    validation::ctrl::JumpTable,
};

use super::{
    env::{ExternalFunctionHandler, Modules},
    stack::StackValue,
};

#[derive(Debug, Clone)]
pub struct ActivationFrame {
    locals_offset: usize,
    func_id: usize,
    arity: usize,
    ip: usize,
    stack_height: usize,
}

pub struct Label {
    stack_height: usize,
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
pub enum InstanceError {
    ImportMissingModule,
    ImportMissingFunction,
    NoCodeInModule,
    NoTypesInModule,
    ImportFunctionTypeDoesNotMatch,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum RuntimeError {
    MemoryAddressOutOfScope,
    UnreachableReached,
    NativeFuncCallError(usize),
}

impl Code {
    pub fn from_module(module: &DecodedBytecode) -> Option<Self> {
        let mut offset: usize = 0;
        let mut code: Vec<Op> = Vec::new();
        let mut functions: Vec<Function> = Vec::new();

        let funcs = module.iter_function_types()?.zip(module.iter_code()?);
        for (t, func) in funcs {
            let locals = func.iter_local_types().collect::<Vec<_>>();
            let ops = func.iter_ops().collect::<Vec<_>>();
            code.extend(ops);

            let entry = Function {
                t,
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
        let val = unsafe { vm.pop_u32() };
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
            LocalValue::I32(val) => Self { i32: val },
            LocalValue::I64(val) => Self { i64: val },
            LocalValue::F32(val) => Self { f32: val },
            LocalValue::F64(val) => Self { f64: val },
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

    pub fn init_from_type_and_val(t: ValueType, val: StackValue) -> Self {
        match t {
            ValueType::I32 => Self::I32(unsafe { val.i32 }),
            ValueType::I64 => Self::I64(unsafe { val.i64 }),
            ValueType::F32 => Self::F32(unsafe { val.f32 }),
            ValueType::F64 => Self::F64(unsafe { val.f64 }),
            ValueType::Funcref => todo!(),
            ValueType::Externref => todo!(),
            ValueType::Vectype => todo!(),
        }
    }
    #[inline]
    pub fn u32(&self) -> u32 {
        let Self::I32(val) = self else { unreachable!() };
        *val
    }
    #[inline]
    pub fn u64(&self) -> u64 {
        let Self::I64(val) = self else { unreachable!() };
        *val
    }
    #[inline]
    pub fn f32(&self) -> f32 {
        let Self::F32(val) = self else { unreachable!() };
        *val
    }

    #[inline]
    pub fn f64(&self) -> f64 {
        let Self::F64(val) = self else { unreachable!() };
        *val
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

impl From<parser::types::Type> for Type {
    fn from(value: parser::types::Type) -> Self {
        let params = value.params.iter().cloned().map(|(t, _)| t).collect();
        let results = value.results.iter().cloned().map(|(t, _)| t).collect();
        Self { params, results }
    }
}

pub enum FunctionType {
    Native(ImportedFunctionInfo),
    Internal(usize),
}

impl FunctionType {
    pub fn get_type<'a>(&self, vm: &'a Vm) -> &'a Type {
        match self {
            FunctionType::Native(info) => &vm.types[info.t],
            FunctionType::Internal(id) => &vm.types[*id],
        }
    }
    pub fn get_type_id(&self) -> usize {
        match self {
            FunctionType::Native(info) => info.t,
            FunctionType::Internal(id) => *id,
        }
    }
}
pub struct ImportedFunctionInfo {
    //TODO: (joh): Referenz auf Hostcode
    module: String,
    name: String,
    t: usize,
    func: ExternalFunctionHandler,
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
    globals: Vec<LocalValue>,
    memory: Vec<u8>,
    jump_table: Vec<JumpTable>,
    //external_func_args: Vec<LocalValue>, //TODO: (joh): Anderer Typ als LocalVal
    start_func_id: Option<usize>,
    functions: Vec<FunctionType>,
}

impl Vm {
    //NOTE: (joh): Vielleicht sollten wir ownership uebernehmen?
    pub fn init_from_bytecode(
        bytecode: &DecodedBytecode,
        jump_table: Vec<JumpTable>,
        env: Modules,
    ) -> Result<Self, InstanceError> {
        //NOTE: (joh): Sollte es moeglich sein ein Modul ohne Code zu erstellen?
        let code = Code::from_module(bytecode).ok_or(InstanceError::NoCodeInModule)?;
        //TODO: (joh): Checke imports/exports

        let inital_memory_pages = bytecode.inital_memory_size(0).unwrap_or(0);
        let memory = vec![0; WASM_PAGE_SIZE];
        let locals = Vec::new();
        let start_func_id = bytecode.start.map(|i| i as usize);
        let types = bytecode
            .iter_types()
            .ok_or(InstanceError::NoCodeInModule)?
            .cloned()
            .map_into::<Type>()
            .collect::<Vec<_>>();

        let mut functions = Vec::new();

        let mut imported_func_count = 0;
        if let Some(imports) = bytecode.iter_imports() {
            for import in imports {
                let module_name = import.ident.module.0;
                let import_name = import.ident.name.0;
                println!("Looking for module: {}", module_name);
                println!("Looking for import: {}", import_name);
                let module = env
                    .get(module_name.as_str())
                    .ok_or(InstanceError::ImportMissingModule)?;

                match import.desc.0 {
                    ImportDesc::TypeIdx(t) => {
                        let func_info = module
                            .functions
                            .get(import_name.as_str())
                            .ok_or(InstanceError::ImportMissingFunction)?;
                        let real_t = types.get(t as usize).unwrap();
                        println!("expecting function: {:?}", real_t);

                        if !real_t.params.eq(&func_info.params)
                            || !real_t.results.eq(&func_info.result)
                        {
                            return Err(InstanceError::ImportFunctionTypeDoesNotMatch);
                        }

                        let info = ImportedFunctionInfo {
                            module: module_name,
                            name: import_name,
                            t: t as usize,
                            func: func_info.handler,
                        };

                        imported_func_count += 1;
                        functions.push(FunctionType::Native(info));
                    }

                    ImportDesc::TableType(limits) => todo!(),
                    ImportDesc::MemType(limits) => todo!(),
                    ImportDesc::GlobalType(global_type) => todo!(),
                }
            }
        }

        functions.extend((0..code.functions.len()).map(|i| FunctionType::Internal(i)));

        let globals = bytecode
            .iter_globals()
            .map(|iter| {
                iter.map(|g| LocalValue::init_from_type(g.t.0.t.0))
                    .collect()
            })
            .unwrap_or(Vec::new());
        let vm = Self {
            globals,
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
            local_offset: 0,
            functions,
        };

        Ok(vm)
    }

    pub fn enter_function(
        &mut self,
        func_id: usize,
        params: &[LocalValue],
    ) -> Result<(), RuntimeError> {
        match &self.functions[func_id] {
            FunctionType::Native(imported_function_info) => {
                println!("native call");
                (imported_function_info.func)(self, params)
                    .map_err(|e| RuntimeError::NativeFuncCallError(e))
            }
            FunctionType::Internal(id) => Ok(self.enter_wasm_function(*id, params)),
        }
    }

    pub fn enter_wasm_function(&mut self, func_id: usize, params: &[LocalValue]) {
        println!("Entering func: {func_id}");
        let func = &self.code.functions[func_id];
        let t = &self.types[func.t as usize];
        println!("expecting t: {:?}", t);
        let locals_offset = self.locals.len();
        //TODO: (joh) Besseres Error Handling falls wir das hier von aussen aufrufen
        if params.len() != t.params.len() {
            panic!("Invalid param count supplied");
        }

        let empty_locals = func
            .locals
            .iter()
            .cloned()
            .map(|t| LocalValue::init_from_type(t));

        let new_locals = params.iter().cloned().chain(empty_locals);

        self.locals.extend(new_locals);
        let ip = func.code_offset;

        let new_frame = ActivationFrame {
            locals_offset,
            func_id,
            arity: t.results.len(),
            ip,
            stack_height: self.value_stack.len(),
        };
        println!("entering: {func_id} with frame {:?}", new_frame);

        self.local_offset = new_frame.locals_offset;
        self.func_id = Some(func_id);
        self.ip = ip;

        self.activation_stack.push(new_frame);
    }

    pub fn leave_wasm_function(&mut self) -> bool {
        debug_assert!(self.activation_stack.len() > 0);
        println!("Leaving wasm function");
        let prev = self.activation_stack.pop().unwrap();
        println!("this frame: {:?}", prev);

        match self.activation_stack.last() {
            Some(frame) => {
                println!("prev frame: {:?}", frame);
                self.func_id = Some(frame.func_id);
                self.ip = frame.ip;
                self.locals.truncate(prev.locals_offset);
                true
            }
            None => false,
        }
    }

    pub fn get_local(&self, id: usize) -> LocalValue {
        self.locals[id + self.local_offset]
    }

    pub fn push_any(&mut self, val: StackValue) {
        self.value_stack.push(val);
    }
    pub fn push_value(&mut self, val: impl Into<StackValue> + Debug) {
        println!("Pushing value: {:?}", val);
        self.value_stack.push(val.into());
    }
    pub fn pop_any(&mut self) -> StackValue {
        println!("pop any");
        self.value_stack.pop().unwrap()
    }

    pub fn discard(&mut self, count: usize) {
        self.value_stack.truncate(self.value_stack.len() - count);
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
        let op = &self.code.instructions[self.ip];
        println!("fetching: {:?}", op);
        op
    }

    pub fn push_label(&mut self, label: Label) {
        self.labels.push(label);
    }

    pub fn exec_local_get(&mut self, id: usize) {
        let local_val = self.locals[self.local_offset + id];
        println!("local get: {:?}", local_val);
        self.push_value(local_val);
        self.ip += 1;
    }

    pub fn exec_global_get(&mut self, id: usize) {
        println!("global get");
        let global_val = self.globals[id];
        self.push_value(global_val);
        self.ip += 1;
    }

    pub fn is_mem_index_valid(&self, n: usize, offset: usize, addr: usize) -> bool {
        self.memory.len() > offset + addr + n
    }

    pub fn try_mem_load_n<const BYTES: usize>(
        &mut self,
        arg: Memarg,
    ) -> Result<[u8; BYTES], RuntimeError> {
        debug_assert!(self.memory.len() > 0);

        let addr = unsafe { self.pop_value::<u32>() as usize };
        let addr_start = addr + arg.offset as usize;
        let range = addr_start..addr + BYTES;

        //Laaangsam...
        Ok(self
            .memory
            .get(range)
            .ok_or(RuntimeError::MemoryAddressOutOfScope)?
            .try_into()
            .unwrap())
    }

    //TODO: (joh): Ein Makro
    pub fn i32_load(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = self.try_mem_load_n::<4>(arg)?;
        self.push_value(u32::from_le_bytes(data));
        self.ip += 1;
        Ok(())
    }
    pub fn i64_load(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = self.try_mem_load_n::<8>(arg)?;
        self.push_value(u64::from_le_bytes(data));
        self.ip += 1;
        Ok(())
    }
    pub fn f32_load(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = self.try_mem_load_n::<4>(arg)?;
        self.push_value(f32::from_le_bytes(data));
        self.ip += 1;
        Ok(())
    }
    pub fn f64_load(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = self.try_mem_load_n::<8>(arg)?;
        self.push_value(f64::from_le_bytes(data));
        self.ip += 1;
        Ok(())
    }

    pub fn try_mem_store<const BYTES: usize>(
        &mut self,
        arg: Memarg,
        data: &[u8; BYTES],
    ) -> Result<(), RuntimeError> {
        debug_assert!(self.memory.len() > 0);

        let addr = unsafe { self.pop_value::<u32>() as usize };
        let addr_start = addr + arg.offset as usize;
        let range = addr_start..addr_start + BYTES;
        self.memory[range].copy_from_slice(data);
        Ok(())
    }
    pub fn i32_store(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = unsafe { self.pop_value::<i32>() };
        let data_buffer = data.to_le_bytes();
        self.try_mem_store::<4>(arg, &data_buffer)?;
        self.ip += 1;
        Ok(())
    }

    pub fn i64_store(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = unsafe { self.pop_value::<i64>() };
        let data_buffer = data.to_le_bytes();
        self.try_mem_store::<8>(arg, &data_buffer)?;
        self.ip += 1;
        Ok(())
    }
    pub fn f32_store(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = unsafe { self.pop_value::<f32>() };
        let data_buffer = data.to_le_bytes();
        self.try_mem_store::<4>(arg, &data_buffer)?;
        self.ip += 1;
        Ok(())
    }
    pub fn f64_store(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
        let data = unsafe { self.pop_value::<f64>() };
        let data_buffer = data.to_le_bytes();
        self.try_mem_store::<8>(arg, &data_buffer)?;
        self.ip += 1;
        Ok(())
    }

    pub fn exec_local_set(&mut self, id: usize) {
        let val = self.value_stack.pop().unwrap();
        let local_val = &mut self.locals[self.local_offset + id];

        unsafe { local_val.set_inner_from_stack_val(val) };
        println!("local set: {:?}", local_val);
        self.ip += 1;
    }

    pub fn exec_global_set(&mut self, id: usize) {
        let val = self.pop_any();
        let global_val = &mut self.globals[id];
        unsafe { global_val.set_inner_from_stack_val(val) };
        self.ip += 1;
    }

    pub fn exec_local_tee(&mut self, id: usize) {
        let val = self.value_stack.last().unwrap();
        let local_val = &mut self.locals[self.local_offset + id];
        unsafe { local_val.set_inner_from_stack_val(*val) };
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
        if self.labels.len() > 0 {
            println!("popping label");
            self.labels.pop();
            self.ip += 1;
            false
        } else {
            if self.activation_stack.len() > 1 {
                println!("end of function");
                self.leave_wasm_function();
                false
            } else {
                true
            }
        }
    }

    pub fn exec_call(&mut self, id: usize) -> Result<(), RuntimeError> {
        println!("calling: {id}");
        let func = &self.functions[id];
        let t_id = func.get_type_id();
        let param_count = self.types[t_id].params.len();
        //TODO: (joh): Collects hier vermeiden

        //Das ist schreklich macht aber den Borrow-Checker happy.
        let in_stack_vals = (0..param_count)
            .map(|_| self.pop_any())
            .collect::<SmallVec<[StackValue; 8]>>();

        let params = &self.types[t_id].params;
        let params: SmallVec<[LocalValue; 8]> = params
            .iter()
            .cloned()
            .zip(in_stack_vals)
            .map(|(param, stack_val)| LocalValue::init_from_type_and_val(param, stack_val))
            .collect();
        println!("params: {:?}", params);

        let this_frame = self.activation_stack.last_mut().unwrap();
        this_frame.ip = self.ip + 1;
        this_frame.stack_height = self.value_stack.len();

        self.enter_function(id, &params)?;
        Ok(())
    }

    pub fn exec_return(&mut self) {
        let current_frame = self.activation_stack.last().cloned().unwrap();

        let return_values = (0..current_frame.arity)
            .map(|_| self.pop_any())
            .collect::<SmallVec<[StackValue; 4]>>();

        self.value_stack.truncate(current_frame.stack_height);
        self.value_stack.extend(return_values.as_slice());
        self.leave_wasm_function();
    }

    pub fn label_from_blocktype(&self, blocktype: &Blocktype) -> Label {
        match blocktype {
            Blocktype::TypeIndex(t_id) => {
                let t = &self.types[*t_id as usize];
                let in_count = t.params.len();
                let stack_height = self.ip - in_count;
                Label { stack_height }
            }
            _ => Label {
                stack_height: self.ip,
            },
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

    pub fn exec_else(&mut self, jmp: isize) {
        self.ip = (jmp + self.ip as isize) as usize;
    }

    pub fn exec_br(&mut self, target: usize, table_index: usize) {
        let table_entry = &self.jump_table[self.func_id.unwrap()];
        let jump = table_entry.get_jump(table_index).unwrap();
        self.ip = (self.ip as isize + jump.delta_ip) as usize;
        //NOTE: (joh): Muessen wir hier die Result Werte vielleicht doch pushen/poppen?

        if target != 0 {
            self.labels.truncate(self.labels.len() - target);
        }
        let target_label = self.labels.pop().unwrap();

        //TODO: (joh): Hilfe
        if jump.out_count > 0 {
            if jump.out_count > 1 {
                todo!()
            } else {
                let result = self.pop_any();
                self.value_stack.truncate(target_label.stack_height);
                self.push_any(result);
            }
        } else {
            self.value_stack.truncate(target_label.stack_height);
        }
    }

    pub fn exec_br_if(&mut self, target: usize, table_index: usize) {
        if unsafe { self.pop_value() } {
            self.exec_br(target, table_index);
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            match self.fetch_instruction() {
                Op::Unreachable => {
                    println!("Reached unreachable!");
                    return Err(RuntimeError::UnreachableReached);
                }
                Op::Nop => {}
                Op::Block(blocktype) => self.exec_block(blocktype.clone()),
                Op::Loop(blocktype) => todo!(),
                Op::If(blocktype, table_entry_id) => {
                    self.exec_if(blocktype.clone(), *table_entry_id)
                }
                Op::Else(jmp) => self.exec_else(*jmp),
                Op::End => {
                    if self.exec_end() {
                        break;
                    }
                }
                Op::Br(target, table_index) => self.exec_br(*target as usize, *table_index),
                Op::BrIf(target, table) => self.exec_br_if(*target as usize, *table),
                Op::Return => self.exec_return(),
                Op::Call(func_id) => self.exec_call(*func_id as usize)?,
                Op::CallIndirect(_, _) => todo!(),
                Op::Drop => {
                    _ = {
                        self.pop_any();
                        self.ip += 1
                    }
                }
                Op::Select(value_type) => todo!(),
                Op::LocalGet(id) => self.exec_local_get(*id as usize),
                Op::LocalSet(id) => self.exec_local_set(*id as usize),
                Op::LocalTee(id) => self.exec_local_tee(*id as usize),
                Op::GlobalGet(id) => self.exec_global_get(*id as usize),
                Op::GlobalSet(id) => self.exec_global_set(*id as usize),
                Op::I32Load(memarg) => self.i32_load(*memarg)?,
                Op::I64Load(memarg) => self.i64_load(*memarg)?,
                Op::F32Load(memarg) => self.f32_load(*memarg)?,
                Op::F64Load(memarg) => self.f64_load(*memarg)?,
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
                Op::I32Store(memarg) => self.i32_store(*memarg)?,
                Op::I64Store(memarg) => self.i64_store(*memarg)?,
                Op::F32Store(memarg) => self.f32_store(*memarg)?,
                Op::F64Store(memarg) => self.f64_store(*memarg)?,
                Op::I32Store8(memarg) => todo!(),
                Op::I32Store16(memarg) => todo!(),
                Op::I64Store8(memarg) => todo!(),
                Op::I64Store16(memarg) => todo!(),
                Op::I64Store32(memarg) => todo!(),
                Op::I32Const(val) => self.exec_push(val.clone()),
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
                Op::I64Geu => self.exec_binop_push(|a: u64, b: u64| a >= b),
                Op::I64Ges => self.exec_binop_push(|a: i64, b: i64| a >= b),
                Op::I32Add => self.exec_binop_push(|a: u32, b: u32| a + b),
                Op::I32Sub => self.exec_binop_push(|a: u32, b: u32| a - b),
                Op::I32Mul => self.exec_binop_push(|a: u32, b: u32| a * b),
                Op::I32Divs => self.exec_binop_push(|a: i32, b: i32| a / b),
                Op::I32Divu => self.exec_binop_push(|a: u32, b: u32| a / b),
                Op::I32Rems => self.exec_binop_push(|a: i32, b: i32| a % b),
                Op::I32Remu => self.exec_binop_push(|a: u32, b: u32| a % b),
                Op::I32And => self.exec_binop_push(|a: u32, b: u32| a & b),
                Op::I32Or => self.exec_binop_push(|a: u32, b: u32| a | b),
                Op::I32Xor => self.exec_binop_push(|a: u32, b: u32| a ^ b),
                Op::I32Shl => self.exec_binop_push(|a: u32, b: u32| a << b),
                Op::I32Shrs => self.exec_binop_push(|a: i32, b: i32| a >> b),
                Op::I32Shru => self.exec_binop_push(|a: u32, b: u32| a >> b),
                Op::I32Rotl => todo!(),
                Op::I32Rotr => todo!(),
                Op::I64Add => self.exec_binop_push(|a: u64, b: u64| a + b),
                Op::I64Sub => self.exec_binop_push(|a: u64, b: u64| a - b),
                Op::I64Mul => self.exec_binop_push(|a: u64, b: u64| a * b),
                Op::I64Divs => self.exec_binop_push(|a: i64, b: i64| a / b),
                Op::I64Divu => self.exec_binop_push(|a: u64, b: u64| a / b),
                Op::I64Rems => self.exec_binop_push(|a: i64, b: i64| a % b),
                Op::I64Remu => self.exec_binop_push(|a: u64, b: u64| a % b),
                Op::I64And => self.exec_binop_push(|a: u64, b: u64| a & b),
                Op::I64Or => self.exec_binop_push(|a: u64, b: u64| a | b),
                Op::I64Xor => self.exec_binop_push(|a: u64, b: u64| a ^ b),
                Op::I64Shl => self.exec_binop_push(|a: u64, b: u64| a << b),
                Op::I64Shrs => self.exec_binop_push(|a: i64, b: i64| a >> b),
                Op::I64Shru => self.exec_binop_push(|a: u64, b: u64| a >> b),
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
    use std::{collections::HashMap, hash::Hash};

    use crate::{
        interpreter::{
            env::{ExternalFunction, Module},
            slow_vm::RuntimeError,
        },
        parser::{
            error::ReaderError, module::DecodedBytecode, op::Op, reader::Reader, types::ValueType,
        },
        validation::{
            error::ValidationError,
            validator::{Context, Validator, patch_jumps},
        },
    };

    use super::{Code, LocalValue, Vm};

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

    fn debug_env_always_fails(vm: &mut Vm, params: &[LocalValue]) -> Result<(), usize> {
        let ret_nr = params[0].u32();
        Err(ret_nr as usize)
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
            vec![ValueType::I32, ValueType::I32]
        );

        assert_eq!(
            code.instructions[code.functions[1].code_offset],
            Op::I32Const(1)
        );

        assert_eq!(code.functions[1].locals, vec![ValueType::I32]);

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
        assert_eq!(code.functions[3].locals, vec![]);

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
        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
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
        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
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
        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
        assert_eq!(vm.run().unwrap_err(), RuntimeError::UnreachableReached);
        Ok(())
    }

    #[test]
    fn run_test_if_else() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (func (local i32 i32)
                    i32.const 0
                    i32.const 1
                    i32.add
                    local.set 0

                    local.get 0
                    (if 
                        (then
                            unreachable
                        )
                        (else
                        )
                    )
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        patch_jumps(&mut module, jumps.iter())?;

        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
        assert_eq!(vm.run().unwrap_err(), RuntimeError::UnreachableReached);
        Ok(())
    }

    #[test]
    fn run_br_simple_no_return() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (func (local i32 i32)
                    (block 
                        i32.const 1 
                        i32.const 2
                        i32.add 
                        i32.const 3
                        i32.eq
                        br_if 0 
                        unreachable 
                    )
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        patch_jumps(&mut module, jumps.iter())?;

        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
        vm.run().unwrap();
        Ok(())
    }

    #[test]
    fn run_block_params() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (func (local i32 i32)
                    i32.const 2
                    (block (param i32)  
                        i32.const 1 
                        i32.add 
                        i32.const 3
                        i32.eq
                        br_if 0 
                        unreachable 
                    )
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        patch_jumps(&mut module, jumps.iter())?;

        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        vm.enter_function(0, &[]);
        vm.run().unwrap();
        Ok(())
    }

    #[test]
    fn run_simple_call() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (func $b (param i32) (result i32)  
                    local.get 0
                    i32.const 1
                    i32.add
                )
                (func $c (param i32) 
                    local.get 0
                    i32.const 4
                    i32.eq
                    (if
                        (then
                            unreachable
                        )
                    )
                )
                (func $a 
                    i32.const 1
                    i32.const 2
                    call $b
                    i32.add
                    call 1
                )
                (start $a)
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jumps = Validator::validate_all(&context)?;
        patch_jumps(&mut module, jumps.iter())?;

        let mut vm = Vm::init_from_bytecode(&module, jumps, HashMap::new()).unwrap();
        println!("code: {:?}", vm.code.instructions);
        let start_func_id = vm.start_func_id.unwrap();
        println!("start: {start_func_id}");
        vm.enter_function(start_func_id, &[]).unwrap();
        assert_eq!(vm.run().unwrap_err(), RuntimeError::UnreachableReached);
        Ok(())
    }

    #[test]
    fn call_simple_native_func() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (import "env" "dbg_fail" (func $fail (param i32)))
                (func $main 
                    i32.const 100
                    call $fail
                )
                (start $main)
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        println!("{:?}", module.types.as_ref().unwrap());
        let jumps = Validator::validate_all(&context)?;
        let dbg_fail_proc = ExternalFunction {
            handler: debug_env_always_fails,
            params: vec![ValueType::I32],
            result: vec![],
        };
        let mut funcs = HashMap::new();
        funcs.insert("dbg_fail", dbg_fail_proc);

        let mut envs = HashMap::new();
        envs.insert("env", Module { functions: funcs });

        let mut vm = Vm::init_from_bytecode(&module, jumps, envs).unwrap();
        vm.enter_function(1, &[]).unwrap();
        assert_eq!(
            vm.run().unwrap_err(),
            RuntimeError::NativeFuncCallError(100)
        );
        Ok(())
    }
    #[test]

    fn run_simple_memory_funcs() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (import "env" "dbg_fail" (func $fail (param i32)))
                (memory 1)
                (func $main 
                    i32.const 10
                    i32.const 50
                    i32.store  
                    i32.const 10
                    i32.load
                    i32.const 50
                    i32.add 
                    call $fail 
                )
                (start $main)
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        println!("{:?}", module.types.as_ref().unwrap());
        let jumps = Validator::validate_all(&context)?;
        let dbg_fail_proc = ExternalFunction {
            handler: debug_env_always_fails,
            params: vec![ValueType::I32],
            result: vec![],
        };
        let mut funcs = HashMap::new();
        funcs.insert("dbg_fail", dbg_fail_proc);

        let mut envs = HashMap::new();
        envs.insert("env", Module { functions: funcs });

        let mut vm = Vm::init_from_bytecode(&module, jumps, envs).unwrap();
        vm.enter_function(1, &[]).unwrap();
        assert_eq!(
            vm.run().unwrap_err(),
            RuntimeError::NativeFuncCallError(100)
        );
        Ok(())
    }

    #[test]
    pub fn run_globals() -> Result<(), InterpreterTestError> {
        let src = r#"
            (module
                (import "env" "dbg_fail" (func $fail (param i32)))
                (global $global_test (mut i32))
                (global $global_test2 (mut i32))

                (func $main 
                    i32.const 10
                    global.set $global_test
                    global.get $global_test
                    call $fail
                )
                (start $main)
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        println!("{:?}", module.types.as_ref().unwrap());
        let jumps = Validator::validate_all(&context)?;
        let dbg_fail_proc = ExternalFunction {
            handler: debug_env_always_fails,
            params: vec![ValueType::I32],
            result: vec![],
        };
        let mut funcs = HashMap::new();
        funcs.insert("dbg_fail", dbg_fail_proc);

        let mut envs = HashMap::new();
        envs.insert("env", Module { functions: funcs });

        let mut vm = Vm::init_from_bytecode(&module, jumps, envs).unwrap();
        vm.enter_function(1, &[]).unwrap();
        assert_eq!(vm.run().unwrap_err(), RuntimeError::NativeFuncCallError(10));
        Ok(())
    }
}
