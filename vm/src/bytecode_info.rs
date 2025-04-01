use itertools::Itertools;

use crate::{op::Op, reader::{ExportDesc, FuncId, FunctionType, Global, ImportDesc, Limits, Locals, MemId, Position, Reader, ReaderError, ValueType}};
pub enum InfoError {
    EndOfBuffer
}

#[derive(Debug)]
pub struct Type {
    params: Box<[Result<(ValueType, Position), ReaderError>]>,
    results: Box<[Result<(ValueType, Position), ReaderError>]>,
}

impl<'src> From<FunctionType<'src>> for Type {
    fn from(mut value: FunctionType<'src>) -> Self {
        //TODO: (joh): Falls der Buffer vorzeitig leer ist, wird das hier ein Problem sein 
        let params = value.params.iter_with_position().collect::<Vec<Result<_,_>>>().into_boxed_slice();
        let results = value.results.iter_with_position().collect::<Vec<Result<_,_>>>().into_boxed_slice();

        Self {params, results} 
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    module: (String, Position),
    name: (String, Position),
    desc: (ImportDesc, Position),
}
impl<'src> From<crate::reader::Import<'src>> for Import {
    fn from(value: crate::reader::Import<'src>) -> Self {
        Import {
            module: (value.module.0.to_string(), value.module.1), 
            name: (value.name.0.to_string(), value.name.1),
            desc: value.desc
        }
    }
}

#[derive(Debug, Clone)]
pub struct Export {
    name: (String, Position),
    desc: (ExportDesc, Position),
}
impl<'src> From<crate::reader::Export<'src>> for Export {
    fn from(value: crate::reader::Export) -> Self {
        let name = (value.name.0.to_string(), value.name.1);  
        Export {name, desc: value.desc}
    }
}

#[derive(Debug)]
pub struct Function {
    locals: Box<[(Locals, Position)]>,
    code: Box<[Result<(Op, Position), ReaderError>]>
}

impl<'src> From<crate::reader::Function<'src>> for Function {
    fn from(value: crate::reader::Function) -> Self {
        let code = value.code.collect::<Vec<Result<_, _>>>().into_boxed_slice();
        Function {locals: value.locals, code}
    }
}

#[derive(Debug)]
pub enum Data {
    Active {
        mem_id: MemId,
        expr: Box<[(Op, Position)]>,
        data: Position
    },
    Passive(Position)
}

impl<'src> From<crate::reader::Data<'src>> for Data {
    fn from(value: crate::reader::Data) -> Self {
        match value {
            crate::reader::Data::Active { mem_id, expr, data } => Self::Active { mem_id, expr, data: data.1} ,
            crate::reader::Data::Passive((_, pos)) => Self::Passive(pos)
        }
    }
}

#[derive(Debug, Default)]
pub struct CustomSection {
    name: (String, Position),
    data: (Vec<u8>, Position) 
}

impl<'src> From<crate::reader::CustomSectionData<'src>> for CustomSection {
    fn from(value: crate::reader::CustomSectionData<'src>) -> Self {
        let name = (value.name.0.to_string(), value.name.1); 
        let data = (value.data.0.into(), value.data.1); 
        Self {name, data} 
    }
}

#[derive(Debug, Default)]
pub struct BytecodeInfo {
    header: Position,
    version: Position,
    type_section: Option<(Box<[(Type, Position)]>, Position)>,
    import_section: Option<(Box<[(Import, Position)]>, Position)>,
    function_section: Option<(Box<[(u32, Position)]>, Position)>, 
    table_section: Option<(Box<[(Limits, Position)]>, Position)>, 
    memory_section: Option<(Box<[(Limits, Position)]>, Position)>, 
    global_section: Option<(Box<[(Global, Position)]>, Position)>,
    export_section: Option<(Box<[(Export, Position)]>, Position)>,
    start_section: Option<(FuncId, Position)>, 
    data_count_section: Option<(u32, Position)>,
    code_section: Option<(Box<[(Function, Position)]>, Position)>,
    data_section: Option<(Box<[(Data, Position)]>, Position)>,
    custom_sections: Vec<CustomSection>
}

impl BytecodeInfo {
    pub fn from_reader<'src>(reader: &'src Reader) -> Result<Self, ReaderError> {
        let mut reader = reader.clone();
        //TODO: (joh): Custom Sections 
        let mut info: BytecodeInfo = Default::default();
        (info.header, info.version) = reader.check_header()?; 
        for s in reader.sections_iter() {
            let s = s?;
            let data = s.0.data; 
            let pos = s.1;
            //TODO: (joh): Das geht bestimmt hübscher
            match data{
                crate::reader::SectionData::Custom(custom_section_data) => {
                    info.custom_sections.push(custom_section_data.into());
                }
                crate::reader::SectionData::Type(mut sub_reader) => {
                    info.type_section = Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                },
                crate::reader::SectionData::Import(mut sub_reader) => {
                    info.import_section = Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                },
                crate::reader::SectionData::Function(mut sub_reader) => {
                    info.function_section= Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                }

                crate::reader::SectionData::Table(mut sub_reader) => {
                    info.table_section = Some((sub_reader.iter_with_position().collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                },
                crate::reader::SectionData::Memory(mut sub_reader) => {
                    info.memory_section = Some((sub_reader.iter_with_position().collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos)) 
                },
                crate::reader::SectionData::Global(mut sub_reader) => {
                    info.global_section= Some((sub_reader.iter_with_position().collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos)) 
                }
                crate::reader::SectionData::Export(mut sub_reader) => {
                    info.export_section = Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                },
                crate::reader::SectionData::Start(s) => {
                    info.start_section = Some(s)
                },
                crate::reader::SectionData::DataCount(count) => {
                    info.data_count_section = Some(count)
                },
                crate::reader::SectionData::Code(mut sub_reader) => {
                    info.code_section = Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))

                },
                crate::reader::SectionData::Data(mut sub_reader) => {
                    info.data_section = Some((sub_reader.iter_with_position().map_ok(|(e, p)| (e.into(), p)).collect::<Result<Vec<_>, _>>()?.into_boxed_slice(), pos))
                },
            }
        }
        Ok(info)
    }
}

#[cfg(test)]
mod tests{
    use std::fs;

    use crate::{bytecode_info::BytecodeInfo, reader::{Reader, ReaderError}};


    fn get_wasm_gen() -> Box<[u8]> {
        let source = include_str!("wat/gen.wat");
        let source = wat::parse_str(source).unwrap().into_boxed_slice();
        fs::write("gen2.wasm", &source).unwrap();
        source
    }
    
    #[test]
    fn check_and_create_info() -> Result<(), ReaderError> {
        let wasm = get_wasm_gen();
        let reader = Reader::new(&wasm, 0);
        let info = BytecodeInfo::from_reader(&reader)?;
        println!("{:#?}", info);
        Ok(())
    }
}
