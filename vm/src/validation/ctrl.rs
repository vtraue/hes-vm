use std::ops::Range;

use crate::parser::{op::Op, types::ValueType};

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
    pub fn new(
        context: &Validator,
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
}

#[derive(Default, Debug, Clone)]
pub struct JumpTable(pub Vec<JumpTableEntry>);

impl JumpTable {
    pub fn push_new(&mut self, ip: usize) -> usize {
        self.0.push(JumpTableEntry {
            ip: ip as isize,
            delta_ip: ip as isize,
        });
        self.0.len() - 1
    }

    pub fn get_jump_mut(&mut self, id: usize) -> Result<&mut JumpTableEntry, ValidationError> {
        self.0.get_mut(id).ok_or(ValidationError::InvalidJumpId)
    }
}
