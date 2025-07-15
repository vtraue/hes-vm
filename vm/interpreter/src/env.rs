use std::collections::HashMap;

use parser::reader::ValueType;

use crate::slow_vm::{InstanceError, LocalValue, NativeFunctionInstance, Vm};

#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub params: Vec<ValueType>,
    pub result: Vec<ValueType>,
    pub id: usize,
}

pub trait Env: Sized {
    fn get_func(&self, env: &str, name: &str) -> Option<ExternalFunction>;
    fn get_global(&self, env: &str, name: &str) -> Option<ExternalGlobal>;
    fn call(&self, vm: &Vm<Self>, params: &[LocalValue], func_id: usize) -> Result<(), usize>;
}

#[derive(Debug, Clone)]
pub struct ExternalGlobal {
    pub value: LocalValue,
    pub mutable: bool,
}
