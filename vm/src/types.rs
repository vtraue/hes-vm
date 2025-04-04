use core::fmt;

use crate::reader::{FromReader, FunctionType, Position, Reader, ReaderError, ValueType};

#[derive(Debug)]
pub struct Type {
    pub params: Box<[(ValueType, Position)]>,
    pub results: Box<[(ValueType, Position)]>,
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
            if i > max.0 as i32 || max.0 < self.min.0  {
                return false
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

