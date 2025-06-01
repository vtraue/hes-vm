use error::ReaderError;
use module::DecodedBytecode;
use reader::Reader;

pub mod error;
pub mod module;
pub mod op;
pub mod reader;
pub mod types;

pub fn parse_wasm(data: &[u8]) -> Result<DecodedBytecode, ReaderError> {
    let mut reader = Reader::new(data);
    reader.read::<DecodedBytecode>()
}
