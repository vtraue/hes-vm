use ctrl::JumpTable;
use error::ValidationError;
use parser::{module::DecodedBytecode, parse_wasm};
use validator::{Context, Validator};

pub mod ctrl;
pub mod error;
pub mod validator;


pub struct ParsedData {
    bytecode: DecodedBytecode,
    jump_table: Vec<JumpTable> 
}


pub fn parse_and_validate(data: &[u8]) -> Result<ParsedData, ValidationError> {
    let bytecode = parse_wasm(data)?; 
    let context = Context::new(&bytecode)?;   
    let jump_table= Validator::validate_all(&context)?;

    Ok(ParsedData {
        bytecode,
        jump_table,
    })
}
