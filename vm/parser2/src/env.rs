/*
use crate::types::{Type, TypedValue};

#[derive(Debug)]
pub struct ExternalFunction<'a> {
    pub t: Type<'a>,
    pub id: usize,
}

#[derive(Debug)]
pub struct ExternalGlobal {
    pub inital_value: TypedValue,
    pub mutable: bool,
}
pub trait Env {
    fn get_func<'a>(env: &str, name: &str) -> Option<ExternalFunction<'a>>;
    fn get_global(env: &str, name: &str) -> Option<ExternalGlobal>;
    fn call(
        &mut self,
        vm: &mut Vm<Self>,
        params: &[LocalValue],
        results: &mut [LocalValue],
        func_id: usize,
    ) -> Result<(), NativeFuncCallError>;
}
*/
