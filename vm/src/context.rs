use std::result;

use itertools::Itertools;

use crate::{bytecode_info::{Function, Import}, op::{self, Blocktype, Op}, reader::{self, Position, Reader, ReaderError, SectionData, ValueType}, types::{FuncId, GlobalId, GlobalType, Limits, Type, TypeId}};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    ReaderError(ReaderError),
    ValueStackUnderflow,
    UnexpectedValueType {got: ValueStackType, expected: ValueStackType},
    UnexpectedEmptyControlStack,
    ReturnTypesDoNotMatch,
    UnbalancedStack,
    UnexpectedNoMemories,
    InvalidAlignment,
    InvalidLocalID(u32),
    InvalidGlobalID(u32),
    CannotSetToImmutableGlobal(u32),
    ExpectedNumericType,
    InvalidTypeId(u32),
    ElseWithoutIf,
    LabelIndexOutOfScope(u32),
    InvalidFuncId(u32),
    InvalidMemId(u32),
}

pub type Result<T> = std::result::Result<T, ValidationError>;

impl From<ReaderError> for ValidationError {
    fn from(value: ReaderError) -> Self {
        Self::ReaderError(value)
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueStackType {
    T(ValueType),
    Unknown,
}

impl ValueStackType {
    pub fn is_num(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => {
                value_type.is_num()
            },
            _ => true,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => {
                value_type.is_vec()
            },
            _ => true,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => {
                value_type.is_ref()
            },
            _ => true,
        }
    }
}
impl From<ValueType> for ValueStackType {
    fn from(value: ValueType) -> Self {
        Self::T(value)
    }
}
pub struct CtrlFrame {
    opcode: (Op, Position),
    in_types: Vec<ValueType>,
    out_types: Vec<ValueType>,
    start_height: usize,
    is_unreachable: bool, 
}
impl CtrlFrame {
    pub fn new(context: &Validator, opcode: (Op, Position), in_types: Vec<ValueType>, out_types: Vec<ValueType>) -> Self {
        let start_height = context.value_stack.len(); 
        CtrlFrame {opcode, in_types, out_types, start_height, is_unreachable: false}
    }
    pub fn label_types<'me>(&'me self) -> &'me[ValueType] {
        if let (Op::Loop(_), _) = self.opcode {
            self.in_types.as_slice()
        } else {
            self.out_types.as_slice()
        }
    }
}

pub struct NamedType {
    name: Option<String>,
    t: TypeId  
}

#[derive(Default)]
pub struct Context {
    types: Vec<Type>, 
    funcs: Vec<NamedType>,
    //TODO: Tables
    mems: Vec<Limits>,
    globals: Vec<GlobalType>,
    
    //TODO: Elems
    //locals: Vec<Vec<ValueType>>,
    start: FuncId,
    data_count: u32,
    code: Vec<Function>,
}
impl Context {
    pub fn from_reader(mut reader: Reader) -> Result<Self> {
        let mut context: Context = Default::default();
        _ = reader.check_header(); 
        for s in reader.sections_iter() {
            let s = s?;
            let data = s.0.data;               
            let pos = s.1; 
            
            match data {
                crate::reader::SectionData::Custom(custom_section_data) => Ok(()), 
                crate::reader::SectionData::Type(sub_reader) => {
                    Ok(context.types = 
                        sub_reader.map(|e| e?.try_into())
                        .collect::<result::Result<Vec<_>, _>>()?)
                },
                crate::reader::SectionData::Import(mut sub_reader) => {
                    sub_reader.try_for_each(|i| context.add_import(i?))
                },
                crate::reader::SectionData::Function(mut sub_reader) => {
                    sub_reader.try_for_each(|t| Ok(context.funcs.push(NamedType {t: t?, name: None})))
                },

                crate::reader::SectionData::Table(sub_reader) => todo!(),
                crate::reader::SectionData::Memory(mut sub_reader) => {
                    sub_reader.try_for_each(|l| Ok(context.mems.push(l?)))
                },
                crate::reader::SectionData::Global(mut sub_reader) => {
                    sub_reader.try_for_each(|g| Ok(context.globals.push(g?.t.0)))
                },
                crate::reader::SectionData::Export(mut sub_reader) => {
                    sub_reader.try_for_each(|e| context.validate_export(e?))
                },
                crate::reader::SectionData::Start(start) => {
                    _ = context.get_func(start.0)?;
                    context.start = start.0;
                    Ok(()) 
                },
                crate::reader::SectionData::DataCount(count) => {
                    context.data_count = count.0;
                    Ok(())
                },
                crate::reader::SectionData::Code(sub_reader) => {
                    context.code = sub_reader
                        .map(|r| r?.try_into())
                        .collect::<result::Result<Vec<Function>, ReaderError>>()?;
        
                    Ok(())
                },
                //TODO: (joh) Wo klonen wir die Daten?
                crate::reader::SectionData::Data(sub_reader) => Ok(()),
            }?;
        }
        Ok(context)
    }

    pub fn get_type(&self, id: TypeId) -> Result<&Type> {
        self.types.get(id as usize).ok_or(ValidationError::InvalidTypeId(id))
    }

    pub fn get_func(&self, id: FuncId) -> Result<(Option<&str>, &Type)> {
        let func = self.funcs.get(id as usize).ok_or(ValidationError::InvalidFuncId(id))?;
        Ok((func.name.as_deref(), &self.types[func.t as usize]))
    }

    pub fn get_mem(&self, id: FuncId) -> Result<Limits> {
        self.mems.get(id as usize).ok_or(ValidationError::InvalidMemId(id)).cloned() 
    }
     
    pub fn get_global(&self, id: GlobalId) -> Result<&GlobalType> {
        self.globals.get(id as usize).ok_or(ValidationError::InvalidGlobalID(id))
    }
    pub fn add_import(&mut self, import: reader::Import) -> Result<()> {
        match import.desc.0 {
            crate::types::ImportDesc::TypeIdx(id) => {
                _ = self.get_type(id)?;
                let name = Some(String::from(import.name.0)); 
                self.funcs.push(NamedType {t: id, name});  
                Ok(())
            },
            crate::types::ImportDesc::TableType(_) => todo!(),
            crate::types::ImportDesc::MemType(limits) => {
                self.mems.push(limits);
                Ok(())
            },
            crate::types::ImportDesc::GlobalType(global_type) => {
                self.globals.push(global_type);                 
                Ok(())
            },
        }
    }
    pub fn validate_export(&mut self, export: reader::Export) -> Result<()> {
        match export.desc.0 {
            reader::ExportDesc::FuncId(id) => Ok(_ = self.get_func(id)?),
            reader::ExportDesc::TableId(_) => todo!(),
            reader::ExportDesc::MemId(id) => Ok(_ = self.get_mem(id)?),
            reader::ExportDesc::GlobalId(id) => Ok(_ = self.get_global(id)?),
        }
    }
}
#[derive(Default)]
pub struct Validator {
    context: Context,
    ctrl_stack: Vec<CtrlFrame>,
    value_stack: Vec<ValueStackType>,
    
    current_func_id: usize,
}

impl Validator {
    pub fn pop_val(&mut self) -> Result<ValueStackType> {
        let current_ctrl = &self.ctrl_stack.last().ok_or(ValidationError::ValueStackUnderflow)?;
        if current_ctrl.start_height == self.value_stack.len() {
            if current_ctrl.is_unreachable {
                Ok(ValueStackType::Unknown)
            } else {
                Err(ValidationError::ValueStackUnderflow) 
            }
        } else {
            self.value_stack.pop().ok_or(ValidationError::ValueStackUnderflow)
        }
    }
    pub fn push_val_t(&mut self, val: ValueType) {
        self.value_stack.push(val.into());
    }
    pub fn pop_val_expect(&mut self, expected: ValueStackType) -> Result<ValueStackType> {
        self.pop_val()
            .map(|v| if v == expected {Ok(v)} else {Err(ValidationError::UnexpectedValueType { got: v, expected })})?
    }
    pub fn pop_val_expect_val(&mut self, expected: ValueType) -> Result<ValueStackType> {
        self.pop_val_expect(expected.into())
    }

    pub fn push_new_ctrl(&mut self, opcode: (Op, Position), in_types: Vec<ValueType>, out_types: Vec<ValueType>) 
    {
        //TODO: (joh): Das ist nicht sehr eleggant
        self.value_stack.extend(in_types.iter().cloned().map_into::<ValueStackType>());
        let ctrl = CtrlFrame::new(self, opcode, in_types, out_types); 
        self.ctrl_stack.push(ctrl); 
 
    }
    pub fn pop_ctrl(&mut self) -> Result<CtrlFrame> {
        let frame = self.ctrl_stack.pop().ok_or(ValidationError::UnexpectedEmptyControlStack)?;
        frame.out_types.iter().cloned().map_into::<ValueStackType>()
            .try_for_each(|t| if self.pop_val()? != t {Err(ValidationError::ReturnTypesDoNotMatch)} else {Ok(())})?;

        if self.value_stack.len() != frame.start_height {
            return Err(ValidationError::UnbalancedStack)
        }
        Ok(frame)
    }

    pub fn set_unreachable(&mut self) -> Result<()>{
        let frame = self.ctrl_stack.last_mut().ok_or(ValidationError::UnexpectedEmptyControlStack)?;
        self.value_stack.truncate(frame.start_height);
        frame.is_unreachable = true;
        Ok(())
    }

    pub fn validate_binop(&mut self, val_type: ValueType) -> Result<()> {
        self.pop_val_expect_val(val_type)?;
        self.pop_val_expect_val(val_type)?;
        self.push_val_t(val_type);
        Ok(()) 
    }

    pub fn validate_relop(&mut self, t: ValueType) -> Result<()> {
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(t)?;
        self.push_val_t(ValueType::I32);
        Ok(()) 
    }

    pub fn check_memarg(&mut self, memarg: op::Memarg, n: u32) -> Result<()> {
        if self.context.mems.len() <= 0 {
            return Err(ValidationError::UnexpectedNoMemories);
        };
        let align = 2_i32.pow(memarg.align);

        if align > (n / 8) as i32 {
            Err(ValidationError::InvalidAlignment)
        } else {
            Ok(())
        }
    }

    pub fn validate_store_n(&mut self, memarg: op::Memarg, n: u32, t: ValueType) -> Result<()>{
        self.check_memarg(memarg, n)?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
    }

    pub fn validate_store(&mut self, memarg: op::Memarg, t: ValueType) -> Result<()> {
        self.check_memarg(memarg, t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32)?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
        
    }

    pub fn validate_load(&mut self, memarg: op::Memarg, t: ValueType) -> Result<()> {
        self.check_memarg(memarg, t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32)?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }

    pub fn validate_load_n(&mut self, memarg: op::Memarg, n: u32, t: ValueType) -> Result<()> {
        self.check_memarg(memarg, n)?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }
    pub fn get_local_type(&self, id: u32) -> Result<ValueType> {
        self.locals[self.current_func_id].get(id as usize).ok_or(ValidationError::InvalidLocalID(id)).copied()
    }
    pub fn get_global_type(&self, id: u32) -> Result<GlobalType> {
        self.globals.get(id as usize).ok_or(ValidationError::InvalidLocalID(id)).cloned()
    }

    pub fn validate_local_get(&mut self, id: u32) -> Result<()> {
        let local_type = self.get_local_type(id)?;
        self.push_val_t(local_type);
        Ok(())
    }
    pub fn validate_local_set(&mut self, id: u32) -> Result<()> {
        let local_type = self.get_local_type(id)?;
        self.pop_val_expect_val(local_type)?;
        Ok(())
    }
    
    pub fn validate_global_get(&mut self, id: u32) -> Result<()> {
        let global_type = self.get_global_type(id)?;
        self.push_val_t(global_type.t.0);
        Ok(())
    }
    pub fn validate_global_set(&mut self, id: u32) -> Result<()> {
        let global_type = self.get_global_type(id)?;
        if global_type.mutable.0 {
            self.pop_val_expect_val(global_type.t.0)?;
            Ok(())
        } else {
            Err(ValidationError::CannotSetToImmutableGlobal(id))
        }
    }
    pub fn validate_local_tee(&mut self, id: u32) -> Result<()> {
        let local_type = self.get_local_type(id)?;
        self.pop_val_expect_val(local_type)?;
        self.push_val_t(local_type);
        Ok(())
    }
    pub fn validate_select(&mut self, t: Option<ValueType>) -> Result<()> {
        match t {
            Some(v) => {
                self.pop_val_expect_val(v)?;
                self.pop_val_expect_val(v)?;
                self.pop_val_expect_val(ValueType::I32)?;
                self.push_val_t(v);
                Ok(())
            }
            None => {
                self.pop_val_expect_val(ValueType::I32)?;
                let t1 = self.pop_val()?;
                let t2 = self.pop_val()?;
                if !(t1.is_num() || t1.is_vec()) {
                    return Err(ValidationError::ExpectedNumericType);
                }
                
                if t1 != t2 {
                    return Err(ValidationError::UnexpectedValueType { got: t2, expected: t1});
                }
                Ok(())
            }
        }
    }
    pub fn get_block_types(&self, blocktype: Blocktype) -> Result<(Vec<ValueType>, Vec<ValueType>)> {
        match blocktype {
            Blocktype::Empty => Ok((vec![], vec![])),
            Blocktype::Value(value_type) => Ok((vec![], vec![value_type])),
            Blocktype::TypeIndex(index) => {
                let t = self.types.get(index as usize).ok_or(ValidationError::InvalidTypeId(index))?;
                let in_t = t.params.iter().cloned().map(|(v, _)| v).collect::<Vec<_>>();
                let out_t = t.params.iter().cloned().map(|(v, _)| v).collect::<Vec<_>>();
                Ok((in_t, out_t))
            },
        }
    }

    pub fn validate_block(&mut self, op: Op, blocktype: Blocktype) -> Result<()> {
        let (in_types, out_types) = self.get_block_types(blocktype)?;
        in_types.iter().cloned().for_each(|f| self.push_val_t(f));     
        self.push_new_ctrl(op, in_types, out_types); 
        Ok(())
    }

    pub fn validate_else(&mut self, op: Op) -> Result<()> {
        let ctrl = self.pop_ctrl()?;
        if let Op::If(_) = ctrl.opcode {
            return Err(ValidationError::ElseWithoutIf);
        }
        self.push_new_ctrl(op, ctrl.in_types, ctrl.out_types);      
        Ok(())
    }

    pub fn validate_end(&mut self) -> Result<()> {
        let ctrl = self.pop_ctrl()?;
        ctrl.out_types.iter().for_each(|t| self.push_val_t(t.clone()));
        Ok(()) 
    }

    pub fn validate_br(&mut self, n: u32) -> Result<()> {
        let vals = self.ctrl_stack.get(n as usize)
            .ok_or(ValidationError::LabelIndexOutOfScope(n))?
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        //TODO: (joh): Das ist schreklich
        vals.iter().try_for_each(|t| {_ = self.pop_val_expect_val(t.clone())?; Ok(())})?;
        self.set_unreachable()?;

        Ok(())
    }

    pub fn validate_br_if(&mut self, n: u32) -> Result<()> {
        self.pop_val_expect_val(ValueType::I32)?;
        let vals = self.ctrl_stack.get(n as usize)
            .ok_or(ValidationError::LabelIndexOutOfScope(n))?
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        vals.iter().try_for_each(|t| {_ = self.pop_val_expect_val(t.clone())?; Ok(())})?;
        self.value_stack.extend(vals.iter().cloned().map_into::<ValueStackType>());
        Ok(())
    }

    pub fn validate_return(&mut self) -> Result<()> {
        let funcs = self.funcs[self.current_func_id].results.clone();
            funcs
            .iter()
            .cloned()
            .try_for_each(|t| {_ = self.pop_val_expect_val(t.0)?; Ok(())})?;
        self.set_unreachable() 
    }
        
    pub fn validate_call(&mut self, call_id: u32) -> Result<()> {
        let func = self.funcs.get(call_id as usize).ok_or(ValidationError::InvalidFuncId)?;    
        func.params.into_iter().try_for_each(|(t, _)| {_ = self.pop_val_expect_val(t)?; Ok(())});
        func.results.into_iter().for_each(|(t, _)| {_ = self.push_val_t(t);});
        Ok(()) 
    }

    pub fn validate_op(&mut self, op: Op) -> Result<()> {
        use ValueType::*;
        match op {
            Op::Unreachable => self.set_unreachable()?,
            Op::Nop => {},
            Op::Block(blocktype) => self.validate_block(op, blocktype)?,
            Op::Loop(blocktype) => self.validate_block(op, blocktype)?,
            Op::If(blocktype) => {
                self.pop_val_expect_val(I32);
                self.validate_block(op, blocktype)?;
            },
            Op::Else => self.validate_else(op)?,
            Op::End => self.validate_end()?,
            Op::Br(n) => self.validate_br(n)?,
            Op::BrIf(n) => self.validate_br_if(n)?,
            Op::Return => self.validate_return()?,
            Op::Call(call_id) => self.validate_call(call_id)?,
            Op::CallIndirect(_, _) => todo!(),
            Op::Drop => _ = self.pop_val()?,
            Op::Select(t) => self.validate_select(t)?,
            Op::LocalGet(id) => self.validate_local_get(id)?, 
            Op::LocalSet(id) => self.validate_local_set(id)?,
            Op::LocalTee(id) => self.validate_local_tee(id)?,
            Op::GlobalGet(id) => self.validate_global_get(id)?,
            Op::GlobalSet(id) => self.validate_global_set(id)?,
            Op::I32Load(memarg) => self.validate_load(memarg, I32)?,
            Op::I64Load(memarg) => self.validate_load(memarg, I64)?,
            Op::F32Load(memarg) => self.validate_load(memarg, F32)?,
            Op::F64Load(memarg) => self.validate_load(memarg, F64)?, 
            Op::I32Load8s(memarg) | 
            Op::I32Load8u(memarg) => self.validate_load_n(memarg, 8, I32)?,
            Op::I32Load16s(memarg) |
            Op::I32Load16u(memarg) => self.validate_load_n(memarg, 16, I32)?,
            Op::I64Load8s(memarg) |
            Op::I64Load8u(memarg) => self.validate_load_n(memarg, 8, I64)?,
            Op::I64Load16s(memarg) |
            Op::I64Load16u(memarg) => self.validate_load_n(memarg, 16, I64)?,
            Op::I64Load32s(memarg) |
            Op::I64Load32u(memarg) => self.validate_load_n(memarg, 32, I64)?,
            Op::I32Store(memarg) => self.validate_store(memarg, I32)?, 
            Op::I64Store(memarg) => self.validate_store(memarg, I64)?,
            Op::F32Store(memarg) => self.validate_store(memarg, F32)?,
            Op::F64Store(memarg) => self.validate_store(memarg, F64)?,
            Op::I32Store8(memarg) => self.validate_store_n(memarg, 8, I32)?,
            Op::I32Store16(memarg) => self.validate_store_n(memarg, 16, I32)?,
            Op::I64Store8(memarg) => self.validate_store_n(memarg, 8, I64)?,
            Op::I64Store16(memarg) => self.validate_store_n(memarg, 16, I64)?,
            Op::I64Store32(memarg)=> self.validate_store_n(memarg, 32, I64)? ,
            Op::I32Const(_) => self.push_val_t(I32), 
            Op::I64Const(_) => self.push_val_t(I64),
            Op::F32Const(_) => self.push_val_t(F32),
            Op::F64Const(_) => self.push_val_t(F64),
            Op::I32Eqz |
            Op::I32Eq  |
            Op::I32Ne  |
            Op::I32Lts |
            Op::I32Ltu |
            Op::I32Gts |
            Op::I32Gtu |
            Op::I32Leu |
            Op::I32Les |
            Op::I32Ges |
            Op::I32Geu => self.validate_relop(I32)?,
            Op::I64Eqz |
            Op::I64Eq  |
            Op::I64Ne  |
            Op::I64Lts |
            Op::I64Ltu |
            Op::I64Gts |
            Op::I64Gtu |
            Op::I64Les |
            Op::I64Leu |
            Op::I64Ges |
            Op::I64Geu => self.validate_relop(I64)?,
            Op::I32Add |
            Op::I32Sub | 
            Op::I32Mul | 
            Op::I32Divs | 
            Op::I32Divu |
            Op::I32Rems | 
            Op::I32Remu | 
            Op::I32And |
            Op::I32Or |
            Op::I32Xor | 
            Op::I32Shl | 
            Op::I32Shrs |
            Op::I32Shru | 
            Op::I32Rotl | 
            Op::I32Rotr => self.validate_binop(I32)?,
            Op::I64Add |
            Op::I64Sub |
            Op::I64Mul |
            Op::I64Divs |
            Op::I64Divu |
            Op::I64Rems |
            Op::I64Remu |
            Op::I64And |
            Op::I64Or |
            Op::I64Xor |
            Op::I64Shl |
            Op::I64Shrs |
            Op::I64Shru |
            Op::I64Rotl |
            Op::I64Rotr => self.validate_binop(I64)?,
            Op::MemoryCopy => todo!(),
            Op::MemoryFill => todo!(),
        };
        Ok(())
    }
}
