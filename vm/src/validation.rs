
use core::fmt;

use itertools::Itertools;

use crate::{bytecode_info::Function, op::{self, Blocktype, Op}, reader::{self, Position, Reader, ReaderError, SectionData, ValueType}, types::{FuncId, GlobalId, GlobalType, Limits, LocalId, Locals, Type, TypeId}};

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    ReaderError(ReaderError),
    ValueStackUnderflow,
    UnexpectedValueType {got: ValueStackType, expected: ValueStackType},
    UnexpectedEmptyControlStack,
    ReturnTypesDoNotMatch{got: ValueStackType, expexted: ValueStackType},
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
    InvalidLocalId(u32),
    MissingEndOnFunctionExit,
    InvalidJump,
    InvalidJumpId,
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
impl fmt::Display for ValueStackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueStackType::T(value_type) => write!(f, "{value_type}"),
            ValueStackType::Unknown => write!(f, "Unknown"),
        }
    }
}
impl From<ValueType> for ValueStackType {
    fn from(value: ValueType) -> Self {
        Self::T(value)
    }
}
#[derive(Clone, Debug)]
pub struct CtrlFrame {
    opcode: Option<(Op, Position)>,
    in_types: Vec<ValueType>,
    out_types: Vec<ValueType>,
    start_height: usize,
    is_unreachable: bool, 
    jump_table_entry: Option<usize>,
    ip: usize,
}
impl CtrlFrame {
    pub fn new(context: &Validator, ip: usize, opcode: Option<(Op, Position)>, jump_table_entry: Option<usize>, in_types: Vec<ValueType>, out_types: Vec<ValueType>) -> Self {
        let start_height = context.value_stack.len(); 
        CtrlFrame {opcode, ip, jump_table_entry, in_types, out_types, start_height, is_unreachable: false}
    }

    pub fn label_types<'me>(&'me self) -> &'me[ValueType] {
        if let Some((Op::Loop(_), _)) = self.opcode {
            self.in_types.as_slice()
        } else {
            self.out_types.as_slice()
        }
    }
}

#[derive(Debug)]
pub struct NamedType {
    name: Option<String>,
    t: TypeId  
}

#[derive(Debug, Default)]
pub struct Context {
    types: Vec<Type>, 
    internal_func_offset: usize,
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
            match data {
                crate::reader::SectionData::Custom(custom_section_data) => Ok(()), 
                crate::reader::SectionData::Type(sub_reader) => {
                    println!("Type section");
                    Ok(context.types = 
                        sub_reader.map(|e| e?.try_into())
                        .collect::<std::result::Result<Vec<_>, _>>()?)
                },

                crate::reader::SectionData::Import(mut sub_reader) => {
                    println!("Import section");
                    sub_reader.try_for_each(|i| context.add_import(i?))
                },
                crate::reader::SectionData::Function(mut sub_reader) => {
                    println!("Function section");
                    sub_reader.try_for_each(|t| Ok(context.funcs.push(NamedType {t: t?, name: None})))
                },

                crate::reader::SectionData::Table(sub_reader) => todo!(),
                crate::reader::SectionData::Memory(mut sub_reader) => {
                    println!("Memory section");
                    sub_reader.try_for_each(|l| Ok(context.mems.push(l?)))
                },
                crate::reader::SectionData::Global(mut sub_reader) => {
                    println!("Global section");
                    sub_reader.try_for_each(|g| Ok(context.globals.push(g?.t.0)))
                },
                crate::reader::SectionData::Export(mut sub_reader) => {
                    println!("Export section");
                    sub_reader.try_for_each(|e| context.validate_export(e?))
                },
                crate::reader::SectionData::Start(start) => {
                    println!("Start section");
                    _ = context.get_func(start.0)?;
                    context.start = start.0;
                    Ok(()) 
                },
                crate::reader::SectionData::DataCount(count) => {
                    println!("Data count section");
                    context.data_count = count.0;
                    Ok(())
                },
                crate::reader::SectionData::Code(sub_reader) => {
                    println!("Code section");
                    context.code = sub_reader
                        .map(|r| r?.try_into())
                        .collect::<std::result::Result<Vec<Function>, ReaderError>>()?;
        
                    println!("...done!");
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
        println!("id: {}, type {}", id, func.t);
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
                self.internal_func_offset += 1;
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
    pub fn function_count(&self) -> usize {
        self.code.len()
    }
}

#[derive(Default, Debug, Clone)]
pub struct JumpTableEntry {
    ip: isize,
    delta_ip: isize,
}

#[derive(Default, Debug, Clone)]
pub struct JumpTable(Vec<JumpTableEntry>);

impl JumpTable {
    pub fn push_new(&mut self, ip: usize) -> usize {
        self.0.push(JumpTableEntry {ip: ip as isize, delta_ip: ip as isize});
        self.0.len() - 1
    }

    pub fn get_jump_mut(&mut self, id: usize) -> Result<&mut JumpTableEntry> {
        self.0.get_mut(id).ok_or(ValidationError::InvalidJumpId) 
    }
    
}

#[derive(Default)]
pub struct Validator {
    ctrl_stack: Vec<CtrlFrame>,
    ctrl_jump_stack: Vec<Vec<usize>>,
    value_stack: Vec<ValueStackType>,
    locals: Vec<ValueType>,    
    jump_table: JumpTable,
    current_func_id: usize,
    instruction_pointer: usize, 
}

impl Validator {
    pub fn pop_val(&mut self) -> Result<ValueStackType> {
        let current_ctrl = &self.ctrl_stack.last().ok_or(ValidationError::UnexpectedEmptyControlStack)?;
        if current_ctrl.start_height == self.value_stack.len() {
            if current_ctrl.is_unreachable {
                Ok(ValueStackType::Unknown)
            } else {
                Err(ValidationError::ValueStackUnderflow) 
            }
        } else {
            let val = self.value_stack.pop().ok_or(ValidationError::ValueStackUnderflow)?;
            println!("Popping {val}, stack: {}", self.value_stack.len());
            Ok(val)
        }
    }
    pub fn push_val_t(&mut self, val: ValueType) {
        println!("Pushing {val}");
        self.value_stack.push(val.into());
        println!("Stack {}", self.value_stack.len());
        
    }
    pub fn pop_val_expect(&mut self, expected: ValueStackType) -> Result<ValueStackType> {
        self.pop_val()
            .map(|v| if v == expected {Ok(v)} else {Err(ValidationError::UnexpectedValueType { got: v, expected })})?
    }

    pub fn pop_val_expect_val(&mut self, expected: ValueType) -> Result<ValueStackType> {
        self.pop_val_expect(expected.into())
    }


    pub fn push_new_ctrl(&mut self, opcode: Option<(Op, Position)>, in_types: Vec<ValueType>, out_types: Vec<ValueType>)  {
        //TODO: (joh): Das ist nicht sehr eleggant
        self.value_stack.extend(in_types.iter().cloned().map_into::<ValueStackType>());
        let jte = match opcode {
            Some((Op::If(_, _), _)) => Some(self.jump_table.push_new(self.instruction_pointer)),   
            _ => None
        };

        let ctrl = CtrlFrame::new(self, self.instruction_pointer, opcode, jte, in_types, out_types); 
        self.ctrl_jump_stack.push(Vec::new()); 
        self.ctrl_stack.push(ctrl); 
    }

    pub fn pop_ctrl(&mut self) -> Result<CtrlFrame> {
        let out_types = self.ctrl_stack.last().ok_or(ValidationError::UnexpectedEmptyControlStack)?.out_types.clone();
        let start_height = self.ctrl_stack.last().unwrap().start_height; 
        println!("pop ctrl count: {}", out_types.len());
        out_types.iter().cloned().map_into::<ValueStackType>()
            .try_for_each(|t| {
                let val = self.pop_val()?;
                if val != t && val != ValueStackType::Unknown {Err(ValidationError::ReturnTypesDoNotMatch {got: val, expexted: t})} else {Ok(())}
            })?;

        if self.value_stack.len() != start_height {
            return Err(ValidationError::UnbalancedStack)
        }

        let frame = self.ctrl_stack.pop().unwrap(); 
        Ok(frame)
    }
    
    pub fn peek_ctrl_at_label(&self, label: u32) -> Result<&CtrlFrame> {
        let id = (self.ctrl_stack.len() as isize - 1) - (label as isize); 
        if id < 0 {
            Err(ValidationError::LabelIndexOutOfScope(label)) 
        } else {
            Ok(&self.ctrl_stack[id as usize])
        }
    }
    pub fn push_ctrl_jump(&mut self, label: u32, jump: usize) -> Result<()> {
        let id = (self.ctrl_jump_stack.len() as isize - 1) - (label as isize); 
        if id < 0 {
            Err(ValidationError::LabelIndexOutOfScope(label))
        }
        else {
            self.ctrl_jump_stack[id as usize].push(jump);
            Ok(())
        }

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

    pub fn check_memarg(&mut self, context: &Context, memarg: op::Memarg, n: u32) -> Result<()> {
        if context.mems.len() <= 0 {
            return Err(ValidationError::UnexpectedNoMemories);
        };
        let align = 2_i32.pow(memarg.align);

        if align > (n / 8) as i32 {
            Err(ValidationError::InvalidAlignment)
        } else {
            Ok(())
        }
    }

    pub fn validate_store_n(&mut self, context: &Context, memarg: op::Memarg, n: u32, t: ValueType) -> Result<()>{
        self.check_memarg(context, memarg, n)?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
    }

    pub fn validate_store(&mut self, context: &Context, memarg: op::Memarg, t: ValueType) -> Result<()> {
        self.check_memarg(context, memarg, t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32)?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
        
    }

    pub fn validate_load(&mut self, context: &Context, memarg: op::Memarg, t: ValueType) -> Result<()> {
        self.check_memarg(context, memarg, t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32)?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }

    pub fn validate_load_n(&mut self, context: &Context, memarg: op::Memarg, n: u32, t: ValueType) -> Result<()> {
        self.check_memarg(context, memarg, n)?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }
    pub fn get_local_type(&self, id: u32) -> Result<ValueType> {
        self.locals.get(id as usize).ok_or(ValidationError::InvalidLocalId(id)).cloned()
    }
    pub fn get_global_type(&self, context: &Context, id: u32) -> Result<GlobalType> {
        context.globals.get(id as usize).ok_or(ValidationError::InvalidLocalID(id)).cloned()
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
    
    pub fn validate_global_get(&mut self, context: &Context, id: u32) -> Result<()> {
        let global_type = self.get_global_type(context, id)?;
        self.push_val_t(global_type.t.0);
        Ok(())
    }
    pub fn validate_global_set(&mut self, context: &Context, id: u32) -> Result<()> {
        let global_type = self.get_global_type(context, id)?;
        if global_type.mutable.0 {
            self.pop_val_expect_val(global_type.t.0)?;
            Ok(())
        } else {
            Err(ValidationError::CannotSetToImmutableGlobal(id))
        }
    }

    pub fn validate_local_tee(&mut self, context: &Context, id: u32) -> Result<()> {
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
    pub fn get_block_types(&self, context: &Context, blocktype: Blocktype) -> Result<(Vec<ValueType>, Vec<ValueType>)> {
        match blocktype {
            Blocktype::Empty => Ok((vec![], vec![])),
            Blocktype::Value(value_type) => Ok((vec![], vec![value_type])),
            Blocktype::TypeIndex(index) => {
                let t = context.types.get(index as usize).ok_or(ValidationError::InvalidTypeId(index))?;
                let in_t = t.params.iter().cloned().map(|(v, _)| v).collect::<Vec<_>>();
                let out_t = t.params.iter().cloned().map(|(v, _)| v).collect::<Vec<_>>();
                Ok((in_t, out_t))
            },
        }
    }

    pub fn validate_block(&mut self, context: &Context, op: (Op, Position), blocktype: Blocktype) -> Result<()> {
        let (in_types, out_types) = self.get_block_types(context, blocktype)?;
        in_types.iter().cloned().for_each(|f| self.push_val_t(f));     
        self.push_new_ctrl(Some(op), in_types, out_types); 
        Ok(())
    }

    pub fn validate_else(&mut self, op: (Op, Position)) -> Result<()> {
        /*
        let jmp = self.jump_table.push_new(self.instruction_pointer);
        self.ctrl_jump_stack
            .last_mut()
            .ok_or(ValidationError::UnexpectedEmptyControlStack)?
            .push(jmp);
        */
        let ctrl = self.pop_ctrl()?;

        println!("Blibb");
        if let Some((Op::If(_, _), _)) = ctrl.opcode {
            println!("Blubb");
            let if_jmp = self.jump_table.get_jump_mut(ctrl.jump_table_entry.unwrap())?;
            println!("ctrl: {:?}", ctrl);
            if_jmp.delta_ip = self.instruction_pointer as isize - ctrl.ip as isize; 
            self.push_new_ctrl(Some(op), ctrl.in_types, ctrl.out_types);
            Ok(())
        } else {
            Err(ValidationError::ElseWithoutIf)
        }
    }

    pub fn validate_end(&mut self) -> Result<()> {
        println!("end!");
        let ctrl = self.pop_ctrl()?;
        ctrl.out_types.iter().for_each(|t| self.push_val_t(t.clone()));
        if let Some((ctrl_op, _)) = ctrl.opcode {
            let jumps_idx = self.ctrl_jump_stack.pop().ok_or(ValidationError::UnexpectedEmptyControlStack)?;
            for idx in jumps_idx {
                println!("ctrl op: {ctrl_op}");
                let jump = self.jump_table.get_jump_mut(idx)?;

                let next_ip = match ctrl_op {
                    Op::Loop(_) => ctrl.ip as isize - jump.ip,
                    Op::Block(_) | Op::If(_,_) | Op::Else => self.instruction_pointer as isize - jump.ip,
                    _ => return Err(ValidationError::InvalidJump)
                };
                jump.delta_ip = next_ip;
            }

            if let Some(jte) = ctrl.jump_table_entry {
                let jump = self.jump_table.get_jump_mut(jte)?;
                jump.delta_ip = self.instruction_pointer as isize - jump.ip  
            }
        }
        Ok(()) 
    }


    pub fn validate_br(&mut self, n: u32) -> Result<()> {
        let jmp = self.jump_table.push_new(self.instruction_pointer);

        self.push_ctrl_jump(n, jmp)?;
            
        let vals = self.peek_ctrl_at_label(n)? 
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        //TODO: (joh): Das ist schreklich
        vals.iter().try_for_each(|t| {_ = self.pop_val_expect_val(t.clone())?; Ok::<_, ValidationError>(())})?;
        self.set_unreachable()?;

        Ok(())
    }

    pub fn validate_br_if(&mut self, n: u32) -> Result<()> {
        let jmp = self.jump_table.push_new(self.instruction_pointer);

        self.push_ctrl_jump(n, jmp)?;

        self.pop_val_expect_val(ValueType::I32)?;

        let vals = self.peek_ctrl_at_label(n)?
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        
        vals.iter().try_for_each(|t| {_ = self.pop_val_expect_val(t.clone())?; Ok::<_, ValidationError>(())})?;
        self.value_stack.extend(vals.iter().cloned().map_into::<ValueStackType>());
        Ok(())
    }

    pub fn validate_return(&mut self, context: &Context) -> Result<()> {
        let funcs = context.get_func(self.current_func_id as u32).unwrap().1.results.clone();
            funcs
            .iter()
            .cloned()
            .try_for_each(|t| -> Result<()> {_ = self.pop_val_expect_val(t.0)?; Ok(())})?;
        self.set_unreachable() 
    }
        
    pub fn validate_call(&mut self, context: &Context, call_id: u32) -> Result<()> {
        let params = context.get_func(call_id)?.1.params.clone(); 
        let results = context.get_func(call_id)?.1.results.clone();
        params.iter().cloned().try_for_each(|(t, _)| -> Result<()> {_ = self.pop_val_expect_val(t)?; Ok(())})?;
        results.iter().cloned().for_each(|(t, _)| {_ = self.push_val_t(t);});
        Ok(()) 
    }

    pub fn validate_op(&mut self, context: &Context, op: (Op, Position)) -> Result<()> {
        use ValueType::*;
        match op.0 {
            Op::Unreachable => self.set_unreachable()?,
            Op::Nop => {},
            Op::Block(blocktype) => self.validate_block(context,op, blocktype)?,
            Op::Loop(blocktype) => self.validate_block(context, op, blocktype)?,
            Op::If(blocktype, _) => {
                self.pop_val_expect_val(I32)?;
                self.validate_block(context, op, blocktype)?;
            },
            Op::Else => self.validate_else(op)?,
            Op::End => self.validate_end()?,
            Op::Br(n, _) => self.validate_br(n)?,
            Op::BrIf(n, _) => self.validate_br_if(n)?,
            Op::Return => self.validate_return(context)?,
            Op::Call(call_id) => self.validate_call(context,call_id)?,
            Op::CallIndirect(_, _) => todo!(),
            Op::Drop => _ = self.pop_val()?,
            Op::Select(t) => self.validate_select(t)?,
            Op::LocalGet(id) => self.validate_local_get(id)?, 
            Op::LocalSet(id) => self.validate_local_set(id)?,
            Op::LocalTee(id) => self.validate_local_tee(context,id)?,
            Op::GlobalGet(id) => self.validate_global_get(context,id)?,
            Op::GlobalSet(id) => self.validate_global_set(context, id)?,
            Op::I32Load(memarg) => self.validate_load(context,memarg, I32)?,
            Op::I64Load(memarg) => self.validate_load(context,memarg, I64)?,
            Op::F32Load(memarg) => self.validate_load(context,memarg, F32)?,
            Op::F64Load(memarg) => self.validate_load(context,memarg, F64)?, 
            Op::I32Load8s(memarg) | 
            Op::I32Load8u(memarg) => self.validate_load_n(context,memarg, 8, I32)?,
            Op::I32Load16s(memarg) |
            Op::I32Load16u(memarg) => self.validate_load_n(context,memarg, 16, I32)?,
            Op::I64Load8s(memarg) |
            Op::I64Load8u(memarg) => self.validate_load_n(context,memarg, 8, I64)?,
            Op::I64Load16s(memarg) |
            Op::I64Load16u(memarg) => self.validate_load_n(context,memarg, 16, I64)?,
            Op::I64Load32s(memarg) |
            Op::I64Load32u(memarg) => self.validate_load_n(context,memarg, 32, I64)?,
            Op::I32Store(memarg) => self.validate_store(context, memarg, I32)?, 
            Op::I64Store(memarg) => self.validate_store(context,memarg, I64)?,
            Op::F32Store(memarg) => self.validate_store(context,memarg, F32)?,
            Op::F64Store(memarg) => self.validate_store(context,memarg, F64)?,
            Op::I32Store8(memarg) => self.validate_store_n(context,memarg, 8, I32)?,
            Op::I32Store16(memarg) => self.validate_store_n(context,memarg, 16, I32)?,
            Op::I64Store8(memarg) => self.validate_store_n(context,memarg, 8, I64)?,
            Op::I64Store16(memarg) => self.validate_store_n(context,memarg, 16, I64)?,
            Op::I64Store32(memarg)=> self.validate_store_n(context,memarg, 32, I64)? ,
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
        self.instruction_pointer += 1;
        Ok(())
    }
    pub fn validate_func(context: &Context, func_id: usize) -> Result<JumpTable> {
        let code = context.code.get(func_id - context.internal_func_offset).ok_or(ValidationError::InvalidLocalID(func_id as u32))?;
        let func_type = context.get_func(func_id as u32)?.1;
        println!("Validating function: {} {func_type} {}", func_id, func_id - context.internal_func_offset);
        let params = &context.get_func(func_id as u32)?.1.params.iter().cloned().map(|(v,_)| v).collect::<Vec<ValueType>>();
        let results = context.get_func(func_id as u32)?.1.results.iter().cloned().map(|(v, p)| v).collect::<Vec<ValueType>>();

        let locals = 
            code.locals
            .iter()
            .cloned()
            .map(|l| l.0.into_iter())
            .flatten();

        let func_locals = 
            params
            .iter()
            .cloned()
            .chain(locals)
            .collect::<Vec<ValueType>>();

        let mut validator = Validator {
            current_func_id: func_id,
            locals: func_locals,
            ..Default::default()
        };
        println!("results {:?}", results);
        validator.push_new_ctrl(None, params.clone().to_vec(), results);
        for op in code.code.iter() {
            println!("Validating {}", op.0);
            validator.validate_op(context, op.clone())?;
        }
        
        Ok(validator.jump_table)
    }

    pub fn validate_all(context: &mut Context) -> Result<()> {
        println!("validating! offset: {}", context.internal_func_offset);
        for i in context.internal_func_offset..context.function_count() + context.internal_func_offset {
            let jump_table = Validator::validate_func(context, i)?;  
            let func = context.code.get_mut(i - context.internal_func_offset).unwrap();
            func.patch_jumps(&jump_table)?;
        }
        Ok(())
    }

}

impl Function {
    pub fn patch_jumps(&mut self, jump_table: &JumpTable) -> Result<()>{
        for jmp in &jump_table.0 {
            let op = self.code[jmp.ip as usize].0.clone(); 
            //...
            println!("jump op: {op}");
            let new_op = match op {
                Op::If(bt, _) => Op::If(bt, jmp.delta_ip),
                Op::Br(bt, _) => Op::Br(bt, jmp.delta_ip),
                Op::BrIf(bt, _) => Op::BrIf(bt, jmp.delta_ip),
                _ => return Err(ValidationError::InvalidJump),
            };

            self.code[jmp.ip as usize].0 = new_op;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn get_wasm_gen() -> Box<[u8]> {
        let source = include_str!("../../ref-project/out/out.wat");
        let source = wat::parse_str(source).unwrap().into_boxed_slice();
        fs::write("gen2.wasm", &source).unwrap();
        source
    }

    #[test]
    fn validate_simple() -> Result<()> {
        let wasm = get_wasm_gen();
        let reader = Reader::new(&wasm, 0);
        let mut context = Context::from_reader(reader)?; 
        println!("Context {:#?}", context);
        Validator::validate_all(&mut context)?;         
        
        context.code.iter().for_each(|c| println!("{c}"));
        Ok(())
    }
}
