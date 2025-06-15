pub mod validator2;
pub mod ctrl;
pub mod validator;
/*
use std::io::Cursor;

use ctrl::JumpTable;
use error::ValidationError;
use parser::reader::{parse_binary, Bytecode};
use validator::{Context, Validator};

pub mod error;
pub struct ParsedData {
    bytecode: Bytecode,
    jump_table: Vec<JumpTable>,
}

pub fn parse_and_validate(data: &[u8]) -> Result<ParsedData, ValidationError> {
    let mut reader = Cursor::new(data);
    let bytecode = parse_binary(&mut reader)?;
    let context = Context::new(&bytecode)?;
    let jump_table = Validator::validate_all(&context)?;

    Ok(ParsedData {
        bytecode,
        jump_table,
    })
}
*/
