use std::ops::DerefMut;

use std::slice;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
use thiserror::Error;

use itertools::Itertools;
use parser::{
    info::BytecodeInfo,
    op::{Blocktype, Memarg, Op},
    reader::{Bytecode, BytecodeReader, ValueType},
};
use smallvec::SmallVec;
use validator::validator::{ReadAndValidateError, ValidateResult};

use crate::{
    env::{
        ExternalFunction, ExternalFunctionHandler, Module, Modules, get_env_func, get_env_global,
    },
    stack::StackValue,
};

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("Unable to read bytecode: {0}")]
    ReadError(#[from] ReadAndValidateError),
    #[error("Import module name does not match")]
    ImportModuleNameDoesNotMatch,
    #[error("Import function name does not match")]
    ImportFunctionNameDoesNotMatch,
    #[error("Import global name does not match")]
    ImportGlobalNameDoesNotMatch,
    #[error("{0} is not a valid const op")]
    InvalidConstOp(Op),
    #[error("Expected 1 return value in const expr, got {0}")]
    InvalidReturnCountInConstExpr(usize),
    #[error("Invalid return type in const expr: {0}")]
    InvalidReturnTypeInConstExpr(ValueType),
}
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Memory address out of scope")]
    MemoryAddressOutOfScope,
    #[error("Native function returned error code: {0}")]
    NativeFuncCallError(usize),
    #[error("Unreachable reached")]
    UnreachableReached,
    #[error("No function to execute")]
    NoFunctionToExecute,
    #[error("Tried entering a start function, but module does not define one")]
    UnexpectedNoStartFunction,
    // #[error("Wrong parameter count provided: Got {0}, expected: {1}")]
    // WrongParamCount,
}

#[derive(Debug, Clone)]
pub struct ActivationFrame {
    locals_offset: usize,
    func_id: usize,
    arity: usize,
    ip: usize,
    stack_height: usize,
    label_stack_offset: usize,
}

#[derive(Debug, Clone)]
pub struct Label {
    stack_height: usize,
    out_count: usize,
}

#[derive(Debug, Clone)]
pub struct InternalFunctionInstance {
    locals: Vec<ValueType>,
    code_offset: usize,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}
impl From<parser::reader::Type> for Type {
    fn from(value: parser::reader::Type) -> Self {
        Self {
            params: value.iter_params().cloned().collect(),
            results: value.iter_results().cloned().collect(),
        }
    }
}
impl From<&parser::reader::Type> for Type {
    fn from(value: &parser::reader::Type) -> Self {
        Self {
            params: value.iter_params().cloned().collect(),
            results: value.iter_results().cloned().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeFunctionInstance {
    module: String,
    name: String,
    func: ExternalFunctionHandler,
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Wasm(InternalFunctionInstance),
    Native(NativeFunctionInstance),
}

#[derive(Debug, Clone)]
pub struct Function {
    t: Type,
    kind: FunctionType,
}

#[derive(Debug, Clone)]
pub struct Code {
    instructions: Vec<Op>,
    functions: Vec<Function>,
}

pub trait PopFromValueStack {
    unsafe fn pop(vm: &mut Vm) -> Self;
}

impl PopFromValueStack for bool {
    unsafe fn pop(vm: &mut Vm) -> Self {
        let val = unsafe { vm.pop_u32() };
        val != 0
    }
}

macro_rules! impl_pop_from_value_stack {
    ($t: tt, $func_name: ident) => {
        impl PopFromValueStack for $t {
            unsafe fn pop(vm: &mut Vm) -> Self {
                unsafe { vm.$func_name() }
            }
        }
    };
}

macro_rules! impl_vm_pop {
    ($func_name: ident, $t: tt, $var_name: ident) => {
        impl Vm {
            pub unsafe fn $func_name(&mut self) -> $t {
                let val = unsafe { self.value_stack.pop().unwrap().$var_name };
                bytemuck::cast(val)
            }
        }
        impl_pop_from_value_stack!($t, $func_name);
    };
}

impl_vm_pop!(pop_u32, u32, i32);
impl_vm_pop!(pop_i32, i32, i32);
impl_vm_pop!(pop_u64, u64, i64);
impl_vm_pop!(pop_i64, i64, i64);
impl_vm_pop!(pop_f32, f32, f32);
impl_vm_pop!(pop_f64, f64, f64);

impl Code {
    fn append_internal_code(
        module: &Bytecode,
        linear_code: &mut Vec<Op>,
        t: usize,
        code_id: usize,
        _export_id: Option<usize>,
        code_offset: usize,
    ) -> (Function, usize) {
        let code = module.get_code(code_id).unwrap();
        linear_code.extend(code.iter_ops().map(|o| o.data));
        let ft = InternalFunctionInstance {
            locals: code.iter_locals().collect(),
            code_offset,
        };
        let t = module.get_type(t).unwrap().clone().into();
        (
            Function {
                t,
                kind: FunctionType::Wasm(ft),
            },
            linear_code.len(),
        )
    }

    fn get_function_instances(
        module: &Bytecode,
        info: &BytecodeInfo,
        env: &Modules,
    ) -> Result<(Vec<Function>, Vec<Op>), InstanceError> {
        let mut linear_code: Vec<Op> = Vec::new();
        let mut code_offset: usize = 0;

        Ok((
            info.functions
                .iter()
                .map(|f| -> Result<Function, InstanceError> {
                    match f.t {
                        parser::info::FunctionType::Internal { code_id, export_id } => {
                            let (func, next_code_offset) = Self::append_internal_code(
                                module,
                                &mut linear_code,
                                f.type_id,
                                code_id,
                                export_id,
                                code_offset,
                            );

                            code_offset = next_code_offset;
                            Ok(func)
                        }
                        parser::info::FunctionType::Imported { import_id } => {
                            let import = module.get_import(import_id).unwrap();

                            let module_name = import.get_mod_name();
                            let name = import.get_name();

                            let func = get_env_func(&env, module_name, name)?;

                            Ok(Function {
                                t: Type {
                                    params: func.params.clone(),
                                    results: func.result.clone(),
                                },
                                kind: FunctionType::Native(NativeFunctionInstance {
                                    module: module_name.to_string(),
                                    name: name.to_string(),
                                    func: func.handler,
                                }),
                            })
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            linear_code,
        ))
    }

    pub fn from_module(
        module: &Bytecode,
        info: &BytecodeInfo,
        env: &Modules,
    ) -> Result<Self, InstanceError> {
        let (functions, instructions) = Self::get_function_instances(module, info, &env)?;

        Ok(Self {
            instructions,
            functions,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum LocalValue {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

impl Display for LocalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Finde einen Weg das besser mit den Typen zu printen ohne dass es eklig wird
        match self {
            LocalValue::I32(i) => write!(f, "{}", i),
            LocalValue::I64(i) => write!(f, "{}", i),
            LocalValue::F32(i) => write!(f, "{}", i),
            LocalValue::F64(i) => write!(f, "{}", i),
        }
    }
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
    pub fn get_value_type(&self) -> ValueType {
        match self {
            LocalValue::I32(_) => ValueType::I32,
            LocalValue::I64(_) => ValueType::I64,
            LocalValue::F32(_) => ValueType::F32,
            LocalValue::F64(_) => ValueType::F64,
        }
    }

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
}
macro_rules! impl_local_value_conversion {
    ($($type: ty => $field_name:ident),+$(,)?) => {
        $(impl From<$type> for LocalValue {
            fn from(value: $type) -> Self {
                Self::$field_name(value)
            }
        })+
    };
}
impl_local_value_conversion! {
    u32 => I32,
    u64 => I64,
    f32 => F32,
    f64 => F64
}

macro_rules! impl_local_value_acc {
    ($fn_name: ident, $field_name: ident, $type: tt) => {
        impl LocalValue {
            pub fn $fn_name(&self) -> $type {
                let Self::$field_name(val) = self else {
                    unreachable!()
                };
                bytemuck::cast(*val)
            }
        }
    };
}
impl_local_value_acc!(u32, I32, u32);
impl_local_value_acc!(i32, I32, i32);
impl_local_value_acc!(u64, I64, u64);
impl_local_value_acc!(i64, I64, i64);
impl_local_value_acc!(f32, F32, f32);
impl_local_value_acc!(f64, F64, f64);

#[derive(Debug, Clone)]
pub struct Vm {
    ip: usize,
    value_stack: Vec<StackValue>,
    activation_stack: Vec<ActivationFrame>,
    labels: Vec<Label>,
    types: Option<Vec<Type>>,
    code: Code,
    locals: Vec<LocalValue>,
    globals: Vec<LocalValue>,
    mem: Option<Vec<u8>>,
    start_func_id: Option<usize>,
    local_offset: usize,
    func_id: Option<usize>,
}

impl Vm {
    fn get_global_init_value(
        module: &Bytecode,
        global_id: usize,
    ) -> Result<LocalValue, InstanceError> {
        let global = module.get_global(global_id).unwrap();
        let result_stack = Self::run_const_expr(global.init_expr.data.iter_ops())?;
        if result_stack.len() > 1 {
            Err(InstanceError::InvalidReturnCountInConstExpr(
                result_stack.len(),
            ))
        } else {
            if let Some(res) = result_stack.get(0) {
                let res_type = res.get_value_type();
                if res.get_value_type() == global.value_type() {
                    Ok(res.clone())
                } else {
                    Err(InstanceError::InvalidReturnTypeInConstExpr(res_type))
                }
            } else {
                Ok(LocalValue::init_from_type(global.value_type()))
            }
        }
    }

    pub fn get_global_instances(
        module: &Bytecode,
        info: &BytecodeInfo,
        env: &Modules,
    ) -> Result<Vec<LocalValue>, InstanceError> {
        info.globals
            .iter()
            .map(|g| match g.info {
                parser::info::GlobalInfo::Internal { global_id, .. } => {
                    Self::get_global_init_value(module, global_id)
                }

                parser::info::GlobalInfo::Imported { import_id } => {
                    let import = module.get_import(import_id).unwrap();
                    let module_name = import.get_mod_name();
                    let name = import.get_name();

                    let func = get_env_global(&env, module_name, name)?;
                    Ok(func.value)
                }
            })
            .collect()
    }
    pub fn run_const_expr(
        expr: impl Iterator<Item = Op>,
    ) -> Result<Vec<LocalValue>, InstanceError> {
        let mut stack = Vec::new();
        for op in expr {
            match op {
                Op::I32Const(val) => stack.push(val.into()),
                Op::I64Const(val) => stack.push(val.into()),
                Op::F32Const(val) => stack.push(val.into()),
                Op::F64Const(val) => stack.push(val.into()),
                Op::GlobalGet(_) => todo!(),
                Op::End => break,
                _ => return Err(InstanceError::InvalidConstOp(op)),
            }
        }
        Ok(stack)
    }

    fn make_memory(bytecode: &Bytecode, info: &BytecodeInfo) -> Option<Vec<u8>> {
        info.inital_mem_size_pages().map(|s| vec![0; s])
    }

    fn init(bytecode: &Bytecode, info: &BytecodeInfo, env: Modules) -> Result<Self, InstanceError> {
        let code = Code::from_module(bytecode, info, &env)?;
        let mem = Self::make_memory(bytecode, info);
        let locals = Vec::new();
        let start_func_id = bytecode.start.as_ref().map(|i| i.data as usize);
        let value_stack = Vec::with_capacity(20);
        let globals = Self::get_global_instances(bytecode, info, &env)?;
        let types = bytecode
            .iter_types()
            .map(|i| i.map_into::<Type>().collect());

        Ok(Self {
            types,
            ip: 0,
            globals,
            value_stack,
            code,
            locals,
            mem,
            start_func_id,
            activation_stack: Vec::new(),
            labels: Vec::new(),
            local_offset: 0,
            func_id: None,
        })
    }

    pub fn init_from_validation_result(
        res: &ValidateResult,
        env: Modules,
    ) -> Result<Self, InstanceError> {
        Vm::init(&res.bytecode, &res.info, env)
    }

    fn push_func_locals<'a>(
        &mut self,
        locals: &[ValueType],
        params: impl Iterator<Item = LocalValue>,
    ) -> usize {
        let empty_locals = locals
            .iter()
            .cloned()
            .map(|t| LocalValue::init_from_type(t));

        let new_locals = params.chain(empty_locals);
        let locals_offset = self.locals.len();
        self.locals.extend(new_locals);
        println!("all locals: {:?}", self.locals);
        locals_offset
    }

    fn leave_wasm_function(&mut self) -> bool {
        assert!(self.activation_stack.len() > 0);
        let prev = self.activation_stack.pop().unwrap();
        match self.activation_stack.last() {
            Some(frame) => {
                self.func_id = Some(frame.func_id);
                self.ip = frame.ip;
                self.locals.truncate(prev.locals_offset);
                self.labels.truncate(prev.label_stack_offset);
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

    pub unsafe fn pop_value<T: PopFromValueStack + Debug>(&mut self) -> T {
        unsafe {
            let val = T::pop(self);
            println!("Popping: {:?}", val);
            val
        }
    }

    #[inline]
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
        self.push_value(local_val);
        self.ip += 1;
    }
    pub fn exec_global_get(&mut self, id: usize) {
        let global_val = self.globals[id];
        self.push_value(global_val);
        self.ip += 1;
    }

    pub fn is_mem_index_valid(&self, n: usize, offset: usize, addr: usize) -> bool {
        self.mem.as_ref().unwrap().len() > offset + addr + n
    }

    pub fn exec_local_set(&mut self, id: usize) {
        let val = self.value_stack.pop().unwrap();
        let local_val = &mut self.locals[self.local_offset + id];

        unsafe { local_val.set_inner_from_stack_val(val) };
        dbg!("local set: {:?}", local_val);
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
            let c2 = self.pop_value::<T>();
            let c1 = self.pop_value::<T>();
            let res = op(c1, c2);
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
            dbg!("popping label");
            self.labels.pop();
            self.ip += 1;
            false
        } else {
            if self.activation_stack.len() > 1 {
                self.leave_wasm_function();
                false
            } else {
                true
            }
        }
    }
    fn get_return_frame(&self) -> Option<ActivationFrame> {
        self.activation_stack.last().cloned().map(|mut f| {
            f.ip = self.ip + 1;
            f.stack_height = self.value_stack.len();
            f
        })
    }

    pub fn enter_function(
        &mut self,
        func_id: usize,
        params: impl Iterator<Item = LocalValue>,
    ) -> Result<(), RuntimeError> {
        let next_frame = self.get_return_frame();

        match &self.code.functions[func_id].kind {
            FunctionType::Wasm(internal_function_instance) => {
                if let Some(f) = next_frame {
                    let frame = self.activation_stack.last_mut().unwrap();
                    *frame = f
                }

                dbg!("internal call");

                self.ip = internal_function_instance.code_offset;
                let locals = internal_function_instance.locals.clone();

                let locals_offset = self.push_func_locals(&locals, params);
                let t = &self.code.functions[func_id].t;

                //TODO: (joh:) Checke irgendwo ob die uebergebenen Params passen
                //oder handle Aufrufe ausserhalb von Call woanders
                let arity = t.results.len();

                let new_frame = ActivationFrame {
                    locals_offset,
                    func_id,
                    arity,
                    ip: self.ip,
                    stack_height: self.value_stack.len(),
                    label_stack_offset: self.labels.len(),
                };

                self.local_offset = locals_offset;
                self.activation_stack.push(new_frame);

                self.func_id = Some(func_id);
                Ok(())
            }
            FunctionType::Native(native_function_instance) => {
                let params: SmallVec<[LocalValue; 16]> = params.collect();
                (native_function_instance.func)(self, &params)
                    .map_err(|e| RuntimeError::NativeFuncCallError(e))?;
                self.ip += 1;
                Ok(())
            }
        }
    }
    pub fn pop_type_params<'a>(
        &mut self,
        params: impl IntoIterator<Item = &'a ValueType>,
    ) -> SmallVec<[LocalValue; 16]> {
        params
            .into_iter()
            .cloned()
            .map(|t| LocalValue::init_from_type_and_val(t, self.pop_any()))
            .collect()
    }

    pub fn exec_call(&mut self, id: usize) -> Result<(), RuntimeError> {
        dbg!("calling: {id}");
        let func = &self.code.functions[id];
        let params = &func.t.params.clone(); //TODO: (joh): Ich hasse das

        let params = params
            .into_iter()
            .cloned()
            .map(|t| LocalValue::init_from_type_and_val(t, self.pop_any()))
            .collect::<Vec<_>>();
        println!("call params {:?}", params);
        self.enter_function(id, params.iter().cloned())
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
                let t = &self.types.as_ref().unwrap()[*t_id as usize];
                let in_count = t.params.len();
                let out_count = t.results.len();
                let stack_height = self.value_stack.len() - in_count;
                Label {
                    stack_height,
                    out_count,
                }
            }
            _ => Label {
                stack_height: self.value_stack.len(),
                out_count: 0,
            },
        }
    }

    pub fn exec_block(&mut self, blocktype: Blocktype) {
        self.push_label(self.label_from_blocktype(&blocktype));
        self.ip += 1;
    }

    pub fn exec_loop(&mut self, blocktype: Blocktype) {
        self.push_label(self.label_from_blocktype(&blocktype));
        self.ip += 1;
    }

    pub fn jump(&mut self, jmp: isize) {
        self.ip = (self.ip as isize + jmp) as usize;
    }
    pub fn exec_if(&mut self, jump: isize, blocktype: Blocktype) {
        let cond = unsafe { self.pop_value::<bool>() };
        let label = self.label_from_blocktype(&blocktype);
        if cond {
            self.ip += 1
        } else {
            self.jump(jump);
        }
        self.push_label(label);
    }
    pub fn exec_else(&mut self, jmp: isize) {
        self.ip = (jmp + self.ip as isize) as usize;
    }

    pub fn exec_br(&mut self, target: usize, jmp: isize) {
        if target != 0 {
            self.labels.truncate(self.labels.len() - target);
        }

        let target_label = self.labels.pop().unwrap();
        self.jump(jmp);
        if target_label.out_count > 0 {
            if target_label.out_count > 1 {
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
    pub fn exec_br_if(&mut self, target: usize, jump: isize) {
        if unsafe { self.pop_value() } {
            self.exec_br(target, jump);
        } else {
            self.ip += 1;
        }
    }
    pub fn exec_memory_init(
        &mut self,
        bytecode: &Bytecode,
        data_id: usize,
    ) -> Result<(), RuntimeError> {
        let data_info = bytecode.get_data(data_id).unwrap();
        assert!(data_info.is_passive());
        let source = unsafe { self.pop_i32() } as usize;
        let size = unsafe { self.pop_i32() } as usize;
        let dest = unsafe { self.pop_i32() } as usize;
        let mem = self.mem.as_mut().unwrap();
        let src = data_info.get_data();
        let src_region_size = (source + size);
        let dst_region_size = (dest + size) as usize;
        println!("data: {:?}", data_info.get_data());
        println!("src region: {}, src len: {}", src_region_size, src.len());
        println!("dst region: {}, dst len: {}", dst_region_size, mem.len());
        if src_region_size > src.len() || dst_region_size >= mem.len() {
            return Err(RuntimeError::MemoryAddressOutOfScope);
        };
        let dst_region = &mut mem[dest..dst_region_size];
        let src_region = &data_info.get_data()[source..src_region_size];
        dst_region.clone_from_slice(src_region);
        self.ip += 1;
        println!("memory now: {:?}", dst_region);
        Ok(())
    }

    pub fn exec_op(&mut self, bytecode: &Bytecode) -> Result<bool, RuntimeError> {
        match self.fetch_instruction() {
            Op::Unreachable => {
                dbg!("Unreachable reached");
                return Err(RuntimeError::UnreachableReached);
            }
            Op::Nop => self.ip += 1,
            Op::Block(blocktype) => self.exec_block(blocktype.clone()),
            Op::Loop(blocktype) => self.exec_loop(blocktype.clone()),
            Op::If { bt, jmp } => self.exec_if(*jmp, bt.clone()),
            Op::Else(jmp) => self.exec_else(*jmp),
            Op::End => {
                if self.exec_end() {
                    return Ok(true);
                };
            }
            Op::Br { label, jmp } => self.exec_br(*label, *jmp),
            Op::BrIf { label, jmp } => self.exec_br_if(*label, *jmp),
            Op::Return => self.exec_return(),
            Op::Call(c) => self.exec_call(*c)?,
            Op::CallIndirect { table, type_id } => todo!(),
            Op::Drop => {
                _ = self.pop_any();
                self.ip += 1
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
            Op::MemoryInit { data_id, extra } => self.exec_memory_init(bytecode, *data_id)?,
            Op::I64Rotl => todo!(),
            Op::I64Rotr => todo!(),
            Op::MemoryCopy => todo!(),
            Op::MemoryFill => todo!(),
        };
        Ok(false)
    }
    pub fn enter_start_function(&mut self) -> Result<(), RuntimeError> {
        if let Some(start) = self.start_func_id {
            //TODO: (joh):
            self.enter_function(start, std::iter::empty())
        } else {
            Err(RuntimeError::UnexpectedNoStartFunction)
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode) -> Result<(), RuntimeError> {
        if self.func_id.is_none() {
            Err(RuntimeError::NoFunctionToExecute)
        } else {
            loop {
                let end = self.exec_op(bytecode)?;
                if end {
                    break;
                }
            }
            Ok(())
        }
    }
    pub fn stack_to_local_vals(
        &self,
        result_types: impl Iterator<Item = ValueType>,
    ) -> Vec<LocalValue> {
        self.value_stack
            .iter()
            .zip(result_types)
            .map(|(stack_val, t)| LocalValue::init_from_type_and_val(t, *stack_val))
            .collect()
    }

    pub fn run_func(
        &mut self,
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        func_id: usize,
        params: impl IntoIterator<Item = LocalValue>,
    ) -> Result<Vec<LocalValue>, RuntimeError> {
        self.enter_function(func_id, params.into_iter())?;
        self.run(bytecode)?;
        let func_t = &self.types.as_ref().unwrap()[info.functions[func_id].type_id];
        let res = self.stack_to_local_vals(func_t.results.iter().cloned());
        assert!(res.len() == func_t.results.len());
        Ok(res)
    }
    pub fn get_bytes_from_mem<'a>(
        &'a self,
        addr: usize,
        count: usize,
    ) -> Result<&'a [u8], RuntimeError> {
        let mem = self
            .mem
            .as_ref()
            .ok_or(RuntimeError::MemoryAddressOutOfScope)?;
        if addr + count >= mem.len() {
            Err(RuntimeError::MemoryAddressOutOfScope)
        } else {
            Ok(&mem[addr..addr + count])
        }
    }
}

#[derive(Debug)]
pub struct ExecutionResult {
    validation_result: ValidateResult,
    exec: Vm,
}
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Unable to instantiate bytecode: {0}")]
    InstanceError(#[from] InstanceError),
    #[error("Runtime Error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

//TODO: (joh): Nutzer sollte Funktion und Argumente manuell callen koennen
pub fn run_validation_result(
    res: ValidateResult,
    env: Modules,
) -> Result<ExecutionResult, ExecutionError> {
    let mut vm = Vm::init(&res.bytecode, &res.info, env)?;
    vm.enter_start_function()?;
    vm.run(&res.bytecode)?;
    Ok(ExecutionResult {
        validation_result: res,
        exec: vm,
    })
}

macro_rules! impl_mem_load {
    ($fn_name: ident, $t: tt) => {
        impl Vm {
            fn $fn_name(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
                debug_assert!(self.mem.as_ref().unwrap().len() > 0);
                let addr = unsafe { self.pop_value::<i32>() as usize };
                let addr_start = addr + arg.offset as usize;
                let range = addr_start..addr_start + std::mem::size_of::<$t>();
                let buffer = self
                    .mem
                    .as_ref()
                    .unwrap()
                    .get(range)
                    .ok_or(RuntimeError::MemoryAddressOutOfScope)?;
                println!("data: {:?}", buffer);
                let val: $t = $t::from_le_bytes(buffer.try_into().unwrap());
                println!("val: {:?}", val);

                self.push_value(val);
                self.ip += 1;
                Ok(())
            }
        }
    };
}

impl_mem_load!(i32_load, u32);
impl_mem_load!(i64_load, u64);
impl_mem_load!(f32_load, f32);
impl_mem_load!(f64_load, f64);

macro_rules! impl_mem_store {
    ($fn_name: ident, $t: tt) => {
        impl Vm {
            pub fn $fn_name(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
                let data = unsafe { self.pop_value::<$t>() };
                let data_buffer = data.to_le_bytes();
                let addr = unsafe { self.pop_value::<u32>() as usize };
                let addr_start = addr + arg.offset as usize;
                let range = addr_start..addr_start + std::mem::size_of::<$t>();

                self.mem
                    .as_mut()
                    .unwrap()
                    .get_mut(range)
                    .ok_or(RuntimeError::MemoryAddressOutOfScope)?
                    .copy_from_slice(&data_buffer);
                self.ip += 1;
                Ok(())
            }
        }
    };
}

impl_mem_store!(i32_store, u32);
impl_mem_store!(i64_store, u64);
impl_mem_store!(f32_store, f32);
impl_mem_store!(f64_store, f64);
fn debug_env_always_fails(_vm: &mut Vm, params: &[LocalValue]) -> Result<(), usize> {
    let ret_nr = params[0].u32();
    Err(ret_nr as usize)
}

fn debug_print_u32(_vm: &mut Vm, params: &[LocalValue]) -> Result<(), usize> {
    let arg = params[0].u32();
    print!("{arg}");
    Ok(())
}

fn debug_print_string(vm: &mut Vm, params: &[LocalValue]) -> Result<(), usize> {
    let ptr = params[0].u32();
    let count = params[1].u32();
    let data = vm
        .get_bytes_from_mem(ptr as usize, count as usize)
        .map_err(|_| 1_usize)?;
    let str = str::from_utf8(data).map_err(|_| 2_usize)?;
    print!("{str}");
    Ok(())
}

pub fn make_test_env() -> Modules<'static> {
    let dbg_fail_proc = ExternalFunction {
        handler: debug_env_always_fails,
        params: vec![ValueType::I32],
        result: vec![],
    };

    let mut funcs = HashMap::new();
    funcs.insert("dbg_fail", dbg_fail_proc);
    funcs.insert(
        "dbg_print_u32",
        ExternalFunction {
            handler: debug_print_u32,
            params: vec![ValueType::I32, ValueType::I32],
            result: vec![],
        },
    );
    funcs.insert(
        "dbg_print_string",
        ExternalFunction {
            handler: debug_print_string,
            params: vec![ValueType::I32],
            result: vec![],
        },
    );

    let mut envs = HashMap::new();
    envs.insert(
        "env",
        Module {
            functions: funcs,
            ..Default::default()
        },
    );
    envs
}

mod tests {
    use std::collections::HashMap;

    use parser::reader::ValueType;
    use validator::validator::read_and_validate_wat;

    use crate::{
        env::{ExternalFunction, Module, Modules},
        slow_vm::{RuntimeError, make_test_env},
    };

    use super::{ExecutionError, ExecutionResult, LocalValue, Vm};

    macro_rules! run_code_expect_result {
        ($fn_name: ident, $func_id: literal, $code: expr, $params: expr, $expecting: expr) => {
            #[test]
            fn $fn_name() -> Result<(), ExecutionError> {
                let src = $code;
                let res = read_and_validate_wat(src).unwrap();

                let mut vm = Vm::init_from_validation_result(&res, make_test_env()).unwrap();
                let results = vm
                    .run_func(&res.bytecode, &res.info, $func_id, $params)
                    .unwrap();
                println!("results: {:?}", results);
                assert!(results == $expecting);
                Ok(())
            }
        };
    }
    macro_rules! run_code_expect_failure {
        ($fn_name: ident, $func_id: literal, $code: expr, $params: expr, $expecting: pat) => {
            #[test]
            fn $fn_name() -> Result<(), ExecutionError> {
                let src = $code;
                let res = read_and_validate_wat(src).unwrap();

                let mut vm = Vm::init_from_validation_result(&res, make_test_env()).unwrap();
                let result = vm
                    .run_func(&res.bytecode, &res.info, $func_id, $params)
                    .unwrap_err();
                assert!(matches!(result, $expecting));
                Ok(())
            }
        };
    }

    run_code_expect_result!(
        run_add_numbers,
        0,
        r#"
            (module
                (func (result i32) 
                    i32.const 5
                    i32.const 1
                    i32.add
                )
                (start 0)
            )
        "#,
        vec![],
        vec![LocalValue::I32(6)]
    );

    run_code_expect_result! {
    run_add_locals,
    0,
    r#"
            (module
                (func (result i32)(local i32 i32) 
                    i32.const 1
                    i32.const 2
                    i32.add
                    local.set 0

                    local.get 0
                    i32.const 1
                    i32.add

                )
                (start 0)
            )
        "#,
        vec![],
        vec![LocalValue::I32(4)]
    }

    run_code_expect_result! {
        run_test_if,
        0,
        r#"
            (module
                (func (result i32) (local i32 i32)
                    i32.const 1
                    i32.const 0
                    i32.add
                    local.set 0

                    local.get 0
                    (if 
                        (then
                            i32.const 99 
                            local.set 1
                        )
                    )
                    local.get 1
                )
            )
        "#,
        vec![],
        vec![LocalValue::I32(99)]
    }

    run_code_expect_result! {
        run_br_simple_no_return,
        0,
        r#"
            (module
                (func (result i32) (local i32 i32)
                    (block 
                        i32.const 99
                        local.set 0

                        i32.const 1 
                        i32.const 2
                        i32.add 
                        i32.const 3
                        i32.eq
                        br_if 0 
                        i32.const 0
                        local.set 0
                    )
                    local.get 0
                )
            )
        "#,
        vec![],
        vec![LocalValue::I32(99)]
    }

    run_code_expect_result! {
        run_block_params,
        0,
        r#"
            (module
                (func (result i32) (local i32 i32)
                    i32.const 2
                    (block (param i32)  
                        i32.const 1 
                        i32.add 
                        i32.const 3
                        i32.eq
                        br_if 0 
                        i32.const 99 
                        local.set 0 
                    )
                    local.get 0
                )
            )
        "#,
        vec![],
        vec![LocalValue::I32(0)]
    }
    run_code_expect_result! {
        run_simple_call,
        2,
        r#"
            (module
                (func $b (param i32) (result i32)  
                    local.get 0
                    i32.const 1
                    i32.add
                )
                (func $c (param i32) (result i32) (local i32) 
                    local.get 0
                    i32.const 4
                    i32.eq
                    (if
                        (then
                            i32.const 99
                            local.set 1
                        )
                    )
                    local.get 1
                )
                (func $a (result i32) 
                    i32.const 1
                    i32.const 2
                    call $b
                    i32.add
                    call 1
                )
                (start $a)
            )

        "#,
        vec![],
        vec![LocalValue::I32(99)]
    }

    run_code_expect_failure! {
        call_simple_native_func,
        1,
        r#"
            (module
                (import "env" "dbg_fail" (func $fail (param i32)))
                (func $main 
                    i32.const 100
                    call $fail
                )
                (start $main)
            )
        "#,
        vec![],
        RuntimeError::NativeFuncCallError(100)
    }

    run_code_expect_result! {
        run_simple_memory_funcs,
        0,
        r#"
            (module
                (memory 1)
                (func $main (result i32) 
                    i32.const 10
                    i32.const 50
                    i32.store  

                    i32.const 10
                    i32.const 10
                    i32.load
                    i32.const 50
                    i32.add 
                    i32.store 

                    i32.const 10
                    i32.load 
                )
                (start $main)
            )
        "#,
        vec![],
        vec![LocalValue::I32(100)]
    }

    run_code_expect_result! {
        run_globals,
        0,
        r#"
            (module
                (global $global_test (mut i32))
                (global $global_test2 (mut i32))
                (global $global_test_init (mut i32) (i32.const 900))
                
                (func $main (result i32) 
                    i32.const 10
                    global.set $global_test
                    global.get $global_test
                    global.get $global_test_init 
                    i32.add
                )
                (start $main)
            )
        "#,
        vec![],
        vec![LocalValue::I32(910)]
    }

    run_code_expect_result! {
        simple_loop,
        0,
        r#"
            (module
                (func $main 
                    (result i32)
                    (local $i i32)

                    (loop $l
                        local.get $i
                        i32.const 1
                        i32.add
                        local.set $i
                        local.get $i
                        i32.const 10
                        i32.lt_s
                        br_if $l
                    )
                    local.get $i
                )
                (start $main)
            )
        "#,
        vec![],
        vec![LocalValue::I32(10)]
    }
    run_code_expect_result! {
        jump_arb_label_return,
        0,
        r#"
            (module
                (func $main 
                    (result i32)
                    (local $i i32)
                
                    (block $outer
                        (block $inner
                            i32.const 100
                            local.set 0 
                            br $outer
                        )
                        unreachable
                    )
                    i32.const 10
                )
                (start $main)
            )
        "#,
        vec![],
        vec![LocalValue::I32(10)]
    }
    run_code_expect_result! {
        return_from_function,
        1,
        r#"
            (module
                (func $assert_eq
                    (param i32 i32)
                    local.get 0
                    local.get 1
                    i32.eq
                    (if
                        (then
                            return 
                        )
                        (else
                            unreachable
                        )
                    )
                )
                (func $main
                    i32.const 1
                    i32.const 1
                    (call $assert_eq)
                )
            )
        "#,
        vec![],
        vec![]
    }
    run_code_expect_result! {
        load_static_data,
        1,
         r#"
            (module
                (func $assert_eq
                    (param i32 i32)
                    local.get 0
                    local.get 1
                    i32.eq
                    (if
                        (then
                            return
                        )
                        (else
                            unreachable
                        )
                    )
                )
                (func $main
                    (local $i i32)
                    (i32.const 0)
                    (i32.const 16)
                    (i32.const 0)
                    (memory.init 0)
                    
                    (i32.const 0)
                    (i32.load)
                    (i32.const 0)
                    (call $assert_eq)

                    (i32.const 4)
                    (i32.load)
                    (i32.const 1)
                    (call $assert_eq)

                    (i32.const 8)
                    (i32.load)
                    (i32.const 2)
                    (call $assert_eq)

                    (i32.const 12)
                    (i32.load)
                    (i32.const 3)
                    (call $assert_eq)
                )
                (memory 1)
                (data "\00\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00")
                (start $main)
            )
        "#,
        vec![],
        vec![]
    }
}
