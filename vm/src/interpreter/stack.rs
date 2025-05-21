use bytemuck::cast;


#[derive(Copy, Clone)]
pub union StackValue {
    pub i32: u32,
    pub i64: u64,
    pub f32: f32,
    pub f64: f64,
}

impl From<u32> for StackValue {
    fn from(value: u32) -> Self {
        Self { i32: value }
    }
}
impl From<i32> for StackValue {
    fn from(value: i32) -> Self {
        Self { i32: cast(value) }
    }
}
impl From<u64> for StackValue {
    fn from(value: u64) -> Self {
        Self { i64: value }
    }
}
impl From<i64> for StackValue {
    fn from(value: i64) -> Self {
        Self { i64: cast(value) }
    }
}
impl From<f32> for StackValue {
    fn from(value: f32) -> Self {
        Self { f32: value }
    }
}
impl From<f64> for StackValue {
    fn from(value: f64) -> Self {
        Self { f64: value }
    }
}

impl From<bool> for StackValue {
    fn from(value: bool) -> Self {
        Self { i32: value.into() }
    }
}

impl From<u16> for StackValue {
    fn from(value: u16) -> Self {
        Self { i32: value.into() }
    }
}
