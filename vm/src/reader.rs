use core::{
    fmt::{self},
    str,
};
use itertools::Itertools;
use std::{io::Read, marker::PhantomData};

use crate::{op::Op, types::*};
use crate::types::GlobalType;

//NOTE: (joh) Inspiriert von: https://github.com/bytecodealliance/wasm-tools/blob/main/crates/wasmparser/src/binary_reader.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ReaderError {
    InvalidLeb,
    EndOfBuffer,
    InvalidUtf8InName(std::str::Utf8Error),
    InvalidBool,
    InvalidTypeId,
    InvalidRefTypeId,
    InvalidValueTypeId(u8),
    InvalidHeaderMagicNumber,
    InvalidWasmVersion,
    InvalidFunctionTypeEncoding(u8),
    InvalidImportDesc(u8),
    UnimplementedOpcode(u8),
    ExpectedConstExpression(Op),
    InvalidLimits,
    InvalidExportDesc,
    MalformedCodeSection,
    InvalidDataMode(u32),
    DataIsNotStringLiteral,
    StringLiteralIsNotValidUtf(std::str::Utf8Error),
}

pub type Result<T, E = ReaderError> = core::result::Result<T, E>;
pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::InvalidLeb => write!(f, "Invalid leb128 formated number"),
            ReaderError::EndOfBuffer => write!(f, "Reached end of buffer"),
            ReaderError::InvalidUtf8InName(utf8_error) => utf8_error.fmt(f),
            ReaderError::InvalidBool => write!(f, "Invalid boolean encoding"),
            ReaderError::InvalidTypeId => write!(f, "Invalid Type id"),
            ReaderError::InvalidRefTypeId => write!(f, "Invalid ref type id"),
            ReaderError::InvalidValueTypeId(id) => {
                write!(f, "Invalid value type id: {}", id)
            }
            ReaderError::InvalidHeaderMagicNumber => todo!(),
            ReaderError::InvalidWasmVersion => todo!(),
            ReaderError::InvalidFunctionTypeEncoding(_) => todo!(),
            ReaderError::InvalidImportDesc(id) => {
                write!(f, "Invalid import desc id: {}", id)
            }
            ReaderError::UnimplementedOpcode(_) => todo!(),
            ReaderError::ExpectedConstExpression(op) => todo!(),
            ReaderError::InvalidLimits => todo!(),
            ReaderError::InvalidExportDesc => todo!(),
            ReaderError::MalformedCodeSection => todo!(),
            ReaderError::InvalidDataMode(_) => todo!(),
            ReaderError::DataIsNotStringLiteral => todo!(),
            ReaderError::StringLiteralIsNotValidUtf(utf8_error) => todo!(),
        }
    }
}

impl From<std::str::Utf8Error> for ReaderError {
    fn from(value: std::str::Utf8Error) -> Self {
        ReaderError::InvalidUtf8InName(value)
    }
}

pub trait FromReader<'src>: Sized {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self>;
}

impl<'src> FromReader<'src> for bool {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        match reader.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ReaderError::InvalidBool),
        }
    }
}

impl<'src> FromReader<'src> for u32 {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_var_u32()
    }
}

impl<'src> FromReader<'src> for usize {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        Ok(reader.read_var_u32()? as usize)
    }
}

impl<'src> FromReader<'src> for i32 {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_var_i32()
    }
}

impl<'src> FromReader<'src> for u8 {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_u8()
    }
}
impl<'src> FromReader<'src> for u64 {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_var_u64()
    }
}
impl<'src> FromReader<'src> for i64 {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_var_i64()
    }
}

impl<'src> FromReader<'src> for &'src str {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_name()
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
impl<'src> FromReader<'src> for GlobalType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        Ok(Self {
            t: reader.read_with_position()?,
            mutable: reader.read_with_position()?,
        })
    }
}

pub trait FixedBinarySize<'src>: Sized {
    fn size_from_reader(reader: &mut Reader) -> Result<usize>;
}
impl FixedBinarySize<'_> for u8 {
    fn size_from_reader(_: &'_ mut Reader) -> Result<usize> {
        Ok(size_of::<u8>())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Position {
    pub offset: usize,
    pub len: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset: {}, len: {}", self.offset, self.len)
    }
}
#[derive(Debug, Clone)]
pub struct Reader<'src> {
    buffer: &'src [u8],
    current_position: usize,
    start_position: usize,
}

impl<'src> Reader<'src> {
    pub fn new(buffer: &'src [u8], start_position: usize) -> Self {
        Self {
            buffer,
            current_position: 0,
            start_position,
        }
    }
    pub fn from_reader(reader: &Reader<'src>) -> Self {
        Reader {
            buffer: reader.buffer,
            current_position: reader.current_position,
            start_position: reader.current_position,
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
            Err(ReaderError::EndOfBuffer)
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

    pub fn read_bytes(&mut self, size: usize) -> Result<(&'src [u8], Position)> {
        self.can_read_bytes(size)?;
        let position = Position {
            offset: self.current_position,
            len: size,
        };
        let new_pos = self.current_position + size;
        let res = &self.buffer[self.current_position..new_pos];

        self.current_position = new_pos;
        Ok((res, position))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.read_bytes(size_of::<u32>())?.0.try_into().unwrap(),
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
                return Err(ReaderError::InvalidLeb);
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
                return Err(ReaderError::InvalidLeb);
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
                    return Err(ReaderError::InvalidLeb);
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
                    return Err(ReaderError::InvalidLeb);
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
                    return Err(ReaderError::InvalidLeb);
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
        Ok(str::from_utf8(self.read_bytes(size)?.0)?)
    }

    pub fn read_name(&mut self) -> Result<&'src str> {
        let len = self.read::<u32>()? as usize;
        self.read_sized_name(len)
    }

    pub fn check_header(&mut self) -> Result<(Position, Position)> {
        let (header, header_pos) = self.read_bytes(4)?;
        if header != WASM_HEADER_MAGIC {
            return Err(ReaderError::InvalidHeaderMagicNumber);
        }
        let (version, version_pos) = self.read_bytes(4)?;
        if version[0] != 1 {
            return Err(ReaderError::InvalidWasmVersion);
        }

        Ok((header_pos, version_pos))
    }

    pub fn read<T>(&mut self) -> Result<T>
    where
        T: FromReader<'src>,
    {
        T::from_reader(self)
    }

    pub fn read_with_position<T>(&mut self) -> Result<(T, Position)>
    where
        T: FromReader<'src>,
    {
        let start = self.current_position;
        let elem = T::from_reader(self)?;
        let bytes_read = self.current_position - start;
        Ok((
            elem,
            Position {
                offset: start + self.start_position,
                len: bytes_read,
            },
        ))
    }

    pub fn read_with_slice<T>(&mut self) -> Result<(T, &'src [u8])>
    where
        T: FromReader<'src>,
    {
        let start = self.current_position;
        let elem = T::from_reader(self);
        Ok((elem?, &self.buffer[start..self.current_position]))
    }

    pub fn read_vec_iter<'me, T>(&'me mut self) -> Result<VecIter<'src, 'me, T>>
    where
        T: FromReader<'src>,
    {
        let size = self.read_var_u32()? as usize;
        Ok(VecIter {
            count: size,
            current_position: 0,
            done: false,
            reader: self,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn read_vec_boxed_slice<T>(&mut self) -> Result<Box<[(T, Position)]>>
    where
        T: FromReader<'src>,
    {
        Ok(self
            .read_vec_iter()?
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice())
    }

    pub fn read_vec_bytes(&mut self) -> Result<(&'src [u8], Position)> {
        let size = self.read_var_u32()? as usize;
        self.read_bytes(size)
    }

    //NOTE: (joh): Ich mag das size argument hier nicht so gerne. es wäre schöner, wenn das hier
    //gelesen werden könnte
    pub fn get_section_reader<T>(&mut self, size: usize) -> Result<SubReader<'src, T>>
    where
        T: FromReader<'src>,
    {
        if self.current_position + size > self.buffer.len() {
            Err(ReaderError::EndOfBuffer)
        } else {
            let slice: &'src [u8] = &self.buffer[self.current_position..self.current_position + size];
            let mut reader = Reader::new(slice, self.current_position);
            let count = reader.read_var_u32()? as usize;
            self.skip_bytes(size)?;
            Ok(SubReader::from_reader(reader, count))
        }

    }

    pub fn read_const_expr_iter<'me>(&'me mut self) -> ConstantExprIter<'src, 'me> {
        ConstantExprIter {
            current_position: 0,
            done: false,
            reader: self,
        }
    }

    pub fn sections_iter<'me>(&'me mut self) -> SectionsIter<'src, 'me> {
        SectionsIter { reader: self }
    }

    pub fn data_at(&self, position: Position) -> &'src [u8] {
        return &self.buffer[position.offset..position.offset + position.len];
    }
}

pub struct SectionsIter<'src, 'me> {
    reader: &'me mut Reader<'src>,
}

impl<'src, 'me> Iterator for SectionsIter<'src, 'me> {
    type Item = Result<(Section<'src>, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.can_read_bytes(1).ok()?;
        Some(self.reader.read_with_position::<Section>())
    }
}

pub struct VecIter<'src, 'me, T: FromReader<'src>> {
    count: usize,
    current_position: usize,
    done: bool,
    reader: &'me mut Reader<'src>,
    _marker: std::marker::PhantomData<T>,
}

impl<'src, 'me, T: FromReader<'src>> Iterator for VecIter<'src, 'me, T> {
    type Item = Result<(T, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position >= self.count || self.done {
            None
        } else {
            let res = self.reader.read_with_position::<T>();
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
    reader: &'me mut Reader<'src>,
}
impl<'src, 'me> Iterator for ConstantExprIter<'src, 'me> {
    type Item = Result<(Op, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            match self.reader.read_with_position::<Op>() {
                Err(e) => {
                    self.done = true;
                    Some(Err(e))
                }
                Ok((Op::End, p)) => {
                    self.done = true;
                    Some(Ok((Op::End, p)))
                }

                Ok((op, p)) => {
                    if op.is_const() {
                        self.current_position += 1;
                        Some(Ok((op, p)))
                    } else {
                        self.done = true;
                        Some(Err(ReaderError::ExpectedConstExpression(op)))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubReader<'src, T: FromReader<'src>> {
    reader: Reader<'src>,
    count: usize,
    read: usize,

    _marker: PhantomData<T>,
}

impl<'src, T: FromReader<'src>> SubReader<'src, T> {
    pub fn from_reader(reader: Reader<'src>, count: usize) -> Self {
        SubReader {
            reader,
            count,
            read: 0,
            _marker: PhantomData,
        }
    }
}
impl<'src, T: FromReader<'src> + FixedBinarySize<'src>> SubReader<'src, T> {
    pub fn from_vec(reader: &mut Reader<'src>) -> Result<Self> {
        let count = reader.read_var_u32()? as usize;
        let mut new_reader = Reader::from_reader(reader);
        let size_bytes = T::size_from_reader(&mut new_reader)? * count;
        reader.skip_bytes(size_bytes)?;
        Ok(Self::from_reader(new_reader, count))
    }
}

impl<'src, T: FromReader<'src>> SubReader<'src, T> {
    pub fn read_with_slice(&mut self) -> Option<Result<(T, &'src [u8])>> {
        if self.read < self.count {
            let elem = self.reader.read_with_slice::<T>();
            self.read += 1;
            Some(elem)
        } else {
            None
        }
    }
    pub fn read_with_position(&mut self) -> Option<Result<(T, Position)>> {
        if self.read < self.count {
            let elem = self.reader.read_with_position::<T>();
            self.read += 1;
            Some(elem)
        } else {
            None
        }
    }

    pub fn read(&mut self) -> Option<Result<T>> {
        Some(self.read_with_slice()?.map(|s| s.0))
    }
    pub fn iter_with_slice<'me>(&'me mut self) -> SubReaderSliceIter<'src, 'me, T> {
        SubReaderSliceIter(self)
    }

    pub fn iter_with_position<'me>(&'me mut self) -> SubReaderPositionIter<'src, 'me, T> {
        SubReaderPositionIter(self)
    }
}

impl<'src, T: FromReader<'src>> Iterator for SubReader<'src, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.count - self.read, Some(self.count - self.read));
    }
}

pub struct SubReaderSliceIter<'src, 'me, T: FromReader<'src>>(&'me mut SubReader<'src, T>);
impl<'src, 'me, T: FromReader<'src>> Iterator for SubReaderSliceIter<'src, 'me, T> {
    type Item = Result<(T, &'src [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.read_with_slice()
    }
}

pub struct SubReaderPositionIter<'src, 'me, T: FromReader<'src>>(&'me mut SubReader<'src, T>);
impl<'src, 'me, T: FromReader<'src>> Iterator for SubReaderPositionIter<'src, 'me, T> {
    type Item = Result<(T, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.read_with_position()
    }
}

#[derive(Debug, Clone)]
pub struct CodeReader<'src> {
    reader: Reader<'src>,
    depth: usize,
    instructions_read: usize,
    done: bool,
}

impl<'src> CodeReader<'src> {
    pub fn new(reader: Reader<'src>) -> Self {
        CodeReader {
            reader,
            depth: 0,
            instructions_read: 0,
            done: false,
        }
    }
}

impl<'src> Iterator for CodeReader<'src> {
    type Item = Result<(Op, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let op = self.reader.read_with_position::<Op>();
            match op {
                Err(e) => {
                    self.done = true;
                    //TODO: (joh) Überprüfe ob EOF, falls ja: Malformed
                    Some(Err(e))
                }
                Ok((Op::End, _)) => {
                    if self.depth == 0 {
                        self.done = true;
                    } else {
                        self.depth -= 1;
                    }
                    Some(op)
                }

                Ok((ref opcode, _)) if opcode.needs_end_terminator() => {
                    self.depth += 1;
                    Some(op)
                }
                Ok(op) => Some(Ok(op)),
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
    type Error = ReaderError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            0x7D => Ok(Self::F32),
            0x7C => Ok(Self::F64),
            _ => Err(ReaderError::InvalidTypeId),
        }
    }
}

impl<'src> FromReader<'src> for NumberType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_u8()?.try_into()
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            NumberType::I32 => "i32",
            NumberType::I64 => "i64",
            NumberType::F32 => "f32",
            NumberType::F64 => "f64",
        };
        f.write_str(str)
    }
}

#[repr(u8)]
pub enum RefType {
    Funcref = 0x70,
    Externref = 0x6F,
}

impl fmt::Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RefType::Funcref => "Funcref",
            RefType::Externref => "Externref",
        };
        f.write_str(str)
    }
}

impl std::convert::TryFrom<u8> for RefType {
    type Error = ReaderError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x70 => Ok(Self::Funcref),
            0x6F => Ok(Self::Externref),
            _ => Err(ReaderError::InvalidRefTypeId),
        }
    }
}

impl<'src> FromReader<'src> for RefType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
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
impl ValueType {
    pub fn is_num(&self) -> bool {
        match self {
            ValueType::I32 | 
            ValueType::I64 | 
            ValueType::F32 |
            ValueType::F64 => true,
            _ => false,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self {
            ValueType::Vectype => true,
            _ => false
        }
    }
    pub fn is_ref(&self) -> bool {
        match self {
            ValueType::Funcref | ValueType::Externref => true,
            _ => false
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

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
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

impl<'src> FromReader<'src> for ValueType {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        reader.read_u8()?.try_into()
    }
}

impl FixedBinarySize<'_> for ValueType {
    fn size_from_reader(_: &'_ mut Reader) -> Result<usize> {
        Ok(size_of::<u8>())
    }
}

#[derive(Debug)]
pub struct FunctionType<'src> {
    pub params: SubReader<'src, ValueType>,
    pub results: SubReader<'src, ValueType>,
}

impl<'src> FromReader<'src> for FunctionType<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        let magic = reader.read_u8()?;
        if magic != 0x60 {
            return Err(ReaderError::InvalidFunctionTypeEncoding(magic));
        }

        let params = SubReader::from_vec(reader)?;
        let results = SubReader::from_vec(reader)?;

        Ok(Self { params, results })
    }
}

impl<'src> fmt::Display for FunctionType<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //NOTE: (joh): Definitv unnoetige Allokationen hier
        let p = self
            .params
            .clone()
            .map(|r| r.map_or_else(|e| e.to_string(), |f| f.to_string()));
        let r = self
            .results
            .clone()
            .map(|r| r.map_or_else(|e| e.to_string(), |f| f.to_string()));

        write!(f, "({}) -> ({})", p.format(", "), r.format(","))
    }
}


impl<'src> FromReader<'src> for ImportDesc {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
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

#[derive(Debug)]
pub struct Import<'src> {
    pub module: (&'src str, Position),
    pub name: (&'src str, Position),
    pub desc: (ImportDesc, Position),
}

impl<'src> FromReader<'src> for Import<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        Ok(Self {
            module: reader.read_with_position()?,
            name: reader.read_with_position()?,
            desc: reader.read_with_position()?,
        })
    }
}
impl<'src> fmt::Display for Import<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {} {})", self.module.0, self.name.0, self.desc.0)
    }
}


pub enum Reftype {
    Funcref,
    Externref,
}
pub type TypeReader<'src> = SubReader<'src, FunctionType<'src>>;
pub type ImportReader<'src> = SubReader<'src, Import<'src>>;
pub type FunctionReader<'src> = SubReader<'src, TypeId>;
pub type LimitsReader<'src> = SubReader<'src, Limits>;
pub type GlobalsReader<'src> = SubReader<'src, Global>;
pub type ExportsReader<'src> = SubReader<'src, Export<'src>>;
pub type FunctionBodyReader<'src> = SubReader<'src, Function<'src>>;
pub type DataReader<'src> = SubReader<'src, Data<'src>>;


impl<'src> FromReader<'src> for Global {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        let t = reader.read_with_position::<GlobalType>()?;
        let init_expr = reader
            .read_const_expr_iter()
            .collect::<Result<Vec<_>>>()?
            .into_boxed_slice();
        Ok(Global { t, init_expr })
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
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
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

#[derive(Debug)]
pub struct Export<'src> {
    pub name: (&'src str, Position),
    pub desc: (ExportDesc, Position),
}

impl<'src> FromReader<'src> for Export<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        Ok(Self {
            name: reader.read_with_position()?,
            desc: reader.read_with_position()?,
        })
    }
} 

impl fmt::Display for Export<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name.0, self.desc.0)
    }
}

impl<'src> FromReader<'src> for Locals {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        let n: u32 = reader.read()?;

        let t: ValueType = reader.read()?;
        Ok(Self { n, t })
    }
}


#[derive(Debug)]
pub struct Function<'src> {
    pub locals: Box<[(Locals, Position)]>,
    pub code: CodeReader<'src>,
}
impl<'src> FromReader<'src> for Function<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        let full_code_size = reader.read_var_u32()?;

        let start_position = reader.current_position;
        let locals = reader.read_vec_boxed_slice::<Locals>()?;
        let locals_size = reader.current_position - start_position;
        let code_size = full_code_size as usize - locals_size;
        let buffer = &reader.current_buffer()[..code_size];
        let new_reader = Reader::new(buffer, reader.start_position + reader.current_position);
        let code_reader = CodeReader::new(new_reader);

        reader.skip_bytes(code_size)?;
        Ok(Function {
            locals,
            code: code_reader,
        })
    }
}
impl fmt::Display for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for locals in self.locals.iter().map(|l| l.0.clone()) {
            write!(f, "Locals: ({})\n", locals.clone().into_iter().format(" ,"))?;
        }
        for res in self.code.clone() {
            if let Ok((op, slice)) = res {
                write!(f, "{op} : {:#04x?}\n", slice)?;
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum Data<'src> {
    Active {
        mem_id: MemId,
        expr: Box<[(Op, Position)]>,
        data: (&'src [u8], Position),
    },
    Passive((&'src [u8], Position)),
}

impl<'src> FromReader<'src> for Data<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        match reader.read_var_u32()? {
            0 => Ok(Self::Active {
                mem_id: 0,
                expr: reader
                    .read_const_expr_iter()
                    .collect::<Result<Vec<_>>>()?
                    .into_boxed_slice(),
                data: reader.read_vec_bytes()?,
            }),
            1 => Ok(Self::Passive(reader.read_vec_bytes()?)),
            2 => Ok(Self::Active {
                mem_id: reader.read()?,
                expr: reader
                    .read_const_expr_iter()
                    .collect::<Result<Vec<_>>>()?
                    .into_boxed_slice(),
                data: reader.read_vec_bytes()?,
            }),
            n => Err(ReaderError::InvalidDataMode(n)),
        }
    }
}

impl<'src> TryFrom<&'src Data<'src>> for &'src str {
    type Error = ReaderError;

    fn try_from(value: &'src Data<'src>) -> std::result::Result<Self, Self::Error> {
        match value {
            Data::Active {
                mem_id: _,
                expr: _,
                data,
            }
            | Data::Passive(data) => {
                let size = u32::from_le_bytes(
                    data.0[0..4]
                        .try_into()
                        .map_err(|_| ReaderError::DataIsNotStringLiteral)?,
                );
                println!("size {size}, bin: {:#x}", size);
                str::from_utf8(&data.0[4..4 + size as usize])
                    .map_err(ReaderError::StringLiteralIsNotValidUtf)
            }
        }
    }
}

#[derive(Debug)]
pub struct CustomSectionData<'src> {
    pub name: (&'src str, Position),
    pub data: (&'src [u8], Position),
}

#[derive(Debug)]
pub enum SectionData<'src> {
    Custom(CustomSectionData<'src>),
    Type(TypeReader<'src>),
    Import(ImportReader<'src>),
    Function(FunctionReader<'src>),
    Table(LimitsReader<'src>),
    Memory(LimitsReader<'src>),
    Global(GlobalsReader<'src>),
    Export(ExportsReader<'src>),
    Start((FuncId, Position)),
    DataCount((u32, Position)),
    Code(FunctionBodyReader<'src>),
    Data(DataReader<'src>),
}

#[derive(Debug)]
pub struct Section<'src> {
    pub size_bytes: usize,
    pub data: SectionData<'src>,
}

impl<'src> FromReader<'src> for Section<'src> {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        let section_id = reader.read_u8()?;
        let size_bytes = reader.read_var_u32()? as usize;

        let data: SectionData = match section_id {
            0 => {
                let start = reader.current_position;
                let name = reader.read_with_position::<&str>()?;
                let size_bytes = reader.current_position - start;
                SectionData::Custom(CustomSectionData {
                    name,
                    data: reader.read_bytes(size_bytes)?,
                })
            }
            1 => SectionData::Type(reader.get_section_reader(size_bytes)?),
            2 => SectionData::Import(reader.get_section_reader(size_bytes)?),
            3 => SectionData::Function(reader.get_section_reader(size_bytes)?),
            4 => SectionData::Table(reader.get_section_reader(size_bytes)?),
            5 => SectionData::Memory(reader.get_section_reader(size_bytes)?),
            6 => SectionData::Global(reader.get_section_reader(size_bytes)?),
            7 => SectionData::Export(reader.get_section_reader(size_bytes)?),
            8 => SectionData::Start(reader.read_with_position()?),
            10 => SectionData::Code(reader.get_section_reader(size_bytes)?),
            11 => SectionData::Data(reader.get_section_reader(size_bytes)?),
            12 => SectionData::DataCount(reader.read_with_position()?),
            _ => panic!("Unknown section id {}", section_id),
        };

        Ok(Section { size_bytes, data })
    }
}


/*
pub struct Module<'src> {
    pub header: Position,
    pub version: Position,
    pub type_section: Option<TypeReader<'src>>,  
    pub import_section: Option<ImportReader<'src>>,
    pub function_section: Option<FunctionReader<'src>>,
    pub table_section: Option<LimitsReader<'src>>,
    pub memory_section: Option<LimitsReader<'src>>,
    pub global_section: Option<GlobalsReader<'src>>,
    pub export_section: Option<ExportsReader<'src>>,
    pub start_section: Option<u32>,
    pub code_section: Option<FunctionBodyReader<'src>>,
    pub data_section: Option<DataReader<'src>>
}
*/

#[cfg(test)]
mod tests {
    use std::{env, fs, iter};

    use super::*;

    fn get_wasm_gen() -> Box<[u8]> {
        let source = include_str!("wat/gen.wat");
        let source = wat::parse_str(source).unwrap().into_boxed_slice();
        fs::write("gen2.wasm", &source).unwrap();
        source
    }

    #[test]
    fn wasm_check_section_iter() -> Result<(), ReaderError> {
        let wasm = get_wasm_gen();
        let mut reader = Reader::new(&wasm, 0);
        reader.check_header()?;
        reader.sections_iter().collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    #[test]
    fn wasm_check_simple() -> Result<()> {
        let path = env::current_dir().unwrap();
        println!("Dir: {}", path.display());
        let wasm = get_wasm_gen();
        let mut reader = Reader::new(&wasm, 0);
        reader.check_header()?;

        for s in reader.sections_iter() {
            match s?.0.data {
                SectionData::Type(sub_reader) => {
                    let types = sub_reader.collect::<Result<Vec<_>>>()?;
                    assert!(types[0].params.count == 2);
                    assert!(types[1].params.count == 1);
                    println!("{}", types.iter().format("\n"))
                }
                SectionData::Import(mut sub_reader) => {
                    let i = sub_reader.next().unwrap()?;
                    assert!(i.module.0 == "env");
                    assert!(i.name.0 == "print");

                    let i = sub_reader.next().unwrap()?;
                    assert!(i.module.0 == "env");
                    assert!(i.name.0 == "printNum");
                }
                SectionData::Function(sub_reader) => {
                    let functions = sub_reader.collect::<Result<Vec<_>>>()?.into_boxed_slice();
                    assert!(functions[0] == 2);
                    assert!(functions[1] == 3);
                    assert!(functions[2] == 4);
                }
                SectionData::Table(sub_reader) => todo!(),
                SectionData::Memory(mut sub_reader) => {
                    let mem = sub_reader.next().unwrap()?;
                    assert!(mem.min.0 == 1);
                }
                SectionData::Global(mut sub_reader) => {
                    let global = sub_reader.next().unwrap()?;
                    assert!(global.init_expr[0].0 == Op::I32Const(0));
                }
                SectionData::Export(mut sub_reader) => {
                    let export = sub_reader.next().unwrap()?;
                    assert!(export.name.0 == "should_work");
                    assert!(export.desc.0 == ExportDesc::FuncId(2));
                    let export = sub_reader.next().unwrap()?;

                    assert!(export.name.0 == "should_work1");
                    assert!(export.desc.0 == ExportDesc::FuncId(3));
                    let export = sub_reader.next().unwrap()?;

                    assert!(export.name.0 == "should_work2");
                    assert!(export.desc.0 == ExportDesc::FuncId(4));
                }
                SectionData::Start(start) => {
                    assert!(start.0 == 6);
                }
                SectionData::DataCount(_) => todo!(),
                SectionData::Code(mut sub_reader) => {
                    let mut code = sub_reader.next().unwrap()?;
                    assert!(code.locals[0].0.n == 1);
                    assert!(code.code.next().unwrap().unwrap().0 == (Op::I32Const(1)));
                }
                SectionData::Data(mut sub_reader) => {
                    let data = sub_reader.next().unwrap()?;
                    if let Data::Active {
                        mem_id,
                        expr,
                        data: _bytes,
                    } = &data
                    {
                        assert!(*mem_id == 0);
                        assert!(expr[0].0 == Op::I32Const(0));
                        let str: &str = (&data).try_into()?;
                        assert!(str == "blubbi");
                    }
                }
                SectionData::Custom(data) => todo!(),
            }
        }
        Ok(())
    }
    #[test]
    fn print_raw_bytecode() -> Result<()> {
        let wasm = get_wasm_gen();
        let mut reader = Reader::new(&wasm, 0);
        reader.check_header()?;
        for s in reader.sections_iter() {
            let section = s?;
            let slice = section.1;
            match section.0.data {
                SectionData::Custom(custom_section_data) => todo!(),
                SectionData::Type(mut sub_reader) => {
                    println!("Type section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("{}:\n{:#04x?}", t, data)
                        }
                    }
                    let types = sub_reader.collect::<Result<Vec<_>>>()?.into_boxed_slice();
                    println!("Types: {}", types.iter().format(" ,"));
                }
                SectionData::Import(mut sub_reader) => {
                    println!("Import section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("{}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Function(mut sub_reader) => {
                    println!("Function section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("Function {}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Table(mut sub_reader) => {
                    println!("Table section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("Table {}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Memory(mut sub_reader) => {
                    println!("Memory section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("Memory {}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Global(mut sub_reader) => {
                    println!("Global section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("Global {}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Export(mut sub_reader) => {
                    println!("Export section: {:0x?}", slice);
                    for res in sub_reader.iter_with_position() {
                        if let Ok((t, data)) = res {
                            println!("Export {}:\n{:#04x?}", t, data);
                        }
                    }
                }
                SectionData::Start(s) => {
                    println!("Start section: {:0x?}", s);
                }
                SectionData::DataCount(_) => todo!(),
                SectionData::Code(mut sub_reader) => {
                    println!("Code section: {:0x?}", slice);
                    for code in sub_reader.iter_with_position() {
                        if let Ok((func, slice)) = code {
                            println!("Function: {func}");
                        }
                    }
                }
                SectionData::Data(sub_reader) => {
                    println!("Data section!");
                }
            }
        }
        Ok(())
    }
}
