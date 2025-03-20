use core::fmt;
use std::marker::PhantomData;

use crate::op::Op;

//NOTE: (joh) Inspiriert von: https://github.com/bytecodealliance/wasm-tools/blob/main/crates/wasmparser/src/binary_reader.rs

#[derive(Debug, PartialEq)]
pub enum BytecodeReaderError {
    InvalidLeb,
    EndOfBuffer,
    InvalidUtf8InName(std::str::Utf8Error),
    InvalidBool,
    InvalidTypeId,
    InvalidRefTypeId,
    InvalidValueTypeId(u8),
    InvalidHeaderMagicNumber,
    InvalidWasmVersion,
    InvalidFunctionTypeEncoding,
    InvalidImportDesc(u8),
    UnimplementedOpcode(u8),
    ExpectedConstExpression(Op),
    InvalidLimits,
    InvalidExportDesc,
    MalformedCodeSection,
}

pub type Result<T, E = BytecodeReaderError> = core::result::Result<T, E>;

pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";
impl fmt::Display for BytecodeReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeReaderError::InvalidLeb => write!(f, "Invalid leb128 formated number"),
            BytecodeReaderError::EndOfBuffer => write!(f, "Reached end of buffer"),
            BytecodeReaderError::InvalidUtf8InName(utf8_error) => utf8_error.fmt(f),
            BytecodeReaderError::InvalidBool => write!(f, "Invalid boolean encoding"),
            BytecodeReaderError::InvalidTypeId => write!(f, "Invalid Type id"),
            BytecodeReaderError::InvalidRefTypeId => write!(f, "Invalid ref type id"),
            BytecodeReaderError::InvalidValueTypeId(id) => {
                                                        write!(f, "Invalid value type id: {}", id)
                                                    }
            BytecodeReaderError::InvalidHeaderMagicNumber => todo!(),
            BytecodeReaderError::InvalidWasmVersion => todo!(),
            BytecodeReaderError::InvalidFunctionTypeEncoding => todo!(),
            BytecodeReaderError::InvalidImportDesc(id) => write!(f, "Invalid import desc id: {}", id),
            BytecodeReaderError::UnimplementedOpcode(_) => todo!(),
            BytecodeReaderError::ExpectedConstExpression(op) => todo!(),
            BytecodeReaderError::InvalidLimits => todo!(),
            BytecodeReaderError::InvalidExportDesc => todo!(),
            BytecodeReaderError::MalformedCodeSection => todo!(),
        }
    }
}

impl From<std::str::Utf8Error> for BytecodeReaderError {
    fn from(value: std::str::Utf8Error) -> Self {
        BytecodeReaderError::InvalidUtf8InName(value)
    }
}

pub trait FromBytecodeReader<'src>: Sized {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self>;
}

impl<'src> FromBytecodeReader<'src> for bool {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        match reader.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(BytecodeReaderError::InvalidBool),
        }
    }
}

impl<'src> FromBytecodeReader<'src> for u32 {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_var_u32()
    }
}

impl<'src> FromBytecodeReader<'src> for usize {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        Ok(reader.read_var_u32()? as usize)
    }
}

impl<'src> FromBytecodeReader<'src> for i32 {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_var_i32()
    }
}

impl<'src> FromBytecodeReader<'src> for u8 {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_u8()
    }
}
impl<'src> FromBytecodeReader<'src> for u64 {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_var_u64()
    }
}
impl<'src> FromBytecodeReader<'src> for i64 {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_var_i64()
    }
}

impl<'src> FromBytecodeReader<'src> for &'src str {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_name()
    }
}

#[derive(Debug, Clone)]
pub struct BytecodeReader<'src> {
    buffer: &'src [u8],
    current_position: usize,
    //TODO: (joh): Erlaube generische Allokatoren
}

impl<'src> BytecodeReader<'src> {
    pub fn new(buffer: &'src [u8]) -> Self {
        Self {
            buffer,
            current_position: 0,
        }
    }

    pub fn current_buffer(&self) -> &'src [u8] {
        &self.buffer[self.current_position..]
    }
    pub fn bytes_left(&self) -> usize {
        self.buffer.len() - self.current_position
    }

    pub fn can_read_bytes(&self, len: usize) -> Result<()> {
        if self.current_position + len > self.buffer.len() {
            Err(BytecodeReaderError::EndOfBuffer)
        } else {
            Ok(())
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.can_read_bytes(size_of::<u8>())?;
        let res = self.buffer[self.current_position];
        self.current_position += 1;
        Ok(res)
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<&'src [u8]> {
        self.can_read_bytes(size)?;
        let new_pos = self.current_position + size;
        let res = &self.buffer[self.current_position..new_pos];

        self.current_position = new_pos;
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.read_bytes(size_of::<u32>())?.try_into().unwrap(),
        ))
    }

    //NOTE: (joh) Die Leb128 Implementationen kommen von: https://github.com/bytecodealliance/wasm-tools/blob/main/crates/wasmparser/src/binary_reader.rs#L516
    #[inline]
    pub fn read_var_u32(&mut self) -> Result<u32> {
        // Optimization for single byte i32.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(u32::from(byte))
        } else {
            self.read_var_u32_big(byte)
        }
    }

    fn read_var_u32_big(&mut self, byte: u8) -> Result<u32> {
        let mut result = (byte & 0x7F) as u32;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as u32) << shift;
            if shift >= 25 && (byte >> (32 - shift)) != 0 {
                return Err(BytecodeReaderError::InvalidLeb);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    #[inline]
    pub fn read_var_u64(&mut self) -> Result<u64> {
        // Optimization for single byte u64.
        let byte = u64::from(self.read_u8()?);
        if (byte & 0x80) == 0 {
            Ok(byte)
        } else {
            self.read_var_u64_big(byte)
        }
    }

    fn read_var_u64_big(&mut self, byte: u64) -> Result<u64> {
        let mut result = byte & 0x7F;
        let mut shift = 7;
        loop {
            let byte = u64::from(self.read_u8()?);
            result |= (byte & 0x7F) << shift;
            if shift >= 57 && (byte >> (64 - shift)) != 0 {
                // The continuation bit or unused bits are set.
                return Err(BytecodeReaderError::InvalidLeb);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    #[inline]
    pub fn read_var_i32(&mut self) -> Result<i32> {
        // Optimization for single byte i32.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(((byte as i32) << 25) >> 25)
        } else {
            self.read_var_i32_big(byte)
        }
    }

    fn read_var_i32_big(&mut self, byte: u8) -> Result<i32> {
        let mut result = (byte & 0x7F) as i32;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i32) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(BytecodeReaderError::InvalidLeb);
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 32 - shift;
        Ok((result << ashift) >> ashift)
    }

    pub fn read_var_s33(&mut self) -> Result<i64> {
        // Optimization for single byte.
        let byte = self.read_u8()?;
        if (byte & 0x80) == 0 {
            return Ok(((byte as i8) << 1) as i64 >> 1);
        }

        let mut result = (byte & 0x7F) as i64;
        let mut shift = 7;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7F) as i64) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (33 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(BytecodeReaderError::InvalidLeb);
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 64 - shift;
        Ok((result << ashift) >> ashift)
    }

    pub fn read_var_i64(&mut self) -> Result<i64> {
        let mut result: i64 = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= i64::from(byte & 0x7F) << shift;
            if shift >= 57 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = ((byte << 1) as i8) >> (64 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(BytecodeReaderError::InvalidLeb);
                }
                return Ok(result);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        let ashift = 64 - shift;
        Ok((result << ashift) >> ashift)
    }

    pub fn skip_bytes(&mut self, size: usize) -> Result<()> {
        self.can_read_bytes(size)?;
        self.current_position += size;
        Ok(())
    }

    pub fn read_sized_name(&mut self, size: usize) -> Result<&'src str> {
        Ok(str::from_utf8(self.read_bytes(size)?)?)
    }

    pub fn read_name(&mut self) -> Result<&'src str> {
        let len = self.read::<u32>()? as usize;
        self.read_sized_name(len)
    }

    pub fn check_header(&mut self) -> Result<()> {
        let header = self.read_bytes(4)?;
        if header != WASM_HEADER_MAGIC {
            return Err(BytecodeReaderError::InvalidHeaderMagicNumber);
        }
        let version = self.read_bytes(4)?;
        if version[0] != 1 {
            return Err(BytecodeReaderError::InvalidWasmVersion);
        }

        Ok(())
    }

    pub fn read<T>(&mut self) -> Result<T>
    where
        T: FromBytecodeReader<'src>,
    {
        T::from_reader(self)
    }

    pub fn read_vec_iter<'me, T>(&'me mut self) -> Result<BytecodeVecIter<'src, 'me, T>>
    where
        T: FromBytecodeReader<'src>,
    {
        let size = self.read_var_u32()? as usize;
        Ok(BytecodeVecIter {
            count: size,
            current_position: 0,
            done: false,
            reader: self,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn read_vec_boxed_slice<T>(&mut self) -> Result<Box<[T]>>
    where
        T: FromBytecodeReader<'src>,
    {
        Ok(self
            .read_vec_iter()?
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice())
    }

    //NOTE: (joh): Ich mag das size argument hier nicht so gerne. es wäre schöner, wenn das hier
    //gelesen werden könnte
    pub fn get_section_reader<T>(&mut self, size: usize) -> Result<BytecodeSubReader<'src, T>>
    where
        T: FromBytecodeReader<'src> 
    {
        let slice: &'src [u8] = &self.buffer[self.current_position..self.current_position + size];
        let mut reader = BytecodeReader::new(slice); 
        let count = reader.read_var_u32()? as usize;
        
        self.skip_bytes(size)?;
         
        Ok(BytecodeSubReader {reader, count, read: 0, _marker: PhantomData}) 
    }

    pub fn read_const_expr_iter<'me>(&'me mut self) -> ConstantExprIter<'src, 'me> {
        ConstantExprIter {current_position: 0, done: false, reader: self}
    }
        
}

pub struct BytecodeVecIter<'src, 'me, T: FromBytecodeReader<'src>> {
    count: usize,
    current_position: usize,
    done: bool,
    reader: &'me mut BytecodeReader<'src>,
    _marker: std::marker::PhantomData<T>,
}

impl<'src, 'me, T: FromBytecodeReader<'src>> Iterator for BytecodeVecIter<'src, 'me, T> {
    type Item = Result<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position >= self.count || self.done {
            None
        } else {
            let res = self.reader.read::<T>();
            if res.is_err() {
                self.done = true;
            } else {
                self.current_position += 1;
            }
            Some(res)
        }
    }
}

pub struct ConstantExprIter<'src, 'me> {
    current_position: usize,
    done: bool,
    reader: &'me mut BytecodeReader<'src>, }
impl<'src, 'me> Iterator for ConstantExprIter<'src, 'me> {
    type Item = Result<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        }
        else {
            match self.reader.read::<Op>() {
                Err(e) => {
                    self.done = true;
                    Some(Err(e))
                },
                Ok(Op::End) => {
                    self.done = true;
                    Some(Ok(Op::End))
                }

                Ok(op) => {
                    if op.is_const() {
                        self.current_position += 1;
                        Some(Ok(op))
                    } else {
                        self.done = true;
                        Some(Err(BytecodeReaderError::ExpectedConstExpression(op)))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BytecodeSubReader<'src, T: FromBytecodeReader<'src>> 
{
    reader: BytecodeReader<'src>,  
    count: usize,
    read: usize,
        
    _marker: PhantomData<T>,
}

impl<'src, T: FromBytecodeReader<'src>> BytecodeSubReader<'src, T> {
    pub fn read(&mut self) -> Option<Result<T>> {
        if self.read < self.count {
            let elem = self.reader.read::<T>(); 
            self.read += 1;
            Some(elem)
        } else {
            println!("No more elements");
            
            None
        }
    }
}

impl<'src, T: FromBytecodeReader<'src>> Iterator for BytecodeSubReader<'src, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read() 
    }
}

#[derive(Debug, Clone)]
pub struct CodeReader<'src> {
    reader: BytecodeReader<'src>,
    depth: usize,
    instructions_read: usize,
    done: bool,
}

impl<'src> CodeReader<'src> {
    pub fn new(buffer: &'src [u8]) -> Self {
        CodeReader {reader: BytecodeReader::new(buffer), depth: 0, instructions_read: 0, done: false}
    }
}
impl<'src> Iterator for CodeReader<'src> {
    type Item = Result<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let op = self.reader.read::<Op>();
            match op {
                Err(e) => {
                    self.done = true;
                    //TODO: (joh) Überprüfe ob EOF, falls ja: Malformed 
                    Some(Err(e))
                }
                Ok(Op::End) => {
                    if self.depth <= 0 {
                        self.done = true;
                    } else {
                        self.depth -= 1;
                    }
                    Some(Ok(Op::End))
                }

                Ok(op) if op.needs_end_terminator() => {
                    self.depth += 1;
                    Some(Ok(op))
                }
                Ok(op) => Some(Ok(op))
            }
        }
    }
}


#[repr(u8)]
pub enum NumberType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl std::convert::TryFrom<u8> for NumberType {
    type Error = BytecodeReaderError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            _ => Err(BytecodeReaderError::InvalidTypeId),
        }
    }
}

impl<'src> FromBytecodeReader<'src> for NumberType {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_u8()?.try_into()
    }
}

#[repr(u8)]
pub enum RefType {
    Funcref = 0x70,
    Externref = 0x6F,
}

impl std::convert::TryFrom<u8> for RefType {
    type Error = BytecodeReaderError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x70 => Ok(Self::Funcref),
            0x6F => Ok(Self::Externref),
            _ => Err(BytecodeReaderError::InvalidRefTypeId),
        }
    }
}

impl<'src> FromBytecodeReader<'src> for RefType {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_u8()?.try_into()
    }
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
    type Error = BytecodeReaderError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            0x70 => Ok(Self::Funcref),
            0x6F => Ok(Self::Externref),
            0x7B => Ok(Self::Vectype),
            _ => Err(BytecodeReaderError::InvalidValueTypeId(value)),
        }
    }
}

impl<'src> FromBytecodeReader<'src> for ValueType {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        reader.read_u8()?.try_into()
    }
}

#[derive(Debug)]
pub struct FunctionType {
    params: Box<[ValueType]>,
    results: Box<[ValueType]>,
}

impl FunctionType {
    pub fn new<P, R>(params: P, results: R) -> Self
    where
        P: IntoIterator<Item = ValueType>,
        R: IntoIterator<Item = ValueType>,
    {
        Self {
            params: params.into_iter().collect::<Vec<_>>().into(),
            results: results.into_iter().collect::<Vec<_>>().into(),
        }
    }
}

impl<'src> FromBytecodeReader<'src> for FunctionType {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        let magic = reader.read_u8()?;
        if magic != 0x60 {
            return Err(BytecodeReaderError::InvalidFunctionTypeEncoding);
        }
        let params = reader.read_vec_iter()?.collect::<Result<Vec<_>>>()?;
        let results = reader.read_vec_iter()?.collect::<Result<Vec<_>>>()?;

        Ok(Self {
            params: params.into_boxed_slice(),
            results: results.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub struct Limits {
    min: u32,
    max: Option<u32>,
}

impl<'src> FromBytecodeReader<'src> for Limits {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        match reader.read_u8()? {
            0x00 => Ok(Self {min: reader.read()?, max : None}),
            0x01 =>  Ok(Self {min: reader.read()?, max : Some(reader.read()?)}),
            _ => Err(BytecodeReaderError::InvalidLimits)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd)]
pub struct GlobalType {
    t: ValueType,
    mutable: bool,
}
impl<'src> FromBytecodeReader<'src> for GlobalType {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        Ok(Self {
            t: reader.read()?,
            mutable: reader.read()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum ImportDesc {
    TypeIdx(TypeId),
    TableType(Limits),
    MemType(Limits),
    GlobalType(GlobalType),
}
impl<'src> FromBytecodeReader<'src> for ImportDesc {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        let id = reader.read_u8()?;
        match id {
            0x00 => Ok(Self::TypeIdx(reader.read()?)),
            0x01 => Ok(Self::TableType(reader.read()?)),
            0x02 => Ok(Self::MemType(reader.read()?)),
            0x03 => Ok(Self::GlobalType(reader.read()?)),
            _ => Err(BytecodeReaderError::InvalidImportDesc(id)),
        }
    }
}

#[derive(Debug)]
pub struct Import<'src> {
    module: &'src str,
    name: &'src str,
    desc: ImportDesc,
}
impl<'src> FromBytecodeReader<'src> for Import<'src> {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        Ok(Self {
            module: reader.read()?,
            name: reader.read()?,
            desc: reader.read()?,
        })
    }
}



pub type LabelId = u32;
pub type FuncId = u32; 
pub type TypeId = u32;
pub type TableId = u32;
pub type LocalId = u32;
pub type GlobalId = u32;
pub type MemId = u32;

pub enum Reftype {
    Funcref,
    Externref
}
type ImportReader<'src> = BytecodeSubReader<'src, Import<'src>>; 
type FunctionReader<'src> = BytecodeSubReader<'src, TypeId>; 
type LimitsReader<'src> = BytecodeSubReader<'src, Limits>; 
type GlobalsReader<'src> = BytecodeSubReader<'src, Global>;
type ExportsReader<'src> = BytecodeSubReader<'src, Export<'src>>;
type FunctionBodyReader<'src> = BytecodeSubReader<'src, Function<'src>>;
#[derive(Debug)]
pub struct Global {
    t: GlobalType,
    init_expr: Box<[Op]>
}

impl<'src> FromBytecodeReader<'src> for Global {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        let t = reader.read::<GlobalType>()?;
        let init_expr = reader.read_const_expr_iter().collect::<Result<Vec<_>>>()?.into_boxed_slice();
        Ok(Global {t, init_expr})
    }
}

#[derive(Debug, PartialEq)]
pub enum ExportDesc {
    FuncId(FuncId),
    TableId(TableId),
    MemId(MemId),
    GlobalId(GlobalId),
}
impl<'src> FromBytecodeReader<'src> for ExportDesc {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        match reader.read_u8()? {
            0x00 => reader.read::<FuncId>().map(|f| Self::FuncId(f)),
            0x01 => reader.read().map(|f| Self::TableId(f)),
            0x02 => reader.read().map(|f| Self::MemId(f)),
            0x03 => reader.read().map(|f| Self::GlobalId(f)),
            _ => Err(BytecodeReaderError::InvalidExportDesc),
            
        }
    }
}

#[derive(Debug)]
pub struct Export<'src> {
    name: &'src str,
    desc: ExportDesc
}

impl<'src> FromBytecodeReader<'src> for Export<'src> {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        Ok(Self {name: reader.read()?, desc: reader.read()?})
    }
}

#[derive(Debug, Clone)]
pub struct Locals {
    n: u32,
    t: ValueType
}

impl<'src> FromBytecodeReader<'src> for Locals {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        let n: u32 = reader.read()?;
        println!("count: {n}"); 
        
        let t: ValueType = reader.read()?; 
        println!("t: {t}"); 
        Ok(Self {n, t})
    }
}

impl IntoIterator for Locals {
    type Item = ValueType;
    type IntoIter = LocalsIterator;

    fn into_iter(self) -> Self::IntoIter {
        LocalsIterator {locals: self, current_position: 0}
    }
}

pub struct LocalsIterator {
    locals: Locals,
    current_position: u32
}
impl<'me> Iterator for LocalsIterator {
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

#[derive(Debug)]
pub struct Function<'src> {
    locals: Box<[Locals]>,
    code: CodeReader<'src>
}
impl<'src> FromBytecodeReader<'src> for Function<'src> {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        println!("Reading function");
        let full_code_size = reader.read_var_u32()?;
        println!("Code size: {full_code_size}");
         
        let start_position = reader.current_position;
        println!("Reading locals");
        let locals = reader.read_vec_boxed_slice::<Locals>()?;
        println!("Done!"); 
        let locals_size = reader.current_position - start_position;
        let code_size = full_code_size as usize - locals_size;   

        let code_reader = CodeReader::new(&reader.current_buffer()[..code_size]);
        reader.skip_bytes(code_size)?; 
        Ok(Function {locals, code: code_reader})
    }
}

#[derive(Debug)]
pub enum SectionData<'src> {
    Type(Box<[FunctionType]>),
    Import(ImportReader<'src>),
    Function(FunctionReader<'src>),
    Table(LimitsReader<'src>),
    Memory(LimitsReader<'src>),
    Global(GlobalsReader<'src>),    
    Export(ExportsReader<'src>),
    Start(FuncId),
    DataCount(u32),
    Code(FunctionBodyReader<'src>),
}

#[derive(Debug)]
pub struct Section<'src> {
    size_bytes: usize,
    data: SectionData<'src>,
}


impl<'src> FromBytecodeReader<'src> for Section<'src> {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> Result<Self> {
        let section_id = reader.read_u8()?;
        let size_bytes = reader.read_var_u32()? as usize;

        let data: SectionData = match section_id {
            1 => SectionData::Type(reader.read_vec_boxed_slice()?),
            2 => SectionData::Import(reader.get_section_reader(size_bytes)?),
            3 => SectionData::Function(reader.get_section_reader(size_bytes)?),
            4 => SectionData::Table(reader.get_section_reader(size_bytes)?),
            5 => SectionData::Memory(reader.get_section_reader(size_bytes)?),
            6 => SectionData::Global(reader.get_section_reader(size_bytes)?),
            7 => SectionData::Export(reader.get_section_reader(size_bytes)?),
            8 => SectionData::Start(reader.read()?),
            10 => SectionData::Code(reader.get_section_reader(size_bytes)?),             
            12 => SectionData::DataCount(reader.read()?),
            _ => panic!("Unknown section id {}", section_id),
        };

        Ok(Section { size_bytes, data })
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn wasm_check_simple() -> Result<()> {
        let path = env::current_dir().unwrap();
        println!("Dir: {}", path.display());
        let wasm = fs::read("gen.wasm").expect("Unable to read file");
        let mut reader = BytecodeReader::new(wasm.as_slice());
        reader.check_header()?;
        let type_section = reader.read::<Section>()?;
        if let SectionData::Type(types) = type_section.data {
            assert!(types[0].params.len() == 2);
            assert!(types[1].params.len() == 1);
            assert!(types[2].params.len() == 1 && types[2].results.len() == 1);
        } else {
            panic!("Unexpected section");
        }

        println!("Type section done!");

        let import_section = reader.read::<Section>()?;
        if let SectionData::Import(imports) = import_section.data {
            let imports = imports.collect::<Result<Vec<_>>>()?.into_boxed_slice();
            assert!(imports[0].module == "env");
            assert!(imports[1].module == "env");
            assert!(imports[0].name == "print");
            assert!(imports[1].name == "printNum");
        } else {
            panic!("Unexpected section");
        }

        println!("Import section done!");
        let function_section = reader.read::<Section>()?;
        if let SectionData::Function(functions) = function_section.data {
            println!("Reading function section");
            let functions = functions.collect::<Result<Vec<_>>>()?.into_boxed_slice();
            assert!(functions[0] == 2);
            assert!(functions[1] == 3);
            assert!(functions[2] == 4);
        } else {
            panic!("Unexpected section");
        }

        let mem_section = reader.read::<Section>()?;
        match mem_section.data {
            SectionData::Memory(mem) => assert!(mem.collect::<Result<Vec<_>>>()?[0].min == 1),
            _ => panic!("Invalid memory section"),
        }

        let globals_section = reader.read::<Section>()?;
        if let SectionData::Global(globals) = globals_section.data {
            let globals = globals.collect::<Result<Vec<_>>>()?;
            assert!(globals[0].init_expr[0] == Op::I32Const(0));
            assert!(globals[0].t.mutable);
        } else {
            panic!("Invalid section");
        }

        let export_section = reader.read::<Section>()?;
        if let SectionData::Export(exports) = export_section.data {
            let exports = exports.collect::<Result<Vec<_>>>()?;    
            assert!(exports[0].name == "should_work");
            assert!(exports[0].desc == ExportDesc::FuncId(2));

            assert!(exports[1].name == "should_work1");
            assert!(exports[1].desc == ExportDesc::FuncId(3));

            assert!(exports[2].name == "should_work2");
            assert!(exports[2].desc == ExportDesc::FuncId(4));

        }
        
        let start_section = reader.read::<Section>()?; 
        if let SectionData::Start(start) = start_section.data {
            assert!(start == 6);
        } else {
            panic!("Invalid section");
        }

        let data_count_section = reader.read::<Section>()?;
        if let SectionData::DataCount(count) = data_count_section.data {
            assert!(count == 1);
        } else {
            panic!("Invalid section");
        }

        let code_section = reader.read::<Section>()?;
        if let SectionData::Code(code) = code_section.data {
            let mut code = code.collect::<Result<Vec<_>>>()?;
            assert!(code[0].locals[0].n == 1);
            assert!(code[0].code.next() == Some(Ok(Op::I32Const(1))));
        } else {
            panic!("Invalid section");
        }
        Ok(())
         
    }
}
