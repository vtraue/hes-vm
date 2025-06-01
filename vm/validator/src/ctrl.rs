use std::ops::Range;

use parser::{op::Op, types::{Function, ValueType}};

use super::{error::ValidationError, validator::Validator};

#[derive(Clone, Debug)]
pub struct CtrlFrame {
    pub opcode: Option<(Op, Range<usize>)>,
    pub in_types: Vec<ValueType>,
    pub out_types: Vec<ValueType>,
    pub start_height: usize,
    pub is_unreachable: bool,
    pub jump_table_entry: Option<usize>,
    pub ip: usize,
}

impl CtrlFrame {
    /*
    pub fn new(
        height: usize,
        ip: usize,
        opcode: Option<(Op, Range<usize>)>,
        jump_table_entry: Option<usize>,
        in_types: Vec<ValueType>,
        out_types: Vec<ValueType>,
    ) -> Self {
        let start_height = context.value_stack.len();
        CtrlFrame {
            opcode,
            ip,
            jump_table_entry,
            in_types,
            out_types,
            start_height,
            is_unreachable: false,
        }
    }
    */
    pub fn label_types<'me>(&'me self) -> &'me [ValueType] {
        if let Some((Op::Loop(_), _)) = self.opcode {
            self.in_types.as_slice()
        } else {
            self.out_types.as_slice()
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

#[derive(Default, Debug, Clone)]
pub struct JumpTable(pub Vec<JumpTableEntry>);

impl JumpTable {
    pub fn iter(&self) -> impl Iterator<Item = &JumpTableEntry> {
        self.0.iter()
    }

    pub fn push(&mut self, entry: JumpTableEntry) -> usize {
        self.0.push(entry);
        self.0.len() - 1
    }

    pub fn patch(&self, function: &mut Function) -> Result<(), ValidationError> {
        for (i, jmp) in self.iter().enumerate() {
            let op = function.get_instruction(jmp.ip as usize).cloned().ok_or(ValidationError::InvalidJump)?; 

            let new_op = match op {
                Op::Else(_) => Op::Else(jmp.delta_ip),
                Op::If(bt, _) => Op::If(bt, i),
                Op::Br(bt, _) => Op::Br(bt, i),
                Op::BrIf(bt, _) => Op::BrIf(bt, i),
                _ => return Err(ValidationError::InvalidJump),
            };

            function.set_instruction(jmp.ip as usize, new_op);
        }
        Ok(())
    }

    pub fn push_new(
        &mut self,
        ip: usize,
        stack_height: usize,
        in_count: usize,
        out_count: usize,
    ) -> usize {
        self.0.push(JumpTableEntry {
            ip: ip as isize,
            delta_ip: ip as isize,
            stack_height,
            out_count,
        });
        self.0.len() - 1
    }

    pub fn get_jump(&self, id: usize) -> Result<&JumpTableEntry, ValidationError> {
        self.0.get(id).ok_or(ValidationError::InvalidJumpId)
    }

    pub fn get_jump_mut(&mut self, id: usize) -> Result<&mut JumpTableEntry, ValidationError> {
        self.0.get_mut(id).ok_or(ValidationError::InvalidJumpId)
    }
}
