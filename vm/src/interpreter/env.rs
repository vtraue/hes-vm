use std::collections::HashMap;

use crate::parser::types::ValueType;

use super::slow_vm::{LocalValue, Vm};

pub type ExternalFunctionHandler = fn(&mut Vm, params: &[LocalValue]) -> Result<(), usize>;

#[derive(Debug)]
pub struct ExternalFunction {
    pub handler: ExternalFunctionHandler,

    pub params: Vec<ValueType>,
    pub result: Vec<ValueType>,
}

#[derive(Debug)]
pub struct Module<'a> {
    pub functions: HashMap<&'a str, ExternalFunction>,
}

pub type Modules<'a> = HashMap<&'a str, Module<'a>>;
