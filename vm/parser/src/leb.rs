use std::{io::Read};

use byteorder::ReadBytesExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LebError {
    #[error("Unable to read from reader: {0}")]
    Io(std::io::Error),
    #[error("Invalid leb")]
    InvalidLeb,
}
impl From<std::io::Error> for LebError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
pub struct Leb {}

impl Leb {
    #[inline]
    pub fn read_u32(reader: &mut impl Read) -> Result<u32, LebError> {
        // Optimization for single byte i32.
        let byte = reader.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(u32::from(byte))
        } else {
            Self::read_u32_big(reader, byte)
        }
    }

    fn read_u32_big(reader: &mut impl Read, first: u8) -> Result<u32, LebError>  {
        let mut result = (first & 0x7F) as u32;
        let mut shift = 7;
        loop {
            let byte = reader.read_u8()?;
            result |= ((byte & 0x7F) as u32) << shift;
            if shift >= 25 && (byte >> (32 - shift)) != 0 {
                return Err(LebError::InvalidLeb);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }
    pub fn read_u64(reader: &mut impl Read) -> Result<u64, LebError> {
        let byte = u64::from(reader.read_u8()?);
        if (byte & 0x80) == 0 {
            Ok(byte)
        } else {
            Self::read_u64_big(reader, byte)
        }
    }

    fn read_u64_big(reader: &mut impl Read, byte: u64) -> Result<u64, LebError> {
        let mut result = byte & 0x7F;
        let mut shift = 7;
        loop {
            let byte = u64::from(reader.read_u8()?);
              
            result |= (byte & 0x7F) << shift;
            if shift >= 57 && (byte >> (64 - shift)) != 0 {
                // The continuation bit or unused bits are set.
                return Err(LebError::InvalidLeb);
            }
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        Ok(result)
    }

    #[inline]
    pub fn read_var_i32(reader: &mut impl Read) -> Result<i32, LebError> {
        // Optimization for single byte i32.
        let byte = reader.read_u8()?;
        if (byte & 0x80) == 0 {
            Ok(((byte as i32) << 25) >> 25)
        } else {
            Self::read_i32_big(reader, byte)
        }
    }

    fn read_i32_big(reader: &mut impl Read, byte: u8) -> Result<i32, LebError> {
        let mut result = (byte & 0x7F) as i32;
        let mut shift = 7;
        loop {
            let byte = reader.read_u8()?;
            result |= ((byte & 0x7F) as i32) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(LebError::InvalidLeb);
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
    pub fn read_s33(reader: &mut impl Read) -> Result<i64, LebError> {
        // Optimization for single byte.
        let byte = reader.read_u8()?;
        if (byte & 0x80) == 0 {
            return Ok(((byte as i8) << 1) as i64 >> 1);
        }

        let mut result = (byte & 0x7F) as i64;
        let mut shift = 7;
        loop {
            let byte = reader.read_u8()?;
            result |= ((byte & 0x7F) as i64) << shift;
            if shift >= 25 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = (byte << 1) as i8 >> (33 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(LebError::InvalidLeb);
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

    pub fn read_var_i64(reader: &mut impl Read) -> Result<i64, LebError> {
        let mut result: i64 = 0;
        let mut shift = 0;
        loop {
            let byte = reader.read_u8()?;
            result |= i64::from(byte & 0x7F) << shift;
            if shift >= 57 {
                let continuation_bit = (byte & 0x80) != 0;
                let sign_and_unused_bit = ((byte << 1) as i8) >> (64 - shift);
                if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                    return Err(LebError::InvalidLeb);
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

