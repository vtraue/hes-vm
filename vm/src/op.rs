use crate::reader::{
    self, BytecodeReader, FromBytecodeReader, FuncId, GlobalId, LabelId, LocalId, TableId, TypeId,
    ValueType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Blocktype {
    Empty,
    Value(ValueType),
    TypeIndex(i32),
}

impl<'src> FromBytecodeReader<'src> for Blocktype {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> reader::Result<Self> {
        let desc = reader.read_var_s33()?;

        match desc {
            0x40 => Ok(Self::Empty),
            n if n < 0 => Ok(Self::Value((n as u8).try_into()?)),
            _ => Ok(Self::TypeIndex(desc as i32)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memarg {
    offset: u32,
    align: u32,
}

impl<'src> FromBytecodeReader<'src> for Memarg {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> reader::Result<Self> {
        Ok(Memarg {
            offset: reader.read()?,
            align: reader.read()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Unreachable,
    Nop,
    Block(Blocktype),
    Loop(Blocktype),
    If(Blocktype),
    Else,
    End,
    Br(LabelId),
    BrIf(LabelId),
    Return,
    Call(FuncId),
    CallIndirect(TableId, TypeId),
    Drop,
    Select, //TODO: Select mit args?
    LocalGet(LocalId),
    LocalSet(LocalId),
    LocalTee(LocalId),
    GlobalGet(GlobalId),
    GlobalSet(GlobalId),
    I32Load(Memarg),
    I64Load(Memarg),
    F32Load(Memarg),
    F64Load(Memarg),
    I32Load8s(Memarg),
    I32Load8u(Memarg),
    I32Load16s(Memarg),
    I32Load16u(Memarg),
    I64Load8s(Memarg),
    I64Load8u(Memarg),
    I64Load16s(Memarg),
    I64Load16u(Memarg),
    I64Load32s(Memarg),
    I64Load32u(Memarg),
    I32Store(Memarg),
    I64Store(Memarg),
    F32Store(Memarg),
    F64Store(Memarg),
    I32Store8(Memarg),
    I32Store16(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32Lts,
    I32Ltu,
    I32Gts,
    I32Gtu,
    I32Leu,
    I32Les,
    I32Ges,
    I32Geu,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64Lts,
    I64Ltu,
    I64Gts,
    I64Gtu,
    I64Les,
    I64Leu,
    I64Ges,
    I64Geu,
    MemoryCopy,
    MemoryFill,
    //TODO: (joh): Float ops
}

impl Op {
    pub fn needs_end_terminator(&self) -> bool {
        match self {
            Op::Block(_) | Op::Loop(_) => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        //TODO: (joh): Das stimmt so nicht 100%, wir muessten
        //testen ob ein Global.Get in der Form Const t ist: https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions
        //Das muessten wir spaeter dann in
        match self {
            Self::I32Const(_) | Self::I64Const(_) | Self::GlobalGet(_) => true,
            _ => false,
        }
    }
}
impl<'src> FromBytecodeReader<'src> for Op {
    fn from_reader(reader: &mut BytecodeReader<'src>) -> reader::Result<Self> {
        let opcode = reader.read_u8()?;
        let instr = match opcode {
            0x00 => Self::Unreachable,
            0x01 => Self::Nop,
            0x02 => Self::Block(reader.read()?),
            0x03 => Self::Loop(reader.read()?),
            0x04 => Self::If(reader.read()?),
            0x05 => Self::Else,
            0x0B => Self::End,
            0x0C => Self::Br(reader.read()?),
            0x0D => Self::BrIf(reader.read()?),
            0x0F => Self::Return,
            0x10 => Self::Call(reader.read()?),
            0x11 => Self::CallIndirect(reader.read()?, reader.read()?),
            0x1A => Self::Drop,
            0x1B => Self::Select,
            0x20 => Self::LocalGet(reader.read()?),
            0x21 => Self::LocalSet(reader.read()?),
            0x22 => Self::LocalTee(reader.read()?),
            0x23 => Self::GlobalGet(reader.read()?),
            0x24 => Self::GlobalSet(reader.read()?),
            0x28 => Self::I32Load(reader.read()?),
            0x29 => Self::I64Load(reader.read()?),
            0x2A => Self::F32Load(reader.read()?),
            0x2B => Self::F64Load(reader.read()?),
            0x2C => Self::I32Load8s(reader.read()?),
            0x2D => Self::I32Load8u(reader.read()?),
            0x2E => Self::I32Load16s(reader.read()?),
            0x2F => Self::I32Load16u(reader.read()?),
            0x30 => Self::I64Load8s(reader.read()?),
            0x31 => Self::I64Load8u(reader.read()?),
            0x32 => Self::I64Load16s(reader.read()?),
            0x33 => Self::I64Load16u(reader.read()?),
            0x34 => Self::I64Load32s(reader.read()?),
            0x35 => Self::I64Load32u(reader.read()?),
            0x36 => Self::I32Store(reader.read()?),
            0x37 => Self::I64Store(reader.read()?),
            0x38 => Self::F32Store(reader.read()?),
            0x39 => Self::F64Store(reader.read()?),
            0x3A => Self::I32Store8(reader.read()?),
            0x3B => Self::I32Store16(reader.read()?),
            0x3C => Self::I64Store8(reader.read()?),
            0x3D => Self::I64Store16(reader.read()?),
            0x3E => Self::I64Store32(reader.read()?),
            0x41 => Self::I32Const(reader.read()?),
            0x42 => Self::I64Const(reader.read()?),
            0x43 => todo!(), //const f32
            0x44 => todo!(), //const f64
            0x45 => Op::I32Eqz,
            0x46 => Op::I32Eq,
            0x47 => Op::I32Ne,
            0x48 => Op::I32Lts,
            0x49 => Op::I32Ltu,
            0x4A => Op::I32Gts,
            0x4B => Op::I32Gtu,
            0x4C => Op::I32Les,
            0x4D => Op::I32Leu,
            0x4E => Op::I32Ges,
            0x4F => Op::I32Geu,

            0x50 => Op::I64Eqz,
            0x51 => Op::I64Eq,
            0x52 => Op::I64Ne,
            0x53 => Op::I64Lts,
            0x54 => Op::I64Ltu,
            0x55 => Op::I64Gts,
            0x56 => Op::I64Gtu,
            0x57 => Op::I64Les,
            0x58 => Op::I64Leu,
            0x59 => Op::I64Ges,
            0x5A => Op::I64Geu,
            0xFC => todo!(), //Memory
            //
            _ => panic!("Unimplemented Opcode"),
        };
        Ok(instr)
    }
}
