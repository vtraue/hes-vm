use crate::reader::{Bytecode, GlobalType, Limits, SortedImports, ValueType};

#[derive(Debug, Clone)]
pub enum IncludeMode {
    Internal,
    Exported,
    Imported
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Internal {code_id: usize, export_id: Option<usize>},   
    Imported {import_id: usize},
}

#[derive(Debug, Clone)]
pub struct Function {
    type_id: usize,
    t: FunctionType, 
}
impl Function {
    pub fn new_internal(type_id: usize, code_id: usize, export_id: Option<usize>) -> Self {
        Function {
            type_id,
            t: FunctionType::Internal { code_id, export_id }
        }
    }

    pub fn new_imported(type_id: usize, import_id: usize) -> Self {
        Function {
            type_id,
            t: FunctionType::Imported {import_id}
        }
    }
}
#[derive(Debug, Clone)]
pub enum GlobalInfo {
    Internal {global_id: usize, export_id: Option<usize>},
    Imported {import_id: usize} 
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
    Internal {export_id: Option<usize>},
    Imported {import_id: usize}
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
            info.functions.extend(imports.functions
                .iter()
                .map(|(id, t_id)| Function::new_imported(*id, *t_id)));

            info.globals.extend(imports.globals
                .iter()
                .map(|(id, gt)| Global::new_imported(gt, *id)));

            info.memories.extend(imports.mems.iter().map(|(id, limits)| Memory::new_imported(*id, limits.clone())));  
            info.imports = Some(imports);  
        };
        info
    }
    pub fn has_memory(&self) -> bool {
        self.memories.len() > 0 
    }
}
