use crate::reader::{BufferReader, ReadError};

macro_rules! impl_decode_leb_unsigned {
    ($name: ident, $t: tt) => {
        pub fn $name(&mut self) -> Result<$t, ReadError> {
            let mut res = 0;
            let mut shift = 0;
            loop {
                let byte = self.read_u8()?;
                res |= ((byte as $t) & 0x7f) << shift;
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            Ok(res)
        }
    };
}
macro_rules! impl_decode_leb_signed {
    ($name: ident, $t: tt) => {
        pub fn $name(&mut self) -> Result<$t, ReadError> {
            let mut res = 0;
            let mut shift = 0;
            let size = size_of::<$t>() * 4;
            let mut byte;
            loop {
                byte = self.read_u8()?;
                res |= ((byte as $t) & 0x7f) << shift;
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            if (shift < size) && (byte & 0x40 != 0) {
                res |= (!0 << shift);
            }
            Ok(res)
        }
    };
}

impl<'src> BufferReader<'src> {
    impl_decode_leb_unsigned!(read_leb_u64, u64);
    impl_decode_leb_unsigned!(read_leb_u32, u32);
    impl_decode_leb_unsigned!(read_leb_u16, u16);

    impl_decode_leb_signed!(read_leb_i64, i64);
    impl_decode_leb_signed!(read_leb_i32, i32);
    impl_decode_leb_signed!(read_leb_i16, i16);
}
/*
pub trait ReadLeb {
    impl_decode_leb_unsigned!(read_leb_u64, u64);
    impl_decode_leb_unsigned!(read_leb_u32, u32);
    impl_decode_leb_unsigned!(read_leb_u16, u16);

    impl_decode_leb_signed!(read_leb_i64, i64);
    impl_decode_leb_signed!(read_leb_i32, i32);
    impl_decode_leb_signed!(read_leb_i16, i16);
}
*/
