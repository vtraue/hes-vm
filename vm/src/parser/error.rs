use super::op::Op;

#[derive(Debug, Clone, PartialEq)]
pub enum ReaderError {
    PosOutOfRange,
    InvalidLeb,
    EndOfBuffer,
    InvalidUtf8InName(std::str::Utf8Error),
    InvalidBlocktypeEncoding,
    InvalidBool,
    InvalidTypeId,
    InvalidRefTypeId,
    InvalidValueTypeId(u8),
    InvalidHeaderMagicNumber,
    InvalidWasmVersion,
    InvalidSectionId(u8),
    InvalidFunctionTypeEncoding(u8),
    InvalidImportDesc(u8),
    UnimplementedOpcode(u8),
    ExpectedConstExpression(Op),
    InvalidLimits,
    InvalidExportDesc,
    MalformedCodeSection,
    InvalidDataMode(u32),
    DataIsNotStringLiteral,
    StringLiteralIsNotValidUtf(std::str::Utf8Error),
}
impl From<std::str::Utf8Error> for ReaderError {
    fn from(value: std::str::Utf8Error) -> Self {
        ReaderError::InvalidUtf8InName(value)
    }
}

pub type Result<T, E = ReaderError> = core::result::Result<T, E>;
