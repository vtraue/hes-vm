use std::marker::PhantomData;
use std::ops::DerefMut;

use parser::reader::{Data, iter_without_position};
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

use crate::env::Env;
use crate::{env::ExternalFunction, stack::StackValue};
const WASM_PAGE_SIZE: usize = 65536;

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
    #[error("Cannot find exported function by name: {0}")]
    UnknownExportedFunc(String), // #[error("Wrong parameter count provided: Got {0}, expected: {1}")]
    #[error("No function set")]
    NoFunctionSet,
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
    id: usize,
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
    unsafe fn pop<E: Env>(vm: &mut Vm<E>) -> Self;
}

impl PopFromValueStack for bool {
    unsafe fn pop<E: Env>(vm: &mut Vm<E>) -> Self {
        let val = unsafe { vm.pop_u32() };
        val != 0
    }
}

macro_rules! impl_pop_from_value_stack {
    ($t: tt, $func_name: ident) => {
        impl PopFromValueStack for $t {
            unsafe fn pop<E: Env>(vm: &mut Vm<E>) -> Self {
                unsafe { vm.$func_name() }
            }
        }
    };
}

macro_rules! impl_vm_pop {
    ($func_name: ident, $t: tt, $var_name: ident) => {
        impl<E: Env> Vm<E> {
            pub unsafe fn $func_name(&mut self) -> $t {
                let val = unsafe { self.value_stack.pop().unwrap_unchecked().$var_name };
                //val as $t
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
        _export_id: Option<&str>,
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

    fn get_function_instances<E: Env>(
        module: &Bytecode,
        info: &BytecodeInfo,
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

                            let func = E::get_func(module_name, name)
                                .ok_or(InstanceError::ImportFunctionNameDoesNotMatch)?;

                            Ok(Function {
                                t: Type {
                                    params: func.params.clone(),
                                    results: func.result.clone(),
                                },
                                kind: FunctionType::Native(NativeFunctionInstance {
                                    module: module_name.to_string(),
                                    name: name.to_string(),
                                    id: func.id,
                                }),
                            })
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            linear_code,
        ))
    }

    pub fn from_module<E: Env>(
        module: &Bytecode,
        info: &BytecodeInfo,
    ) -> Result<Self, InstanceError> {
        let (functions, instructions) = Self::get_function_instances::<E>(module, info)?;

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
                Self::$field_name(bytemuck::cast(value))
            }
        })+
    };
}
impl_local_value_conversion! {
    u32 => I32,
    i32 => I32,
    u64 => I64,
    i64 => I64,
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
                //*val as $type
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

macro_rules! impl_binop_push {
    ($this: ident, $t: tt, $a: ident, $b: ident, $action: expr) => {
        unsafe {
            debug_assert!($this.value_stack.len() >= 1);
            let $b = $this.pop_value::<$t>();
            let $a = $this.pop_value::<$t>();
            let res = $action;
            $this.push_value(res);
            $this.ip += 1;
        }
    };
}

macro_rules! impl_convert {
    ($this: ident, $a: ident, $src_t: tt, $action: expr) => {
        unsafe {
            let $a = $this.pop_value::<$src_t>();
            let res = $action;
            $this.push_value(res);
            $this.ip += 1;
        }
    };
}
#[derive(Debug, Clone)]
pub struct Vm<E: Env> {
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
    _marker: PhantomData<E>,
}

impl<E: Env> Vm<E> {
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

                    let global = E::get_global(module_name, name)
                        .ok_or(InstanceError::ImportGlobalNameDoesNotMatch)?;
                    Ok(global.value)
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

    fn copy_active_mem_section(
        mem: &mut [u8],
        init_expr: impl Iterator<Item = Op>,
        data: &[u8],
    ) -> Result<(), InstanceError> {
        let offset = Self::run_const_expr(init_expr)?;
        assert!(offset.len() == 1);
        let offset = offset[0].u32() as usize;
        assert!(data.len() <= mem.len());

        mem[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn init(bytecode: &Bytecode, info: &BytecodeInfo) -> Result<Vm<E>, InstanceError> {
        let code = Code::from_module::<E>(bytecode, info)?;
        let mut mem = Self::make_memory(bytecode, info);
        let locals = Vec::with_capacity(20);
        let start_func_id = bytecode.start.as_ref().map(|i| i.data as usize);
        let value_stack = Vec::with_capacity(20);
        let globals = Self::get_global_instances(bytecode, info)?;
        let types = bytecode
            .iter_types()
            .map(|i| i.map_into::<Type>().collect());

        if let Some(data) = bytecode.iter_data()
            && let Some(ref mut mem) = mem
        {
            data.filter_map(|d| {
                if let Data::Active { expr, data, .. } = d {
                    Some((expr, data))
                } else {
                    None
                }
            })
            .try_for_each(|(expr, data)| {
                Self::copy_active_mem_section(
                    mem.as_mut_slice(),
                    expr.data.iter().map(|p| p.data),
                    &data.data,
                )
            })?;
        };
        Ok(Vm {
            types,
            ip: 0,
            globals,
            value_stack,
            code,
            locals,
            mem,
            start_func_id,
            activation_stack: Vec::with_capacity(20),
            labels: Vec::with_capacity(20),
            local_offset: 0,
            func_id: None,
            _marker: PhantomData {},
        })
    }

    pub fn init_from_validation_result(res: &ValidateResult) -> Result<Self, InstanceError> {
        Vm::init(&res.bytecode, &res.info)
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
        //println!("all locals: {:?}", self.locals);
        locals_offset
    }

    fn leave_wasm_function(&mut self) -> bool {
        assert!(self.activation_stack.len() > 0);
        //println!("leaving current function");
        let prev = self.activation_stack.pop().unwrap();
        match self.activation_stack.last() {
            Some(frame) => {
                //println!("huh");
                self.func_id = Some(frame.func_id);
                self.ip = frame.ip;
                self.locals.truncate(prev.locals_offset);
                self.labels.truncate(prev.label_stack_offset);
                self.local_offset = frame.locals_offset;
                true
            }
            None => {
                println!("Function is over");
                false
            }
        }
    }
    pub fn get_local(&self, id: usize) -> LocalValue {
        self.locals[id + self.local_offset]
    }

    pub fn push_any(&mut self, val: StackValue) {
        self.value_stack.push(val);
    }

    pub fn push_value(&mut self, val: impl Into<StackValue> + Debug) {
        // println!("Pushing value: {:?}", val);
        self.value_stack.push(val.into());
    }
    pub fn pop_any(&mut self) -> StackValue {
        //println!("pop any");
        self.value_stack.pop().unwrap()
    }

    pub fn discard(&mut self, count: usize) {
        self.value_stack.truncate(self.value_stack.len() - count);
    }

    pub unsafe fn pop_value<T: PopFromValueStack + Debug>(&mut self) -> T {
        unsafe {
            let val = T::pop(self);
            //println!("Popping: {:?}", val);
            val
        }
    }

    #[inline]
    pub fn fetch_instruction(&self) -> &Op {
        let op = &self.code.instructions[self.ip];
        //println!("fetching: {:?}", op);
        op
    }

    pub fn push_label(&mut self, label: Label) {
        self.labels.push(label);
    }
    pub fn exec_local_get(&mut self, id: usize) {
        debug_assert!(self.locals.get(self.local_offset + id).is_some());
        let local_val = self.locals.get(self.local_offset + id).unwrap();
        self.push_value(*local_val);
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
        debug_assert!(self.locals.get(self.local_offset + id).is_some());
        let local_val = self.locals.get_mut(self.local_offset + id).unwrap();

        unsafe { local_val.set_inner_from_stack_val(val) };
        //dbg!("local set: {:?}", local_val);
        self.ip += 1;
    }
    pub fn exec_global_set(&mut self, id: usize) {
        let val = self.pop_any();
        let global_val = &mut self.globals[id];
        unsafe { global_val.set_inner_from_stack_val(val) };
        self.ip += 1;
    }
    pub fn exec_local_tee(&mut self, id: usize) {
        let val = unsafe { self.value_stack.last().unwrap_unchecked() };
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

    #[inline(always)]
    pub fn exec_push(&mut self, value: impl Into<StackValue> + Debug) {
        self.push_value(value);
        self.ip += 1;
    }

    pub fn exec_end(&mut self) -> bool {
        if self.labels.len() > 0 {
            self.labels.pop();
            self.ip += 1;
            false
        } else {
            if self.activation_stack.len() > 1 {
                self.leave_wasm_function();
                false
            } else {
                //println!("Blub");
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

    pub fn enter_native_function(
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
                //println!("internal call");

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
            _ => unreachable!(),
        }
    }
    pub fn enter_function(
        &mut self,
        func_id: usize,
        params: impl Iterator<Item = LocalValue>,
        results: Vec<LocalValue>,
        env: &mut E,
    ) -> Result<(), RuntimeError> {
        let mut res = results;
        let next_frame = self.get_return_frame();

        match &self.code.functions[func_id].kind {
            FunctionType::Wasm(_) => self.enter_native_function(func_id, params),

            FunctionType::Native(native_function_instance) => {
                //println!("native call");
                //TODO: (joh): Mache Fehler teil der Funktion
                let params: SmallVec<[LocalValue; 32]> = params.collect();
                env.call(self, &params, &mut res, native_function_instance.id)
                    .map_err(|e| RuntimeError::NativeFuncCallError(e))?;
                res.iter().for_each(|r| self.push_value(*r));

                self.ip += 1;

                Ok(())
            }
        }
    }
    pub fn pop_type_arams<'a>(
        &mut self,
        params: impl IntoIterator<Item = &'a ValueType>,
    ) -> SmallVec<[LocalValue; 16]> {
        params
            .into_iter()
            .cloned()
            .map(|t| LocalValue::init_from_type_and_val(t, self.pop_any()))
            .collect()
    }

    pub fn exec_call(&mut self, id: usize, env: &mut E) -> Result<(), RuntimeError> {
        //println!("calling: {id}");
        let func = &self.code.functions[id];
        let params = &func.t.params.clone(); //TODO: (joh): Ich hasse das

        let popped = (1..params.len() + 1)
            .rev()
            .map(|i| self.value_stack[self.value_stack.len() - i]);

        let params = params
            .into_iter()
            .cloned()
            .zip(popped)
            .map(|(p, t)| LocalValue::init_from_type_and_val(p, t))
            .collect::<Vec<_>>();
        self.value_stack
            .truncate(self.value_stack.len() - params.len());

        let results = func
            .t
            .results
            .iter()
            .map(|v| LocalValue::init_from_type(*v));

        //println!("call params {:?}", params);
        self.enter_function(id, params.iter().cloned(), results.collect(), env)
    }

    pub fn exec_return(&mut self) -> bool {
        let current_frame = self.activation_stack.last().cloned().unwrap();
        let return_values = (0..current_frame.arity)
            .map(|_| self.pop_any())
            .collect::<SmallVec<[StackValue; 4]>>();
        self.value_stack.truncate(current_frame.stack_height);
        self.value_stack.extend(return_values.as_slice());
        self.leave_wasm_function()
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
        let size = unsafe { self.pop_i32() } as usize;
        let source = unsafe { self.pop_i32() } as usize;
        let dest = unsafe { self.pop_i32() } as usize;
        let mem = self.mem.as_mut().unwrap();
        let src = data_info.get_data();
        let src_region_size = (source + size);
        let dst_region_size = (dest + size) as usize;
        //println!("data: {:?}", data_info.get_data());
        //println!("src region: {}, src len: {}", src_region_size, src.len());
        //println!("dst region: {}, dst len: {}", dst_region_size, mem.len());
        if src_region_size > src.len() || dst_region_size >= mem.len() {
            return Err(RuntimeError::MemoryAddressOutOfScope);
        };
        let dst_region = &mut mem[dest..dst_region_size];
        let src_region = &data_info.get_data()[source..src_region_size];
        dst_region.clone_from_slice(src_region);
        self.ip += 1;
        //println!("memory now: {:?}", dst_region);
        Ok(())
    }

    pub fn exec_memory_grow(&mut self) {
        let grow_by = unsafe { self.pop_u32() as usize } * WASM_PAGE_SIZE;
        let mem = self.mem.as_mut().unwrap();
        let old_size = mem.len();
        mem.resize(old_size + grow_by, 0);
        self.push_value(old_size as u32);
        self.ip += 1;
    }

    pub fn exec_memory_fill(&mut self) {
        let (n, val, dest) = unsafe {
            (
                self.pop_u32() as usize,
                self.pop_u32(),
                self.pop_u32() as usize,
            )
        };
        let mem = self.mem.as_mut().unwrap();
        let region = &mut mem[dest..dest + n];
        region.fill(val as u8);
        self.ip += 1;
    }
    pub fn exec_op(&mut self, bytecode: &Bytecode, env: &mut E) -> Result<bool, RuntimeError> {
        match self.fetch_instruction() {
            Op::Unreachable => {
                //dbg!("Unreachable reached");
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
            Op::Return => {
                if !self.exec_return() {
                    //println!("Done!");
                    return Ok(true);
                }
            }
            Op::Call(c) => self.exec_call(*c, env)?,
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
            Op::I32Load8s(memarg) => self.i32_load8s(*memarg)?,
            Op::I32Load8u(memarg) => self.i32_load8u(*memarg)?,
            Op::I32Load16s(memarg) => self.i32_load16s(*memarg)?,
            Op::I32Load16u(memarg) => self.i32_load16u(*memarg)?,
            Op::I64Load8s(memarg) => self.i64_load8s(*memarg)?,
            Op::I64Load8u(memarg) => self.i64_load8u(*memarg)?,
            Op::I64Load16s(memarg) => self.i64_load16s(*memarg)?,
            Op::I64Load16u(memarg) => self.i64_load16u(*memarg)?,
            Op::I64Load32s(memarg) => self.i64_load32s(*memarg)?,
            Op::I64Load32u(memarg) => self.i64_load32u(*memarg)?,
            Op::I32Store(memarg) => self.i32_store(*memarg)?,
            Op::I64Store(memarg) => self.i64_store(*memarg)?,
            Op::F32Store(memarg) => self.f32_store(*memarg)?,
            Op::F64Store(memarg) => self.f64_store(*memarg)?,
            Op::I32Store8(memarg) => self.i32_store8(*memarg)?,
            Op::I32Store16(memarg) => self.i32_store16(*memarg)?,
            Op::I64Store8(memarg) => self.i64_store8(*memarg)?,
            Op::I64Store16(memarg) => self.i64_store16(*memarg)?,
            Op::I64Store32(memarg) => self.i64_store32(*memarg)?,
            Op::I32Const(val) => self.exec_push(val.clone()),
            Op::I64Const(val) => self.exec_push(val.clone()),
            Op::F32Const(val) => self.exec_push(val.clone()),
            Op::F64Const(val) => self.exec_push(val.clone()),
            Op::I32Eqz => self.exec_unop_push(|val: u32| val == 0),
            Op::I32Eq => impl_binop_push!(self, u32, a, b, a == b),
            Op::I32Ne => impl_binop_push!(self, u32, a, b, a != b),
            Op::I32Lts => impl_binop_push!(self, i32, a, b, a < b),
            Op::I32Ltu => impl_binop_push!(self, u32, a, b, a < b),
            Op::I32Gts => impl_binop_push!(self, i32, a, b, a > b),
            Op::I32Gtu => impl_binop_push!(self, u32, a, b, a > b),
            Op::I32Leu => impl_binop_push!(self, u32, a, b, a <= b),
            Op::I32Les => impl_binop_push!(self, i32, a, b, a <= b),
            Op::I32Geu => impl_binop_push!(self, i32, a, b, a >= b),
            Op::I32Ges => impl_binop_push!(self, i32, a, b, a >= b),
            Op::I64Eqz => self.exec_unop_push(|val: u32| val == 0),
            Op::I64Eq => impl_binop_push!(self, u32, a, b, a == b),
            Op::I64Ne => impl_binop_push!(self, u32, a, b, a != b),
            Op::I64Lts => impl_binop_push!(self, i32, a, b, a < b),
            Op::I64Ltu => impl_binop_push!(self, u32, a, b, a < b),
            Op::I64Gts => impl_binop_push!(self, i32, a, b, a > b),
            Op::I64Gtu => impl_binop_push!(self, u32, a, b, a > b),
            Op::I64Les => impl_binop_push!(self, u32, a, b, a <= b),
            Op::I64Leu => impl_binop_push!(self, i32, a, b, a <= b),
            Op::I64Geu => impl_binop_push!(self, i32, a, b, a >= b),
            Op::I64Ges => impl_binop_push!(self, i32, a, b, a >= b),
            Op::I32Add => self.exec_binop_push(|a: u32, b: u32| a + b),
            Op::I32Sub => self.exec_binop_push(|a: u32, b: u32| a.wrapping_sub(b)),
            Op::I32Mul => self.exec_binop_push(|a: u32, b: u32| a * b),
            Op::I32Divs => impl_binop_push!(self, u32, a, b, a / b),
            Op::I32Divu => impl_binop_push!(self, u32, a, b, a / b),
            Op::I32Rems => self.exec_binop_push(|a: i32, b: i32| a % b),
            Op::I32Remu => self.exec_binop_push(|a: u32, b: u32| a % b),
            Op::I32And => self.exec_binop_push(|a: u32, b: u32| a & b),
            Op::I32Or => self.exec_binop_push(|a: u32, b: u32| a | b),
            Op::I32Xor => self.exec_binop_push(|a: u32, b: u32| a ^ b),
            Op::I32Shl => impl_binop_push!(self, u32, a, b, a.wrapping_shl(b)),
            Op::I32Shrs => self.exec_binop_push(|a: i32, b: i32| a >> b),
            Op::I32Shru => self.exec_binop_push(|a: u32, b: u32| a >> b),
            Op::I32Rotl => todo!(),
            Op::I32Rotr => todo!(),
            Op::I64Add => impl_binop_push!(self, u32, a, b, a + b),
            Op::I64Sub => impl_binop_push!(self, u32, a, b, a - b),
            Op::I64Mul => self.exec_binop_push(|a: u64, b: u64| a * b),
            Op::I64Divs => self.exec_binop_push(|a: i64, b: i64| a / b),
            Op::I64Divu => self.exec_binop_push(|a: u64, b: u64| a / b),
            Op::I64Rems => self.exec_binop_push(|a: i64, b: i64| a % b),
            Op::I64Remu => self.exec_binop_push(|a: u64, b: u64| a % b),
            Op::I64And => impl_binop_push!(self, u32, a, b, a & b),
            Op::I64Or => impl_binop_push!(self, u32, a, b, a | b),
            Op::I64Xor => self.exec_binop_push(|a: u64, b: u64| a ^ b),
            Op::I64Shl => self.exec_binop_push(|a: u64, b: u64| a << b),
            Op::I64Shrs => self.exec_binop_push(|a: i64, b: i64| a >> b),
            Op::I64Shru => self.exec_binop_push(|a: u64, b: u64| a >> b),
            Op::MemoryInit { data_id, .. } => self.exec_memory_init(bytecode, *data_id)?,
            Op::I64Rotl => todo!(),
            Op::I64Rotr => todo!(),
            Op::MemoryCopy => todo!(),
            Op::MemoryFill { .. } => self.exec_memory_fill(),
            Op::MemoryGrow { .. } => self.exec_memory_grow(),
            Op::I32WrapI64 => impl_convert!(self, a, u64, a as u32),
            Op::I64ExtendI32s => impl_convert!(self, a, i32, a as i64),
            Op::I64ExtendI32u => impl_convert!(self, a, u32, a as u64),
        };
        Ok(false)
    }
    pub fn enter_start_function(&mut self, env: &mut E) -> Result<(), RuntimeError> {
        if let Some(start) = self.start_func_id {
            //TODO: (joh):
            self.enter_function(start, std::iter::empty(), vec![], env)
        } else {
            Err(RuntimeError::UnexpectedNoStartFunction)
        }
    }

    pub fn run(&mut self, bytecode: &Bytecode, env: &mut E) -> Result<(), RuntimeError> {
        if self.func_id.is_none() {
            Err(RuntimeError::NoFunctionToExecute)
        } else {
            loop {
                let end = self.exec_op(bytecode, env)?;
                //println!("stack now: {:?}", self.value_stack);
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

    pub fn set_func(
        &mut self,
        func_id: usize,
        params: impl IntoIterator<Item = LocalValue>,
    ) -> Result<(), RuntimeError> {
        self.enter_native_function(func_id, params.into_iter())
    }

    pub fn run_func(
        &mut self,
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        env: &mut E,
    ) -> Result<Vec<LocalValue>, RuntimeError> {
        self.run(bytecode, env)?;
        let res = if let Some(func_id) = self.func_id {
            let func_t = &self.types.as_ref().unwrap()[info.functions[func_id].type_id];
            let res = self.stack_to_local_vals(func_t.results.iter().cloned());
            // println!("res: {:?}", res);
            assert!(res.len() == func_t.results.len());
            Ok(res)
        } else {
            Err(RuntimeError::NoFunctionSet)
        };
        self.reset_state();
        res
    }

    pub fn reset_state(&mut self) {
        self.activation_stack.truncate(0);
        self.value_stack.truncate(0);
        self.labels.truncate(0);
        self.locals.truncate(0);
        self.ip = 0;
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
    pub fn get_bytes_from_mem_mut<'a>(
        &'a mut self,
        addr: usize,
        count: usize,
    ) -> Result<&'a mut [u8], RuntimeError> {
        let mem = self
            .mem
            .as_mut()
            .ok_or(RuntimeError::MemoryAddressOutOfScope)?;
        if addr + count >= mem.len() {
            Err(RuntimeError::MemoryAddressOutOfScope)
        } else {
            Ok(&mut mem[addr..addr + count])
        }
    }
}

#[derive(Debug)]
pub struct ExecutionResult<E: Env> {
    validation_result: ValidateResult,
    exec: Vm<E>,
}
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Unable to instantiate bytecode: {0}")]
    InstanceError(#[from] InstanceError),
    #[error("Runtime Error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

//TODO: (joh): Nutzer sollte Funktion und Argumente manuell callen koennen
pub fn run_validation_result<E: Env>(
    res: ValidateResult,
    env: &mut E,
) -> Result<ExecutionResult<E>, ExecutionError> {
    let mut vm = Vm::init(&res.bytecode, &res.info)?;
    vm.enter_start_function(env)?;
    vm.run(&res.bytecode, env)?;
    Ok(ExecutionResult {
        validation_result: res,
        exec: vm,
    })
}

macro_rules! impl_mem_load {
    ($fn_name: ident, $storage_type: tt, $target_type: tt) => {
        impl<E: Env> Vm<E> {
            fn $fn_name(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
                debug_assert!(self.mem.as_ref().unwrap().len() > 0);

                let addr = unsafe { self.pop_value::<i32>() as usize };
                let addr_start = addr + arg.offset as usize;
                let range = addr_start..addr_start + std::mem::size_of::<$storage_type>();
                let buffer = self
                    .mem
                    .as_ref()
                    .unwrap()
                    .get(range)
                    .ok_or(RuntimeError::MemoryAddressOutOfScope)?;

                let val: $storage_type =
                    $storage_type::from_le_bytes(unsafe { buffer.try_into().unwrap_unchecked() });
                //println!("val: {:?}", val);
                let target: $target_type = val.into();
                self.push_value(target);
                self.ip += 1;
                Ok(())
            }
        }
    };
}

impl_mem_load!(i32_load, u32, u32);
impl_mem_load!(i64_load, u64, u64);

impl_mem_load!(i32_load8s, i8, i32);
impl_mem_load!(i32_load16s, i16, i32);

impl_mem_load!(i32_load8u, u8, u32);
impl_mem_load!(i32_load16u, u16, u32);

impl_mem_load!(i64_load8s, i8, i64);
impl_mem_load!(i64_load16s, i16, i64);
impl_mem_load!(i64_load32s, i32, i64);

impl_mem_load!(i64_load8u, u8, u64);
impl_mem_load!(i64_load16u, u16, u64);
impl_mem_load!(i64_load32u, u32, u64);

impl_mem_load!(f32_load, f32, f32);
impl_mem_load!(f64_load, f64, f64);

macro_rules! impl_mem_store {
    ($fn_name: ident, $pop_type: tt, $real_type: tt) => {
        impl<E: Env> Vm<E> {
            pub fn $fn_name(&mut self, arg: Memarg) -> Result<(), RuntimeError> {
                let raw = unsafe { self.pop_value::<$pop_type>() };
                let data: $real_type = raw as $real_type;
                let data_buffer = data.to_le_bytes();

                let addr = unsafe { self.pop_value::<u32>() as usize };
                let addr_start = addr + arg.offset as usize;
                let range = addr_start..addr_start + std::mem::size_of::<$real_type>();

                let dest = unsafe {
                    self.mem
                        .as_mut()
                        .unwrap_unchecked()
                        .get_mut(range)
                        .ok_or(RuntimeError::MemoryAddressOutOfScope)?
                };

                dest.copy_from_slice(&data_buffer);

                // println!(
                //     "store op: addr: {}, raw: {raw}, data: {:?}, buffer: {:?}",
                //     addr, data, data_buffer
                // );
                // println!("mem: {:?}", dest);

                self.ip += 1;

                Ok(())
            }
        }
    };
}

impl_mem_store!(i32_store, u32, u32);

impl_mem_store!(i32_store8, u32, u8);
impl_mem_store!(i32_store16, u32, u16);
impl_mem_store!(i64_store, u64, u64);
impl_mem_store!(i64_store8, u64, u8);
impl_mem_store!(i64_store16, u64, u16);
impl_mem_store!(i64_store32, u64, u32);
impl_mem_store!(f32_store, f32, f32);
impl_mem_store!(f64_store, f64, f64);

pub struct DebugEnv {}

impl Env for DebugEnv {
    fn get_func(env: &str, name: &str) -> Option<ExternalFunction> {
        if env != "env" {
            return None;
        };
        match name {
            "dbg_fail" => Some(ExternalFunction {
                params: vec![ValueType::I32],
                result: vec![],
                id: 0,
            }),
            "dbg_print_u32" => Some(ExternalFunction {
                params: vec![ValueType::I32],
                result: vec![],
                id: 1,
            }),
            "dbg_print_string" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32],
                result: vec![],
                id: 2,
            }),
            _ => None,
        }
    }

    fn get_global(env: &str, name: &str) -> Option<crate::env::ExternalGlobal> {
        None
    }

    fn call(
        &mut self,
        vm: &mut Vm<Self>,
        params: &[LocalValue],
        _results: &mut [LocalValue],
        func_id: usize,
    ) -> Result<(), usize> {
        match func_id {
            0 => Err(params[0].u32() as usize),
            1 => Ok(println!("{}", params[0].u32())),
            2 => {
                let ptr = params[0].u32();
                let count = params[1].u32();
                let data = vm
                    .get_bytes_from_mem(ptr as usize, count as usize)
                    .map_err(|_| 1_usize)?;
                let str = str::from_utf8(data).map_err(|_| 2_usize)?;
                print!("{str}");
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

mod tests {
    use std::collections::HashMap;

    use parser::reader::ValueType;
    use validator::validator::read_and_validate_wat;

    use crate::{env::ExternalFunction, slow_vm::RuntimeError};

    use super::{DebugEnv, ExecutionError, ExecutionResult, LocalValue, Vm};

    macro_rules! run_code_expect_result {
        ($fn_name: ident, $func_id: literal, $code: expr, $params: expr, $expecting: expr) => {
            #[test]
            fn $fn_name() -> Result<(), ExecutionError> {
                let src = $code;
                let res = read_and_validate_wat(src).unwrap();

                let mut env = DebugEnv {};
                let mut vm = Vm::init_from_validation_result(&res).unwrap();
                vm.set_func($func_id, $params).unwrap();

                let results = vm.run_func(&res.bytecode, &res.info, &mut env).unwrap();
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
                let mut env = DebugEnv {};
                let mut vm = Vm::init_from_validation_result(&res).unwrap();
                vm.set_func($func_id, $params).unwrap();
                let result = vm.run_func(&res.bytecode, &res.info, &mut env).unwrap_err();
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
                (func $a (param i32 i32)(result i32)
                    (local.get 0)
                    (local.get 1)
                    (i32.add)
                )
                (func $b (param i32) (result i32)
                    (local.get 0)
                )
                (func $c (result i32)
                    (i32.const 96)
                    (i32.const 2)
                    (call $a)
                    (call $b)
                    (i32.const 2)
                    i32.add
                    (call $b)
                )
            )

        "#,
        vec![],
        vec![LocalValue::I32(100)]
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
    run_code_expect_result! {
        load_static_data8,
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
                    (i32.const 4)
                    (i32.const 0)
                    (memory.init 0)
                    
                    (i32.const 0)
                    (i32.load8_u)
                    (i32.const 0)
                    (call $assert_eq)

                    (i32.const 1)
                    (i32.load8_u)
                    (i32.const 1)
                    (call $assert_eq)

                    (i32.const 2)
                    (i32.load8_u)
                    (i32.const 2)
                    (call $assert_eq)

                    (i32.const 3)
                    (i32.load8_u)
                    (i32.const 3)
                    (call $assert_eq)
                )
                (memory 1)
                (data "\00\01\02\03")
                (start $main)
            )
        "#,
        vec![],
        vec![]
    }
    run_code_expect_result! {
        load_static8_2,
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
                    (i32.const 4)
                    (i32.const 0)
                    (memory.init 0)
                    
                    (i32.const 0)
                    (i32.load8_u)
                    (i32.const 104)
                    (call $assert_eq)

                    (i32.const 1)
                    (i32.load8_u)
                    (i32.const 97)
                    (call $assert_eq)

                    (i32.const 2)
                    (i32.load8_u)
                    (i32.const 108)
                    (call $assert_eq)

                    (i32.const 3)
                    (i32.load8_u)
                    (i32.const 108)
                    (call $assert_eq)
                )
                (memory 1)
                (data "hallo")
                (start $main)
            )
        "#,
        vec![],
        vec![]
    }
    run_code_expect_result! {
        load_store_sizes,
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
                    (i32.const 100)
                    (i32.store8)

                    (i32.const 1)
                    (i32.const 100)
                    (i32.store8)

                    (i32.const 0)
                    (i32.load16_u)
                    (i32.const 25700)
                    (call $assert_eq)

                    (i32.const 2)
                    (i32.const 50)
                    (i32.store8)

                    (i32.const 3)
                    (i32.const 50)
                    (i32.store8)

                    (i32.const 0)
                    (i32.load)
                    (i32.const 842163300)
                    (call $assert_eq)
                    
                )
                (memory 1)
                (data "hallo")
                (start $main)
            )
        "#,
        vec![],
        vec![]
    }
}
