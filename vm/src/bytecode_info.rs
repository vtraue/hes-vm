use core::fmt;

use itertools::Itertools;

use crate::{
    op::Op,
    reader::{
        ExportDesc, FuncId, Global, ImportDesc, MemId, Position,
        Reader, ReaderError, ValueType,
    }, types::{Import, Limits, Locals, Type},
};
pub enum InfoError {
    EndOfBuffer,
}




#[derive(Debug, Clone)]
pub struct Export {
    name: (String, Position),
    desc: (ExportDesc, Position),
}
impl<'src> From<crate::reader::Export<'src>> for Export {
    fn from(value: crate::reader::Export) -> Self {
        let name = (value.name.0.to_string(), value.name.1);
        Export {
            name,
            desc: value.desc,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub locals: Box<[(Locals, Position)]>,
    pub code: Box<[(Op, Position)]>,
}

impl Function {
    pub fn get_local(&self, id: u32) -> Option<ValueType> {

    }
}
impl<'src> TryFrom<crate::reader::Function<'src>> for Function {
    fn try_from(value: crate::reader::Function) -> Result<Self, Self::Error> {
        let code = value.code.collect::<Result<Vec<_>, _>>()?.into_boxed_slice();
        Ok(Function {
            locals: value.locals,
            code,
        })
    }
    type Error = ReaderError;
}

#[derive(Debug)]
pub enum Data {
    Active {
        mem_id: MemId,
        expr: Box<[(Op, Position)]>,
        data: Position,
    },
    Passive(Position),
}

impl<'src> From<crate::reader::Data<'src>> for Data {
    fn from(value: crate::reader::Data) -> Self {
        match value {
            crate::reader::Data::Active { mem_id, expr, data } => Self::Active {
                mem_id,
                expr,
                data: data.1,
            },
            crate::reader::Data::Passive((_, pos)) => Self::Passive(pos),
        }
    }
}

#[derive(Debug, Default)]
pub struct CustomSection {
    name: (String, Position),
    data: (Vec<u8>, Position),
}

impl<'src> From<crate::reader::CustomSectionData<'src>> for CustomSection {
    fn from(value: crate::reader::CustomSectionData<'src>) -> Self {
        let name = (value.name.0.to_string(), value.name.1);
        let data = (value.data.0.into(), value.data.1);
        Self { name, data }
    }
}

#[derive(Debug, Default)]
pub struct BytecodeInfo {
    pub header: Position,
    pub version: Position,
    pub type_section: Option<(Box<[(Type, Position)]>, Position)>,
    pub import_section: Option<(Box<[(Import, Position)]>, Position)>,
    pub function_section: Option<(Box<[(u32, Position)]>, Position)>,
    pub table_section: Option<(Box<[(Limits, Position)]>, Position)>,
    pub memory_section: Option<(Box<[(Limits, Position)]>, Position)>,
    pub global_section: Option<(Box<[(Global, Position)]>, Position)>,
    pub export_section: Option<(Box<[(Export, Position)]>, Position)>,
    pub start_section: Option<(FuncId, Position)>,
    pub data_count_section: Option<(u32, Position)>,
    pub code_section: Option<(Box<[(Function, Position)]>, Position)>,
    pub data_section: Option<(Box<[(Data, Position)]>, Position)>,
    pub custom_sections: Vec<CustomSection>,
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
            match data {
                crate::reader::SectionData::Custom(custom_section_data) => {
                    info.custom_sections.push(custom_section_data.into());
                }
                crate::reader::SectionData::Type(mut sub_reader) => {
                    info.type_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map(|e| {let (t, p) = e?; Ok((t.try_into()?, p))})
                            .collect::<Result<Vec<_>, ReaderError>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Import(mut sub_reader) => {
                    info.import_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map_ok(|(e, p)| (e.into(), p))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Function(mut sub_reader) => {
                    info.function_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map_ok(|(e, p)| (e.into(), p))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }

                crate::reader::SectionData::Table(mut sub_reader) => {
                    info.table_section = Some((
                        sub_reader
                            .iter_with_position()
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Memory(mut sub_reader) => {
                    info.memory_section = Some((
                        sub_reader
                            .iter_with_position()
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Global(mut sub_reader) => {
                    info.global_section = Some((
                        sub_reader
                            .iter_with_position()
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Export(mut sub_reader) => {
                    info.export_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map_ok(|(e, p)| (e.into(), p))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Start(s) => info.start_section = Some(s),
                crate::reader::SectionData::DataCount(count) => {
                    info.data_count_section = Some(count)
                }
                crate::reader::SectionData::Code(mut sub_reader) => {
                    info.code_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map(|r| Ok((r?.0.try_into()?, r?.1)))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
                crate::reader::SectionData::Data(mut sub_reader) => {
                    info.data_section = Some((
                        sub_reader
                            .iter_with_position()
                            .map_ok(|(e, p)| (e.into(), p))
                            .collect::<Result<Vec<_>, _>>()?
                            .into_boxed_slice(),
                        pos,
                    ))
                }
            }
        }
        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        bytecode_info::BytecodeInfo,
        reader::{Reader, ReaderError},
    };

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
        let types = info.type_section.as_ref().unwrap();
        println!(
            "Type section position: {}\ndata:{:04x?}",
            types.1,
            reader.data_at(types.1)
        );

        println!("Type section count: {}", types.0.len());
        for t in types.0.iter() {
            println!("{:?}: {:0x?}", t.0, reader.data_at(t.1));
        }
        let imports = info.import_section.as_ref().unwrap();
        println!(
            "Import section position: {}\ndata:{:04x?}",
            imports.1,
            reader.data_at(imports.1)
        );

        for i in imports.0.iter() {
            println!("{:?}: {:0x?}", i.0, reader.data_at(i.1));
        }

        let function_ids = info.function_section.as_ref().unwrap();
        println!(
            "function section position: {}\ndata:{:04x?}",
            function_ids.1,
            reader.data_at(function_ids.1)
        );
        for id in function_ids.0.iter() {
            println!("id: {}", id.0);
        }

        let functions_with_types = info
            .function_section
            .as_ref()
            .unwrap()
            .0
            .iter()
            .map(|f| &types.0[f.0 as usize])
            .collect::<Vec<_>>();

        for t in &functions_with_types {
            println!("fn type: {:?}", t.0);
        }

        let memories = info.memory_section.unwrap();
        println!(
            "memory section position: {}\ndata:{:0x?}",
            memories.1,
            reader.data_at(memories.1)
        );

        for t in memories.0.iter() {
            println!("memory {:?}, pos: {}", t.0, t.1)
        }

        let code = info.code_section.unwrap();
        println!(
            "code section pos: {}\ndata:{:0x?}",
            code.1,
            reader.data_at(code.1)
        );

        for (i, func) in code.0.iter().enumerate() {
            let func_t = functions_with_types.get(i).unwrap();
            println!("func {}, t: {:?}, pos: {}, data: {:0x?}", i, func_t.0, func.1, reader.data_at(func.1));
            for o in func.0.code.iter() {
                let (op, pos) = o.as_ref().unwrap();
                println!("op: {op}, pos: {pos}, data: {:0x?}", reader.data_at(pos.clone()));
            }
        }
        Ok(())
    }
}
