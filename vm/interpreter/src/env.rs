use core::fmt;
use std::{collections::HashMap, ffi::os_str::Display};

use parser::reader::ValueType;
use thiserror::Error;

use crate::slow_vm::{InstanceError, LocalValue, NativeFunctionInstance, Vm};

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub params: Vec<ValueType>,
    pub result: Vec<ValueType>,
    pub id: usize,
}

#[derive(Debug, Error)]
pub enum NativeFuncCallErrorType {
    #[error("Echo: {0}")]
    Echo(usize),
    #[error("Invalid address: {0}")]
    InvalidAddressSupplied(usize),
    #[error(
        "Memory out of bounds: Addr: {addr}, requested size: {requested_size}, actual: {actual}"
    )]
    MemoryOutOfBounds {
        addr: usize,
        requested_size: usize,
        actual: usize,
    },

    #[error("Invalid UTF-8 Argument")]
    InvalidUTF8,
    #[error("Invalid Console Key")]
    InvalidConsoleKey,
}
#[derive(Debug)]
pub struct NativeFuncCallError {
    t: NativeFuncCallErrorType,
    func_id: usize,
}
impl fmt::Display for NativeFuncCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while calling function: {}: {}",
            self.func_id, self.t
        )
    }
}
impl NativeFuncCallError {
    pub fn new(t: NativeFuncCallErrorType, func_id: usize) -> Self {
        NativeFuncCallError { t, func_id }
    }
}
pub trait Env: Sized {
    fn get_func(env: &str, name: &str) -> Option<ExternalFunction>;
    fn get_global(env: &str, name: &str) -> Option<ExternalGlobal>;
    fn call(
        &mut self,
        vm: &mut Vm<Self>,
        params: &[LocalValue],
        results: &mut [LocalValue],
        func_id: usize,
    ) -> Result<(), NativeFuncCallError>;
}

#[derive(Debug, Clone)]
pub struct ExternalGlobal {
    pub value: LocalValue,
    pub mutable: bool,
}
