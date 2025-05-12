use crate::parser::op::Op;

use super::{stack::ValueStack, types::Value};

pub struct ActivationFrame {
    locals_offset: usize,
    func_id: isize,  
    arity: usize,
}

pub struct Function { 
     
}

pub struct Vm {
    value_stack: ValueStack,
    activation_stack: Vec<ActivationFrame>,

    ip: usize,
    func_id: usize,

    code: Vec<Vec<Op>>,
      
}


