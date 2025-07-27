use interpreter::env::NativeFuncCallErrorType;
use interpreter::{
    env::{Env, ExternalFunction, ExternalGlobal, NativeFuncCallError},
    slow_vm::{LocalValue, Vm},
};
use parser::reader::ValueType;

pub struct HeadlessEnv {}
impl Env for HeadlessEnv {
    fn get_func(env: &str, name: &str) -> Option<ExternalFunction> {
        if env != "env" {
            return None;
        };
        match name {
            "print_string" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32],
                result: vec![],
                id: 0,
            }),

            "print_int" => Some(ExternalFunction {
                params: vec![ValueType::I32],
                result: vec![],
                id: 1,
            }),
            "println_string" => Some(ExternalFunction {
                params: vec![ValueType::I32, ValueType::I32],
                result: vec![],
                id: 2,
            }),
            _ => None,
        }
    }

    fn get_global(env: &str, name: &str) -> Option<ExternalGlobal> {
        None
    }

    fn call(
        &mut self,
        vm: &mut Vm<Self>,
        params: &[LocalValue],
        results: &mut [LocalValue],
        func_id: usize,
    ) -> std::result::Result<(), NativeFuncCallError> {
        match func_id {
            0 => {
                let ptr = params[0].u32();
                let count = params[1].u32();
                let data = vm
                    .get_bytes_from_mem(ptr as usize, count as usize)
                    .map_err(|_| {
                        NativeFuncCallError::new(
                            NativeFuncCallErrorType::InvalidAddressSupplied(ptr as usize),
                            0,
                        )
                    })?;
                let str = str::from_utf8(data).map_err(|_| {
                    NativeFuncCallError::new(NativeFuncCallErrorType::InvalidUTF8, 0)
                })?;
                print!("{str}");
                Ok(())
            }
            1 => Ok(print!("{}", params[0].u32())),
            2 => {
                let ptr = params[0].u32();
                let count = params[1].u32();
                let data = vm
                    .get_bytes_from_mem(ptr as usize, count as usize)
                    .map_err(|_| {
                        NativeFuncCallError::new(
                            NativeFuncCallErrorType::InvalidAddressSupplied(ptr as usize),
                            2,
                        )
                    })?;

                let str = str::from_utf8(data).map_err(|_| {
                    NativeFuncCallError::new(NativeFuncCallErrorType::InvalidUTF8, 2)
                })?;
                println!("{str}");
                Ok(())
            }

            _ => unreachable!(),
        }
    }
}
