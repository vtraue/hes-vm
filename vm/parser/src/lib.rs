use error::ReaderError;
use module::DecodedBytecode;
use reader::Reader;

pub mod error;
pub mod module;
pub mod op;
pub mod op2;
pub mod reader;
pub mod types;
pub mod reader2;
pub mod leb;
pub fn parse_wasm(data: &[u8]) -> Result<DecodedBytecode, ReaderError> {
    let mut reader = Reader::new(data);
    reader.read::<DecodedBytecode>()
}
