use super::op::Op;
use super::types::Section;
use super::error::*;
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Position {
    pub offset: usize,
    pub len: usize
}

#[derive(Debug, Clone)]
pub struct Reader<'src> {
    buffer: &'src [u8],
    pos: usize,
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
        Ok(reader.read_str()?.0)
    }
}
impl<'src> FromReader<'src> for String {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self> {
        Ok(reader.read_str()?.0.to_string())
    }
}

pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";

impl<'src> Reader<'src> {
    pub fn new(buffer: &'src [u8]) -> Self {
        Self { buffer, pos: 0}
    }
    pub fn buffer(&self) -> &'src [u8] {
        &self.buffer[self.pos..]
    }

    pub fn bytes_left(&self) -> usize {
        self.buffer.len() - self.pos
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

    pub fn can_read_bytes(&self, len: usize) -> Result<()> {
        if self.bytes_left() < len {
            Err(ReaderError::EndOfBuffer)
        } else {
            Ok(())
        }
    }

    pub fn can_read<T: Sized>(&self) -> Result<()> {
        self.can_read_bytes(size_of::<T>())
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.can_read::<u8>()?;
        let res = self.buffer[self.pos];
        self.pos += 1;
        Ok(res)
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<(&'src [u8], Position)> {
        self.can_read_bytes(size)?;
        let pos = Position {
            offset: self.pos,
            len: size
        };

        let new_pos = self.pos + size;
        let res = &self.buffer[self.pos..new_pos]; 
        self.pos = new_pos;

        Ok((res, pos))
    }
    
    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(
            self.read_bytes(size_of::<u32>())?.0.try_into().unwrap(),
        ))
    }
    
    pub fn read_str(&mut self) -> Result<(&'src str, Position)> {
        let len = self.read::<usize>()?;
        let bytes = self.read_bytes(len)?;
        let str = str::from_utf8(bytes.0)?; 
        Ok((str, bytes.1))
    }

    pub fn skip_bytes(&mut self, size: usize) -> Result<Position> {
        self.can_read_bytes(size)?;
        let pos = self.pos;
        self.pos += size;
        Ok(Position { offset: pos, len: size })
    }
    
    pub fn read_and_skip_size(&mut self) -> Result<Position> {
        let size = self.read::<usize>()?;
        self.skip_bytes(size)  
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
        let start = self.pos;
        let elem = T::from_reader(self)?;
        let bytes_read = self.pos - start;
        Ok((
            elem,
            Position {
                offset: start,
                len: bytes_read,
            },
        ))
    }

    pub fn read_with_slice<T>(&mut self) -> Result<(T, &'src [u8])>
    where
        T: FromReader<'src>,
    {
        let start = self.pos;
        let elem = T::from_reader(self)?;
        Ok((elem, &self.buffer[start..self.pos]))
    }

    pub fn read_vec_iter<'me, T: FromReader<'src>>(&'me mut self) -> Result<VecIter<'src, 'me, T>> {
        let size = self.read_var_u32()? as usize;
        Ok(VecIter {
            count: size,
            pos: 0,
            done: false,
            reader: self,
            _marker: std::marker::PhantomData,
        })
    }
   
    pub fn read_vec<T: FromReader<'src>>(&mut self) -> Result<Box<[(T, Position)]>> {
        Ok(self.read_vec_iter()?.collect::<Result<Vec<_>, _>>()?.into_boxed_slice()) 
    }

    pub fn read_vec_bytes(&mut self) -> Result<(&'src [u8], Position)> {
        let size = self.read_var_u32()? as usize;
        self.read_bytes(size)
    }

    pub fn read_const_expr_iter<'me>(&'me mut self) -> ConstantExprIter<'src, 'me> {
        ConstantExprIter {
            current_position: 0,
            done: false,
            reader: self,
        }
    }
    pub fn read_expr_iter<'me>(&'me mut self) -> ExprIter<'src, 'me> {
        ExprIter {done: false, depth: 0, reader: self}
    }

    pub fn sections<'me>(&'me mut self) -> SectionsIter<'src, 'me> {
        SectionsIter { reader: self }
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
}

pub struct VecIter<'src, 'me, T: FromReader<'src>> {
    count: usize,
    pos: usize,
    done: bool,
    reader: &'me mut Reader<'src>,
    _marker: std::marker::PhantomData<T>,
}

impl<'src, 'me, T: FromReader<'src>> Iterator for VecIter<'src, 'me, T> {
    type Item = Result<(T, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.count || self.done {
            None
        } else {
            let res = self.reader.read_with_position::<T>();
            if res.is_err() {
                self.done = true;
            } else {
                self.pos += 1;
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

#[derive(Debug)]
pub struct ExprIter<'src, 'me> {
    done: bool,
    depth: usize,
    reader: &'me mut Reader<'src>
}
impl<'src, 'me> Iterator for ExprIter<'src, 'me> {
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

pub struct SectionsIter<'src, 'me> {
    reader: &'me mut Reader<'src>
}

impl<'src, 'me> Iterator for SectionsIter<'src, 'me> {
    type Item = Result<(Section, Position)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.can_read_bytes(1).ok()?;
        Some(self.reader.read_with_position())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::parser::types::Section;
    use crate::parser::error;

    fn get_wasm_gen() -> Box<[u8]> {
        let source = include_str!("../wat/gen.wat");
        let source = wat::parse_str(source).unwrap().into_boxed_slice();
        fs::write("gen2.wasm", &source).unwrap();
        source
    }
    
    #[test]
    fn wasm_check_sections_iter() -> Result<(), ReaderError> {
        let wasm = get_wasm_gen();
        let mut reader = Reader::new(&wasm);
        let (header, version) = reader.check_header()?;  
        let mut sections_reader =  reader.sections();
        sections_reader.collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
    
}
