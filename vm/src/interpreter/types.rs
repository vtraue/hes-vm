use crate::parser::op::Op;

pub struct InternalFunctionInstance {
    type_id: usize,
    code: Vec<Op>
}

//NOTE: (joh):
//Viel besser waere vermutlich ein Stack pro Typ, aber wir machen es erstmal so  

pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

