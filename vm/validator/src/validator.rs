use std::fmt::{Debug, Display};
use thiserror::Error;

use itertools::Itertools;
use parser::{
    info::{BytecodeInfo, FunctionType},
    op::{Blocktype, Memarg, Op},
    reader::{
        self, Bytecode, BytecodeReader, Code, Function, ParserError, Type, ValueType, WithPosition,
        parse_binary, parse_wat,
    },
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueStackType {
    T(ValueType),
    Unknown,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Unexpected empty control stack")]
    UnexpetedEmptyControlStack,
    #[error("Underflow in type stack")]
    TypeStackUnderflow,
    #[error("Popped an unexpected value from type stack. Expected: {expected}, got: {got}")]
    PoppedUnexpectedType {
        got: ValueStackType,
        expected: ValueStackType,
    },
    #[error("Stack is not balanced. Expected size {expected}, got:  {got}")]
    UnbalancedStack { got: usize, expected: usize },

    #[error("Label out of scope: Maximum depth: {max}, got {got}")]
    LabelIndexOutOfScope { got: usize, max: usize },
    #[error("Tried using memory instructions but module does not define memory")]
    UnexpectedNoMemories,
    #[error("Invalid memory alignment")]
    InvalidMemoryAlignment,
    #[error("Invalid local id: {0}")]
    InvalidLocalId(usize),
    #[error("Invalid global id: {0}")]
    InvalidGlobalId(usize),
    #[error("Expected an numeric type")]
    ExpectedNumericType,
    #[error("Got an invalid block type: Got: {0}")]
    InvalidBlockType(u32),
    #[error("Invalid Jump Table Destionation: Got {0}")]
    InvalidJumpTableDestination(usize),
    #[error("Else instruction is missing if")]
    ElseMissingIf,
    #[error("Unexpected emptry jump stack")]
    UnexpectedEmptyJumpStack,
    #[error("Invalid Op for ctrl frame: Got: {0}")]
    InvalidCtrlOp(Op),
    #[error("Invalid function type id: Got: {0}")]
    InvalidFunctionTypeId(usize),
    #[error("Invalid function id: Got: {0}")]
    InvalidFunctionId(usize),
    #[error("Invalid jump target: {0}")]
    InvalidJump(usize),

    #[error("Unexpected no data defined in module")]
    UnexpectedNoData,

    #[error("Invalid data ID: {0}")]
    InvalidDataId(usize),

    #[error("Trying to init active data section: {0}")]
    InitActiveDataId(usize),
}

impl ValueStackType {
    pub fn is_num(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => value_type.is_num(),
            _ => true,
        }
    }
    pub fn is_vec(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => value_type.is_vec(),
            _ => true,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            ValueStackType::T(value_type) => value_type.is_ref(),
            _ => true,
        }
    }
}

impl Display for ValueStackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueStackType::T(value_type) => write!(f, "{value_type}"),
            ValueStackType::Unknown => write!(f, "Unknown"),
        }
    }
}
impl From<ValueType> for ValueStackType {
    fn from(value: ValueType) -> Self {
        ValueStackType::T(value)
    }
}
impl From<&ValueType> for ValueStackType {
    fn from(value: &ValueType) -> Self {
        Self::T(*value)
    }
}

#[derive(Debug, Clone)]
pub struct CtrlFrame {
    prev_stack_len: usize,
    is_unreachable: bool,
    op: Option<WithPosition<Op>>,
    in_types: Vec<ValueType>,
    out_types: Vec<ValueType>,
    jump_table_entry: Option<usize>,
    ip: isize,
}
impl CtrlFrame {
    pub fn iter_label_types(&self) -> Option<impl Iterator<Item = ValueType>> {
        if let Op::Loop(_) = self.op.as_ref()?.data {
            Some(self.in_types.iter().cloned())
        } else {
            Some(self.out_types.iter().cloned())
        }
    }
}
#[derive(Default, Debug, Clone)]
pub struct JumpTableEntry {
    pub ip: isize,
    pub delta_ip: isize,
    pub stack_height: usize,

    pub out_count: usize,
}

impl JumpTableEntry {
    pub fn new(ip: isize, stack_height: usize, out_count: usize) -> Self {
        JumpTableEntry {
            ip,
            delta_ip: ip,
            stack_height,
            out_count,
        }
    }
}

pub fn last_ctrl(stack: &impl AsRef<[CtrlFrame]>) -> Result<&CtrlFrame, ValidationError> {
    stack
        .as_ref()
        .last()
        .ok_or(ValidationError::UnexpetedEmptyControlStack)
}

pub fn pop_type(
    stack: &mut Vec<ValueStackType>,
    frame_stack_len: usize,
    unreachable: bool,
) -> Result<ValueStackType, ValidationError> {
    if frame_stack_len == stack.len() {
        println!("blubbi!");
        if unreachable {
            Ok(ValueStackType::Unknown)
        } else {
            Err(ValidationError::TypeStackUnderflow)
        }
    } else {
        let val = stack.pop().ok_or(ValidationError::TypeStackUnderflow)?;
        println!("popping {}", val);
        Ok(val)
    }
}

pub fn pop_type_expect(
    stack: &mut Vec<ValueStackType>,
    frame_stack_len: usize,
    unreachable: bool,
    expected: impl Into<ValueStackType>,
) -> Result<ValueStackType, ValidationError> {
    pop_type(stack, frame_stack_len, unreachable).map(|got| {
        let expected = expected.into();
        if got == expected || got == ValueStackType::Unknown || expected == ValueStackType::Unknown
        {
            Ok(got)
        } else {
            Err(ValidationError::PoppedUnexpectedType { got, expected })
        }
    })?
}

pub fn pop_values(
    stack: &mut Vec<ValueStackType>,
    frame_stack_len: usize,
    unreachable: bool,
    expected: impl Iterator<Item = impl Into<ValueStackType>>,
) -> Result<(), ValidationError> {
    println!("unreachable: {unreachable}");
    for val in expected {
        pop_type_expect(stack, frame_stack_len, unreachable, val)?;
    }
    Ok(())
}
pub fn push_type(stack: &mut Vec<ValueStackType>, val: impl Into<ValueStackType>) {
    let val = val.into();
    println!("Pushing: {}", val);
    stack.push(val)
}

pub fn push_all(
    stack: &mut Vec<ValueStackType>,
    values: impl IntoIterator<Item = impl Into<ValueStackType>> + Debug,
) {
    stack.extend(values.into_iter().map_into());
}

fn pop_out_values(
    stack: &mut Vec<ValueStackType>,
    frame: &CtrlFrame,
) -> Result<(), ValidationError> {
    pop_values(
        stack,
        frame.prev_stack_len,
        frame.is_unreachable,
        frame.out_types.iter(),
    )
}

fn get_ctrl_peek_id(stack_size: usize, label: usize) -> isize {
    (stack_size as isize - 1) - label as isize
}

fn peek_ctrl(stack: &impl AsRef<[CtrlFrame]>, label: usize) -> Result<&CtrlFrame, ValidationError> {
    let stack = stack.as_ref();
    let id = get_ctrl_peek_id(stack.len(), label);

    stack
        .get(id as usize)
        .ok_or(ValidationError::LabelIndexOutOfScope {
            got: id as usize,
            max: stack.len(),
        })
}

#[derive(Debug, Default)]
pub struct ValidatorContext {
    ip: isize,
    func_id: usize,
    type_stack: Vec<ValueStackType>,
    ctrl_stack: Vec<CtrlFrame>,
    locals: Vec<ValueType>,
    jump_table: Vec<JumpTableEntry>,
    ctrl_jump_stack: Vec<Vec<usize>>,
}

macro_rules! validate_types {
    ($validator:ident, [$($in_value: expr),*$(,)?] =>[$($out_value: expr),*$(,)?] ) => {
        $($validator.pop($in_value)?;)*
        $($validator.push($out_value);)*
    }
}

impl ValidatorContext {
    pub fn get_jump(&self, id: usize) -> Result<&JumpTableEntry, ValidationError> {
        self.jump_table
            .get(id)
            .ok_or(ValidationError::InvalidJumpTableDestination(id))
    }
    pub fn get_jump_mut(&mut self, id: usize) -> Result<&mut JumpTableEntry, ValidationError> {
        self.jump_table
            .get_mut(id)
            .ok_or(ValidationError::InvalidJumpTableDestination(id))
    }

    fn current_ctrl(&self) -> Result<&CtrlFrame, ValidationError> {
        last_ctrl(&self.ctrl_stack)
    }

    fn push_branch_op_jte(&mut self, op: Op, out_type_count: usize) -> Option<usize> {
        if op.is_branch() {
            let entry = JumpTableEntry::new(self.ip, self.type_stack.len(), out_type_count);
            self.jump_table.push(entry);
            Some(self.jump_table.len() - 1)
        } else {
            None
        }
    }

    pub fn pop_any(&mut self) -> Result<(), ValidationError> {
        let len = self.current_ctrl()?.prev_stack_len;
        let unreachable = self.current_ctrl()?.is_unreachable;
        let _ = pop_type(&mut self.type_stack, len, unreachable)?;
        Ok(())
    }
    pub fn pop(&mut self, expected: impl Into<ValueStackType>) -> Result<(), ValidationError> {
        let len = self.current_ctrl()?.prev_stack_len;
        let unreachable = self.current_ctrl()?.is_unreachable;
        let _ = pop_type_expect(&mut self.type_stack, len, unreachable, expected)?;
        Ok(())
    }
    pub fn pop_numeric(&mut self) -> Result<ValueStackType, ValidationError> {
        let len = self.current_ctrl()?.prev_stack_len;
        let unreachable = self.current_ctrl()?.is_unreachable;
        let val = pop_type(&mut self.type_stack, len, unreachable)?;
        if !(val.is_num() || val.is_vec()) {
            Err(ValidationError::ExpectedNumericType)
        } else {
            Ok(val)
        }
    }

    pub fn push(&mut self, t: impl Into<ValueStackType> + Display) {
        self.type_stack.push(t.into());
    }

    pub fn push_ctrl(
        &mut self,
        op: Option<WithPosition<Op>>,
        in_types: Vec<ValueType>,
        out_types: Vec<ValueType>,
    ) {
        let jump_table_entry = if let Some(WithPosition {
            data: op,
            position: _,
        }) = op
        {
            self.push_branch_op_jte(op, out_types.len())
        } else {
            None
        };
        let prev_stack_len = self.type_stack.len();
        push_all(&mut self.type_stack, &in_types);

        let ctrl = CtrlFrame {
            prev_stack_len,
            is_unreachable: false,
            in_types,
            out_types,
            op,
            ip: self.ip,
            jump_table_entry,
        };
        self.ctrl_stack.push(ctrl);
        self.ctrl_jump_stack.push(Vec::new());
    }

    pub fn pop_ctrl(&mut self) -> Result<CtrlFrame, ValidationError> {
        let current_ctrl = last_ctrl(&self.ctrl_stack)?;
        let start_height = current_ctrl.prev_stack_len;
        pop_out_values(&mut self.type_stack, current_ctrl)?;
        if self.type_stack.len() != start_height {
            Err(ValidationError::UnbalancedStack {
                got: self.type_stack.len(),
                expected: start_height,
            })
        } else {
            Ok(self.ctrl_stack.pop().unwrap())
        }
    }

    pub fn get_current_frame_mut(&mut self) -> Result<&mut CtrlFrame, ValidationError> {
        self.ctrl_stack
            .last_mut()
            .ok_or(ValidationError::UnexpetedEmptyControlStack)
    }

    pub fn set_unreachable(&mut self) -> Result<(), ValidationError> {
        let current_frame = self.get_current_frame_mut()?;
        current_frame.is_unreachable = true;
        Ok(())
    }

    pub fn validate_binop(&mut self, val_type: ValueType) -> Result<(), ValidationError> {
        validate_types!(self, [val_type, val_type] => [val_type]);
        Ok(())
    }

    pub fn validate_relop(&mut self, val_type: ValueType) -> Result<(), ValidationError> {
        validate_types!(self, [val_type, val_type] => [ValueType::I32]);
        Ok(())
    }
    pub fn validate_testop(&mut self, val_type: ValueType) -> Result<(), ValidationError> {
        validate_types!(self, [val_type] => [ValueType::I32]);
        Ok(())
    }
    pub fn check_memarg(
        &self,
        info: &BytecodeInfo,
        memarg: Memarg,
        n: u32,
    ) -> Result<(), ValidationError> {
        if !info.has_memory() {
            Err(ValidationError::UnexpectedNoMemories)
        } else {
            let align = 2_i32.pow(memarg.align);

            if align > (n / 8) as i32 {
                Err(ValidationError::InvalidMemoryAlignment)
            } else {
                Ok(())
            }
        }
    }

    pub fn validate_store_n(
        &mut self,
        info: &BytecodeInfo,
        memarg: Memarg,
        n: u32,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(info, memarg, n)?;
        validate_types!(self, [t, ValueType::I32] => []);
        Ok(())
    }

    pub fn validate_store(
        &mut self,
        info: &BytecodeInfo,
        memarg: Memarg,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(
            info,
            memarg,
            t.bit_width()
                .ok_or(ValidationError::InvalidMemoryAlignment)? as u32,
        )?;
        validate_types!(self, [t, ValueType::I32] => []);

        Ok(())
    }
    pub fn validate_load(
        &mut self,
        info: &BytecodeInfo,
        memarg: Memarg,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(
            info,
            memarg,
            t.bit_width()
                .ok_or(ValidationError::InvalidMemoryAlignment)? as u32,
        )?;
        validate_types!(self, [ValueType::I32] => [t]);
        Ok(())
    }
    pub fn validate_load_n(
        &mut self,
        info: &BytecodeInfo,
        memarg: Memarg,
        n: u32,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(info, memarg, n)?;
        validate_types!(self, [ValueType::I32] => [t]);
        Ok(())
    }

    pub fn get_local_type(&self, id: usize) -> Result<ValueType, ValidationError> {
        self.locals
            .get(id as usize)
            .ok_or(ValidationError::InvalidLocalId(id))
            .cloned()
    }

    pub fn validate_local_get(&mut self, id: usize) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.push(local_type);
        Ok(())
    }

    pub fn validate_local_set(&mut self, id: usize) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.pop(local_type)?;
        Ok(())
    }
    pub fn validate_local_tee(&mut self, id: usize) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        validate_types!(self, [local_type] => [local_type]);
        Ok(())
    }

    pub fn validate_global_get(
        &mut self,
        info: &BytecodeInfo,
        id: usize,
    ) -> Result<(), ValidationError> {
        let global_type = info
            .globals
            .get(id)
            .ok_or(ValidationError::InvalidGlobalId(id))?;
        self.push(global_type.t);
        Ok(())
    }
    pub fn validate_global_set(
        &mut self,
        info: &BytecodeInfo,
        id: usize,
    ) -> Result<(), ValidationError> {
        let global_type = info
            .globals
            .get(id)
            .ok_or(ValidationError::InvalidGlobalId(id))?;
        self.pop(global_type.t)?;
        Ok(())
    }
    pub fn validate_select(&mut self, t: Option<ValueType>) -> Result<(), ValidationError> {
        match t {
            Some(v) => {
                validate_types!(self, [v, v, ValueType::I32] => [v]);
                Ok(())
            }
            None => {
                self.pop(ValueType::I32)?;
                let t1 = self.pop_numeric()?;
                let t2 = self.pop_numeric()?;
                if t1 != t2 {
                    Err(ValidationError::PoppedUnexpectedType {
                        got: t2,
                        expected: t1,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn validate_block(
        &mut self,
        bytecode: &Bytecode,
        op: &WithPosition<Op>,
        blocktype: &Blocktype,
    ) -> Result<(), ValidationError> {
        let (in_types, out_types) = match blocktype {
            Blocktype::Empty => (Vec::new(), Vec::new()),
            Blocktype::Value(value_type) => {
                //self.push(value_type);
                (Vec::new(), vec![value_type.clone()])
            }
            Blocktype::TypeIndex(id) => {
                let t = bytecode
                    .get_type(*id as usize)
                    .ok_or(ValidationError::InvalidBlockType(*id))?;
                t.params.data.iter().try_for_each(|p| self.pop(p.data))?;
                (
                    t.iter_params().cloned().collect(),
                    t.iter_results().cloned().collect(),
                )
            }
        };
        self.push_ctrl(Some(op.clone()), in_types, out_types);
        Ok(())
    }

    pub fn validate_else(&mut self, op: WithPosition<Op>) -> Result<(), ValidationError> {
        let ctrl = self.pop_ctrl()?;
        if let Some(Op::If { bt: _, jmp: _ }) = ctrl.op.as_ref().map(|d| d.data) {
            if let Some(jump_id) = ctrl.jump_table_entry {
                self.get_jump_mut(jump_id)?.delta_ip = (self.ip - ctrl.ip) + 1;
            };
            self.push_ctrl(Some(op), ctrl.in_types, ctrl.out_types);
            Ok(())
        } else {
            Err(ValidationError::ElseMissingIf)
        }
    }

    fn get_jump_delta_ip(
        ip: isize,
        op: &Op,
        jump_ip: isize,
        block_ip: isize,
    ) -> Result<isize, ValidationError> {
        match op {
            Op::Loop(_) => Ok(block_ip - jump_ip),
            Op::Block(_) | Op::If { bt: _, jmp: _ } | Op::Else(_) => Ok((ip - jump_ip) + 1),
            _ => Err(ValidationError::InvalidCtrlOp(op.clone())),
        }
    }

    pub fn validate_end(&mut self) -> Result<(), ValidationError> {
        println!("Validate end");
        let ctrl = self.pop_ctrl()?;

        ctrl.out_types.iter().for_each(|t| self.push(t));
        if let Some(ctrl_op) = ctrl.op {
            let jump_idx = self
                .ctrl_jump_stack
                .pop()
                .ok_or(ValidationError::UnexpectedEmptyJumpStack)?;
            let ip = self.ip;

            for idx in jump_idx {
                let jump = self.get_jump_mut(idx)?;
                let jump_ip = jump.ip;
                let delta_ip = Self::get_jump_delta_ip(ip, &ctrl_op.data, jump_ip, ctrl.ip)?;
                jump.delta_ip = delta_ip;
            }

            if let Some(ctrl_jump) = ctrl.jump_table_entry {
                let ip = self.ip;
                let jmp = self.get_jump_mut(ctrl_jump)?;
                jmp.delta_ip = (ip - jmp.ip) + 1;
            }
        }
        Ok(())
    }

    fn push_jmp(&mut self, entry: JumpTableEntry) -> usize {
        self.jump_table.push(entry);
        self.jump_table.len() - 1
    }

    fn push_ctrl_jump(&mut self, label: usize, jmp: usize) -> Result<(), ValidationError> {
        let id = get_ctrl_peek_id(self.ctrl_jump_stack.len(), label);
        let stack_len = self.ctrl_jump_stack.len();
        self.ctrl_jump_stack
            .get_mut(id as usize)
            .ok_or(ValidationError::LabelIndexOutOfScope {
                got: label,
                max: stack_len,
            })?
            .push(jmp);
        Ok(())
    }

    fn push_break_jte(&mut self, n: usize) -> Result<(), ValidationError> {
        let out_count = peek_ctrl(&self.ctrl_stack, n)?.out_types.len();
        let entry = JumpTableEntry {
            ip: self.ip,
            delta_ip: self.ip,
            stack_height: self.type_stack.len(),
            out_count,
        };
        let jmp = self.push_jmp(entry);
        self.push_ctrl_jump(n, jmp)
    }

    pub fn pop_label_types(&mut self, label: usize) -> Result<(), ValidationError> {
        let vals = peek_ctrl(&self.ctrl_stack, label)?
            .iter_label_types()
            .unwrap()
            .collect::<Vec<_>>();

        vals.iter().try_for_each(|t| self.pop(t))?;
        Ok(())
    }
    pub fn push_label_types(&mut self, label: usize) -> Result<(), ValidationError> {
        let vals = peek_ctrl(&self.ctrl_stack, label)?
            .iter_label_types()
            .unwrap();
        self.type_stack.extend(vals.map_into::<ValueStackType>());
        Ok(())
    }

    pub fn validate_br(&mut self, n: usize) -> Result<(), ValidationError> {
        //TODO: ???
        self.pop_label_types(n)?;
        self.push_break_jte(n)?;
        self.set_unreachable()?;
        Ok(())
    }

    pub fn validate_br_if(&mut self, n: usize) -> Result<(), ValidationError> {
        self.pop(ValueType::I32)?;
        self.pop_label_types(n)?;
        self.push_break_jte(n)?;
        self.push_label_types(n)?;
        Ok(())
    }

    pub fn validate_return(&mut self, t: &Type) -> Result<(), ValidationError> {
        println!("func return t: {}", t);
        t.iter_results().try_for_each(|t| self.pop(t))?;
        self.set_unreachable()
    }

    pub fn validate_call(
        &mut self,
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        id: usize,
    ) -> Result<(), ValidationError> {
        let func = info
            .functions
            .get(id)
            .ok_or(ValidationError::InvalidFunctionId(id))?;
        let t = bytecode
            .get_type(func.type_id)
            .ok_or(ValidationError::InvalidFunctionTypeId(func.type_id))?;
        t.iter_params().try_for_each(|t| self.pop(t))?;
        t.iter_results().for_each(|t| self.push(t));
        Ok(())
    }
    pub fn validate_memory_init(
        &mut self,
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        data_id: usize,
    ) -> Result<(), ValidationError> {
        if !info.has_memory() {
            Err(ValidationError::UnexpectedNoMemories)
        } else {
            let is_passive = bytecode
                .data
                .as_ref()
                .ok_or(ValidationError::UnexpectedNoData)?
                .data
                .get(data_id)
                .ok_or(ValidationError::InvalidDataId(data_id))?
                .data
                .is_passive();

            if !is_passive {
                Err(ValidationError::InitActiveDataId(data_id))
            } else {
                validate_types!(self, [ValueType::I32, ValueType::I32, ValueType::I32] => []);
                Ok(())
            }
        }
    }
    pub fn validate_op(
        &mut self,
        bytecode: &Bytecode,
        t: &Type,
        info: &BytecodeInfo,
        op: WithPosition<Op>,
    ) -> Result<(), ValidationError> {
        use ValueType::*;
        println!("Validating op: {}", op.data);
        match op.data {
            Op::Unreachable => self.set_unreachable()?,
            Op::Drop => self.pop_any()?,
            Op::Nop => {}
            Op::Block(blocktype) => self.validate_block(bytecode, &op, &blocktype)?,
            Op::Loop(blocktype) => self.validate_block(bytecode, &op, &blocktype)?,
            Op::If { bt, .. } => {
                self.pop(I32)?;
                self.validate_block(bytecode, &op, &bt)?;
            }
            Op::Else(_) => self.validate_else(op)?,
            Op::End => self.validate_end()?,
            Op::Br { label, .. } => self.validate_br(label)?,
            Op::BrIf { label, .. } => self.validate_br_if(label)?,
            Op::Return => self.validate_return(t)?,
            Op::Call(id) => self.validate_call(bytecode, info, id)?,
            Op::CallIndirect { .. } => todo!(),
            Op::Select(value_type) => self.validate_select(value_type)?,
            Op::LocalGet(id) => self.validate_local_get(id)?,
            Op::LocalSet(id) => self.validate_local_set(id)?,
            Op::LocalTee(id) => self.validate_local_tee(id)?,
            Op::GlobalGet(id) => self.validate_global_get(info, id)?,
            Op::GlobalSet(id) => self.validate_global_set(info, id)?,
            Op::I32Load(memarg) => self.validate_load(info, memarg, I32)?,
            Op::I64Load(memarg) => self.validate_load(info, memarg, I64)?,
            Op::F32Load(memarg) => self.validate_load(info, memarg, F32)?,
            Op::F64Load(memarg) => self.validate_load(info, memarg, F64)?,
            Op::I32Load8s(memarg) => self.validate_load_n(info, memarg, 8, I32)?,
            Op::I32Load8u(memarg) => self.validate_load_n(info, memarg, 8, I32)?,
            Op::I32Load16s(memarg) => self.validate_load_n(info, memarg, 16, I32)?,
            Op::I32Load16u(memarg) => self.validate_load_n(info, memarg, 16, I32)?,
            Op::I64Load8s(memarg) => self.validate_load_n(info, memarg, 8, I64)?,
            Op::I64Load8u(memarg) => self.validate_load_n(info, memarg, 8, I64)?,
            Op::I64Load16s(memarg) => self.validate_load_n(info, memarg, 16, I64)?,
            Op::I64Load16u(memarg) => self.validate_load_n(info, memarg, 16, I64)?,
            Op::I64Load32s(memarg) => self.validate_load_n(info, memarg, 32, I64)?,
            Op::I64Load32u(memarg) => self.validate_load_n(info, memarg, 32, I64)?,
            Op::I32Store(memarg) => self.validate_store(info, memarg, I32)?,
            Op::I64Store(memarg) => self.validate_store(info, memarg, I64)?,
            Op::F32Store(memarg) => self.validate_store(info, memarg, F32)?,
            Op::F64Store(memarg) => self.validate_store(info, memarg, F64)?,
            Op::I32Store8(memarg) => self.validate_store_n(info, memarg, 8, I32)?,
            Op::I32Store16(memarg) => self.validate_store_n(info, memarg, 8, I32)?,
            Op::I64Store8(memarg) => self.validate_store_n(info, memarg, 8, I64)?,
            Op::I64Store16(memarg) => self.validate_store_n(info, memarg, 16, I64)?,
            Op::I64Store32(memarg) => self.validate_store_n(info, memarg, 32, I64)?,
            Op::I32Const(_) => self.push(I32),
            Op::I64Const(_) => self.push(I64),
            Op::F32Const(_) => self.push(F32),
            Op::F64Const(_) => self.push(F64),
            Op::I32Eqz => self.validate_testop(I32)?,
            Op::I32Eq
            | Op::I32Ne
            | Op::I32Lts
            | Op::I32Ltu
            | Op::I32Gts
            | Op::I32Gtu
            | Op::I32Leu
            | Op::I32Les
            | Op::I32Ges
            | Op::I32Geu => self.validate_relop(I32)?,
            Op::I64Eqz => self.validate_testop(I64)?,
            Op::I64Eq
            | Op::I64Ne
            | Op::I64Lts
            | Op::I64Ltu
            | Op::I64Gts
            | Op::I64Gtu
            | Op::I64Les
            | Op::I64Leu
            | Op::I64Ges
            | Op::I64Geu => self.validate_relop(I64)?,
            Op::I32Add
            | Op::I32Sub
            | Op::I32Mul
            | Op::I32Divs
            | Op::I32Divu
            | Op::I32Rems
            | Op::I32Remu
            | Op::I32And
            | Op::I32Or
            | Op::I32Xor
            | Op::I32Shl
            | Op::I32Shrs
            | Op::I32Shru
            | Op::I32Rotl
            | Op::I32Rotr => self.validate_binop(I32)?,
            Op::I64Add
            | Op::I64Sub
            | Op::I64Mul
            | Op::I64Divs
            | Op::I64Divu
            | Op::I64Rems
            | Op::I64Remu
            | Op::I64And
            | Op::I64Or
            | Op::I64Xor
            | Op::I64Shl
            | Op::I64Shrs
            | Op::I64Shru
            | Op::I64Rotl
            | Op::I64Rotr => self.validate_binop(I64)?,
            Op::MemoryCopy => todo!(),
            Op::MemoryFill => todo!(),
            Op::MemoryInit { data_id, .. } => self.validate_memory_init(bytecode, info, data_id)?,
        };
        self.ip += 1;
        println!("Stack now: {:?}", self.type_stack);
        Ok(())
    }
    pub fn set_locals_from_func_t(&mut self, t: &Type, code: &Function) {
        self.locals = t
            .iter_params()
            .cloned()
            .chain(code.iter_locals())
            .collect::<Vec<_>>();
    }

    pub fn validate_code(
        mut self,
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        t: &Type,
        code: &Function,
    ) -> Result<Vec<JumpTableEntry>, ValidationError> {
        self.set_locals_from_func_t(t, code);

        let results = t.iter_results().cloned().collect::<Vec<_>>();

        println!("=====Validating func with t: {}=====", t);
        println!("out_count: {}", results.len());
        self.push_ctrl(None, Vec::new(), results);
        code.iter_ops()
            .try_for_each(|op| self.validate_op(bytecode, t, info, op))?;
        Ok(self.jump_table)
    }

    pub fn validate_func(
        bytecode: &Bytecode,
        info: &BytecodeInfo,
        func: &parser::info::Function,
    ) -> Result<Vec<JumpTableEntry>, ValidationError> {
        match func.t {
            FunctionType::Internal { code_id, .. } => {
                let code = bytecode.get_code(code_id).unwrap();

                let validator = ValidatorContext {
                    func_id: code_id,
                    ..Default::default()
                };
                let t = bytecode.get_type(func.type_id).unwrap();
                println!("validating function type: {}", t);
                validator.validate_code(bytecode, info, t, code)
            }
            FunctionType::Imported { .. } => Ok(Vec::new()),
        }
    }
    pub fn validate_all(
        bytecode: &Bytecode,
        info: &BytecodeInfo,
    ) -> Result<Vec<Vec<JumpTableEntry>>, ValidationError> {
        if let Some(ft) = bytecode.iter_function_types() {
            ft.zip(bytecode.iter_code().unwrap())
                .enumerate()
                .map(|(id, (t, code))| {
                    let validator = ValidatorContext {
                        func_id: id,
                        ..Default::default()
                    };
                    validator.validate_code(bytecode, info, t, code)
                })
                .collect::<Result<Vec<_>, ValidationError>>()
        } else {
            Ok(Vec::new())
        }
    }
}

fn patch_op_jump(op: &Op, jump: &JumpTableEntry, jump_id: usize) -> Result<Op, ValidationError> {
    match op {
        Op::Else(_) => Ok(Op::Else(jump.delta_ip)),
        Op::If { bt, jmp: _ } => Ok(Op::If {
            bt: *bt,
            jmp: jump.delta_ip,
        }),
        Op::Br { label, jmp: _ } => Ok(Op::Br {
            label: *label,
            jmp: jump.delta_ip,
        }),
        Op::BrIf { label, jmp: _ } => Ok(Op::BrIf {
            label: *label,
            jmp: jump.delta_ip,
        }),
        _ => return Err(ValidationError::InvalidJump(jump_id)),
    }
}

pub fn patch_function_jumps<'a>(
    function: &mut Function,
    jumps: impl IntoIterator<Item = &'a JumpTableEntry>,
) -> Result<(), ValidationError> {
    jumps
        .into_iter()
        .enumerate()
        .try_for_each(|(jump_id, jump)| {
            let op = function.get_op_mut(jump.ip as usize).unwrap();
            *op = patch_op_jump(op, jump, jump_id)?;
            Ok(())
        })
}

pub fn valiadate_and_patch_bytecode(
    bytecode: &mut Bytecode,
) -> Result<(Vec<Vec<JumpTableEntry>>, BytecodeInfo), ValidationError> {
    let info = BytecodeInfo::new(bytecode);
    let jumps = ValidatorContext::validate_all(bytecode, &info)?;
    if let Some(code) = bytecode.iter_code_mut() {
        code.zip(jumps.iter())
            .try_for_each(|(f, j)| patch_function_jumps(f, j))?;
    };
    Ok((jumps, info))
}

#[derive(Debug)]
pub struct ValidateResult {
    pub bytecode: Bytecode,
    pub info: BytecodeInfo,
    pub jumps: Vec<Vec<JumpTableEntry>>,
    //TODO: Jumps?
}
#[derive(Error, Debug)]
pub enum ReadAndValidateError {
    #[error("Unable to parse bytecode: {0}")]
    ReadError(#[from] ParserError),

    #[error("Unable to validate bytecode: {0}")]
    ValidationError(#[from] ValidationError),
}

pub fn read_and_validate(
    reader: &mut impl BytecodeReader,
) -> Result<ValidateResult, ReadAndValidateError> {
    let mut bytecode = parse_binary(reader)?;
    let (jumps, info) = valiadate_and_patch_bytecode(&mut bytecode)?;

    Ok(ValidateResult {
        jumps,
        bytecode,
        info,
    })
}

pub fn read_and_validate_wat(
    source: impl AsRef<str>,
) -> Result<ValidateResult, ReadAndValidateError> {
    let mut bytecode = parse_wat(source)?;
    let (jumps, info) = valiadate_and_patch_bytecode(&mut bytecode)?;

    Ok(ValidateResult {
        jumps,
        bytecode,
        info,
    })
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use parser::op::Op;
    use validator_derive::{test_invalid_wast, test_valid_wast};

    use super::{ReadAndValidateError, ValidationError, read_and_validate_wat};
    macro_rules! expect_src_ok {
        ($src: ident) => {
            return Ok(_ = read_and_validate_wat($src)?);
        };
    }

    macro_rules! assert_validation_err {
        ($src: ident, $err: pat) => {
            let err = read_and_validate_wat($src);
            assert!(matches!(
                err.unwrap_err(),
                ReadAndValidateError::ValidationError($err)
            ));
        };
    }
    #[test]
    fn validate_empty_module() -> Result<(), ReadAndValidateError> {
        let src = "(module)";
        _ = read_and_validate_wat(src)?;
        Ok(())
    }

    #[test]
    fn validate_simple_add() -> Result<(), ReadAndValidateError> {
        let src = r#"
             (module
                 (func (param $p i32) (result i32)
                     local.get $p
                     i32.const 5
                     i32.add
                 )
             )
         "#;
        Ok(_ = read_and_validate_wat(src)?)
    }

    #[test]
    fn validate_simple_add_unbalanced() -> Result<(), ReadAndValidateError> {
        let src = r#"
             (module
                 (func (param $p i32) (result i32)
                     i32.const 1
                     local.get $p
                     i32.const 5
                     i32.add
                 )
             )
         "#;
        assert_validation_err!(src, ValidationError::UnbalancedStack { .. });
        Ok(())
    }
    #[test]
    fn invalid_local_id() -> Result<(), ReadAndValidateError> {
        let src = r#"
             (module
                 (func (param $p i32) (result i32)
                     local.get 1
                     i32.const 5
                     i32.add
                 )
             )
        "#;
        assert_validation_err!(src, ValidationError::InvalidLocalId(1));
        Ok(())
    }
    #[test]
    fn valid_local_id_tee() -> Result<(), ReadAndValidateError> {
        let src = r#"
            (module
                (func (param $p i32) (result i32) (local i32)
                    local.get 0
                    i32.const 5
                    i32.add
                    local.tee 1
                )
            )
        "#;
        Ok(_ = read_and_validate_wat(src)?)
    }
    #[test]
    fn valid_multiple_functions() -> Result<(), ReadAndValidateError> {
        let src = r#"
            (module
                (func (param $p i32) (result i32) (local i32)
                    local.get 0
                    i32.const 5
                    i32.add
                    local.tee 1
                )
                (func (param i32) (result i32)
                    i32.const 5
                    call 0
                    i32.const 10
                    i32.add
                )
                (func (param i32) (result i32)
                    i32.const 5
                    call 2
                    i32.const 10
                    i32.add
                )
            )
        "#;
        Ok(_ = read_and_validate_wat(src)?)
    }

    test_invalid_wast! {
        invalid_multiple_functions,
            r#"
            (module
                (func (param $p i32) (result i32) (local i32)
                    local.get 0
                    i32.const 5
                    i32.add
                    local.tee 1
                )
                (func (param i32) (result i32)
                    i32.const 5
                    call 0
                    i32.const 10
                    i32.add
                )
                (func (param i32) (result i32)
                    i32.const 5
                    call 3
                    i32.const 10
                    i32.add
                )
            )
        "#,
        ValidationError::InvalidFunctionId(3)
    }

    test_valid_wast! {
        valid_param_count,
        r#"
                (module
                    (func (param i32) (param i32) (param i32) (result i32) (local i32)
                        local.get 0
                        i32.const 5
                        i32.add
                        local.tee 1
                    )
                    (func (param i32) (result i32)
                        i32.const 1
                        i32.const 2
                        i32.const 3
                        call 0
                    )
                )
            "#
    }
    test_valid_wast! {
        call_imported_function,
             r#"
             (module
                 (import "console" "log" (func $log (param i32)))
                 (func (param i32 i32)
                     i32.const 1
                     call 0
                 )

                 (func (param i32 i32) (result i32)
                     i32.const 1
                     call 0
                     i32.const 2
                 )
             )
         "#
    }
    test_invalid_wast! {
        memory_instructions_without_memory,
            r#"
             (module
                 (func (param $p i32) (result i32)
                     i32.const 1
                     i32.load
                 )
             )
         "#,
         ValidationError::UnexpectedNoMemories
    }

    test_valid_wast! {
        basic_memory,
            r#"
             (module
                 (memory 1)
                 (func (param $p i32) (result i32)
                     i32.const 1
                     i32.load
                 )
             )
         "#
    }

    #[test]
    pub fn jump_block() -> Result<(), ReadAndValidateError> {
        let src = r#"
             (module
                 (import "console" "log" (func $log (param i32)))
                 (func (param i32)
                     (block $block
                         i32.const 1
                         i32.const 2
                         i32.add
                         i32.const 10
                         i32.lt_s
                         br_if $block
                         i32.const 5
                         i32.const 10
                         i32.add
                         drop
                     )
                     i32.const 100
                     call $log
                 )
             )
         "#;
        let code = read_and_validate_wat(src)?;
        let func = code.bytecode.get_code(0).unwrap();
        let after_block = func.get_op_after(6).unwrap();
        assert!(matches!(after_block.0, Op::End));
        Ok(())
    }

    #[test]
    pub fn jump_table_if_else() -> Result<(), ReadAndValidateError> {
        let src = r#"
             (module
                 (import "console" "log" (func $log (param i32)))
                 (func (param i32)
                     i32.const 0
                     (if
                         (then
                             i32.const 1
                             call $log
                         )
                         (else
                             i32.const 100
                             call $log
                         )
                     )
                 )
             )
        "#;
        let code = read_and_validate_wat(src)?;
        let func = code.bytecode.get_code(0).unwrap();
        let (after_block_op, after_block_ip) = func.get_op_after(1).unwrap();
        assert!(matches!(after_block_op, Op::Else(_)));
        assert!(matches!(
            func.get_op((after_block_ip + 1) as usize).unwrap(),
            Op::I32Const(100)
        ));

        let after_else = func.get_op_after(after_block_ip).unwrap();
        assert!(matches!(after_else.0, Op::End));
        Ok(())
    }

    #[test]
    pub fn if_no_else() -> Result<(), ReadAndValidateError> {
        let src = r#"
            (module
                (import "console" "log" (func $log (param i32)))
                (func (param i32)
                    i32.const 0
                    (if
                        (then
                            i32.const 1
                            call $log
                        )
                    )
                    i32.const 100
                    call $log
                )
            )
        "#;
        let code = read_and_validate_wat(src)?;
        let func = code.bytecode.get_code(0).unwrap();
        let after_if_op = func.get_op_after(1).unwrap();
        let after_end_op = func.get_op(after_if_op.1 as usize + 1).unwrap();
        assert!(matches!(after_if_op.0, Op::End));
        assert!(matches!(after_end_op, Op::I32Const(100)));
        Ok(())
    }

    #[test]
    pub fn loop_jumps() -> Result<(), ReadAndValidateError> {
        let src = r#"
            (module
                 (import "console" "log" (func $log (param i32)))
                 (func (param i32)
                     (loop $loop
                         i32.const 50
                         br_if $loop
                         i32.const 5
                         i32.const 6
                         i32.add
                         br_if $loop
                     )
                 )
             )
         "#;
        let code = read_and_validate_wat(src)?;
        let func = code.bytecode.get_code(0).unwrap();
        let jmp1 = func.get_op_after_offset(2, 0).unwrap();
        let jmp2 = func.get_op_after_offset(6, 0).unwrap();

        assert!(matches!(jmp1.0, Op::Loop(_)));
        assert!(matches!(jmp2.0, Op::Loop(_)));
        Ok(())
    }
}
