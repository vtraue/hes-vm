use core::fmt;
<<<<<<< HEAD
use std::fmt::write;
=======
use std::fmt::{Display, write};
>>>>>>> main

use itertools::Itertools;

use crate::{op::Op, reader::{FromReader, FunctionType, Position, Reader, ReaderError, ValueType}};

#[derive(Debug, Clone)]
pub struct Type {
    pub params: Box<[(ValueType, Position)]>,
    pub results: Box<[(ValueType, Position)]>,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.params.iter().map(|(v, p)| v);
        let r = self.results.iter().map(|(v, p)| v);

        write!(f, "({}) -> ({})", p.format(", "), r.format(", "))
    }
}
impl<'src> TryFrom<FunctionType<'src>> for Type {
    type Error = ReaderError;
    fn try_from(mut value: FunctionType<'src>) -> Result<Self, Self::Error> {
        //TODO: (joh): Falls der Buffer vorzeitig leer ist, wird das hier ein Problem sein
        let params = value
            .params
            .iter_with_position()
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        let results = value
            .results
            .iter_with_position()
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok(Self { params, results })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub struct Limits {
    pub min: (u32, Position),
    pub max: Option<(u32, Position)>,
}
impl Limits {
    pub fn in_range(&self, i: i32) -> bool {
        if self.min.0 as i32 > i {
            return false;
        }
        if let Some(max) = self.max {
            if i > max.0 as i32 || max.0 < self.min.0 {
                return false;
            }
        }
        true
    }
}

impl<'src> fmt::Display for Limits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.max {
            Some((m, _)) => write!(f, "({}..{})", self.min.0, m),
            None => write!(f, "({}..)", self.min.0),
        }
    }
}
#[derive(Debug)]
pub struct Global {
    pub t: (GlobalType, Position),
    pub init_expr: Box<[(Op, Position)]>,
}

impl<'src> fmt::Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = {}",
            self.t.0,
            self.init_expr.iter().map(|v| v.0).format(" ,")
        )
    }
}
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub struct GlobalType {
    pub t: (ValueType, Position),
    pub mutable: (bool, Position),
}

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut_str = if self.mutable.0 { "mut" } else { "" };
        write!(f, "{} {}", mut_str, self.t.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Locals {
    pub n: u32,
    pub t: ValueType,
}

impl IntoIterator for Locals {
    type Item = ValueType;
    type IntoIter = LocalsIterator;

    fn into_iter(self) -> Self::IntoIter {
        LocalsIterator {
            locals: self,
            current_position: 0,
        }
    }
}

pub struct LocalsIterator {
    locals: Locals,
    current_position: u32,
}
impl Iterator for LocalsIterator {
    type Item = ValueType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position >= self.locals.n {
            None
        } else {
            self.current_position += 1;
            Some(self.locals.t)
        }
    }
}
<<<<<<< HEAD

impl fmt::Display for Locals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone().into_iter().try_for_each(|v| write!(f, "{}\n", v))
    }
}
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum ImportDesc {
    TypeIdx(TypeId),
    TableType(Limits),
    MemType(Limits),
    GlobalType(GlobalType),
}

pub type LabelId = u32;
pub type FuncId = u32;
pub type TypeId = u32;
pub type TableId = u32;
pub type LocalId = u32;
pub type GlobalId = u32;
pub type MemId = u32;

#[derive(Debug, Clone)]
pub struct Import {
    module: (String, Position),
    name: (String, Position),
    desc: (ImportDesc, Position),
}

impl<'src> From<crate::reader::Import<'src>> for Import {
    fn from(value: crate::reader::Import<'src>) -> Self {
        Import {
            module: (value.module.0.to_string(), value.module.1),
            name: (value.name.0.to_string(), value.name.1),
            desc: value.desc,
        }
    }
}

=======
>>>>>>> main
