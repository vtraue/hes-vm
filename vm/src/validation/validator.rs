use core::fmt;
use std::{iter, ops::Range};

use itertools::Itertools;

use crate::{
    interpreter::slow_vm::LocalValue,
    parser::{
        module::{
            self, DecodedBytecode, FunctionInfo, ImportedFunction, InternalFunction, SortedImports,
        },
        op::{self, Op},
        types::{Expression, Function, GlobalType, Limits, Type, ValueType},
    },
};

use super::{
    ctrl::{CtrlFrame, JumpTable, JumpTableEntry},
    error::ValidationError,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValueStackType {
    T(ValueType),
    Unknown,
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

impl Function {
    pub fn patch_jumps(&mut self, jump_table: &JumpTable) -> Result<(), ValidationError> {
        for (i, jmp) in jump_table.0.iter().enumerate() {
            let op = self.code.0[jmp.ip as usize].0.clone();
            //...
            println!("jump op: {op}");
            let new_op = match op {
                Op::Else(_) => Op::Else(jmp.delta_ip),
                Op::If(bt, _) => Op::If(bt, i),
                Op::Br(bt, _) => Op::Br(bt, i),
                Op::BrIf(bt, _) => Op::BrIf(bt, i),
                _ => return Err(ValidationError::InvalidJump),
            };

            self.code.0[jmp.ip as usize].0 = new_op;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Context<'src> {
    pub bytecode: &'src DecodedBytecode,
    pub imports: SortedImports<'src>,
    pub function_types: Vec<&'src Type>,
    pub memories: Vec<&'src Limits>,
    pub globals: Vec<&'src GlobalType>,
}

impl<'src> Context<'src> {
    pub fn new(bytecode: &'src DecodedBytecode) -> Result<Context<'src>, ValidationError> {
        let imports = bytecode.sort_imports()?;

        let mut function_types = imports.functions.clone();
        if let Some(funcs) = &bytecode.functions {
            for type_id in &funcs.0 {
                let (t, _) = bytecode.get_type(type_id.0 as usize)?;
                function_types.push(t);
            }
        }

        let mut memories = imports.memories.clone();
        if let Some(mems) = bytecode.memories.as_ref() {
            memories.extend(mems.0.iter().map(|(mem, _)| mem));
        }

        let mut globals = imports.globals.clone();
        if let Some(internal_globals) = bytecode.globals.as_ref() {
            globals.extend(internal_globals.0.iter().map(|(g, _)| &g.t.0));
        }

        //TODO: Tables
        let context = Context {
            bytecode,
            imports,
            function_types,
            memories,
            globals,
        };
        Ok(context)
    }

    pub fn get_func_type(&self, id: usize) -> Result<&'src Type, ValidationError> {
        self.function_types
            .get(id)
            .ok_or(ValidationError::InvalidFuncId(id))
            .copied()
    }
    pub fn get_internal_func_type(&self, id: usize) -> Result<&'src Type, ValidationError> {
        self.get_func_type(self.imports.functions.len() + id)
    }
    pub fn get_memory(&self, id: usize) -> Result<&'src Limits, ValidationError> {
        self.memories
            .get(id)
            .ok_or(ValidationError::InvalidMemId(id))
            .copied()
    }

    pub fn get_global(&self, id: usize) -> Result<&'src GlobalType, ValidationError> {
        self.globals
            .get(id)
            .ok_or(ValidationError::InvalidGlobalID(id))
            .copied()
    }

    pub fn get_function_count(&self) -> usize {
        self.bytecode
            .functions
            .as_ref()
            .map_or(0, |(funcs, _)| funcs.len())
    }
}

#[derive(Debug, Default)]
pub struct Validator {
    ctrl_stack: Vec<CtrlFrame>,
    ctrl_jump_stack: Vec<Vec<usize>>,
    pub value_stack: Vec<ValueStackType>,
    locals: Vec<ValueType>,
    jump_table: JumpTable,
    current_func_id: usize,
    instruction_pointer: usize,
}

impl<'src> Validator {
    pub fn pop_val(&mut self) -> Result<ValueStackType, ValidationError> {
        let current_ctrl = &self
            .ctrl_stack
            .last()
            .ok_or(ValidationError::UnexpectedEmptyControlStack)?;
        if current_ctrl.start_height == self.value_stack.len() {
            if current_ctrl.is_unreachable {
                Ok(ValueStackType::Unknown)
            } else {
                Err(ValidationError::ValueStackUnderflow)
            }
        } else {
            let val = self
                .value_stack
                .pop()
                .ok_or(ValidationError::ValueStackUnderflow)?;
            println!("Popping {val}, stack: {}", self.value_stack.len());
            Ok(val)
        }
    }
    pub fn push_val_t(&mut self, val: ValueType) {
        println!("Pushing {val}");
        self.value_stack.push(val.into());
        println!("Stack {}", self.value_stack.len());
    }
    pub fn pop_val_expect(
        &mut self,
        expected: ValueStackType,
    ) -> Result<ValueStackType, ValidationError> {
        self.pop_val().map(|v| {
            if v == expected {
                Ok(v)
            } else {
                Err(ValidationError::UnexpectedValueType { got: v, expected })
            }
        })?
    }

    pub fn pop_val_expect_val(
        &mut self,
        expected: ValueType,
    ) -> Result<ValueStackType, ValidationError> {
        self.pop_val_expect(expected.into())
    }

    pub fn push_new_ctrl(
        &mut self,
        opcode: Option<(Op, Range<usize>)>,
        in_types: Vec<ValueType>,
        out_types: Vec<ValueType>,
    ) {
        //TODO: (joh): Das ist nicht sehr elegant
        let prev_stack_len = self.value_stack.len();
        in_types.iter().cloned().for_each(|f| self.push_val_t(f));

        /*
        self.value_stack
            .extend(in_types.iter().cloned().map_into::<ValueStackType>());
        */
        let jte = if matches!(opcode, Some((Op::If(_, _), _)))
            || matches!(opcode, Some((Op::Else(_), _)))
        {
            let entry = JumpTableEntry {
                ip: self.instruction_pointer as isize,
                delta_ip: self.instruction_pointer as isize,
                stack_height: prev_stack_len,
                out_count: out_types.len(),
            };
            Some(self.jump_table.push(entry))
        } else {
            None
        };
        let ctrl = CtrlFrame {
            opcode,
            ip: self.instruction_pointer,
            jump_table_entry: jte,
            in_types,
            out_types,
            start_height: prev_stack_len,
            is_unreachable: false,
        };

        self.ctrl_jump_stack.push(Vec::new());
        self.ctrl_stack.push(ctrl);
    }

    pub fn pop_ctrl(&mut self) -> Result<CtrlFrame, ValidationError> {
        let out_types = self
            .ctrl_stack
            .last()
            .ok_or(ValidationError::UnexpectedEmptyControlStack)?
            .out_types
            .clone();
        let start_height = self.ctrl_stack.last().unwrap().start_height;
        println!("pop ctrl count: {}", out_types.len());
        out_types
            .iter()
            .cloned()
            .map_into::<ValueStackType>()
            .try_for_each(|t| {
                let val = self.pop_val()?;
                if val != t && val != ValueStackType::Unknown {
                    Err(ValidationError::ReturnTypesDoNotMatch {
                        got: val,
                        expexted: t,
                    })
                } else {
                    Ok(())
                }
            })?;

        if self.value_stack.len() != start_height {
            return Err(ValidationError::UnbalancedStack {
                got: self.value_stack.len(),
                expected: start_height,
            });
        }

        let frame = self.ctrl_stack.pop().unwrap();
        Ok(frame)
    }

    pub fn peek_ctrl_at_label(&self, label: u32) -> Result<&CtrlFrame, ValidationError> {
        let id = (self.ctrl_stack.len() as isize - 1) - (label as isize);
        if id < 0 {
            Err(ValidationError::LabelIndexOutOfScope(label))
        } else {
            Ok(&self.ctrl_stack[id as usize])
        }
    }

    pub fn push_ctrl_jump(&mut self, label: u32, jump: usize) -> Result<(), ValidationError> {
        let id = (self.ctrl_jump_stack.len() as isize - 1) - (label as isize);
        if id < 0 {
            Err(ValidationError::LabelIndexOutOfScope(label))
        } else {
            self.ctrl_jump_stack[id as usize].push(jump);
            Ok(())
        }
    }

    pub fn set_unreachable(&mut self) -> Result<(), ValidationError> {
        let frame = self
            .ctrl_stack
            .last_mut()
            .ok_or(ValidationError::UnexpectedEmptyControlStack)?;
        self.value_stack.truncate(frame.start_height);
        frame.is_unreachable = true;
        Ok(())
    }

    pub fn validate_binop(&mut self, val_type: ValueType) -> Result<(), ValidationError> {
        self.pop_val_expect_val(val_type)?;
        self.pop_val_expect_val(val_type)?;
        self.push_val_t(val_type);
        Ok(())
    }

    pub fn validate_relop(&mut self, t: ValueType) -> Result<(), ValidationError> {
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(t)?;
        self.push_val_t(ValueType::I32);
        Ok(())
    }

    pub fn check_memarg(
        &mut self,
        context: &Context,
        memarg: op::Memarg,
        n: u32,
    ) -> Result<(), ValidationError> {
        if context.memories.len() <= 0 {
            return Err(ValidationError::UnexpectedNoMemories);
        };
        let align = 2_i32.pow(memarg.align);

        if align > (n / 8) as i32 {
            Err(ValidationError::InvalidAlignment)
        } else {
            Ok(())
        }
    }

    pub fn validate_store_n(
        &mut self,
        context: &Context,
        memarg: op::Memarg,
        n: u32,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(context, memarg, n)?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
    }

    pub fn validate_store(
        &mut self,
        context: &Context,
        memarg: op::Memarg,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(
            context,
            memarg,
            t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32,
        )?;
        self.pop_val_expect_val(t)?;
        self.pop_val_expect_val(ValueType::I32)?;
        Ok(())
    }

    pub fn validate_load(
        &mut self,
        context: &Context,
        memarg: op::Memarg,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(
            context,
            memarg,
            t.bit_width().ok_or(ValidationError::InvalidAlignment)? as u32,
        )?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }

    pub fn validate_load_n(
        &mut self,
        context: &Context,
        memarg: op::Memarg,
        n: u32,
        t: ValueType,
    ) -> Result<(), ValidationError> {
        self.check_memarg(context, memarg, n)?;
        self.pop_val_expect_val(ValueType::I32)?;
        self.push_val_t(t);
        Ok(())
    }

    pub fn get_local_type(&self, id: u32) -> Result<ValueType, ValidationError> {
        self.locals
            .get(id as usize)
            .ok_or(ValidationError::InvalidLocalId(id))
            .cloned()
    }

    pub fn validate_local_get(&mut self, id: u32) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.push_val_t(local_type);
        Ok(())
    }

    pub fn validate_local_set(&mut self, id: u32) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.pop_val_expect_val(local_type)?;
        Ok(())
    }

    pub fn validate_global_get(
        &mut self,
        context: &Context,
        id: u32,
    ) -> Result<(), ValidationError> {
        let global_type = context.get_global(id as usize)?;
        self.push_val_t(global_type.t.0);
        Ok(())
    }

    pub fn validate_global_set(
        &mut self,
        context: &Context,
        id: u32,
    ) -> Result<(), ValidationError> {
        let global_type = context.get_global(id as usize)?;
        if global_type.mutable.0 {
            self.pop_val_expect_val(global_type.t.0)?;
            Ok(())
        } else {
            Err(ValidationError::CannotSetToImmutableGlobal(id))
        }
    }

    pub fn validate_local_tee(
        &mut self,
        context: &Context,
        id: u32,
    ) -> Result<(), ValidationError> {
        let local_type = self.get_local_type(id)?;
        self.pop_val_expect_val(local_type)?;
        self.push_val_t(local_type);
        Ok(())
    }

    pub fn validate_select(&mut self, t: Option<ValueType>) -> Result<(), ValidationError> {
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
                    return Err(ValidationError::UnexpectedValueType {
                        got: t2,
                        expected: t1,
                    });
                }
                Ok(())
            }
        }
    }
    pub fn get_block_types(
        &self,
        context: &Context,
        blocktype: op::Blocktype,
    ) -> Result<(Vec<ValueType>, Vec<ValueType>), ValidationError> {
        match blocktype {
            op::Blocktype::Empty => Ok((vec![], vec![])),
            op::Blocktype::Value(value_type) => Ok((vec![], vec![value_type])),
            op::Blocktype::TypeIndex(index) => {
                let (t, _) = context.bytecode.get_type(index as usize)?;
                let in_t = t.params.iter().cloned().map(|(v, _)| v).collect::<Vec<_>>();
                let out_t = t
                    .results
                    .iter()
                    .cloned()
                    .map(|(v, _)| v)
                    .collect::<Vec<_>>();
                Ok((in_t, out_t))
            }
        }
    }

    pub fn validate_block(
        &mut self,
        context: &Context,
        op: (Op, Range<usize>),
        blocktype: op::Blocktype,
    ) -> Result<(), ValidationError> {
        let (in_types, out_types) = self.get_block_types(context, blocktype)?;
        println!("block in: {:?}, out: {:?}", in_types, out_types);
        in_types
            .iter()
            .cloned()
            .try_for_each(|f| self.pop_val_expect_val(f).map(|_| ()))?;
        self.push_new_ctrl(Some(op), in_types.clone(), out_types);

        Ok(())
    }

    pub fn validate_else(&mut self, op: (Op, Range<usize>)) -> Result<(), ValidationError> {
        /*
        let jmp = self.jump_table.push_new(self.instruction_pointer);
        self.ctrl_jump_stack
            .last_mut()
            .ok_or(ValidationError::UnexpectedEmptyControlStack)?
            .push(jmp);
        */
        let ctrl = self.pop_ctrl()?;

        if let Some((Op::If(_, _), _)) = ctrl.opcode {
            let if_jmp = self
                .jump_table
                .get_jump_mut(ctrl.jump_table_entry.unwrap())?;
            println!("ctrl: {:?}", ctrl);
            if_jmp.delta_ip = self.instruction_pointer as isize - ctrl.ip as isize + 1;
            self.push_new_ctrl(Some(op), ctrl.in_types, ctrl.out_types);
            Ok(())
        } else {
            Err(ValidationError::ElseWithoutIf)
        }
    }

    pub fn validate_end(&mut self) -> Result<(), ValidationError> {
        let ctrl = self.pop_ctrl()?;
        ctrl.out_types
            .iter()
            .for_each(|t| self.push_val_t(t.clone()));

        if let Some((ctrl_op, _)) = ctrl.opcode {
            let jumps_idx = self
                .ctrl_jump_stack
                .pop()
                .ok_or(ValidationError::UnexpectedEmptyControlStack)?;
            for idx in jumps_idx {
                println!("ctrl op: {ctrl_op}");
                let jump = self.jump_table.get_jump_mut(idx)?;

                let next_ip = match ctrl_op {
                    Op::Loop(_) => ctrl.ip as isize - jump.ip,
                    Op::Block(_) | Op::If(_, _) | Op::Else(_) => {
                        self.instruction_pointer as isize - jump.ip
                    }
                    _ => return Err(ValidationError::InvalidJump),
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

    pub fn validate_br(&mut self, n: u32) -> Result<(), ValidationError> {
        let out_types_len = self.peek_ctrl_at_label(n)?.out_types.len();
        let entry = JumpTableEntry {
            ip: self.instruction_pointer as isize,
            delta_ip: self.instruction_pointer as isize,
            stack_height: self.value_stack.len(),
            out_count: out_types_len,
        };

        let jmp = self.jump_table.push(entry);
        self.push_ctrl_jump(n, jmp)?;

        let vals = self
            .peek_ctrl_at_label(n)?
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        //TODO: (joh): Das ist schreklich
        vals.iter().try_for_each(|t| {
            _ = self.pop_val_expect_val(t.clone())?;
            Ok::<_, ValidationError>(())
        })?;
        self.set_unreachable()?;

        Ok(())
    }

    pub fn validate_br_if(&mut self, n: u32) -> Result<(), ValidationError> {
        self.pop_val_expect_val(ValueType::I32)?;

        let out_types_len = self.peek_ctrl_at_label(n)?.out_types.len();
        let vals = self
            .peek_ctrl_at_label(n)?
            .label_types()
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        vals.iter().try_for_each(|t| {
            _ = self.pop_val_expect_val(t.clone())?;
            Ok::<_, ValidationError>(())
        })?;
        let prev_stack_len = self.value_stack.len();
        self.value_stack
            .extend(vals.iter().cloned().map_into::<ValueStackType>());

        let entry = JumpTableEntry {
            ip: self.instruction_pointer as isize,
            delta_ip: self.instruction_pointer as isize,
            stack_height: prev_stack_len,
            out_count: out_types_len,
        };

        let jmp = self.jump_table.push(entry);
        self.push_ctrl_jump(n, jmp)?;
        Ok(())
    }

    pub fn validate_return(&mut self, context: &Context) -> Result<(), ValidationError> {
        let funcs = &context.get_func_type(self.current_func_id).unwrap().results;
        funcs
            .iter()
            .try_for_each(|t| -> Result<(), ValidationError> {
                _ = self.pop_val_expect_val(t.0)?;
                Ok(())
            })?;
        self.set_unreachable()
    }

    pub fn validate_call(
        &mut self,
        context: &Context,
        call_id: u32,
    ) -> Result<(), ValidationError> {
        let params = &context.get_func_type(call_id as usize)?.params;
        let results = context.get_func_type(call_id as usize)?.results.clone();
        params
            .iter()
            .cloned()
            .try_for_each(|(t, _)| -> Result<(), ValidationError> {
                let _ = self.pop_val_expect_val(t)?;

                Ok(())
            })?;
        results.iter().cloned().for_each(|(t, _)| {
            _ = self.push_val_t(t);
        });
        Ok(())
    }

    pub fn validate_op(
        &mut self,
        context: &Context,
        op: (Op, Range<usize>),
    ) -> Result<(), ValidationError> {
        use ValueType::*;
        match op.0 {
            Op::Unreachable => self.set_unreachable()?,
            Op::Nop => {}
            Op::Block(blocktype) => self.validate_block(context, op, blocktype)?,
            Op::Loop(blocktype) => self.validate_block(context, op, blocktype)?,
            Op::If(blocktype, _) => {
                self.pop_val_expect_val(I32)?;
                self.validate_block(context, op, blocktype)?;
            }
            Op::Else(_) => self.validate_else(op)?,
            Op::End => self.validate_end()?,
            Op::Br(n, _) => self.validate_br(n)?,
            Op::BrIf(n, _) => self.validate_br_if(n)?,
            Op::Return => self.validate_return(context)?,
            Op::Call(call_id) => self.validate_call(context, call_id)?,
            Op::CallIndirect(_, _) => todo!(),
            Op::Drop => _ = self.pop_val()?,
            Op::Select(t) => self.validate_select(t)?,
            Op::LocalGet(id) => self.validate_local_get(id)?,
            Op::LocalSet(id) => self.validate_local_set(id)?,
            Op::LocalTee(id) => self.validate_local_tee(context, id)?,
            Op::GlobalGet(id) => self.validate_global_get(context, id)?,
            Op::GlobalSet(id) => self.validate_global_set(context, id)?,
            Op::I32Load(memarg) => self.validate_load(context, memarg, I32)?,
            Op::I64Load(memarg) => self.validate_load(context, memarg, I64)?,
            Op::F32Load(memarg) => self.validate_load(context, memarg, F32)?,
            Op::F64Load(memarg) => self.validate_load(context, memarg, F64)?,
            Op::I32Load8s(memarg) | Op::I32Load8u(memarg) => {
                self.validate_load_n(context, memarg, 8, I32)?
            }
            Op::I32Load16s(memarg) | Op::I32Load16u(memarg) => {
                self.validate_load_n(context, memarg, 16, I32)?
            }
            Op::I64Load8s(memarg) | Op::I64Load8u(memarg) => {
                self.validate_load_n(context, memarg, 8, I64)?
            }
            Op::I64Load16s(memarg) | Op::I64Load16u(memarg) => {
                self.validate_load_n(context, memarg, 16, I64)?
            }
            Op::I64Load32s(memarg) | Op::I64Load32u(memarg) => {
                self.validate_load_n(context, memarg, 32, I64)?
            }
            Op::I32Store(memarg) => self.validate_store(context, memarg, I32)?,
            Op::I64Store(memarg) => self.validate_store(context, memarg, I64)?,
            Op::F32Store(memarg) => self.validate_store(context, memarg, F32)?,
            Op::F64Store(memarg) => self.validate_store(context, memarg, F64)?,
            Op::I32Store8(memarg) => self.validate_store_n(context, memarg, 8, I32)?,
            Op::I32Store16(memarg) => self.validate_store_n(context, memarg, 16, I32)?,
            Op::I64Store8(memarg) => self.validate_store_n(context, memarg, 8, I64)?,
            Op::I64Store16(memarg) => self.validate_store_n(context, memarg, 16, I64)?,
            Op::I64Store32(memarg) => self.validate_store_n(context, memarg, 32, I64)?,
            Op::I32Const(_) => self.push_val_t(I32),
            Op::I64Const(_) => self.push_val_t(I64),
            Op::F32Const(_) => self.push_val_t(F32),
            Op::F64Const(_) => self.push_val_t(F64),
            Op::I32Eqz
            | Op::I32Eq
            | Op::I32Ne
            | Op::I32Lts
            | Op::I32Ltu
            | Op::I32Gts
            | Op::I32Gtu
            | Op::I32Leu
            | Op::I32Les
            | Op::I32Ges
            | Op::I32Geu => self.validate_relop(I32)?,
            Op::I64Eqz
            | Op::I64Eq
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
        };
        self.instruction_pointer += 1;
        Ok(())
    }

    pub fn validate_func(context: &Context, code_id: usize) -> Result<JumpTable, ValidationError> {
        //TODO: (joh): Code und Typ zusammen um das hier etwas zu vereinfachen?
        let code = context
            .bytecode
            .code
            .as_ref()
            .ok_or(ValidationError::UnexpectedNoCode)?
            .0
            .get(code_id)
            .ok_or(ValidationError::InvalidCodeId(code_id))?;

        let func_type = context.get_internal_func_type(code_id)?;

        println!("Validating function: {}", code_id);

        let locals = code
            .0
            .locals
            .iter()
            .cloned()
            .map(|l| l.0.into_iter())
            .flatten()
            .chain(func_type.params.iter().map(|(v, p)| v.clone()))
            .collect();

        let mut validator = Validator {
            current_func_id: code_id,
            locals,
            ..Default::default()
        };

        let params: Vec<ValueType> = func_type.params.iter().map(|(v, _)| v.clone()).collect();
        let results: Vec<ValueType> = func_type.results.iter().map(|(v, _)| v.clone()).collect();

        validator.push_new_ctrl(None, Vec::new(), results.to_vec());

        for op in code.0.code.0.iter() {
            println!("Validating {}", op.0);
            validator.validate_op(context, op.clone())?;
        }

        Ok(validator.jump_table)
    }

    pub fn validate_all(context: &Context) -> Result<Vec<JumpTable>, ValidationError> {
        (0..context.get_function_count())
            .map(|i| Validator::validate_func(context, i))
            .collect()
    }
}

pub fn patch_jumps<'a, I: IntoIterator<Item = &'a JumpTable>>(
    module: &mut DecodedBytecode,
    jumps: I,
) -> Result<(), ValidationError> {
    if let Some(funcs) = module.code.as_mut() {
        for (func, table) in funcs.0.iter_mut().zip(jumps) {
            func.0.patch_jumps(table)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{
            error::ReaderError,
            module::DecodedBytecode,
            op::{Blocktype, Op},
            reader::Reader,
        },
        validation::{error::ValidationError, validator::patch_jumps},
    };

    use super::{Context, Validator};

    #[derive(Debug)]
    enum ValidationTestError {
        Validation(ValidationError),
        Parsing(ReaderError),
    }

    impl From<ReaderError> for ValidationTestError {
        fn from(value: ReaderError) -> Self {
            Self::Parsing(value)
        }
    }

    impl From<ValidationError> for ValidationTestError {
        fn from(value: ValidationError) -> Self {
            Self::Validation(value)
        }
    }

    #[test]
    fn empty_module() -> Result<(), ValidationTestError> {
        let src = "(module)";
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let _ = Validator::validate_all(&context)?;

        Ok(())
    }

    #[test]
    fn valid_module_simple() -> Result<(), ValidationTestError> {
        let src = r#"
            (module
                (func (param $p i32) (result i32)
                    local.get $p
                    i32.const 5
                    i32.add
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let _ = Validator::validate_all(&context)?;

        Ok(())
    }

    #[test]
    fn invalid_local_id() -> Result<(), ValidationTestError> {
        let src = r#"
            (module
                (func (param $p i32) (result i32)
                    local.get 1
                    i32.const 5
                    i32.add
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let res = Validator::validate_all(&context);
        assert_eq!(res.unwrap_err(), ValidationError::InvalidLocalId(1));

        Ok(())
    }

    #[test]
    fn valid_local_id_tee() -> Result<(), ValidationTestError> {
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
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let res = Validator::validate_all(&context)?;

        //assert_eq!(res.unwrap_err(), ValidationError::InvalidLocalId(1));

        Ok(())
    }

    #[test]
    fn valid_multiple_functions() -> Result<(), ValidationTestError> {
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
                    call 0
                    i32.const 10
                    i32.add
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let res = Validator::validate_all(&context)?;
        Ok(())
    }

    #[test]
    fn invalid_multiple_functions() -> Result<(), ValidationTestError> {
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
                    call 3
                    i32.const 10
                    i32.add
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let res = Validator::validate_all(&context);
        assert_eq!(res.unwrap_err(), ValidationError::InvalidFuncId(3));

        Ok(())
    }

    #[test]
    fn valid_param_count() -> Result<(), ValidationTestError> {
        let src = r#"
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
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let res = Validator::validate_all(&context)?;

        Ok(())
    }

    #[test]
    fn invalid_param_count() -> Result<(), ValidationTestError> {
        let src = r#"
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
                    i32.const 4  
                    call 0  
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let res = Validator::validate_all(&context);
        assert!(matches!(
            res.unwrap_err(),
            ValidationError::UnbalancedStack {
                got: _,
                expected: _
            }
        ));
        Ok(())
    }

    #[test]
    fn call_imported_function() -> Result<(), ValidationTestError> {
        let src = r#"
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
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let _ = Validator::validate_all(&context)?;
        Ok(())
    }

    #[test]
    fn jump_table_block_works() -> Result<(), ValidationTestError> {
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
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jump_table = Validator::validate_all(&context)?;

        patch_jumps(&mut module, jump_table.iter())?;
        let func = &module.code.unwrap().0[0].0.code.0;
        let Op::BrIf(_, jmp) = func[6].0.clone() else {
            panic!("Unexpected instruction");
        };
        let cont = jump_table[0].get_jump(jmp as usize)?.delta_ip;
        let after_block = func[7 + cont as usize].0.clone();

        assert_eq!(after_block, Op::I32Const(100));
        Ok(())
    }

    #[test]
    fn jump_table_if_else_work() -> Result<(), ValidationTestError> {
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
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jump_table = Validator::validate_all(&context)?;

        patch_jumps(&mut module, jump_table.iter())?;

        let func = &module.code.unwrap().0[0].0.code.0;
        let Op::If(_, jmp) = func[1].0.clone() else {
            panic!("Unexpected instruction");
        };
        let cont = jump_table[0].get_jump(jmp as usize)?.delta_ip;

        let after_else = func[1 + cont as usize].0.clone();
        assert_eq!(after_else, Op::I32Const(100));

        let else_pos = (cont) as usize;
        let op_at = func[else_pos].0.clone();
        let Op::Else(else_jmp) = op_at else {
            panic!("Unexpected instruction: {:?}", op_at);
        };

        let after_else_block = func[(else_pos as isize + else_jmp) as usize].0.clone();
        assert_eq!(after_else_block, Op::End);
        Ok(())
    }

    #[test]
    fn if_no_else() -> Result<(), ValidationTestError> {
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
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jump_table = Validator::validate_all(&context)?;

        for (func, jump_table) in module
            .code
            .as_mut()
            .unwrap()
            .0
            .iter_mut()
            .zip(jump_table.iter())
        {
            func.0.patch_jumps(jump_table)?;
        }

        let func = &module.code.unwrap().0[0].0.code.0;
        let Op::If(_, jmp) = func[1].0.clone() else {
            panic!("Unexpected instruction");
        };
        let cont = jump_table[0].get_jump(jmp as usize)?.delta_ip;

        let after_if = func[2 + cont as usize].0.clone();
        assert_eq!(after_if, Op::I32Const(100));
        Ok(())
    }

    #[test]
    fn loop_jumps() -> Result<(), ValidationTestError> {
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
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let mut module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;

        let jump_table = Validator::validate_all(&context)?;

        for (func, jump_table) in module
            .code
            .as_mut()
            .unwrap()
            .0
            .iter_mut()
            .zip(jump_table.iter())
        {
            func.0.patch_jumps(jump_table)?;
        }

        let func = &module.code.unwrap().0[0].0.code.0;

        let jmp1_ip: isize = 2;
        let Op::BrIf(_, jmp1) = func[jmp1_ip as usize].0.clone() else {
            panic!("Unexpected instruction");
        };
        let cont1 = jump_table[0].get_jump(jmp1 as usize)?.delta_ip;

        println!("jump 1: {jmp1}");
        let after_jmp1 = func[(jmp1_ip + cont1 + 1) as usize].0.clone();
        assert_eq!(after_jmp1, Op::I32Const(50));

        let jmp2_ip: isize = 6;
        let Op::BrIf(_, jmp2) = func[jmp2_ip as usize].0.clone() else {
            panic!("Unexpected instruction");
        };
        println!("jump 2: {jmp2}");
        let cont2 = jump_table[0].get_jump(jmp2 as usize)?.delta_ip;
        let after_jmp2 = func[(jmp2_ip + cont2 + 1) as usize].0.clone();
        assert_eq!(after_jmp2, Op::I32Const(50));

        Ok(())
    }

    #[test]
    fn memory_instructions_without_memory() -> Result<(), ValidationTestError> {
        let src = r#"
            (module
                (func (param $p i32) (result i32)
                    i32.const 1
                    i32.load 
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let res = Validator::validate_all(&context);
        assert_eq!(res.unwrap_err(), ValidationError::UnexpectedNoMemories);
        Ok(())
    }

    #[test]
    fn basic_memory() -> Result<(), ValidationTestError> {
        let src = r#"
            (module
                (memory 1)
                (func (param $p i32) (result i32)
                    i32.const 1
                    i32.load 
                )
            )
        "#;
        let code = wat::parse_str(src).unwrap().into_boxed_slice();
        let mut reader = Reader::new(&code);
        let module = reader.read::<DecodedBytecode>()?;
        let context = Context::new(&module)?;
        let res = Validator::validate_all(&context)?;
        Ok(())
    }
}
