use std::fmt::Display;

use itertools::Itertools;
use parser::{info::BytecodeInfo, op::{Blocktype, Memarg, Op}, reader::{Bytecode, Type, ValueType, WithPosition}};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueStackType {
    T(ValueType),
    Unknown,
}


#[derive(Debug)]
pub enum ValidationError {
    UnexpetedEmptyControlStack, 
    TypeStackUnderflow, 
    PoppedUnexpectedType {got: ValueStackType, expected: ValueStackType},
    UnbalancedStack {got: usize, expected: usize}, 
    LabelIndexOutOfScope {got: usize, max: usize},
    UnexpectedNoMemories,
    InvalidMemoryAlignment,
    InvalidLocalId(u32),
    InvalidGlobalId(u32),
    ExpectedNumericType,
    InvalidBlockType(u32),
    InvalidJumpTableDestination(usize),
    ElseMissingIf,
    UnexpectedEmptyJumpStack,
    InvalidCtrlOp(Op),
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
    stack.as_ref().last().ok_or(ValidationError::UnexpetedEmptyControlStack)
}

pub fn pop_type(stack: &mut Vec<ValueStackType>, frame_stack_len: usize, unreachable: bool) -> Result<ValueStackType, ValidationError> {
    if frame_stack_len == stack.len() {
        if unreachable {
            Ok(ValueStackType::Unknown)
        } else {
            Err(ValidationError::TypeStackUnderflow)
        }
    } else {
        stack.pop().ok_or(ValidationError::TypeStackUnderflow) 
    }
}

pub fn pop_type_expect(stack: &mut Vec<ValueStackType>, frame_stack_len: usize, unreachable: bool, expected: impl Into<ValueStackType>) -> Result<ValueStackType, ValidationError> {
    pop_type(stack, frame_stack_len, unreachable).map(|got| {
        let expected = expected.into();
        if got == expected || got == ValueStackType::Unknown || expected == ValueStackType::Unknown {
            Ok(got)
        } else {
            Err(ValidationError::PoppedUnexpectedType { got, expected,})
        }
    })? 
}

pub fn pop_values(stack: &mut Vec<ValueStackType>, frame_stack_len: usize, unreachable: bool, expected: impl Iterator<Item = impl Into<ValueStackType>>) -> Result<(), ValidationError> {
    for val in expected {
        pop_type_expect(stack, frame_stack_len, unreachable, val)?;
    };
    Ok(())
}
pub fn push_type(stack: &mut Vec<ValueStackType>, val: impl Into<ValueStackType>) {
    stack.push(val.into()) 
}

pub fn push_all(stack: &mut Vec<ValueStackType>, values: impl IntoIterator<Item = impl Into<ValueStackType>>) {
    stack.extend(values.into_iter().map_into()); 
}

fn pop_out_values(stack: &mut Vec<ValueStackType>, frame: &CtrlFrame) -> Result<(), ValidationError> {
    pop_values(stack, frame.prev_stack_len, frame.is_unreachable, frame.out_types.iter()) 
}

fn get_ctrl_peek_id(stack_size: usize, label: usize) -> isize {
    (stack_size as isize - 1) - label as isize
}

fn peek_ctrl(stack: &impl AsRef<[CtrlFrame]>, label: usize) -> Result<&CtrlFrame, ValidationError> {
    let stack = stack.as_ref();
    let id = get_ctrl_peek_id(stack.len(), label); 

    stack.get(id as usize).ok_or(ValidationError::LabelIndexOutOfScope { got: id as usize, max: stack.len()})   
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
        self.jump_table.get(id).ok_or(ValidationError::InvalidJumpTableDestination(id))
    }
    pub fn get_jump_mut(&mut self, id: usize) -> Result<&mut JumpTableEntry, ValidationError> {
        self.jump_table.get_mut(id).ok_or(ValidationError::InvalidJumpTableDestination(id))
    }

    fn current_ctrl(&self) -> Result<&CtrlFrame, ValidationError> {
        last_ctrl(&self.ctrl_stack)        
    }

    fn push_branch_op_jte(&mut self, op: Op, out_type_count: usize) -> Option<usize>{
        if op.is_branch() {
            let entry = JumpTableEntry::new(self.ip, self.type_stack.len(), out_type_count);
            self.jump_table.push(entry);
            Some(self.jump_table.len() - 1)
        } else {
            None
        }
    }

    pub fn pop(&mut self, expected: impl Into<ValueStackType>) -> Result<ValueStackType, ValidationError> {
        let len = self.current_ctrl()?.prev_stack_len;
        let unreachable = self.current_ctrl()?.is_unreachable;
        pop_type_expect(&mut self.type_stack, len, unreachable, expected)
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

    pub fn push(&mut self, t: impl Into<ValueStackType>) {
        self.type_stack.push(t.into());
    }

    pub fn push_ctrl(&mut self, op: Option<WithPosition<Op>>, in_types: Vec<ValueType>, out_types: Vec<ValueType>) {
        let jump_table_entry= if let Some(WithPosition {data: op, position: _}) = op {
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
                expected: start_height
            })
        } else {
            Ok(self.ctrl_stack.pop().unwrap()) 
        }
    }

    pub fn get_current_frame_mut(&mut self) -> Result<&mut CtrlFrame, ValidationError> {
        self.ctrl_stack.get_mut(0).ok_or(ValidationError::UnexpetedEmptyControlStack)
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

    pub fn check_memarg(&self, info: &BytecodeInfo, memarg: Memarg, n: u32) -> Result<(), ValidationError> {
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

    pub fn validate_store_n(&mut self, info: &BytecodeInfo, memarg: Memarg, n: u32, t: ValueType) -> Result<(), ValidationError>{
        self.check_memarg(info, memarg, n)?;
        validate_types!(self, [t, ValueType::I32] => []); 
        Ok(())
    }

    pub fn validate_store(&mut self, info: &BytecodeInfo, memarg: Memarg, t: ValueType) -> Result<(), ValidationError> {
        self.check_memarg(info, memarg, t.bit_width().ok_or(ValidationError::InvalidMemoryAlignment)? as u32)?;
        validate_types!(self, [t, ValueType::I32] => []);

        Ok(())
    }
    pub fn validate_load(&mut self, info: &BytecodeInfo, memarg: Memarg, t: ValueType) -> Result<(), ValidationError> {
        self.check_memarg(info, memarg, t.bit_width().ok_or(ValidationError::InvalidMemoryAlignment)? as u32)?;
        validate_types!(self, [ValueType::I32] => [t]);
        Ok(())
    }
    pub fn validate_load_n(&mut self, info: &BytecodeInfo, memarg: Memarg, n: u32, t: ValueType) -> Result<(), ValidationError> {
        self.check_memarg(info, memarg, n)?;
        validate_types!(self, [ValueType::I32] => [t]);
        Ok(())
    }

    pub fn get_local_type(&self, id: u32) -> Result<ValueType, ValidationError> {
        self.locals.get(id as usize).ok_or(ValidationError::InvalidLocalId(id)).cloned()
    }
    
    pub fn validate_local_get(&mut self, id: u32) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.push(local_type);
        Ok(())
    }

    pub fn validate_local_set(&mut self, id: u32) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.pop(local_type)?;
        Ok(())
    }
    pub fn validate_local_tee(&mut self, id: u32) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        validate_types!(self, [local_type] => [local_type]);
        Ok(())
    }

    pub fn validate_global_get(&mut self, info: &BytecodeInfo, id: u32) -> Result<(), ValidationError> {
        let global_type = info.globals.get(id as usize).ok_or(ValidationError::InvalidGlobalId(id))?;
        self.push(global_type.t); 
        Ok(())
    }
    pub fn validate_global_set(&mut self, info: &BytecodeInfo, id: u32) -> Result<(), ValidationError> {
        let global_type = info.globals.get(id as usize).ok_or(ValidationError::InvalidGlobalId(id))?;
        self.pop(global_type.t)?; 
        Ok(())
    }
    pub fn validate_select(&mut self, t: Option<ValueType>) -> Result<(), ValidationError> {
        match t {
            Some(v) => {validate_types!(self, [v, v, ValueType::I32] => [v]); Ok(())}
            None => {
                self.pop(ValueType::I32)?; 
                let t1 = self.pop_numeric()?;
                let t2 = self.pop_numeric()?;
                if t1 != t2 {
                    Err(ValidationError::PoppedUnexpectedType { got: t2, expected: t1 })
                } else {
                    Ok(())
                }
            },
        }
    }

    fn new_jump_at(&self, out_count: usize) -> JumpTableEntry {
        JumpTableEntry::new(self.ip, self.type_stack.len(), out_count)
    }

    pub fn validate_block(&mut self, bytecode: &Bytecode, op: &WithPosition<Op>, blocktype: &Blocktype) -> Result<(), ValidationError> {
        let (in_types, out_types ) = match blocktype {
            Blocktype::Empty => {(Vec::new(), Vec::new())},
            Blocktype::Value(value_type) => {
                self.push(value_type);
                (Vec::new(), vec![value_type.clone()])
            },
            Blocktype::TypeIndex(id) => {
                let t = bytecode.get_type(*id as usize).ok_or(ValidationError::InvalidBlockType(*id))?;
                t.params.data.iter().try_for_each(|p| Ok(_ = self.pop(p.data)?))?; 
                (t.iter_params().cloned().collect(), t.iter_results().cloned().collect())  
            },
        }; 
        self.push_ctrl(Some(op.clone()), in_types, out_types);
        Ok(())
    }

    pub fn validate_else(&mut self, op: WithPosition<Op>) -> Result<(), ValidationError> {
        let ctrl = self.pop_ctrl()?;
        if let Some(Op::If { bt: _, jmp: _ }) = ctrl.op.as_ref().map(|d| d.data) {
            if let Some(jump_id) = ctrl.jump_table_entry {
                self.get_jump_mut(jump_id)?.delta_ip = self.ip - ctrl.ip + 1;
            };
            self.push_ctrl(Some(op), ctrl.in_types, ctrl.out_types);
            Ok(())
        } else {
            Err(ValidationError::ElseMissingIf)
        }
    }
    fn get_jump_delta_ip(ip: isize, op: &Op, jump_ip: isize, block_ip: isize) -> Result<isize, ValidationError> {
        match op {
            Op::Loop(_) => Ok(block_ip - jump_ip),
            Op::Block(_) | Op::If { bt: _, jmp: _ } | Op::Else(_) => Ok(ip - jump_ip),
            _ => Err(ValidationError::InvalidCtrlOp(op.clone())),
        }
    }

    pub fn validate_end(&mut self) -> Result<(), ValidationError> {
        let ctrl =self.pop_ctrl()?;
        ctrl.out_types.iter().for_each(|t| self.push(t));
        if let Some(ctrl_op) = ctrl.op {
            let jump_idx = self.ctrl_jump_stack.pop().ok_or(ValidationError::UnexpectedEmptyJumpStack)?;
            let ip = self.ip;

            for idx in jump_idx {
                let jump = self.get_jump_mut(idx)?;
                let jump_ip = jump.delta_ip;
                let delta_ip= Self::get_jump_delta_ip(ip, &ctrl_op.data, jump_ip, ctrl.ip)?;

                jump.delta_ip = delta_ip; 
            };
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
            .ok_or(ValidationError::LabelIndexOutOfScope { got: label, max: stack_len })?
            .push(jmp);
        Ok(())
    }

    fn push_break_jte(&mut self, n: usize) -> Result<(), ValidationError> {
        let out_count = peek_ctrl(&self.ctrl_stack, n)?.out_types.len(); 
        let entry = JumpTableEntry {
            ip: self.ip,
            delta_ip: self.ip,
            stack_height: self.type_stack.len(),
            out_count
        };
        let jmp = self.push_jmp(entry);
        self.push_ctrl_jump(n, jmp)
    }
    pub fn pop_label_types(&mut self, label: usize) -> Result<(), ValidationError> {
        let vals = peek_ctrl(&self.ctrl_stack, label)?.iter_label_types().unwrap().collect::<Vec<_>>();

        vals.iter().try_for_each(|t| {
            Ok(_ = self.pop(t)?)
        })?; 
        Ok(())
    }
    pub fn push_label_types(&mut self, label: usize) -> Result<(), ValidationError> {
        let vals = peek_ctrl(&self.ctrl_stack, label)?.iter_label_types().unwrap();
        self.type_stack.extend(vals.map_into::<ValueStackType>());
        Ok(())
    }

    pub fn validate_br(&mut self, n: usize) -> Result<(), ValidationError> {
        //TODO: ???
        self.pop_label_types(n);
        self.push_break_jte(n)?;
        self.push_label_types(n)?;
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
}

