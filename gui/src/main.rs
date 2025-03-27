use std::fs;

use eframe::{egui, run_native};
use gui::HesApp;
use vm::{
    self,
    reader::{
        self, Data, Export, FuncId, Function, FunctionType, Global, Import, Limits, Reader,
        ReaderError, TypeId,
    },
};

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

fn main() -> eframe::Result {
    let path = "../../ref-project/app/web/gen.wasm";
    let mut program: WasmProgram = Default::default();

    program.bytecode = fs::read(path).unwrap();
    let module = Module::new(&program.bytecode).unwrap();
    program.module = module;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1800.0, 1200.0])
            .with_min_inner_size([660.0, 440.0]), //.with_icon(
        //    eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
        //        .expect("Failed to load icon"),
        ..Default::default()
    };
    run_native(
        "hes-vm",
        native_options,
        Box::new(|cc| Ok(Box::new(HesApp::new(cc)))),
    )
}

#[derive(Debug, Default)]
struct WasmProgram<'src> {
    module: Module<'src>,
    bytecode: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct Module<'src> {
    bytecode: &'src [u8],
    type_section: Option<Box<[FunctionType]>>,
    import_section: Option<Box<[Import<'src>]>>,
    function_section: Option<Box<[TypeId]>>,
    table_section: Option<Box<[Limits]>>,
    memory_section: Option<Box<[Limits]>>,
    global_section: Option<Box<[Global]>>,
    export_section: Option<Box<[Export<'src>]>>,
    start_section: Option<FuncId>,
    data_count_section: Option<u32>,
    code_section: Option<Box<[Function<'src>]>>,
    data_section: Option<Box<[Data<'src>]>>,
}

impl<'src> Module<'src> {
    pub fn new(bytecode: &'src [u8]) -> Result<Self> {
        let mut module = Self {
            bytecode,
            ..Default::default()
        };

        let mut reader = Reader::new(bytecode);
        reader.check_header()?;

        for s in reader.iter_section() {
            match (s?).data {
                vm::reader::SectionData::Type(function_types) => {
                    module.type_section = Some(function_types);
                }
                vm::reader::SectionData::Import(sub_reader) => {
                    module.import_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Function(sub_reader) => {
                    module.function_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Table(sub_reader) => {
                    module.table_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Memory(sub_reader) => {
                    module.memory_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Global(sub_reader) => {
                    module.global_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Export(sub_reader) => {
                    module.export_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Start(id) => {
                    module.start_section = Some(id);
                }
                vm::reader::SectionData::DataCount(count) => {
                    module.data_count_section = Some(count);
                }
                vm::reader::SectionData::Code(sub_reader) => {
                    module.code_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
                vm::reader::SectionData::Data(sub_reader) => {
                    module.data_section = Some(
                        sub_reader
                            .collect::<reader::Result<Vec<_>>>()?
                            .into_boxed_slice(),
                    );
                }
            }
        }

        Ok(module)
    }
}
