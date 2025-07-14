use core::fmt::{self, Display};
use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom},
    iter::repeat,
    ops::Range,
    string::FromUtf8Error,
    usize,
};

use byteorder::ReadBytesExt;
use itertools::Itertools;
use parser_derive::FromBytecode;

use crate::{
    leb::{Leb, LebError},
    op::Op,
};
use thiserror::Error;
const TYPE_MAGIC: u8 = 0x60;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unable to read from reader: {0}")]
    Io(#[from] std::io::Error),
    #[error(
        "Invalid LEB encoding: {0}\n
        See: https://webassembly.github.io/spec/core/binary/values.html#integers"
    )]
    Leb(#[from] LebError),
    #[error(
        "Invalid wasm header. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module"
    )]
    InvalidHeader([u8; 4]),
    #[error(
        "Invalid wasm version. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module"
    )]
    InvalidVersion([u8; 4]),

    #[error(
        "Invalid value type. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/types.html#number-types"
    )]
    InvalidValueTypeId(u8),

    #[error("Invalid function type encoding. Expected: 0x60 got: {0}")]
    InvalidFunctionTypeEncoding(u8),

    #[error("Invalid bool encoding. Expected: 0x60 got: {0}")]
    InvalidBool(u8),

    #[error("Invalid blocktype encoding")]
    InvalidBlocktype,

    #[error("Invalid limits encoding: Got {0}, expected either 0x00 or 0x01")]
    InvalidLimitsEncoding(u8),

    #[error("Invalid Import Type: Got {0}, expected 0x00, 0x01, 0x02 or 0x03")]
    InvalidImportType(u8),

    #[error("Unable to decode string: {0}")]
    InvalidUtf(#[from] FromUtf8Error),

    #[error("Invalid Export Type Encoding: Got {0}, expected 0x00, 0x01, 0x02 or 0x03")]
    InvalidExportDesc(u8),

    #[error("Invalid Data Mode Encoding: Got {0}, expected 0, 1 or 2")]
    InvalidDataMode(u32),

    #[error("Invalid section id: Got {0}, expected 0..11")]
    InvalidSectionId(u8),

    #[error("{0}")]
    WatParseError(#[from] wat::Error),
}
impl ParserError {
    pub fn is_eof(&self) -> bool {
        if let Self::Io(e) = self {
            e.kind() == std::io::ErrorKind::UnexpectedEof
        } else {
            false
        }
    }
}

pub trait BytecodeReader: Read + Seek + Sized {
    fn parse<T: FromBytecode>(&mut self) -> Result<T, ParserError> {
        T::from_reader(self)
    }
}
impl<T: Read + Seek> BytecodeReader for T {}

pub trait FromBytecode: Sized {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError>;
}

impl<T: FromBytecode> FromBytecode for Vec<T> {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        parse_vec(reader)
    }
}
impl FromBytecode for u32 {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Leb::read_u32(reader)?)
    }
}
impl FromBytecode for u64 {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Leb::read_u64(reader)?)
    }
}

impl FromBytecode for usize {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Leb::read_u32(reader)? as usize)
    }
}
impl FromBytecode for isize {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Leb::read_u32(reader)? as isize)
    }
}

impl FromBytecode for bool {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        match reader.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            num => Err(ParserError::InvalidBool(num)),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct WithPosition<T> {
    pub data: T,
    pub position: Range<usize>,
}

impl<T> WithPosition<T> {
    pub fn new(data: T, position: Range<usize>) -> Self {
        Self { data, position }
    }
    pub fn as_ref(&self) -> WithPosition<&T> {
        WithPosition::new(&self.data, self.position.clone())
    }
    pub fn inner_ref(&self) -> &T {
        &self.data
    }
}
pub fn iter_without_position<T>(
    iter: impl Iterator<Item = WithPosition<T>>,
) -> impl Iterator<Item = T> {
    iter.map(|p| p.data)
}

impl<T: FromBytecode> FromBytecode for WithPosition<T> {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        parse_with_pos(reader)
    }
}
impl<T: FromBytecode> From<(T, Range<usize>)> for WithPosition<T> {
    fn from(value: (T, Range<usize>)) -> Self {
        WithPosition::new(value.0, value.1)
    }
}
pub fn read_with_pos<R: Read + Seek, T: Sized, F>(
    reader: &mut R,
    read_op: F,
) -> Result<WithPosition<T>, ParserError>
where
    F: FnOnce(&mut R) -> T,
{
    let start = reader.seek(SeekFrom::Current(0))? as usize;
    let data = read_op(reader);
    let end = reader.seek(SeekFrom::Current(0))? as usize;

    let range = start..end - start;
    Ok(WithPosition::new(data, range))
}

pub fn try_read_with_pos<R: Read + Seek, T: Sized, F>(
    reader: &mut R,
    read_op: F,
) -> Result<WithPosition<T>, ParserError>
where
    F: FnOnce(&mut R) -> Result<T, ParserError>,
{
    let start = reader.seek(SeekFrom::Current(0))? as usize;
    let data = read_op(reader)?;
    let end = reader.seek(SeekFrom::Current(0))? as usize;

    let range = start..end;
    Ok(WithPosition::new(data, range))
}

pub fn parse_with_pos<R: BytecodeReader, T: FromBytecode>(
    reader: &mut R,
) -> Result<WithPosition<T>, ParserError> {
    try_read_with_pos(reader, |r| r.parse())
}

pub fn iter_vec<R: BytecodeReader, T: FromBytecode>(
    reader: &mut R,
) -> Result<impl Iterator<Item = Result<T, ParserError>>, ParserError> {
    let count = Leb::read_u32(reader)?;
    Ok((0..count).map(|_| reader.parse::<T>()))
}
pub fn iter_vec_with_pos<R: BytecodeReader, T: FromBytecode>(
    reader: &mut R,
) -> Result<impl Iterator<Item = Result<WithPosition<T>, ParserError>>, ParserError> {
    let count = Leb::read_u32(reader)?;
    Ok((0..count).map(|_| parse_with_pos(reader)))
}

pub fn parse_vec<R: BytecodeReader, T: FromBytecode>(
    reader: &mut R,
) -> Result<Vec<T>, ParserError> {
    iter_vec(reader)?.collect()
}
pub fn parse_vec_pos<R: BytecodeReader, T: FromBytecode>(
    reader: &mut R,
) -> Result<WithPosition<Vec<WithPosition<T>>>, ParserError> {
    let start = reader.seek(SeekFrom::Current(0))? as usize;
    let data = iter_vec_with_pos(reader)?.collect::<Result<Vec<WithPosition<T>>, _>>()?;
    let end = reader.seek(SeekFrom::Current(0))? as usize;
    let range = start..end - start;
    Ok(WithPosition::new(data, range))
}

pub fn parse_string<R: BytecodeReader>(reader: &mut R) -> Result<String, ParserError> {
    let len = reader.parse::<usize>()?;
    let mut buffer = vec![0; len];
    reader.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

pub fn parse_data_with_pos<R: BytecodeReader>(
    reader: &mut R,
) -> Result<WithPosition<Vec<u8>>, ParserError> {
    try_read_with_pos(reader, |r| {
        let data_size: usize = r.parse()?;
        let mut buffer = vec![0; data_size];
        r.read_exact(&mut buffer)?;
        Ok(buffer)
    })
}
pub fn iter_const_expr<R: BytecodeReader>(
    reader: &mut R,
) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>> {
    repeat(0)
        .map(|_| reader.parse::<WithPosition<Op>>())
        .take_while(|op| op.as_ref().is_ok_and(|op| !op.data.is_terminator()) || op.is_err())
}

pub fn iter_expr<R: BytecodeReader>(
    reader: &mut R,
) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>> {
    (0..).scan((0, true), |(depth, cont), _| {
        if *cont {
            let op = reader.parse::<WithPosition<Op>>();
            Some(op.inspect(|op| {
                let (new_depth, should_cont) = op.data.continues(*depth);
                *depth = new_depth;
                *cont = should_cont;
            }))
        } else {
            None
        }
    })
}

impl FromBytecode for String {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(parse_string(reader)?)
    }
}
#[derive(Debug, Clone, Default)]
pub struct Header {
    header: Range<usize>,
    version: Range<usize>,
}

pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";
pub const WASM_HEADER_VERSION: &[u8; 4] = &[1, 0, 0, 0];

impl FromBytecode for Header {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let header = try_read_with_pos(reader, |reader| {
            let mut header: [u8; 4] = [0; 4];
            reader.read_exact(&mut header)?;
            Ok(header)
        })?;

        if header.data != *WASM_HEADER_MAGIC {
            return Err(ParserError::InvalidHeader(header.data));
        }

        let version = try_read_with_pos(reader, |reader| {
            let mut version: [u8; 4] = [0; 4];

            reader.read_exact(&mut version)?;
            if version != *WASM_HEADER_VERSION {
                Err(ParserError::InvalidVersion(version))
            } else {
                Ok(version)
            }
        })?;

        Ok(Header {
            header: header.position,
            version: version.position,
        })
    }
}

pub fn read_wasm_header(reader: &mut impl BytecodeReader) -> Result<Header, ParserError> {
    reader.parse()
}

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
impl Display for ValueType {
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
    type Error = ParserError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            0x70 => Ok(Self::Funcref),
            0x6F => Ok(Self::Externref),
            0x7B => Ok(Self::Vectype),
            _ => Err(ParserError::InvalidValueTypeId(value)),
        }
    }
}

impl std::convert::TryFrom<i8> for ValueType {
    type Error = ParserError;

    fn try_from(value: i8) -> std::result::Result<Self, Self::Error> {
        let value = value as u8;
        value.try_into()
    }
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

impl FromBytecode for ValueType {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        reader.read_u8()?.try_into()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Type {
    pub params: WithPosition<Vec<WithPosition<ValueType>>>,
    pub results: WithPosition<Vec<WithPosition<ValueType>>>,
}

impl FromBytecode for Type {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let magic = reader.read_u8()?;
        if magic != TYPE_MAGIC {
            Err(ParserError::InvalidFunctionTypeEncoding(magic))
        } else {
            Ok(Self {
                params: reader.parse()?,
                results: reader.parse()?,
            })
        }
    }
}
impl Type {
    pub fn iter_params(&self) -> impl Iterator<Item = &ValueType> {
        self.params.data.iter().map(|v| &v.data)
    }
    pub fn iter_results(&self) -> impl Iterator<Item = &ValueType> {
        self.results.data.iter().map(|v| &v.data)
    }

    pub fn get_param(&self, id: usize) -> Option<ValueType> {
        self.params.data.get(id).map(|id| id.data)
    }
    pub fn get_result(&self, id: usize) -> Option<ValueType> {
        self.results.data.get(id).map(|id| id.data)
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.iter_params();
        let r = self.iter_results();

        write!(f, "({}) -> ({})", p.format(", "), r.format(", "))
    }
}

#[derive(FromBytecode, Debug, PartialEq, Clone)]
pub struct GlobalType {
    pub t: WithPosition<ValueType>,
    pub mutable: WithPosition<bool>,
}

impl GlobalType {
    pub fn is_mut(&self) -> bool {
        self.mutable.data
    }
    pub fn value_type(&self) -> ValueType {
        self.t.data
    }
}
impl Display for GlobalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut_str = if self.is_mut() { "mut" } else { "" };
        write!(f, "{} {}", mut_str, self.t.data)
    }
}

#[derive(Debug, Clone)]
pub struct ConstExpr {
    expr: Vec<WithPosition<Op>>,
}

impl FromBytecode for ConstExpr {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(ConstExpr {
            expr: iter_const_expr(reader).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl ConstExpr {
    pub fn iter_ops(&self) -> impl Iterator<Item = Op> {
        self.expr.iter().map(|op| op.data)
    }
}

#[derive(FromBytecode, Debug, Clone)]
pub struct Global {
    pub t: WithPosition<GlobalType>,
    pub init_expr: WithPosition<ConstExpr>,
}

impl Global {
    pub fn value_type(&self) -> ValueType {
        self.t.data.value_type()
    }
    pub fn is_mut(&self) -> bool {
        self.t.data.is_mut()
    }

    pub fn iter_init_expr(&self) -> impl Iterator<Item = &Op> {
        self.init_expr.data.expr.iter().map(|v| &v.data)
    }
}

impl Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = {}",
            self.t.data,
            self.iter_init_expr().format(" ,")
        )
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Limits {
    pub min: WithPosition<u32>,
    pub max: Option<WithPosition<u32>>,
}
impl Limits {
    pub fn in_range(&self, i: i32) -> bool {
        if self.min.data as i32 > i {
            return false;
        }
        if let Some(WithPosition {
            data: max,
            position: _,
        }) = &self.max
        {
            if i > *max as i32 || *max < self.min.data {
                return false;
            }
        }
        true
    }
}
impl FromBytecode for Limits {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        match reader.read_u8()? {
            0x00 => Ok(Self {
                min: reader.parse()?,
                max: None,
            }),
            0x01 => Ok(Self {
                min: reader.parse()?,
                max: Some(reader.parse()?),
            }),
            num => Err(ParserError::InvalidLimitsEncoding(num)),
        }
    }
}
impl Display for Limits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.max {
            Some(WithPosition {
                data: m,
                position: _,
            }) => write!(f, "({}..{})", self.min.data, m),
            None => write!(f, "({}..)", self.min.data),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ImportDesc {
    TypeIdx(usize),
    TableType(Limits),
    MemType(Limits),
    GlobalType(GlobalType),
}

impl FromBytecode for ImportDesc {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let id = reader.read_u8()?;
        match id {
            0x00 => Ok(Self::TypeIdx(reader.parse()?)),
            0x01 => Ok(Self::TableType(reader.parse()?)),
            0x02 => Ok(Self::MemType(reader.parse()?)),
            0x03 => Ok(Self::GlobalType(reader.parse()?)),
            _ => Err(ParserError::InvalidImportType(id)),
        }
    }
}
impl Display for ImportDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportDesc::TypeIdx(i) => write!(f, "{i}"),
            ImportDesc::TableType(limits) => write!(f, "table {limits}"),
            ImportDesc::MemType(limits) => write!(f, "mem {limits}"),
            ImportDesc::GlobalType(global_type) => write!(f, "{global_type}"),
        }
    }
}

#[derive(FromBytecode, Debug, Clone)]
pub struct ImportIdent {
    pub module: WithPosition<String>,
    pub name: WithPosition<String>,
}
impl Display for ImportIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "module: {}, name: {}", self.module.data, self.name.data)
    }
}

#[derive(FromBytecode, Debug, Clone)]
pub struct Import {
    pub ident: WithPosition<ImportIdent>,
    pub desc: WithPosition<ImportDesc>,
}

impl Import {
    pub fn get_name(&self) -> &str {
        &self.ident.data.name.data
    }
    pub fn get_mod_name(&self) -> &str {
        &self.ident.data.module.data
    }
}
impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}): {}", self.ident.data, self.desc.data)
    }
}

#[derive(FromBytecode, Debug, Clone, PartialEq)]
pub struct Locals {
    pub n: u32,
    pub t: ValueType,
}

impl Locals {
    pub fn flat_iter(&self) -> impl Iterator<Item = ValueType> {
        (0..self.n).map(|_| self.t)
    }
}

impl Display for Locals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.flat_iter().try_for_each(|v| write!(f, "{}\n", v))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportDesc {
    FuncId(usize),
    TableId(usize),
    MemId(usize),
    GlobalId(usize),
}
impl ExportDesc {
    pub fn new(export_type: u8, id: usize) -> Result<Self, ParserError> {
        match export_type {
            0x00 => Ok(Self::FuncId(id)),
            0x01 => Ok(Self::TableId(id)),
            0x02 => Ok(Self::MemId(id)),
            0x03 => Ok(Self::GlobalId(id)),
            _ => Err(ParserError::InvalidExportDesc(export_type)),
        }
    }
}

impl FromBytecode for ExportDesc {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let t = reader.read_u8()?;
        let id = reader.parse::<usize>()?;

        Self::new(t, id)
    }
}

impl Display for ExportDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportDesc::FuncId(id) => write!(f, "func id: {id}"),
            ExportDesc::TableId(id) => write!(f, "table id: {id}"),
            ExportDesc::MemId(id) => write!(f, "mem id {id}"),
            ExportDesc::GlobalId(id) => write!(f, "global id {id}"),
        }
    }
}

#[derive(FromBytecode, Debug, Clone)]
pub struct Export {
    pub name: WithPosition<String>,
    pub desc: WithPosition<ExportDesc>,
}

impl fmt::Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.data, self.desc.data)
    }
}
impl Export {
    pub fn get_function_id(&self) -> Option<usize> {
        match self.desc.data {
            ExportDesc::FuncId(id) => Some(id),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Data {
    Active {
        mem_id: usize,
        expr: WithPosition<Vec<WithPosition<Op>>>,
        data: WithPosition<Vec<u8>>,
    },
    Passive(WithPosition<Vec<u8>>),
}
impl Data {
    fn parse_active<R: BytecodeReader>(reader: &mut R, mem_id: usize) -> Result<Data, ParserError> {
        let expr = try_read_with_pos(reader, |r| {
            iter_const_expr(r).collect::<Result<Vec<_>, _>>()
        })?;
        let buffer = parse_data_with_pos(reader)?;

        Ok(Self::Active {
            mem_id,
            expr,
            data: buffer,
        })
    }
    pub fn is_passive(&self) -> bool {
        matches!(self, Data::Passive(_))
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self {
            Self::Active { data, .. } => data.data.as_slice(),
            Self::Passive(data) => data.data.as_slice(),
        }
    }
}

impl FromBytecode for Data {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        println!("Reading data");
        match reader.parse::<u32>()? {
            0 => Data::parse_active(reader, 0),
            1 => Ok(Self::Passive(parse_data_with_pos(reader)?)),
            2 => {
                let id: usize = reader.parse()?;
                Data::parse_active(reader, id)
            }
            n => Err(ParserError::InvalidDataMode(n)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Expression {
    data: Vec<WithPosition<Op>>,
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.iter().map(|op| op.data).format("\n"))
    }
}
impl FromBytecode for Expression {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        println!("Reading expression...");
        Ok(Self {
            data: iter_expr(reader).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(FromBytecode, Debug, Clone)]
pub struct Function {
    pub size: usize,
    pub locals: WithPosition<Vec<WithPosition<Locals>>>,
    pub code: WithPosition<Expression>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locals\n")?;
        self.locals
            .data
            .iter()
            .try_for_each(|l| write!(f, "{}\n", l.data))?;
        write!(f, "Code:\n {}\n", self.code.data)
    }
}

impl Function {
    pub fn iter_locals(&self) -> impl Iterator<Item = ValueType> {
        self.locals
            .data
            .iter()
            .map(|l| l.data.flat_iter())
            .flatten()
    }

    pub fn get_local(&self, id: usize) -> Option<ValueType> {
        //TODO: (joh): Langsam!
        let mut locals = self.iter_locals();
        locals.nth(id)
    }

    pub fn get_op(&self, index: usize) -> Option<&Op> {
        self.code.data.data.get(index).map(|op| &op.data)
    }
    pub fn get_op_mut(&mut self, index: usize) -> Option<&mut Op> {
        self.code.data.data.get_mut(index).map(|op| &mut op.data)
    }
    pub fn iter_ops(&self) -> impl Iterator<Item = WithPosition<Op>> {
        self.code.data.data.iter().cloned()
    }
    pub fn iter_ops_mut(&mut self) -> impl Iterator<Item = &mut WithPosition<Op>> {
        self.code.data.data.iter_mut()
    }
    pub fn get_op_after_offset(&self, ip: isize, offset: isize) -> Option<(&Op, isize)> {
        let op = self.get_op(ip as usize)?;
        let jmp = op.get_jmp()?;
        let next = (ip + jmp) + offset;
        println!("jmp: {}", next);
        let op = self.get_op(next as usize)?;

        Some((op, next))
    }
    pub fn get_op_after(&self, ip: isize) -> Option<(&Op, isize)> {
        self.get_op_after_offset(ip, -1)
    }
}

#[derive(Debug, Clone)]
pub struct CustomSection {
    pub name: WithPosition<String>,
    pub data: WithPosition<Vec<u8>>,
}
impl CustomSection {
    pub fn init(
        reader: &mut impl BytecodeReader,
        section_size: usize,
    ) -> Result<Self, ParserError> {
        let name: WithPosition<String> = reader.parse()?;
        let data_size = section_size - name.position.clone().count();
        let data = try_read_with_pos(reader, |r| {
            let mut buffer = vec![0; data_size];
            r.read_exact(&mut buffer)?;
            Ok(buffer)
        })?;

        Ok(Self { name, data })
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
    Code = 10,
    Data = 11,
    DataCount = 12,
}

pub type Types = Vec<WithPosition<Type>>;
pub type Imports = Vec<WithPosition<Import>>;
pub type Functions = Vec<WithPosition<usize>>;
pub type Tables = Vec<WithPosition<Limits>>;
pub type Memories = Vec<WithPosition<Limits>>;
pub type Globals = Vec<WithPosition<Global>>;
pub type Exports = Vec<WithPosition<Export>>;
pub type Start = u32;
pub type DataCount = u32;
pub type Code = Vec<WithPosition<Function>>;
pub type ModuleData = Vec<WithPosition<Data>>;

#[derive(Debug, Clone)]
pub enum SectionData {
    Type(Types),
    Import(Imports),
    Function(Functions),
    Table(Tables),
    Memory(Memories),
    Global(Globals),
    Export(Exports),
    Start(Start),
    Code(Code),
    Data(ModuleData),
    DataCount(DataCount),
}
macro_rules! impl_match_sec_data {
    ($reader:ident, $id:ident, $($case:literal => $section_type:path), + $(,)?) => {
        match $id {
            $($case => Ok($section_type($reader.parse()?))),+,
            num => Err(ParserError::InvalidSectionId(num))
        }
    };
}

impl SectionData {
    pub fn init<R: BytecodeReader>(reader: &mut R, id: u8) -> Result<Self, ParserError> {
        impl_match_sec_data! {
            reader,
            id,
            0x01 => Self::Type,
            0x02 => Self::Import,
            0x03 => Self::Function,
            0x04 => Self::Table,
            0x05 => Self::Memory,
            0x06 => Self::Global,
            0x07 => Self::Export,
            0x08 => Self::Start,
            0x09 => Self::DataCount,
            0x0A => Self::Code,
            0x0B => Self::Data,
            0x0C => Self::DataCount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub id: u8,
    pub size: usize,
    pub data: WithPosition<SectionDataOrCustom>,
}
impl Section {
    pub fn new_section(id: u8, size: usize, data: WithPosition<SectionData>) -> Self {
        Self {
            id,
            size,
            data: WithPosition::new(SectionDataOrCustom::Section(data.data), data.position),
        }
    }

    pub fn new_custom(
        reader: &mut impl BytecodeReader,
        id: u8,
        size: usize,
    ) -> Result<Self, ParserError> {
        let section = try_read_with_pos(reader, |r| CustomSection::init(r, size))?;

        Ok(Self {
            id,
            size,
            data: WithPosition::new(SectionDataOrCustom::Custom(section.data), section.position),
        })
    }
}

#[derive(Debug, Clone)]
pub enum SectionDataOrCustom {
    Section(SectionData),
    Custom(CustomSection),
}
impl FromBytecode for Section {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let id = reader.read_u8()?;
        let size = reader.parse::<usize>()?;
        match id {
            0x00 => Section::new_custom(reader, id, size),
            _ => {
                let data = try_read_with_pos(reader, |r| SectionData::init(r, id))?;
                Ok(Section::new_section(id, size, data))
            }
        }
    }
}

pub fn parse_until_eof<T: FromBytecode>(
    reader: &mut impl BytecodeReader,
) -> impl Iterator<Item = Result<T, ParserError>> {
    (0..)
        .map(|_| reader.parse())
        .take_while(|op| op.as_ref().is_err_and(|e| !e.is_eof()) || op.is_ok())
}

pub fn iter_sections(
    reader: &mut impl BytecodeReader,
) -> impl Iterator<Item = Result<WithPosition<Section>, ParserError>> {
    parse_until_eof(reader)
}

#[derive(Debug, Default)]
pub struct SortedImports {
    pub functions: Vec<(usize, usize)>,
    pub tables: Vec<(usize, Limits)>,
    pub mems: Vec<(usize, Limits)>,
    pub globals: Vec<(usize, GlobalType)>,
}
impl SortedImports {
    pub fn add(&mut self, import: &Import, id: usize) {
        match &import.desc.data {
            ImportDesc::TypeIdx(t_id) => self.functions.push((id, *t_id)),
            ImportDesc::TableType(limits) => self.tables.push((id, limits.clone())),
            ImportDesc::MemType(limits) => self.mems.push((id, limits.clone())),
            ImportDesc::GlobalType(gt) => self.globals.push((id, gt.clone())),
        }
    }
}

type MaybeAt<T> = Option<WithPosition<T>>;
#[derive(Debug, Default)]
pub struct Bytecode {
    pub header: Header,
    pub types: MaybeAt<Types>,
    pub imports: MaybeAt<Imports>,
    pub functions: MaybeAt<Functions>,
    pub tables: MaybeAt<Tables>,
    pub memories: MaybeAt<Memories>,
    pub globals: MaybeAt<Globals>,
    pub exports: MaybeAt<Exports>,
    pub start: MaybeAt<Start>,
    pub data_count: MaybeAt<DataCount>,
    pub code: MaybeAt<Code>,
    pub data: MaybeAt<ModuleData>,
    pub custom_sections: Vec<WithPosition<CustomSection>>,
}
macro_rules! impl_add_section {
    ($section:ident -> $module:ident {$($case:path => $mod_name:expr),+ $(,)?}) => {
        match $section.data {
            $($case(data) => $mod_name = Some(WithPosition::new(data, $section.position))),+
        }
    };
}

impl Bytecode {
    pub fn get_exports_as_map<'src>(&'src self) -> Option<HashMap<&'src str, ExportDesc>> {
        self.iter_exports().map(|exports| {
            let mut result = HashMap::new();
            exports.for_each(|e| {
                //TODO: Handle doppelte Namen
                _ = result.insert(e.name.data.as_str(), e.desc.data.clone());
            });
            result
        })
    }
    fn add_section(&mut self, section: WithPosition<SectionData>) {
        impl_add_section!(
            section -> self {
                SectionData::Type => self.types,
                SectionData::Import => self.imports,
                SectionData::Function => self.functions,
                SectionData::Table => self.tables,
                SectionData::Memory => self.memories,
                SectionData::Global => self.globals,
                SectionData::Export => self.exports,
                SectionData::Start => self.start,
                SectionData::DataCount => self.data_count,
                SectionData::Code => self.code,
                SectionData::Data => self.data,
            }
        );
    }

    fn add_custom_section(&mut self, section: WithPosition<CustomSection>) {
        self.custom_sections.push(section);
    }

    fn find_export_by_name(&self, name: &str) -> Option<ExportDesc> {
        self.iter_exports()?.find_map(|e| {
            (e.name.data == name)
                .then_some(&e)
                .map(|e| e.desc.data.clone())
        })
    }

    pub fn is_func_id_exported(&self, id: usize) -> Option<usize> {
        if let Some(mut exports) = self.iter_exports() {
            exports
                .enumerate()
                .find_map(|(i, e)| (e.get_function_id()? == id).then_some(i))
        } else {
            None
        }
    }
}

impl FromBytecode for Bytecode {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let mut module: Bytecode = Default::default();
        module.header = reader.parse()?;
        for section in iter_sections(reader) {
            let section = section?;
            let section_data = section.data;
            let pos = section.position;
            match section_data.data.data {
                SectionDataOrCustom::Section(section_data) => {
                    module.add_section(WithPosition::new(section_data, pos))
                }
                SectionDataOrCustom::Custom(custom_section) => {
                    module.add_custom_section(WithPosition::new(custom_section, pos))
                }
            }
        }
        Ok(module)
    }
}

macro_rules! impl_bytecode_vec_accessor {
    ($($name:ident, $pos_name:ident, $field:ident=> $res_type: ty),+$(,)?) => {
        impl Bytecode {
            $(pub fn $name(&self, id: usize) -> Option<$res_type> {
                Some(&self.$field.as_ref()?.data.get(id)?.data)
            })+
            $(pub fn $pos_name(&self, id: usize) -> Option<WithPosition<$res_type>> {
                Some(self.$field.as_ref()?.data.get(id)?.as_ref())
            })+
        }
    };
}

macro_rules! impl_bytecode_iter {
    ($($name:ident, $field:ident=> $res_type: ty),+$(,)?) => {
        impl Bytecode {
            $(pub fn $name(&self) -> Option<impl Iterator<Item = &$res_type>> {
                Some(self.$field.as_ref()?.data.iter().map(|d| &d.data))
            })+
        }
    }
}
impl_bytecode_vec_accessor! {
    get_type, get_type_pos, types => &Type,
    get_import, get_import_pos, imports => &Import,
    get_function, get_function_pos, functions => &usize,
    get_table, get_table_pos, tables => &Limits,
    get_memory, get_memory_pos, memories => &Limits,
    get_global, get_global_pos, globals => &Global,
    get_export, get_export_pos, exports => &Export,
    get_code, get_code_pos, code => &Function,
    get_data, get_data_pos, data => &Data,
}
impl_bytecode_iter! {
    iter_types, types => Type,
    iter_imports, imports => Import,
    iter_functions, functions => usize,
    iter_tables, tables => Limits,
    iter_memories, memories => Limits,
    iter_globals, globals => Global,
    iter_exports,  exports => Export,
    iter_code, code => Function,
    iter_data, data => Data,
}
impl Bytecode {
    pub fn sort_imports(&self) -> Option<SortedImports> {
        if let Some(imports) = &self.imports {
            let mut sorted: SortedImports = Default::default();

            imports
                .data
                .iter()
                .enumerate()
                .for_each(|(id, i)| sorted.add(&i.data, id));

            Some(sorted)
        } else {
            None
        }
    }
    pub fn iter_function_types(&self) -> Option<impl Iterator<Item = &Type>> {
        Some(self.iter_functions()?.map(|id| self.get_type(*id).unwrap()))
    }

    pub fn iter_code_mut(&mut self) -> Option<impl Iterator<Item = &mut Function>> {
        Some(self.code.as_mut()?.data.iter_mut().map(|f| &mut f.data))
    }
}

pub fn parse_binary(reader: &mut impl BytecodeReader) -> Result<Bytecode, ParserError> {
    reader.parse()
}

pub fn parse_wat(code: impl AsRef<str>) -> Result<Bytecode, ParserError> {
    let data = wat::parse_str(code)?;
    let mut reader = Cursor::new(data);

    parse_binary(&mut reader)
}

pub fn is_wasm_bytecode(reader: &mut impl BytecodeReader) -> Result<bool, ParserError> {
    let res = Header::from_reader(reader).map_or_else(
        |e| match e {
            ParserError::InvalidHeader(_) | ParserError::InvalidVersion(_) => Ok(false),
            _ => Err(e),
        },
        |_| Ok(true),
    );
    reader.rewind()?;
    res
}
#[cfg(test)]
mod tests {
    use crate::reader::{ValueType, parse_wat};

    use super::{Data, ParserError};

    #[test]
    fn empty_module() -> Result<(), ParserError> {
        let src = "(module)";

        let module = parse_wat(src)?;
        assert!(module.types.is_none());
        assert!(module.imports.is_none());
        assert!(module.functions.is_none());
        assert!(module.tables.is_none());
        assert!(module.memories.is_none());
        assert!(module.globals.is_none());
        assert!(module.exports.is_none());
        assert!(module.start.is_none());
        assert!(module.data_count.is_none());
        assert!(module.code.is_none());
        assert!(module.data.is_none());
        Ok(())
    }
    #[test]
    fn simple_func() -> Result<(), ParserError> {
        let src = r#"
            (module
                (func (param i32) (param f32) (local f64)
                    local.get 0
                    local.get 1
                    local.get 2
                    i32.add 
                    global.get 5
                )
            )
        "#;
        let module = parse_wat(src)?;
        println!("module: {:?}", module);
        assert_eq!(
            module.get_type(0).unwrap().get_param(0).unwrap(),
            ValueType::I32
        );
        assert_eq!(
            module.get_type(0).unwrap().get_param(1).unwrap(),
            ValueType::F32
        );

        let locals = module
            .get_code(0)
            .unwrap()
            .iter_locals()
            .collect::<Vec<_>>();
        assert_eq!(*locals.get(0).unwrap(), ValueType::F64);

        Ok(())
    }
    #[test]
    fn some_imports() -> Result<(), ParserError> {
        let src = r#"
            (module
                (import "console" "log" (func $log (param i32 i32)))
                (import "js" "mem" (memory 1)))
        "#;
        let module = parse_wat(src)?;
        let imports = module.imports.unwrap().data;
        assert_eq!(imports.len(), 2);
        Ok(())
    }

    #[test]
    fn some_data() -> Result<(), ParserError> {
        let src = r#"
            (module
                (data "Hello world")
                (data "Blubbi")
            ) 
        "#;
        let module = parse_wat(src)?;
        let data = module.get_data(0).unwrap();
        if let Data::Passive(d) = data {
            let str = str::from_utf8(&d.data).unwrap();
            assert_eq!(str, "Hello world");
        } else {
            unreachable!();
        }

        let data = module.get_data(1).unwrap();
        if let Data::Passive(d) = data {
            let str = str::from_utf8(&d.data).unwrap();
            assert_eq!(str, "Blubbi");
        } else {
            unreachable!();
        }

        Ok(())
    }
}
