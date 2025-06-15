/*
use parser::reader::ParserError;

use super::validator::ValueStackType;

#[derive(Debug)]
pub enum ValidationError {
    ReaderError(ParserError),
    ValueStackUnderflow,
    UnexpectedValueType {
        got: ValueStackType,
        expected: ValueStackType,
    },
    UnexpectedEmptyControlStack,
    ReturnTypesDoNotMatch {
        got: ValueStackType,
        expexted: ValueStackType,
    },
    UnbalancedStack {
        got: usize,
        expected: usize,
    },
    UnexpectedNoMemories,
    InvalidAlignment,
    InvalidLocalID(u32),
    InvalidGlobalID(usize),
    CannotSetToImmutableGlobal(u32),
    ExpectedNumericType,
    InvalidTypeId(usize),
    UnexpectedNoTypes,
    UnexpectedNoCode,
    ElseWithoutIf,
    LabelIndexOutOfScope(u32),
    InvalidFuncId(usize),
    InvalidMemId(usize),
    InvalidLocalId(u32),
    MissingEndOnFunctionExit,
    InvalidJump,
    InvalidJumpId,
    InvalidCodeId(usize),
    NotAConstOp,
}

pub type Result<T> = std::result::Result<T, ValidationError>;

impl From<ParserError> for ValidationError {
    fn from(value: ParserError) -> Self {
        Self::ReaderError(value)
    }
}
*/
