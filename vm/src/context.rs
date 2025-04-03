use itertools::Itertools;

use crate::{bytecode_info::BytecodeInfo, op::{self, Op}, reader::{ValueType}, types::{GlobalType, Limits, Type}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValidationError {
    ValueStackUnderflow,
    UnexpectedValueType {got: ValueStackType, expected: ValueStackType},
    UnexpectedEmptyControlStack,
    ReturnTypesDoNotMatch,
    UnbalancedStack,
    UnexpectedNoMemories,
    InvalidAlignment,
}

pub type Result<T> = std::result::Result<T, ValidationError>;

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
    opcode: Op,
    in_types: Vec<ValueType>,
    out_types: Vec<ValueType>,
    start_height: usize,
    is_unreachable: bool, 
}
impl CtrlFrame {
    pub fn new(context: &Context, opcode: Op, in_types: Vec<ValueType>, out_types: Vec<ValueType>) -> Self {
        let start_height = context.value_stack.len(); 
        CtrlFrame {opcode, in_types, out_types, start_height, is_unreachable: false}
    }
    pub fn label_types<'me>(&'me self) -> &'me[ValueType] {
        if let Op::Loop(_) = self.opcode {
            self.in_types.as_slice()
        } else {
            self.out_types.as_slice()
        }
    }
}

pub struct Context {
    current_locals: Vec<ValueType>, 
    return_type: Vec<ValueType>, 

    ctrl_stack: Vec<CtrlFrame>,
    value_stack: Vec<ValueStackType>,
    types: Vec<Type>, 
    funcs: Vec<Type>,
    //TODO: Tables
    mems: Vec<Limits>,
    globals: Vec<GlobalType>,
    //TODO: Elems
    locals: Vec<ValueType>,
    labels: Vec<Vec<ValueType>>,
    return_types: Option<Vec<ValueType>> 
    //TODO: Refs
}

impl Context {
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

    pub fn push_new_ctrl<T, O>(&mut self, opcode: Op, in_types: T, out_types: O) 
        where T: IntoIterator<Item = ValueType>,
              O: IntoIterator<Item = ValueType>  
    {
        //TODO: (joh): Das ist nicht sehr eleggant
        let in_types = in_types.into_iter().collect::<Vec<_>>();
        self.value_stack.extend(in_types.iter().cloned().map_into::<ValueStackType>());
        let ctrl = CtrlFrame::new(self, opcode, in_types, out_types.into_iter().collect()); 
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
    pub fn validate_store_n(&mut self, memarg: op::Memarg, n: u32, t: ValueType) -> Result<()>{
        if self.mems.len() <= 0 {
            return Err(ValidationError::UnexpectedNoMemories)
        }
        let align = 2_i32.pow(memarg.align);
        if align > (n / 8) as i32 {
            Err(ValidationError::InvalidAlignment)
        } else {
            self.pop_val_expect_val(ValueType::I32);
            self.pop_val_expect_val(t);
            Ok(())
        }
    }

    pub fn validate_store(&mut self, memarg: op::Memarg, t: ValueType) -> Result<()> {
        if self.mems.len() <= 0 {
            return Err(ValidationError::UnexpectedNoMemories);
        };
        let align = 2_i32.pow(memarg.align);
        if align > (t.bit_width().ok_or(ValidationError::InvalidAlignment)? / 8) as i32 {
            Err(ValidationError::InvalidAlignment)
        } else {
            self.pop_val_expect_val(ValueType::I32);
            self.pop_val_expect_val(t);
            Ok(())
        }
    }
    pub fn validate_op(&mut self, op: Op) -> Result<()> {
        use ValueType::*;
        match op {
            Op::Unreachable => self.set_unreachable()?,
            Op::Nop => {},
            Op::Block(blocktype) => todo!(),
            Op::Loop(blocktype) => todo!(),
            Op::If(blocktype) => todo!(),
            Op::Else => todo!(),
            Op::End => todo!(),
            Op::Br(_) => todo!(),
            Op::BrIf(_) => todo!(),
            Op::Return => todo!(),
            Op::Call(_) => todo!(),
            Op::CallIndirect(_, _) => todo!(),
            Op::Drop => todo!(),
            Op::Select => todo!(),
            Op::LocalGet(_) => todo!(),
            Op::LocalSet(_) => todo!(),
            Op::LocalTee(_) => todo!(),
            Op::GlobalGet(_) => todo!(),
            Op::GlobalSet(_) => todo!(),
            Op::I32Load(memarg) => todo!(),
            Op::I64Load(memarg) => todo!(),
            Op::F32Load(memarg) => todo!(),
            Op::F64Load(memarg) => todo!(),
            Op::I32Load8s(memarg) => todo!(),
            Op::I32Load8u(memarg) => todo!(),
            Op::I32Load16s(memarg) => todo!(),
            Op::I32Load16u(memarg) => todo!(),
            Op::I64Load8s(memarg) => todo!(),
            Op::I64Load8u(memarg) => todo!(),
            Op::I64Load16s(memarg) => todo!(),
            Op::I64Load16u(memarg) => todo!(),
            Op::I64Load32s(memarg) => todo!(),
            Op::I64Load32u(memarg) => todo!(),
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
