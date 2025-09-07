use crate::{
    parser::{ParseError, RawSectionInfo, SectionId, SplitSectionReader},
    types::{Limits, RawImportDesc, Reftype, Type, ValueType},
};
use bumpalo::{
    Bump,
    boxed::{self, Box},
    collections::{self, CollectIn, String},
};
use core::{
    fmt::Display,
    ops::{Index, Range},
    str::Utf8Error,
};
use itertools::Itertools;
use log::{info, trace};
use thiserror::Error;
#[derive(Error, Clone, Debug)]

pub enum ReadErrorType {
    #[error("Parsing Error")]
    ParseError(#[from] ParseError),

    #[error("Tried reading beyond buffer size")]
    ReaderDone,

    #[error("Invalid boolean encoding")]
    InvalidBool(u8),

    #[error("Invalid limit type encoding: {0}")]
    InvalidLimitsEncoding(u8),

    #[error("Invalid value type encoding: {0}")]
    InvalidValueTypeEncoding(u8),

    #[error("Invalid string encoding")]
    InvalidStringEncoding(#[from] Utf8Error),

    #[error("Invalid Header")]
    InvalidHeader([u8; 4]),

    #[error("Invalid Version")]
    InvalidVersion([u8; 4]),

    #[error("Invalid setion id")]
    InvalidSectionId(u8),

    #[error("Invalid import kind encoding")]
    InvalidImportKind(u8),

    #[error("Invalid ref type encoding")]
    InvalidRefType(u8),

}

#[derive(Error, Debug, Clone)]
pub struct ReadError {
    position: usize,
    #[source]
    t: ReadErrorType,
}
impl ReadError {
    pub fn eof(&self) -> bool {
        match self.t {
            ReadErrorType::ReaderDone => true,
            _ => false,
        }
    }
    pub fn from_reader(reader: &BufferReader, t: ReadErrorType) -> Self {
        ReadError {
            position: reader.position(),
            t,
        }
    }
}
impl Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ReadError at {}: {}", self.position, self.t)
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

/*
pub trait ReadBytecode<'src>: Sized {
    fn bytes_left(&self) -> usize;
    fn position(&self) -> usize;

    fn bytes(&mut self, size: usize) -> Result<&'src [u8], ReadError>;
    fn skip(&mut self, size: usize) -> Result<(), ReadError>;
    fn read_u8(&mut self) -> Result<u8, ReadError>;
    fn read_u16(&mut self) -> Result<u16, ReadError>;
    fn read_u32(&mut self) -> Result<u32, ReadError>;
    fn read_u64(&mut self) -> Result<u64, ReadError>;

    fn read_i8(&mut self) -> Result<i8, ReadError>;
    fn read_i16(&mut self) -> Result<i16, ReadError>;
    fn read_i32(&mut self) -> Result<i32, ReadError>;
    fn read_i64(&mut self) -> Result<i64, ReadError>;

    fn read_f32(&mut self) -> Result<f32, ReadError>;
    fn read_f64(&mut self) -> Result<f64, ReadError>;

    fn can_read_bytes(&self, bytes: usize) -> Result<(), ReadError> {
        if self.bytes_left() >= bytes {
            Err(ReadError::from_reader(self, ReadErrorType::ReaderDone))
        } else {
            Ok(())
        }
    }
    fn can_read<T>(&self) -> Result<(), ReadError> {
        self.can_read_bytes(size_of::<T>())
    }

    fn try_with_position<T, F>(&mut self, op: F) -> Result<WithPosition<T>, ReadError>
    where
        F: FnOnce(&mut Self) -> Result<T, ReadError>,
    {
        let start = self.position();
        let res = op(self)?;
        let end = self.position();
        Ok(WithPosition::new(res, start..end))
    }

    fn read_in<'a, T: FromBytecodeIn<'a>>(&mut self, alloc: &'a Bump) -> Result<T, ReadError> {
        T::read_from_bytecode_in(alloc, self)
    }

    fn iter_vec_item_in<'a, T: FromBytecodeIn<'a>>(
        &mut self,
        alloc: &'a Bump,
    ) -> Result<impl Iterator<Item = Result<T, ReadError>>, ReadError> {
        let len = self.read_leb_u32()?;
        Ok((0..len).map(|_| self.read_in::<T>(alloc)))
    }

    fn read_vec_in<'a, T: FromBytecodeIn<'a>>(
        &mut self,
        alloc: &'a Bump,
    ) -> Result<Box<'a, [T]>, ReadError> {
        trace!("Reading Vec");
        self.iter_vec_item_in(alloc)?.collect_in(alloc)
    }
}
*/

macro_rules! impl_read_op {
    ($name: ident, $t: tt) => {
        pub fn $name(&mut self) -> Result<$t, ReadError> {
            self.can_read::<$t>()?;
            let res = $t::from_le_bytes(
                self.buffer[self.pos..self.pos + size_of::<$t>()]
                    .try_into()
                    .unwrap(),
            );
            self.pos += size_of::<$t>();
            Ok(res as $t)
        }
    };
}
#[derive(Clone, Debug)]
pub struct BufferReader<'src> {
    buffer: &'src [u8],
    pos: usize,
}
pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";
pub const WASM_HEADER_VERSION: &[u8; 4] = &[1, 0, 0, 0];

impl<'src> BufferReader<'src> {
    pub fn new(buffer: &'src [u8]) -> Self {
        BufferReader { buffer, pos: 0 }
    }

    pub fn from_buffer(buffer: &'src [u8], pos: usize) -> Self {
        BufferReader { buffer, pos }
    }
    impl_read_op!(read_u8, u8);
    impl_read_op!(read_u16, u16);
    impl_read_op!(read_u32, u32);
    impl_read_op!(read_u64, u64);

    impl_read_op!(read_i8, i8);
    impl_read_op!(read_i16, i16);
    impl_read_op!(read_i32, i32);
    impl_read_op!(read_i64, i64);
    impl_read_op!(read_f32, f32);
    impl_read_op!(read_f64, f64);

    pub fn bytes_left(&self) -> usize {
        self.buffer.len() - self.pos
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn bytes(&mut self, size: usize) -> Result<&'src [u8], ReadError> {
        let result = self
            .buffer
            .get(self.pos..self.pos + size)
            .ok_or_else(|| ReadError::from_reader(self, ReadErrorType::ReaderDone))?;
        self.pos += 1;
        Ok(result)
    }

    pub fn skip(&mut self, size: usize) -> Result<(), ReadError> {
        trace!("Skipping bytes: {}", size);
        self.can_read_bytes(size)?;
        self.pos += 1;
        Ok(())
    }

    pub fn can_read_bytes(&self, bytes: usize) -> Result<(), ReadError> {
        if self.bytes_left() >= bytes {
            Err(ReadError::from_reader(self, ReadErrorType::ReaderDone))
        } else {
            Ok(())
        }
    }
    pub fn can_read<T>(&self) -> Result<(), ReadError> {
        self.can_read_bytes(size_of::<T>())
    }

    pub fn try_with_position<T, F>(&mut self, op: F) -> Result<WithPosition<T>, ReadError>
    where
        F: FnOnce(&mut Self) -> Result<T, ReadError>,
    {
        let start = self.position();
        let res = op(self)?;
        let end = self.position();
        Ok(WithPosition::new(res, start..end))
    }

    pub fn read_in<'a, T: FromBytecodeIn<'a, 'src>>(
        &mut self,
        alloc: &'a Bump,
    ) -> Result<T, ReadError> {
        T::read_from_bytecode_in(alloc, self)
    }

    pub fn iter_vec_item_in<'a, T: FromBytecodeIn<'a, 'src>>(
        &mut self,
        alloc: &'a Bump,
    ) -> Result<impl Iterator<Item = Result<T, ReadError>>, ReadError> {
        let len = self.read_leb_u32()?;
        Ok((0..len).map(|_| self.read_in::<T>(alloc)))
    }

    pub fn read_vec_in<'a, T: FromBytecodeIn<'a, 'src>>(
        &mut self,
        alloc: &'a Bump,
    ) -> Result<Box<'a, [T]>, ReadError> {
        trace!("Reading Vec");
        self.iter_vec_item_in(alloc)?.collect_in(alloc)
    }

    pub fn check_wasm_header(&mut self) -> Result<(), ReadError> {
        trace!("Reading bytecode header");
        let header = self.bytes(4)?;
        if header != WASM_HEADER_MAGIC {
            info!("Expected wasm header magic, got: {:?}", header);
            return Err(ReadError::from_reader(
                self,
                ReadErrorType::InvalidHeader(header.try_into().unwrap()),
            ));
        };
        let version = self.bytes(4)?;
        if version != WASM_HEADER_VERSION {
            info!("Expected wasm version, got: {:?}", version);
            return Err(ReadError::from_reader(
                self,
                ReadErrorType::InvalidVersion(version.try_into().unwrap()),
            ));
        };

        Ok(())
    }

    pub fn read_section_header(&mut self) -> Result<(SectionId, usize), ReadError> {
        trace!("Reading section header");
        let id_byte = self.read_u8()?;
        trace!("Section id is: {}", id_byte);
        let id = SectionId::from_repr(id_byte).ok_or(ReadError::from_reader(
            self,
            ReadErrorType::InvalidSectionId(id_byte),
        ))?;
        let size = self.read_u32()? as usize;
        trace!("Section size is: {}", size);
        Ok((id, size))
    }

    pub fn iter_section_readers(
        &mut self,
    ) -> impl Iterator<Item = Result<(SectionId, BufferReader<'src>), ReadError>> {
        (0..)
            .map(|_| {
                let (id, size) = self.read_section_header()?;
                let buf = self.bytes(size)?;
                Ok((id, BufferReader::from_buffer(buf, 0)))
            })
            .take_while(|s| !s.as_ref().is_err_and(|e: &ReadError| e.eof()))
    }

    pub fn split_required_sections(&mut self) -> Result<SplitSectionReader<'src>, ReadError> {
        let mut sections = SplitSectionReader::default();
        self.iter_section_readers().try_for_each(|r| {
            let (id, reader) = r?;
            match id {
                SectionId::Custom => {}
                _ => sections[id] = Some(reader),
            }
            Ok(())
        })?;

        Ok(sections)
    }
}

pub trait FromBytecode<'src>: Sized {
    fn read_from_bytecode(reader: &mut BufferReader<'src>) -> Result<Self, ReadError>;
}

pub trait FromBytecodeIn<'a, 'src>: Sized {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError>;
}

impl<'a, 'src, T: FromBytecode<'src>> FromBytecodeIn<'a, 'src> for T {
    fn read_from_bytecode_in(
        _alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        T::read_from_bytecode(reader)
    }
}
macro_rules! impl_read {
    ($fn_name: ident, $t: tt) => {
        impl<'a, 'src> FromBytecodeIn<'a, 'src> for $t {
            fn read_from_bytecode_in(
                _alloc: &'a Bump,
                reader: &mut BufferReader,
            ) -> Result<Self, ReadError> {
                Ok(reader.$fn_name()? as $t)
            }
        }
    };
}
impl_read!(read_u8, u8);
impl_read!(read_leb_u16, u16);
impl_read!(read_leb_u32, u32);
impl_read!(read_leb_u64, u64);
impl_read!(read_leb_u64, usize);

impl_read!(read_leb_i16, i16);
impl_read!(read_leb_i32, i32);
impl_read!(read_leb_i64, i64);
impl_read!(read_leb_i64, isize);

impl_read!(read_f32, f32);
impl_read!(read_f64, f64);
impl<'src> FromBytecode<'src> for bool {
    fn read_from_bytecode(reader: &mut BufferReader<'src>) -> Result<Self, ReadError> {
        trace!("Reading Bool");
        match reader.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            num => Err(ReadError::from_reader(
                reader,
                ReadErrorType::InvalidBool(num),
            )),
        }
    }
}
impl<'src> FromBytecode<'src> for ValueType {
    fn read_from_bytecode(reader: &mut BufferReader<'src>) -> Result<Self, ReadError> {
        let id = reader.read_u8()?;
        Self::from_repr(id).ok_or(ReadError::from_reader(
            reader,
            ReadErrorType::InvalidValueTypeEncoding(id),
        ))
    }
}

impl<'a, 'src, T: FromBytecode<'src>> FromBytecodeIn<'a, 'src> for Box<'a, [T]> {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        reader.read_vec_in(alloc)
    }
}

impl<'a, 'src, T: FromBytecodeIn<'a, 'src>> FromBytecodeIn<'a, 'src> for &'a mut [T] {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        //NOTE: (joh): Da wir Arenen nutzen sollte hier kein Speicher geleaked werden
        let d: Box<'a, [T]> = reader.read_vec_in(alloc)?;
        Ok(Box::leak(d))
    }
}
impl<'a, 'src, T: FromBytecodeIn<'a, 'src>> FromBytecodeIn<'a, 'src> for &'a [T] {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        //NOTE: (joh): Da wir Arenen nutzen sollte hier kein Speicher geleaked werden
        let d: Box<'a, [T]> = reader.read_vec_in(alloc)?;
        Ok(Box::leak(d))
    }
}
impl<'a, 'src> FromBytecodeIn<'a, 'src> for &'src str {
    fn read_from_bytecode_in(
        _alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        let len = reader.read_leb_u32()?;
        let data = reader.bytes(len as usize)?;

        str::from_utf8(data).map_err(|e| ReadError::from_reader(reader, e.into()))
    }
}

impl<'a, 'src> FromBytecodeIn<'a, 'src> for String<'a> {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        let str: &'src str = reader.read_in(alloc)?;

        Ok(String::from_str_in(str, alloc))
    }
}
impl<'a, 'src> FromBytecodeIn<'a, 'src> for Limits {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        match reader.read_u8()? {
            0x00 => Ok(Self {
                min: reader.read_in(alloc)?,
                max: None,
            }),
            0x01 => Ok(Self {
                min: reader.read_in(alloc)?,
                max: Some(reader.read_in(alloc)?),
            }),
            num => Err(ReadError::from_reader(
                reader,
                ReadErrorType::InvalidLimitsEncoding(num),
            )),
        }
    }
}

impl<'a, 'src> FromBytecodeIn<'a, 'src> for RawImportDesc {
    fn read_from_bytecode_in(
        alloc: &'a bumpalo::Bump,
        reader: &mut crate::reader::BufferReader<'src>,
    ) -> Result<Self, crate::reader::ReadError> {
        let id = reader.read_u8()?;
        match id {
            0x00 => Ok(Self::Function(reader.read_in(alloc)?)),
            0x01 => Ok(Self::TableType(reader.read_in(alloc)?)),
            0x02 => Ok(Self::MemType(reader.read_in(alloc)?)),
            0x03 => Ok(Self::GlobalType(reader.read_in(alloc)?)),
            _ => Err(ReadError::from_reader(
                reader,
                ReadErrorType::InvalidImportKind(id),
            )),
        }
    }
}

impl<'a, 'src> FromBytecodeIn<'a, 'src> for Reftype {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        let id = reader.read_u8()?;
        let t = Reftype::from_repr(id).ok_or(ReadError::from_reader(reader, ReadErrorType::InvalidRefType(id)))
    }
}
/*
impl<'a, 'src> FromBytecodeIn<'a, 'src> for Type<'a> {
    fn read_from_bytecode_in(
        alloc: &'a Bump,
        reader: &mut BufferReader<'src>,
    ) -> Result<Self, ReadError> {
        Ok(Self {
            params: FromBytecodeIn::read_from_bytecode_in(alloc, reader)?,
            results: FromBytecodeIn::read_from_bytecode_in(alloc, reader)?,
        })
    }
}
*/
