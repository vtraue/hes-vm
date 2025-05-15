
use crate::parser::{self, module::DecodedBytecode, op::Op, types::{Type, TypeId, ValueType}};

use super::{stack::ValueStack, types::Value};

pub struct ActivationFrame {
    locals_offset: usize,
    func_id: isize,  
    arity: usize,
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

impl Code {
    pub fn from_module(module: &DecodedBytecode) -> Option<Self> {
        let mut offset: usize = 0;
        let funcs = &module.code.as_ref()?.0;
        let mut code: Vec<Op> = Vec::new();
        let mut functions: Vec<Function> = Vec::new();

        for (i, (func, _,)) in funcs.iter().enumerate() {
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
        };
        Some(Self {
            instructions: code,
            functions,
        })
    }


}

pub struct Vm {
    value_stack: ValueStack,
    activation_stack: Vec<ActivationFrame>,

    ip: usize,
    func_id: usize,
    local_offset: usize,
    code: Code,
    locals: Vec<Value>,  
}

impl Vm {
    pub fn fetch_instruction(&mut self) -> &Op {
        &self.code.instructions[self.ip]
    }

}

mod tests {
    use crate::{parser::{error::ReaderError, module::DecodedBytecode, op::Op, reader::Reader}, validation::{error::ValidationError, validator::{Context, Validator}}};

    use super::Code;

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
                (func (param i32)
                    i32.const 0
                    call $log
                )
                (func (param i32)
                    i32.const 1
                    call $log
                )
                (func (param i32)
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
        assert_eq!(code.instructions[code.functions[0].code_offset], Op::I32Const(0));        
        assert_eq!(code.instructions[code.functions[1].code_offset], Op::I32Const(1));        
        assert_eq!(code.instructions[code.functions[2].code_offset], Op::I32Const(2));        
        assert_eq!(code.instructions[code.functions[3].code_offset], Op::I32Const(3));        

        Ok(())
    }
}
