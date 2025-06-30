use std::collections::HashMap;

use parser::reader::ValueType;

use crate::slow_vm::{InstanceError, LocalValue, Vm};

pub type ExternalFunctionHandler = fn(&mut Vm, params: &[LocalValue]) -> Result<(), usize>;

#[derive(Debug)]
pub struct ExternalFunction {
    pub handler: ExternalFunctionHandler,

    pub params: Vec<ValueType>,
    pub result: Vec<ValueType>,
}

#[derive(Debug)]
pub struct ExternalGlobal {
    pub value: LocalValue,
    pub mutable: bool,
}

#[derive(Debug, Default)]
pub struct Module<'a> {
    pub functions: HashMap<&'a str, ExternalFunction>,
    pub globals: HashMap<&'a str, ExternalGlobal>,
}

pub type Modules<'a> = HashMap<&'a str, Module<'a>>;

pub fn get_env_func<'a>(
    modules: &'a Modules,
    module_name: impl AsRef<str>,
    name: impl AsRef<str>,
) -> Result<&'a ExternalFunction, InstanceError> {
    let module = modules
        .get(module_name.as_ref())
        .ok_or(InstanceError::ImportModuleNameDoesNotMatch)?;
    module
        .functions
        .get(name.as_ref())
        .ok_or(InstanceError::ImportFunctionNameDoesNotMatch)
}

pub fn get_env_global<'a>(
    modules: &'a Modules,
    module_name: impl AsRef<str>,
    name: impl AsRef<str>,
) -> Result<&'a ExternalGlobal, InstanceError> {
    let module = modules
        .get(module_name.as_ref())
        .ok_or(InstanceError::ImportModuleNameDoesNotMatch)?;
    module
        .globals
        .get(name.as_ref())
        .ok_or(InstanceError::ImportGlobalNameDoesNotMatch)
}
