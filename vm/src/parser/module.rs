
use super::{error::ReaderError, reader::{FromReader, Position, Reader}, types::{CustomSection, Data, Export, Function, Global, Import, ImportDesc, Limits, Section, SectionData, SectionDataOrCustom, SectionId, Type, TypeId}};

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleError { 
    Reader(ReaderError),
    NoTypes,
    NoMemory,
    NoImports,
    InvalidTypeId(usize), 
    InvalidImportId(usize),
    UnexpectedMissingCode,
}

impl From<ReaderError> for ModuleError {
    fn from(value: ReaderError) -> Self {
        Self::Reader(value)
    }
}

#[derive(Debug, Clone)]
pub struct ImportedFunction<'src> {
    name: &'src str,
    func_type: &'src Type,  
}
#[derive(Debug, Clone)]
pub struct InternalFunction<'src> {
    pub func_type: &'src Type,     
    pub code: &'src Function         
}

#[derive(Debug, Clone)]
pub enum FunctionInfo<'src> {
    Imported(ImportedFunction<'src>),
    Internal(InternalFunction<'src>),
}

#[derive(Debug, Default)]
pub struct DecodedBytecode { 
    pub header: Position,
    pub version: Position,

    pub types: Option<(Box<[(Type, Position)]>, Position)>,
    pub imports: Option<(Box<[(Import, Position)]>, Position)>,
    pub functions: Option<(Box<[(TypeId, Position)]>, Position)>, 
    pub tables: Option<(Box<[(Limits, Position)]>, Position)>,
    pub memories: Option<(Box<[(Limits, Position)]>, Position)>,
    pub globals: Option<(Box<[(Global, Position)]>, Position)>,
    pub exports: Option<(Box<[(Export, Position)]>, Position)>,
    pub start: Option<u32>,
    pub data_count: Option<u32>,
    pub code: Option<(Box<[(Function, Position)]>, Position)>,
    pub data: Option<(Box<[(Data, Position)]>, Position)>,

    pub custom_sections: Vec<CustomSection>,
}


impl<'src> FromReader<'src> for DecodedBytecode {
    fn from_reader(reader: &mut Reader<'src>) -> Result<Self, ReaderError> {
        let mut reader =  reader.clone();
        let mut module: DecodedBytecode = Default::default(); 
        let (header, version) = reader.check_header()?;
        module.header = header;
        module.version = version;

        for section in reader.sections() {
            let (section, pos) = section?;
            match section.data {
                SectionDataOrCustom::Section(section_data) => module.add_section(section_data, pos), 
                SectionDataOrCustom::Custom(custom_section) => module.custom_sections.push(custom_section),
            }
        };
        Ok(module)
    }
}

impl<'src> DecodedBytecode {
    fn add_section(&mut self, section: SectionData, position: Position) {
        match section {
            SectionData::Type(items) => self.types = Some((items, position)),
            SectionData::Import(items) => self.imports = Some((items, position)),
            SectionData::Function(items) => self.functions = Some((items, position)),
            SectionData::Table(items) => self.tables = Some((items, position)),
            SectionData::Memory(items) => self.memories = Some((items, position)),
            SectionData::Global(items) => self.globals = Some((items, position)),
            SectionData::Export(items) => self.exports = Some((items, position)),
            SectionData::Start(start) => self.start = Some(start),
            SectionData::DataCount(count) => self.data_count = Some(count),
            SectionData::Code(items) => self.code = Some((items, position)),
            SectionData::Data(items) => self.data = Some((items, position)),
        };
    }

    pub fn get_type(&'src self, id: usize) -> Result<&'src (Type, Position), ModuleError> {
        let (types, _) = self.types.as_ref().ok_or(ModuleError::NoTypes)?; 
        types.get(id).ok_or(ModuleError::InvalidTypeId(id))
    }
    
    pub fn get_import(&self, id: usize) -> Result<&(Import, Position), ModuleError> {
        let (imports, _) = self.imports.as_ref().ok_or(ModuleError::NoImports)?;
        imports.get(id).ok_or(ModuleError::InvalidImportId(id))
    }
    fn get_imported_functions(&'src self, imports: &'src[(Import, Position)]) -> Result<Vec<FunctionInfo<'src>>, ModuleError> {
        imports.iter().filter_map(|i| {
            let (i, _) = i;
            if let Some((name, id)) = i.is_function() {
                let t = self.get_type(id as usize);
                match t {
                    Ok((func_type, _)) => Some(Ok(FunctionInfo::Imported(ImportedFunction {name, func_type}))),
                    Err(e) => Some(Err(e))
                }
            } else {
                None
            }
        })
        .collect()

    }
    pub fn get_imported_functions_data(&'src self) -> Result<Vec<FunctionInfo<'src>>, ModuleError> {
        self.imports.as_ref().map_or(Ok(Vec::new()), |f| self.get_imported_functions(&f.0))
    }

    pub fn get_internal_function_data(&'src self) -> Result<Vec<FunctionInfo<'src>>, ModuleError> {
        self.functions.as_ref().map_or(Ok(Vec::new()), |f| {
            f.0.iter().map(|f| {
                let (t, _) = self.get_type(f.0 as usize)?;
                let (code, _) = self.code
                     .as_ref()
                    .ok_or(ModuleError::UnexpectedMissingCode)?.0
                    .get(f.0 as usize).ok_or(ModuleError::UnexpectedMissingCode)?; 

                    Ok(FunctionInfo::Internal(InternalFunction { func_type: t, code}))  
            }).collect() 
        })
    }
    
}
