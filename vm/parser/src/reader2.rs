use core::fmt::{self, Display};
use std::{io::{Read, Seek, SeekFrom}, iter::repeat, ops::Range, string::FromUtf8Error};

use byteorder::ReadBytesExt;
use itertools::Itertools;
use strum_macros::{Display, FromRepr};

use crate::{leb::{Leb, LebError}, op2::Op, reader::FromReader};
use thiserror::Error;
const TYPE_MAGIC: u8 = 0x60;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unable to read from reader: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid LEB encoding: {0}\n
        See: https://webassembly.github.io/spec/core/binary/values.html#integers")] 
    Leb(#[from] LebError),
    #[error("Invalid wasm header. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module")] 
    InvalidHeader([u8; 4]),
    #[error("Invalid wasm version. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module")] 
    InvalidVersion([u8; 4]),

    #[error("Invalid value type. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/types.html#number-types")] 
    InvalidValueTypeId(u8),

    #[error("Invalid function type encoding. Expected: 0x60 got: {0}")]
    InvalidFunctionTypeEncoding(u8),

    #[error("Invalid bool encoding. Expected: 0x60 got: {0}")]
    InvalidBool(u8),

    #[error("Invalid blocktype encoding: Got {0}")]
    InvalidBlocktype(i64),

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
}


pub trait BytecodeReader: Read + Seek + Sized {
    fn parse<T: FromBytecode>(&mut self) -> Result<T, ParserError> {
        T::from_reader(self)
    }
}

pub trait FromBytecode : Sized {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError>;  
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
    pub position: Range<usize>
}

impl<T> WithPosition<T> {
    pub fn new(data: T, position: Range<usize>) -> Self{
        Self {data, position}
    }
}
impl<T: FromBytecode> FromBytecode for WithPosition<T> {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        parse_with_pos(reader)
    }
}

pub fn read_with_pos<R: Read + Seek, T: Sized, F>(reader: &mut R, read_op: F) -> Result<WithPosition<T>, ParserError> 
    where F: FnOnce(&mut R) -> T
{
    let start = reader.seek(SeekFrom::Current(0))? as usize;    
    let data = read_op(reader); 
    let end = reader.seek(SeekFrom::Current(0))? as usize;

    let range = start .. end - start;  
    Ok(WithPosition::new(data, range))
}

pub fn try_read_with_pos<R: Read + Seek, T: Sized, F>(reader: &mut R, read_op: F) -> Result<WithPosition<T>, ParserError> 
    where F: FnOnce(&mut R) -> Result<T, ParserError>
{
    let start = reader.seek(SeekFrom::Current(0))? as usize;    
    let data = read_op(reader)?; 
    let end = reader.seek(SeekFrom::Current(0))? as usize;

    let range = start .. end - start;  
    Ok(WithPosition::new(data, range))
}

pub fn parse_with_pos<R: BytecodeReader, T: FromBytecode>(reader: &mut R) -> Result<WithPosition<T>, ParserError> {  
    try_read_with_pos(reader, |r| r.parse())
}

pub fn iter_vec<R: BytecodeReader, T: FromBytecode>(reader: &mut R) -> Result<impl Iterator<Item = Result<T, ParserError>>, ParserError> {
    let count = Leb::read_u32(reader)?; 
    Ok((0..count).map(|_| reader.parse::<T>()))

}
pub fn iter_vec_with_pos<R: BytecodeReader, T: FromBytecode>(reader: &mut R) -> Result<impl Iterator<Item = Result<WithPosition<T>, ParserError>>, ParserError> {
    let count = Leb::read_u32(reader)?; 
    Ok((0..count).map(|_| parse_with_pos(reader)))
}

pub fn parse_vec<R: BytecodeReader, T: FromBytecode>(reader: &mut R) -> Result<Vec<T>, ParserError> {
    iter_vec(reader)?.collect()
}
pub fn parse_vec_pos<R: BytecodeReader, T: FromBytecode>(reader: &mut R) -> Result<WithPosition<Vec<WithPosition<T>>>, ParserError> {
    let start = reader.seek(SeekFrom::Current(0))? as usize;    
    let data =  iter_vec_with_pos(reader)?.collect::<Result<Vec<WithPosition<T>>, _>>()?;
    let end = reader.seek(SeekFrom::Current(0))? as usize;
    let range = start .. end - start;  
    Ok(WithPosition::new(data, range)) 
}

pub fn parse_string<R: BytecodeReader>(reader: &mut R) -> Result<String, ParserError> {
    let len = reader.parse::<usize>()?; 
    let mut buffer= vec![0; len]; 
    reader.read_exact(&mut buffer)?; 

    Ok(String::from_utf8(buffer)?) 

}

pub fn parse_data_with_pos<R: BytecodeReader>(reader: &mut R) -> Result<WithPosition<Vec<u8>>, ParserError> {
    try_read_with_pos(reader, |r| {
                    let data_size: usize = r.parse()?;
                    let mut buffer = vec![0; data_size];
                    r.read_exact(&mut buffer); 
                    Ok(buffer)})
}
pub fn iter_const_expr<R: BytecodeReader>(reader: &mut R) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>>{
    repeat(0)
        .map(|_| reader.parse::<WithPosition<Op>>())
        .take_while(|op|op.as_ref().is_ok_and(|op| !op.data.is_terminator()) || op.is_err())
}
/*
pub fn iter_expr<R: BytecodeReader>(reader: &mut R) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>> {
    (0..)
        .map(|_| reader.parse::<WithPosition<Op>>())
        .scan(0, |depth, op| {
            op.                
        })
}
*/
impl FromBytecode for String {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(parse_string(reader)?)
    }
}
pub struct Header {
    header: Range<usize>, 
    version: Range<usize>
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
            return Err(ParserError::InvalidHeader(header.data))
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
            version: version.position
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
                params: parse_vec_pos(reader)?, 
                results: parse_vec_pos(reader)?,
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

}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = self.iter_params();
        let r = self.iter_results();

        write!(f, "({}) -> ({})", p.format(", "), r.format(", "))
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalType {
    pub t: WithPosition<ValueType>,
    pub mutable: WithPosition<bool>,
}
impl GlobalType {
    pub fn is_mut(&self) -> bool {
        self.mutable.data
    }
}
impl Display for GlobalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut_str = if self.is_mut() { "mut" } else { "" };
        write!(f, "{} {}", mut_str, self.t.data)
    }
}

impl FromBytecode for GlobalType {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Self {
            t: parse_with_pos(reader)?,
            mutable: reader.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Global {
    pub t: WithPosition<GlobalType>,
    pub init_expr: Vec<WithPosition<Op>>,
}
impl Global {
    pub fn value_type(&self) -> ValueType {
        self.t.data.t.data
    }

    pub fn iter_init_expr(&self) -> impl Iterator<Item = &Op> {
        self.init_expr.iter().map(|v| &v.data)
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
        if let Some(WithPosition {data: max, position: _}) = &self.max {
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
            Some(WithPosition {data: m, position: _}) => write!(f, "({}..{})", self.min.data, m),
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


#[derive(Debug, Clone)]
pub struct ImportIdent {
    pub module: WithPosition<String>,
    pub name: WithPosition<String>,
}
impl Display for ImportIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "module: {}, name: {}", self.module.data, self.name.data) 
    }
}

impl FromBytecode for ImportIdent {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Self {
            module: parse_with_pos(reader)?,
            name: parse_with_pos(reader)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    pub ident: WithPosition<ImportIdent>,
    pub desc: WithPosition<ImportDesc>
}
impl FromBytecode for Import {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Self {
            ident: parse_with_pos(reader)?,
            desc: parse_with_pos(reader)?,
        })
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}): {}", self.ident.data, self.desc.data)
    }
}
#[derive(Debug, Clone, PartialEq)]
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

impl FromBytecode for Locals {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let n: u32 = reader.parse()?;        
        let t: ValueType = reader.parse()?;
        Ok(Self {n, t}) 
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
            _ => Err(ParserError::InvalidExportDesc(export_type)) 
        }
    }
}

impl FromBytecode for ExportDesc {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let t =  reader.read_u8()?;
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

#[derive(Debug, Clone)]
pub struct Export {
    pub name: WithPosition<String>,
    pub desc: WithPosition<ExportDesc>
}

impl FromBytecode for Export {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Self {name: reader.parse()?, desc: reader.parse()?})
    }
}
impl fmt::Display for Export {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.data, self.desc.data)
    }
}

#[derive(Debug, Clone)]
pub enum Data {
    Active {
        mem_id: usize, 
        expr: WithPosition<Vec<WithPosition<Op>>>,
        data: WithPosition<Vec<u8>>, 
    },
    Passive(WithPosition<Vec<u8>>)
}
impl Data {
    fn parse_active<R: BytecodeReader>(reader: &mut R, mem_id: usize) -> Result<Data, ParserError>{
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
}

impl FromBytecode for Data {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        match reader.parse::<u32>()? {
            0 => Data::parse_active(reader, 0),
            1 => Ok(Self::Passive(parse_data_with_pos(reader)?)),
            2 => {
                let id: usize = reader.parse()?;
                Data::parse_active(reader, id)
            },
            n => Err(ParserError::InvalidDataMode(n)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    data: Vec<WithPosition<Op>> 
}

/*
#[derive(Debug, Clone)]
pub struct Function {
    pub size: usisze,
    pub locals: Vec<WithPosition<Locals>>,
    pub code: Exp
}
*/
