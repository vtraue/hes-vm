use core::fmt;
use std::ops::Range;

use itertools::Itertools;

use crate::op::Op;

use super::{
    error::ReaderError,
    reader::{FromReader, Reader},
};

pub type LabelId = u32;
pub type FuncId = u32;
pub type TypeId = u32;
pub type TableId = u32;
pub type LocalId = u32;
pub type GlobalId = u32;
pub type MemId = u32;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq)]
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
impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ValueType::I32 => "i32",
            ValueType::I64 => "i64",
            ValueType::F32 => "f32",
            ValueType::F64 => "f64",
            ValueType::Funcref => "funcref",
            ValueType::Externref => "externref",
            ValueType::Vectype => "vec",
        };
        write!(f, "{str}")
    }
}
impl std::convert::TryFrom<u8> for ValueType {
    type Error = ReaderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            0x70 => Ok(Self::Funcref),
            0x6F => Ok(Self::Externref),
            0x7B => Ok(Self::Vectype),
            _ => Err(ReaderError::InvalidValueTypeId(value)),
        }
    }
}

impl std::convert::TryFrom<i8> for ValueType {
    type Error = ReaderError;

    fn try_from(value: i8) -> std::result::Result<Self, Self::Error> {
        let value = value as u8;
        value.try_into()
    }
}

impl<'src> FromReader<'src> for ValueType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        reader.read_u8()?.try_into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Type {
    pub params: Box<[(ValueType, Range<usize>)]>,
    pub results: Box<[(ValueType, Range<usize>)]>,
}

impl<'src> FromReader<'src> for Type {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let magic = reader.read_u8()?;
        if magic != 0x60 {
            return Err(ReaderError::InvalidFunctionTypeEncoding(magic));
        }

        Ok(Self {
            params: reader.read_vec()?,
            results: reader.read_vec()?,
        })
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.params.iter().map(|(v, _)| v);
        let r = self.results.iter().map(|(v, _)| v);

        write!(f, "({}) -> ({})", p.format(", "), r.format(", "))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalType {
    pub t: (ValueType, Range<usize>),
    pub mutable: (bool, Range<usize>),
}
impl GlobalType {
    pub fn is_mut(&self) -> bool {
        return self.mutable.0;
    }
}
impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut_str = if self.mutable.0 { "mut" } else { "" };
        write!(f, "{} {}", mut_str, self.t.0)
    }
}
impl<'src> FromReader<'src> for GlobalType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        Ok(Self {
            t: reader.read_with_position()?,
            mutable: reader.read_with_position()?,
        })
    }
}

#[derive(Debug)]
pub struct Global {
    pub t: (GlobalType, Range<usize>),
    pub init_expr: Box<[(Op, Range<usize>)]>,
}
impl Global {
    pub fn value_type(&self) -> ValueType {
        self.t.0.t.0
    }
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

impl<'src> FromReader<'src> for Global {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let t = reader.read_with_position::<GlobalType>()?;
        let init_expr = reader
            .read_const_expr_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        Ok(Global { t, init_expr })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ImportDesc {
    TypeIdx(TypeId),
    TableType(Limits),
    MemType(Limits),
    GlobalType(GlobalType),
}
impl<'src> FromReader<'src> for ImportDesc {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let id = reader.read_u8()?;
        match id {
            0x00 => Ok(Self::TypeIdx(reader.read()?)),
            0x01 => Ok(Self::TableType(reader.read()?)),
            0x02 => Ok(Self::MemType(reader.read()?)),
            0x03 => Ok(Self::GlobalType(reader.read()?)),
            _ => Err(ReaderError::InvalidImportDesc(id)),
        }
    }
}

impl fmt::Display for ImportDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportDesc::TypeIdx(i) => write!(f, "{i}"),
            ImportDesc::TableType(limits) => write!(f, "table {limits}"),
            ImportDesc::MemType(limits) => write!(f, "mem {limits}"),
            ImportDesc::GlobalType(global_type) => write!(f, "{global_type}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Limits {
    pub min: (u32, Range<usize>),
    pub max: Option<(u32, Range<usize>)>,
}
impl Limits {
    pub fn in_range(&self, i: i32) -> bool {
        if self.min.0 as i32 > i {
            return false;
        }
        if let Some(max) = &self.max {
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

impl<'src> FromReader<'src> for Limits {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        match reader.read_u8()? {
            0x00 => Ok(Self {
                min: reader.read_with_position()?,
                max: None,
            }),
            0x01 => Ok(Self {
                min: reader.read_with_position()?,
                max: Some(reader.read_with_position()?),
            }),
            _ => Err(ReaderError::InvalidLimits),
        }
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

impl fmt::Display for Locals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone()
            .into_iter()
            .try_for_each(|v| write!(f, "{}\n", v))
    }
}

impl<'src> FromReader<'src> for Locals {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let n: u32 = reader.read()?;

        let t: ValueType = reader.read()?;
        Ok(Self { n, t })
    }
}

#[derive(Debug, Clone)]
pub struct ImportIdent {
    pub module: (String, Range<usize>),
    pub name: (String, Range<usize>),
}
impl<'src> FromReader<'src> for ImportIdent {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        Ok(Self {
            module: reader.read_with_position()?,
            name: reader.read_with_position()?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Import {
    pub ident: ImportIdent,
    pub desc: (ImportDesc, Range<usize>),
}

impl<'src> FromReader<'src> for Import {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        Ok(Self {
            ident: reader.read()?,
            desc: reader.read_with_position()?,
        })
    }
}
impl<'src> fmt::Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}: {} {})",
            self.ident.module.0, self.ident.name.0, self.desc.0
        )
    }
}

impl<'src> Import {
    pub fn is_function(&'src self) -> Option<(&'src str, TypeId)> {
        match self.desc {
            (ImportDesc::TypeIdx(id), _) => Some((&self.ident.name.0, id)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportDesc {
    FuncId(FuncId),
    TableId(TableId),
    MemId(MemId),
    GlobalId(GlobalId),
}
impl<'src> FromReader<'src> for ExportDesc {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        match reader.read_u8()? {
            0x00 => reader.read::<FuncId>().map(Self::FuncId),
            0x01 => reader.read().map(Self::TableId),
            0x02 => reader.read().map(Self::MemId),
            0x03 => reader.read().map(Self::GlobalId),
            _ => Err(ReaderError::InvalidExportDesc),
        }
    }
}

impl fmt::Display for ExportDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportDesc::FuncId(id) => write!(f, "func id: {id}"),
            ExportDesc::TableId(id) => write!(f, "table id: {id}"),
            ExportDesc::MemId(id) => write!(f, "mem id {id}"),
            ExportDesc::GlobalId(id) => write!(f, "global id {id}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: (String, Range<usize>),
    pub desc: (ExportDesc, Range<usize>),
}

impl<'src> FromReader<'src> for Export {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let name = reader.read_with_position::<String>()?;
        let desc = reader.read_with_position::<ExportDesc>()?;

        Ok(Self { name, desc })
    }
}
impl fmt::Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.0, self.desc.0)
    }
}

#[derive(Debug)]
pub enum Data {
    Active {
        mem_id: MemId,
        expr: Box<[(Op, Range<usize>)]>,
        data: Range<usize>,
    },
    Passive(Range<usize>),
}

impl<'src> FromReader<'src> for Data {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        match reader.read::<u32>()? {
            0 => Ok(Self::Active {
                mem_id: 0,
                expr: reader
                    .read_const_expr_iter()
                    .collect::<Result<Vec<_>, _>>()?
                    .into_boxed_slice(),
                data: reader.read_and_skip_size()?,
            }),
            1 => Ok(Self::Passive(reader.read_and_skip_size()?)),
            2 => Ok(Self::Active {
                mem_id: reader.read()?,
                expr: reader
                    .read_const_expr_iter()
                    .collect::<Result<Vec<_>, _>>()?
                    .into_boxed_slice(),
                data: reader.read_and_skip_size()?,
            }),
            n => Err(ReaderError::InvalidDataMode(n)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression(pub Box<[(Op, Range<usize>)]>);

impl<'src> FromReader<'src> for Expression {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        Ok(Expression(
            reader
                .read_expr_iter()
                .collect::<Result<Vec<(Op, Range<usize>)>, _>>()?
                .into_boxed_slice(),
        ))
    }
}
#[derive(Debug, Clone)]
pub struct Function {
    pub size: usize,
    pub locals: Box<[(Locals, Range<usize>)]>,
    pub code: Expression,
}

impl Function {
    pub fn get_instruction(&self, index: usize) -> Option<&Op> {
        self.code.0.get(index).map(|(i, _)| i)
    }

    pub fn set_instruction(&mut self, index: usize, instruction: Op) {
        self.code.0[index].0 = instruction;
    }

    pub fn get_local(&self, id: u32) -> Option<ValueType> {
        self.locals.iter().find(|l| id < l.0.n).map(|i| i.0.t)
    }

    pub fn iter_local_types(&self) -> impl Iterator<Item = ValueType> {
        self.locals
            .iter()
            .cloned()
            .map(|l| l.0.into_iter())
            .flatten()
    }
    pub fn iter_ops(&self) -> impl Iterator<Item = Op> {
        self.code.0.iter().cloned().map(|(f, _)| f)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locals\n")?;
        self.locals
            .iter()
            .try_for_each(|l| write!(f, "{}\n", l.0))?;
        write!(f, "Code:\n")?;
        self.code.0.iter().try_for_each(|c| write!(f, "{}\n", c.0))
    }
}

impl<'src> FromReader<'src> for Function {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let size = reader.read::<usize>()?;
        Ok(Self {
            size,
            locals: reader.read_vec()?,
            code: reader.read()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CustomSection {
    pub name: (String, Range<usize>),
    pub data: Range<usize>,
}

#[derive(Debug)]
pub enum SectionData {
    Type(Box<[(Type, Range<usize>)]>),
    Import(Box<[(Import, Range<usize>)]>),
    Function(Box<[(TypeId, Range<usize>)]>),
    Table(Box<[(Limits, Range<usize>)]>),
    Memory(Box<[(Limits, Range<usize>)]>),
    Global(Box<[(Global, Range<usize>)]>),
    Export(Box<[(Export, Range<usize>)]>),
    Start(u32),
    DataCount(u32),
    Code(Box<[(Function, Range<usize>)]>),
    Data(Box<[(Data, Range<usize>)]>),
}

#[derive(Debug)]
pub enum SectionDataOrCustom {
    Section(SectionData),
    Custom(CustomSection),
}

#[derive(Debug)]
pub struct Section {
    pub id: u8,
    pub size: usize,
    pub data: SectionDataOrCustom,
}

impl<'src> FromReader<'src> for Section {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let id = reader.read::<u8>()?;
        let size = reader.read::<usize>()?;
        println!("Reading section {id}");
        //TODO: (joh): Das geht bestimmt besser
        let data = match id {
            0x00 => {
                let (name, name_range) = reader.read_with_position::<String>()?;
                let range = reader.skip_bytes(size - (name_range.end - name_range.start))?;
                let section = CustomSection {
                    name: (name, name_range),
                    data: range,
                };
                Ok(SectionDataOrCustom::Custom(section))
            }
            0x01 => Ok(SectionDataOrCustom::Section(SectionData::Type(
                reader.read_vec()?,
            ))),
            0x02 => Ok(SectionDataOrCustom::Section(SectionData::Import(
                reader.read_vec()?,
            ))),
            0x03 => Ok(SectionDataOrCustom::Section(SectionData::Function(
                reader.read_vec()?,
            ))),
            0x04 => Ok(SectionDataOrCustom::Section(SectionData::Table(
                reader.read_vec()?,
            ))),
            0x05 => Ok(SectionDataOrCustom::Section(SectionData::Memory(
                reader.read_vec()?,
            ))),
            0x06 => Ok(SectionDataOrCustom::Section(SectionData::Global(
                reader.read_vec()?,
            ))),
            0x07 => Ok(SectionDataOrCustom::Section(SectionData::Export(
                reader.read_vec()?,
            ))),
            0x08 => Ok(SectionDataOrCustom::Section(SectionData::Start(
                reader.read()?,
            ))),
            0x09 => Ok(SectionDataOrCustom::Section(SectionData::DataCount(
                reader.read()?,
            ))),
            0xa => Ok(SectionDataOrCustom::Section(SectionData::Code(
                reader.read_vec()?,
            ))),
            0xb => Ok(SectionDataOrCustom::Section(SectionData::Data(
                reader.read_vec()?,
            ))),
            _ => Err(ReaderError::InvalidSectionId(id)),
        }?;

        Ok(Section { id, size, data })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SectionId {
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    DataCount = 9,
    Code = 10,
    Data = 11,
}
