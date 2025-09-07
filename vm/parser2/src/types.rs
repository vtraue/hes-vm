use core::fmt::Display;

use alloc::alloc::Allocator;
use bumpalo::boxed::Box;
use itertools::Itertools;
use parser_derive2::FromBytecode;
use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants, FromRepr};

use crate::{
    op::Op,
    reader::{FromBytecodeIn, WithPosition},
};
type Pos<T> = WithPosition<T>;

#[derive(Display, FromRepr, Debug, PartialEq, PartialOrd, Copy, Clone, Eq)]
#[repr(u8)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    Funcref = 0x70,
    Externref = 0x6F,
    Vectype = 0x7B,
}

impl ValueType {
    pub fn is_num(&self) -> bool {
        match self {
            ValueType::I32 | ValueType::I64 | ValueType::F32 | ValueType::F64 => true,
            _ => false,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self {
            ValueType::Vectype => true,
            _ => false,
        }
    }
    pub fn is_ref(&self) -> bool {
        match self {
            ValueType::Funcref | ValueType::Externref => true,
            _ => false,
        }
    }

    pub fn bit_width(&self) -> Option<usize> {
        match self {
            ValueType::I32 => Some(32),
            ValueType::I64 => Some(64),
            ValueType::F32 => Some(32),
            ValueType::F64 => Some(64),
            ValueType::Funcref => None,
            ValueType::Externref => None,
            ValueType::Vectype => Some(128),
        }
    }
}

#[derive(Display, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TypedValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
macro_rules! typed_val_impl_constr {
    ($fn_name: ident, $var_n: ident, $t: tt) => {
        fn $fn_name(value: $t) -> Self {
            Self::$var_n(value)
        }
    };
}

impl TypedValue {
    typed_val_impl_constr!(i32, I32, i32);
    typed_val_impl_constr!(i64, I64, i64);
    typed_val_impl_constr!(f32, F32, f32);
    typed_val_impl_constr!(f64, F64, f64);
}

#[derive(Display, FromRepr, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Reftype {
    Funcref = 0x70,
    Externref = 0x6F,
}
#[derive(Debug, Default, Clone, FromBytecode)]
pub struct Type<'bump> {
    pub params: &'bump [ValueType],
    pub results: &'bump [ValueType],
}

impl<'bump> Type<'bump> {
    pub fn get_param(&self, id: usize) -> Option<ValueType> {
        self.params.get(id).cloned()
    }
    pub fn get_result(&self, id: usize) -> Option<ValueType> {
        self.results.get(id).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Global {
    pub mutable: bool,
    pub init: TypedValue,
}

impl Display for Global {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            if self.mutable { "mut" } else { "const" },
            self.init
        )
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl Display for Limits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.max {
            Some(max) => write!(f, "{}..{}", self.min, max),
            None => write!(f, "{}..", self.min),
        }
    }
}

#[derive(Debug, PartialEq, Clone, FromBytecode)]
pub struct TableType {
    et: Reftype,
    lim: Limits,
}

#[derive(FromBytecode, Debug)]
pub struct ImportIdent<'src> {
    pub env: &'src str,
    pub name: &'src str,
}

#[derive(FromBytecode, Debug, Clone, PartialEq)]
pub struct RawGlobal {
    pub t: ValueType,
    pub mutable: bool,
}

#[derive(Clone, Debug, Copy, Default, FromBytecode, PartialEq)]
pub struct TypeId(pub usize);

#[derive(EnumCount, EnumDiscriminants, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum RawImportDesc {
    Function(TypeId) = 0,
    TableType(TableType) = 1,
    MemType(Limits) = 2,
    GlobalType(RawGlobal) = 3,
}

#[derive(Debug, FromBytecode)]
pub struct RawImport<'src> {
    pub ident: ImportIdent<'src>,
    pub desc: RawImportDesc,
}

#[derive(Debug)]
pub struct ImportedFunction<'a, 'src> {
    pub t: &'a Type<'a>,
    pub import_info: &'a RawImport<'src>,
}
