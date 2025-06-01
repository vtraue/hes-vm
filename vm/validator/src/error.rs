use parser::{error::ReaderError, module::ModuleError};

use super::validator::ValueStackType;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    ReaderError(ReaderError),
    ModuleError(ModuleError),
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

impl From<ReaderError> for ValidationError {
    fn from(value: ReaderError) -> Self {
        Self::ReaderError(value)
    }
}

impl From<ModuleError> for ValidationError {
    fn from(value: ModuleError) -> Self {
        Self::ModuleError(value)
    }
}
