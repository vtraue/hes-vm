use core::fmt;

use bytemuck::cast;

#[derive(Copy, Clone)]
pub union StackValue {
    pub i32: u32,
    pub i64: u64,
    pub f32: f32,
    pub f64: f64,
}
impl fmt::Debug for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyStackValue")
    }
}
macro_rules! impl_from_num_stackval {
    ($field_name: ident, $type: tt) => {
        impl From<$type> for StackValue {
            fn from(value: $type) -> Self {
                Self {
                    $field_name: cast(value),
                }
            }
        }
    };
}

impl_from_num_stackval!(i32, u32);
impl_from_num_stackval!(i32, i32);
impl_from_num_stackval!(i64, u64);
impl_from_num_stackval!(i64, i64);
impl_from_num_stackval!(f32, f32);
impl_from_num_stackval!(f64, f64);

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
