use crate::parser::{error::ReaderError, module::ModuleError};

use super::validator::ValueStackType;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    ReaderError(ReaderError),
    ModuleError(ModuleError),
    ValueStackUnderflow,
    UnexpectedValueType {got: ValueStackType, expected: ValueStackType},
    UnexpectedEmptyControlStack,
    ReturnTypesDoNotMatch{got: ValueStackType, expexted: ValueStackType},
    UnbalancedStack,
    UnexpectedNoMemories,
    InvalidAlignment,
    InvalidLocalID(u32),
    InvalidGlobalID(u32),
    CannotSetToImmutableGlobal(u32),
    ExpectedNumericType,
    InvalidTypeId(u32),
    ElseWithoutIf,
    LabelIndexOutOfScope(u32),
    InvalidFuncId(u32),
    InvalidMemId(u32),
    InvalidLocalId(u32),
    MissingEndOnFunctionExit,
    InvalidJump,
    InvalidJumpId,
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

