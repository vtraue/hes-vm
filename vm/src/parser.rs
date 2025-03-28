use crate::reader::{CustomSectionData, FunctionType, ImportReader, Reader, ReaderError, TypeId};
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InvalidTypeId(TypeId),
}

const MAX_MEMORY_PAGES: u32 = 10;

/*
pub struct FunctionInstance {
    t: FunctionType,

}
*/

/*
pub struct ModuleParser<'src> {
    custom_sections: Vec<CustomSectionData<'src>>,
    function_types: Box<[FunctionType]>,
    imports: Option<ImportReader<'src>>,
    functions: Option<FunctionReader<'src>>,


}
pub struct ModuleInfo {
    types: Box<[FunctionType]>,
    //TODO: (joh): Namen per Name Section
    functions: Box<[FunctionType]>,
}

impl ModuleInfo {
    pub fn from_reader(reader: Reader) -> Result<Self, ReaderError> {

    }
}
*/

pub fn print_raw_module() {}
