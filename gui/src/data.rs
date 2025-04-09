use vm::reader::ReaderError;

type Result<T, E = ProgrammError> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum ProgrammError {
    IoError(std::io::Error),
    ReaderError(ReaderError),
}

impl From<std::io::Error> for ProgrammError {
    fn from(value: std::io::Error) -> Self {
        ProgrammError::IoError(value)
    }
}

impl From<ReaderError> for ProgrammError {
    fn from(value: ReaderError) -> Self {
        ProgrammError::ReaderError(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionType {
    Header,
    Version,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    DataCount,
    Code,
    Data,
    Custom,
}
