use core::{
    fmt::Display,
    mem,
    ops::{Index, IndexMut},
};

use alloc::alloc::Allocator;
use bumpalo::{Bump, boxed::Box, collections::Vec};
use log::{error, info, trace};
use parser_derive2::FromBytecode;
use strum::{EnumCount, IntoDiscriminant};
use strum_macros::{Display, EnumCount, FromRepr};
use thiserror::Error;

use crate::{
    op::{BrTableEntry, Op},
    reader::{BufferReader, ReadError, ReadErrorType},
    types::{
        ImportIdent, ImportedFunction, Limits, RawGlobal, RawImport, RawImportDesc,
        RawImportDescDiscriminants, TableType, Type, TypeId,
    },
};

#[derive(Error, Clone, Debug)]
pub enum ParseError {
    #[error("Invalid Type id")]
    InvalidTypeId(usize),

    #[error("Multiple memories defined")]
    MultipleMemories,
}
pub struct BrTableIndex(usize);

#[derive(Debug, Default)]
pub struct BrTable<'a>(Box<'a, [&'a [BrTableEntry]]>);

impl<'a> BrTable<'a> {
    pub fn entries(&self, index: BrTableIndex) -> Option<&[BrTableEntry]> {
        self.0.get(index.0).map(|v| *v)
    }
}

#[derive(Debug, Default)]
pub struct Code<'a> {
    code: Box<'a, [Op]>,
    br_table: BrTable<'a>,
}

#[derive(Debug)]
pub struct InternalFunction<'a> {
    t: &'a Type<'a>,
    code_offset: usize,
}

#[derive(Default, Debug)]
pub struct Module<'a, 'src> {
    types: &'a [Type<'a>],
    code: Code<'a>,
    imports: Imports<'a, 'src>,
    internal_functions: &'a [InternalFunction<'a>],
    memory: Option<Limits>,
}

impl<'a, 'src> Module<'a, 'src> {
    pub fn try_get_type(&self, id: TypeId) -> Result<&'a Type<'a>, ParseError> {
        self.types.get(id.0).ok_or(ParseError::InvalidTypeId(id.0))
    }
}
pub struct Parser<'bump, 'src> {
    alloc: &'bump Bump,
    reader: BufferReader<'src>,
    raw_sections: SplitSectionReader<'src>,
    module: Module<'bump, 'src>,
}

#[derive(Display, FromRepr, Debug, Copy, Clone, Eq, PartialEq, EnumCount)]
#[repr(u8)]
pub enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}

#[derive(Debug, Clone)]
pub struct RawSectionInfo {
    pub start: usize,
    pub size: usize,
    pub id: SectionId,
}

#[derive(Debug, Default)]
pub struct SplitSectionReader<'src> {
    sections: [Option<BufferReader<'src>>; SectionId::COUNT],
}

impl<'src> Index<SectionId> for SplitSectionReader<'src> {
    type Output = Option<BufferReader<'src>>;

    fn index(&self, index: SectionId) -> &Self::Output {
        &self.sections[index as usize]
    }
}
impl<'src> IndexMut<SectionId> for SplitSectionReader<'src> {
    fn index_mut(&mut self, index: SectionId) -> &mut Self::Output {
        &mut self.sections[index as usize]
    }
}

/*
pub fn parse<'a, 'src>(
    alloc: &'a Bump,
    reader: &mut BufferReader<'src>,
) -> Result<Module<'a, 'src>, ReadError> {
    let sections = reader.split_required_sections()?;
    let types = sections[SectionId::Type].map(|r| r.read_in::<Type<'a>>(alloc));
    //NOTE: (joh): Falls die Typen leer sind hier abbrechen?
}
*/
enum Import<'a, 'src> {
    Function(ImportedFunction<'a, 'src>),
    TableType(&'a RawImport<'src>),
    MemType(Limits),
    Global(&'a RawImport<'src>),
}
impl<'a, 'src> Import<'a, 'src> {
    pub fn from_raw(
        raw: &'a RawImport<'src>,
        module: &Module<'a, 'src>,
    ) -> Result<Self, ParseError> {
        match &raw.desc {
            RawImportDesc::Function(type_id) => {
                let t = module.try_get_type(type_id.clone())?;
                let import_info = &raw;
                let func = ImportedFunction { t, import_info };
                Ok(Import::Function(func))
            }
            RawImportDesc::TableType(_) => Ok(Import::TableType(&raw)),
            RawImportDesc::MemType(limits) => match &module.memory {
                Some(_) => Err(ParseError::MultipleMemories),
                None => Ok(Import::MemType(limits.clone())),
            },
            RawImportDesc::GlobalType(raw_global) => Ok(Import::Global(&raw)),
        }
    }
}

#[derive(Default, Debug)]
pub struct Imports<'a, 'src> {
    raw: &'a [RawImport<'src>],
    counts: [u32; RawImportDesc::COUNT],
    functions: &'a [ImportedFunction<'a, 'src>],
}
impl<'a, 'src> Imports<'a, 'src> {
    fn new_from_reader(reader: BufferReader<'src>)
}

impl<'a, 'src> Parser<'a, 'src> {
    fn count_import_types(imports: &[RawImport<'src>]) -> [u32; RawImportDesc::COUNT] {
        let mut counts = [0_u32; RawImportDesc::COUNT];
        for import in imports {
            counts[import.desc.discriminant() as usize] += 1;
        }
        counts
    }

    pub fn parse_imports(&mut self) -> Result<Option<Imports<'a, 'src>>, ReadError> {
        let raw_imports = self.raw_sections[SectionId::Type].clone();
        //NOTE: (joh): Wir iterieren jetzt mehrmals durch die Imports,
        // vermutlich geht das anders schöner.
        if let Some(mut raw_imports) = raw_imports {
            let raw: &[RawImport<'src>] = raw_imports.read_in(self.alloc)?;
            let counts = Self::count_import_types(raw);

            let functions = Vec::with_capacity_in(
                counts[RawImportDescDiscriminants::Function as usize] as usize,
                self.alloc,
            );

            Ok(())
        }
    }
}
/*
impl<'bump, 'src> Parser<'bump, 'src> {
    pub fn new(alloc: &'bump Bump, reader: BufferReader<'src>) -> Self {
        Self {
            alloc,
            reader,
            module: Default::default(),
        }
    }

    pub fn handle_section(&mut self, id: SectionId, size: usize) -> Result<(), ReadError> {
        match id {
            SectionId::Custom => Ok(self.reader.skip(size)?),
            SectionId::Type => Ok(self.module.types = self.reader.read_in(self.alloc)?),
            SectionId::Import => todo!(),
            SectionId::Function => todo!(),
            SectionId::Table => todo!(),
            SectionId::Memory => todo!(),
            SectionId::Global => todo!(),
            SectionId::Export => todo!(),
            SectionId::Start => todo!(),
            SectionId::Element => todo!(),
            SectionId::Code => todo!(),
            SectionId::Data => todo!(),
            SectionId::DataCount => todo!(),
        }
    }
}
*/
