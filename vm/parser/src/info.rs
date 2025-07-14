use std::collections::HashMap;

use crate::reader::{Bytecode, ExportDesc, GlobalType, Limits, SortedImports, ValueType};
const WASM_PAGE_SIZE: usize = 65536;

#[derive(Debug, Clone)]
pub enum IncludeMode {
    Internal,
    Exported,
    Imported,
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Internal {
        code_id: usize,
        export_id: Option<usize>,
    },
    Imported {
        import_id: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub type_id: usize,
    pub t: FunctionType,
}
impl Function {
    pub fn new_internal(type_id: usize, code_id: usize, export_id: Option<usize>) -> Self {
        Function {
            type_id,
            t: FunctionType::Internal { code_id, export_id },
        }
    }

    pub fn new_imported(type_id: usize, import_id: usize) -> Self {
        Function {
            type_id,
            t: FunctionType::Imported { import_id },
        }
    }
    pub fn is_exported(&self, bytecode: &Bytecode, name: &str) -> Option<usize> {
        match self.t {
            FunctionType::Internal { export_id, code_id } => {
                if let Some(e_id) = export_id {
                    let export = bytecode.get_export(e_id).unwrap();
                    println!("export name: {}", export.name.data);
                    (export.name.data == name).then_some(code_id)
                } else {
                    None
                }
            }
            FunctionType::Imported { .. } => None,
        }
    }

    pub fn get_code_id(&self) -> Option<usize> {
        match self.t {
            FunctionType::Internal { code_id, .. } => Some(code_id),
            FunctionType::Imported { .. } => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum GlobalInfo {
    Internal {
        global_id: usize,
        export_id: Option<usize>,
    },
    Imported {
        import_id: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Global {
    pub t: ValueType,
    pub mutable: bool,

    pub info: GlobalInfo,
}
impl Global {
    pub fn new_imported(global_type: &GlobalType, import_id: usize) -> Self {
        let info = GlobalInfo::Imported { import_id };

        Global {
            t: global_type.t.data,
            mutable: global_type.mutable.data,
            info,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemoryInfo {
    Internal { export_id: Option<usize> },
    Imported { import_id: usize },
}

#[derive(Debug, Clone)]
pub struct Memory {
    limits: Limits,
    info: MemoryInfo,
}

impl Memory {
    pub fn new_imported(import_id: usize, limits: Limits) -> Memory {
        Memory {
            limits,
            info: MemoryInfo::Imported { import_id },
        }
    }
}
#[derive(Debug, Default)]
pub struct BytecodeInfo {
    pub imports: Option<SortedImports>,
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub memories: Vec<Memory>,
}

impl BytecodeInfo {
    pub fn new(bytecode: &Bytecode) -> Self {
        let mut info: BytecodeInfo = Default::default();
        if let Some(imports) = bytecode.sort_imports() {
            info.functions.extend(
                imports
                    .functions
                    .iter()
                    .map(|(id, t_id)| Function::new_imported(*t_id, *id)),
            );

            info.globals.extend(
                imports
                    .globals
                    .iter()
                    .map(|(id, gt)| Global::new_imported(gt, *id)),
            );

            info.memories.extend(
                imports
                    .mems
                    .iter()
                    .map(|(id, limits)| Memory::new_imported(*id, limits.clone())),
            );
            info.imports = Some(imports);
        };
        //TODO: (joh): Exports
        if let Some(funcs) = bytecode.iter_functions() {
            info.functions.extend(
                funcs
                    .cloned()
                    .enumerate()
                    //TODO:(joh): Das ist mega dumm und langsam.
                    .map(|(code_id, type_id)| Function {
                        type_id,
                        t: FunctionType::Internal {
                            code_id,
                            export_id: None,
                        },
                    }),
            );
        }
        if let Some(globals) = bytecode.iter_globals() {
            //TODO: (joh): Exports
            info.globals
                .extend(globals.cloned().enumerate().map(|(global_id, global)| {
                    let t = global.value_type();
                    let mutable = global.is_mut();
                    Global {
                        t,
                        mutable,
                        info: GlobalInfo::Internal {
                            global_id,
                            export_id: None,
                        },
                    }
                }));
        }
        if let Some(memories) = bytecode.iter_memories() {
            info.memories.extend(memories.cloned().map(|limits| Memory {
                limits,
                info: MemoryInfo::Internal { export_id: None },
            }));
        }
        info
    }

    pub fn imported_function_count(&self) -> usize {
        self.imports.as_ref().map_or(0, |i| i.functions.len())
    }

    pub fn has_memory(&self) -> bool {
        self.memories.len() > 0
    }

    pub fn iter_code_locals(
        &self,
        bytecode: &Bytecode,
        func_id: usize,
    ) -> Option<impl Iterator<Item = ValueType>> {
        let func = self.functions.get(func_id)?;
        match func.t {
            FunctionType::Internal { code_id, .. } => {
                let t = bytecode.get_type(func.type_id).unwrap();
                let code = bytecode.get_code(code_id).unwrap();
                Some(t.iter_params().cloned().chain(code.iter_locals()))
            }
            FunctionType::Imported { .. } => None,
        }
    }

    pub fn inital_mem_size_pages(&self) -> Option<usize> {
        self.memories
            .get(0)
            .map(|l| l.limits.min.data as usize * WASM_PAGE_SIZE)
    }

    pub fn get_exported_function_code_id(&self, bytecode: &Bytecode, name: &str) -> Option<usize> {
        self.functions
            .iter()
            .find_map(|f| f.is_exported(bytecode, name))
    }
    pub fn find_function_by_code_id(&self, id: usize) -> Option<(usize, &Function)> {
        self.functions.iter().enumerate().find_map(|(func_id, f)| {
            let code_id = f.get_code_id()?;
            println!("searching: {}", code_id);
            (code_id == id).then_some((func_id, f))
        })
    }
}
