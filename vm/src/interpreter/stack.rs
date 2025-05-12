use super::types::Value;

pub struct ValueStack {
    values: Vec<Value> 
}

impl ValueStack {
    pub fn push(&mut self, value: Value) {
        self.values.push(value); 
    }

    pub fn pop(&mut self) -> Value {
        self.values.pop().unwrap()
    }
}


