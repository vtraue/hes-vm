use byteorder::ReadBytesExt;
use core::fmt;

use crate::{
    leb::Leb,
    reader::{BytecodeReader, FromBytecode, ParserError, ValueType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocktype {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}

impl FromBytecode for Blocktype {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let b = reader.read_i8()?;
        println!("b: {}", b);

        match b {
            0x40 => Ok(Self::Empty),
            0x6F..=0x7F => Ok(Self::Value(b.try_into()?)),
            _ => {
                //TODO: Finde eine huebschere Loesung!
                reader.seek(std::io::SeekFrom::Current(-1))?;
                let value = Leb::read_u32(reader)?;
                println!("value: {:x?}", value);

                Ok(Self::TypeIndex(value.try_into().unwrap()))
            }
        }
    }
}

impl fmt::Display for Blocktype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Blocktype::Empty => write!(f, ""),
            Blocktype::Value(value_type) => write!(f, "-> <{value_type}>"),
            Blocktype::TypeIndex(id) => write!(f, "{id}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memarg {
    pub offset: u32,
    pub align: u32,
}

impl FromBytecode for Memarg {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Memarg {
            align: reader.parse()?,
            offset: reader.parse()?,
        })
    }
}
impl fmt::Display for Memarg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.offset, self.align)
    }
}

#[derive(Debug, Clone)]
pub enum JumpDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Unreachable,
    Nop,
    Block(Blocktype),

    Loop(Blocktype),
    If { bt: Blocktype, jmp: isize },
    Else(isize),
    End,
    Br { label: usize, jmp: isize },
    BrIf { label: usize, jmp: isize },
    Return,
    Call(usize),
    CallIndirect { table: usize, type_id: isize },
    Drop,
    Select(Option<ValueType>),
    LocalGet(usize),
    LocalSet(usize),
    LocalTee(usize),
    GlobalGet(usize),
    GlobalSet(usize),
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
    I32Const(u32),
    I64Const(u64),
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
    I32Add,
    I32Sub,
    I32Mul,
    I32Divs,
    I32Divu,
    I32Rems,
    I32Remu,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32Shrs,
    I32Shru,
    I32Rotl,
    I32Rotr,
    I64Add,
    I64Sub,
    I64Mul,
    I64Divs,
    I64Divu,
    I64Rems,
    I64Remu,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64Shrs,
    I64Shru,
    I64Rotl,
    I64Rotr,

    MemoryCopy,
    MemoryFill,
    MemoryInit { data_id: usize, extra: usize }, //TODO: (joh): Float ops
}

impl Op {
    pub fn needs_end_terminator(&self) -> bool {
        matches!(self, Op::Block(_) | Op::Loop(_) | Op::If { bt: _, jmp: _ })
    }

    pub fn is_const(&self) -> bool {
        //TODO: (joh): Das stimmt so nicht 100%, wir muessten
        //testen ob ein Global.Get in der Form Const t ist: https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions
        matches!(
            self,
            Self::I32Const(_) | Self::I64Const(_) | Self::GlobalGet(_)
        )
    }
    pub fn is_terminator(&self) -> bool {
        matches!(self, Self::End)
    }

    pub fn continues(&self, depth: i32) -> (i32, bool) {
        if self.needs_end_terminator() {
            (depth + 1, true)
        } else if self.is_terminator() {
            if depth <= 0 {
                (0, false)
            } else {
                (depth - 1, true)
            }
        } else {
            (depth, true)
        }
    }
    pub fn is_branch(&self) -> bool {
        return matches!(self, Op::If { bt: _, jmp: _ }) || matches!(self, Op::Else(_));
    }

    pub fn get_jmp(&self) -> Option<isize> {
        match self {
            Op::If { jmp, .. } => Some(*jmp),
            Op::Else(jmp) => Some(*jmp),
            Op::Br { jmp, .. } => Some(*jmp),
            Op::BrIf { jmp, .. } => Some(*jmp),
            _ => None,
        }
    }
}
pub fn read_fc_op(reader: &mut impl BytecodeReader) -> Result<Op, ParserError> {
    let opcode = reader.read_u8()?;
    let instr = match opcode {
        0x08 => Op::MemoryInit {
            data_id: reader.parse()?,
            extra: reader.parse()?,
        },
        _ => todo!(),
    };
    Ok(instr)
}

impl FromBytecode for Op {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        let opcode = reader.read_u8()?;
        let instr = match opcode {
            0x00 => Self::Unreachable,
            0x01 => Self::Nop,
            0x02 => Self::Block(reader.parse()?),
            0x03 => Self::Loop(reader.parse()?),
            0x04 => Self::If {
                bt: reader.parse()?,
                jmp: 0,
            },
            0x05 => Self::Else(0),
            0x0B => Self::End,
            0x0C => Self::Br {
                label: reader.parse()?,
                jmp: 0,
            },
            0x0D => Self::BrIf {
                label: reader.parse()?,
                jmp: 0,
            },
            0x0F => Self::Return,
            0x10 => Self::Call(reader.parse()?),
            0x11 => Self::CallIndirect {
                table: reader.parse()?,
                type_id: reader.parse()?,
            },
            0x1A => Self::Drop,
            0x1B => Self::Select(None),
            0x1C => Self::Select(Some(reader.parse()?)),
            0x20 => Self::LocalGet(reader.parse()?),
            0x21 => Self::LocalSet(reader.parse()?),
            0x22 => Self::LocalTee(reader.parse()?),
            0x23 => Self::GlobalGet(reader.parse()?),
            0x24 => Self::GlobalSet(reader.parse()?),
            0x28 => Self::I32Load(reader.parse()?),
            0x29 => Self::I64Load(reader.parse()?),
            0x2A => Self::F32Load(reader.parse()?),
            0x2B => Self::F64Load(reader.parse()?),
            0x2C => Self::I32Load8s(reader.parse()?),
            0x2D => Self::I32Load8u(reader.parse()?),
            0x2E => Self::I32Load16s(reader.parse()?),
            0x2F => Self::I32Load16u(reader.parse()?),
            0x30 => Self::I64Load8s(reader.parse()?),
            0x31 => Self::I64Load8u(reader.parse()?),
            0x32 => Self::I64Load16s(reader.parse()?),
            0x33 => Self::I64Load16u(reader.parse()?),
            0x34 => Self::I64Load32s(reader.parse()?),
            0x35 => Self::I64Load32u(reader.parse()?),
            0x36 => Self::I32Store(reader.parse()?),
            0x37 => Self::I64Store(reader.parse()?),
            0x38 => Self::F32Store(reader.parse()?),
            0x39 => Self::F64Store(reader.parse()?),
            0x3A => Self::I32Store8(reader.parse()?),
            0x3B => Self::I32Store16(reader.parse()?),
            0x3C => Self::I64Store8(reader.parse()?),
            0x3D => Self::I64Store16(reader.parse()?),
            0x3E => Self::I64Store32(reader.parse()?),
            0x41 => Self::I32Const(reader.parse()?),
            0x42 => Self::I64Const(reader.parse()?),
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

            0x6A => Op::I32Add,
            0x6B => Op::I32Sub,
            0x6C => Op::I32Mul,
            0x6D => Op::I32Divs,
            0x6E => Op::I32Divu,
            0x6F => Op::I32Rems,
            0x70 => Op::I32Remu,
            0x71 => Op::I32And,
            0x72 => Op::I32Or,
            0x73 => Op::I32Xor,
            0x74 => Op::I32Shl,
            0x75 => Op::I32Shrs,
            0x76 => Op::I32Shru,
            0x77 => Op::I32Rotl,
            0x78 => Op::I32Rotr,

            0x7C => Op::I64Add,
            0x7D => Op::I64Sub,
            0x7E => Op::I64Mul,
            0x7F => Op::I64Divs,
            0x80 => Op::I64Divu,
            0x81 => Op::I64Rems,
            0x82 => Op::I64Remu,
            0x83 => Op::I64And,
            0x84 => Op::I64Or,
            0x85 => Op::I64Xor,
            0x86 => Op::I64Shl,
            0x87 => Op::I64Shrs,
            0x89 => Op::I64Shru,
            0x8A => Op::I64Rotl,
            0x8B => Op::I64Rotr,

            0xFC => read_fc_op(reader)?, //Memory
            //
            _ => panic!("Unimplemented Opcode {:0X}", opcode),
        };

        Ok(instr)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Unreachable => write!(f, "unreachable"),
            Op::Nop => write!(f, "nop"),
            Op::Block(blocktype) => write!(f, "block {blocktype}"),
            Op::Loop(blocktype) => write!(f, "loop {blocktype}"),
            Op::If { bt, jmp } => write!(f, "if {bt} (jmp: {jmp})"),
            Op::Else(jmp) => write!(f, "else (delta ip: {jmp}"),
            Op::End => write!(f, "end"),
            Op::Br { label, jmp } => write!(f, "br {label} (jmp: {jmp})"),
            Op::BrIf { label, jmp } => write!(f, "br_if {label} (jmp: {jmp})"),
            Op::Return => write!(f, "return"),
            Op::Call(func_id) => write!(f, "call {func_id}"),
            Op::CallIndirect { table, type_id } => write!(f, "call_indirect {table} {type_id}"),
            Op::Drop => write!(f, "drop"),
            Op::Select(_) => write!(f, "select"), //TODO: (joh): Argumente fuer Select
            Op::LocalGet(id) => write!(f, "local.get {id}"),
            Op::LocalSet(id) => write!(f, "local.set {id}"),
            Op::LocalTee(id) => write!(f, "local.tee {id}"),
            Op::GlobalGet(id) => write!(f, "global.get {id}"),
            Op::GlobalSet(id) => write!(f, "global.set {id}"),
            Op::I32Load(memarg) => write!(f, "i32.load {memarg}"),
            Op::I64Load(memarg) => write!(f, "i64.load {memarg}"),
            Op::F32Load(memarg) => write!(f, "f32.load {memarg}"),
            Op::F64Load(memarg) => write!(f, "f64.load {memarg}"),
            Op::I32Load8s(memarg) => write!(f, "i32.load8s {memarg}"),
            Op::I32Load8u(memarg) => write!(f, "i32.load8u {memarg}"),
            Op::I32Load16s(memarg) => write!(f, "i32.load16s {memarg}"),
            Op::I32Load16u(memarg) => write!(f, "i32.load16u {memarg}"),
            Op::I64Load8s(memarg) => write!(f, "i64.load8s {memarg}"),
            Op::I64Load8u(memarg) => write!(f, "i64.load8u {memarg}"),
            Op::I64Load16s(memarg) => write!(f, "i64.load16s {memarg}"),
            Op::I64Load16u(memarg) => write!(f, "i64.load16u {memarg}"),
            Op::I64Load32s(memarg) => write!(f, "i64.load32s {memarg}"),
            Op::I64Load32u(memarg) => write!(f, "i64.load32u {memarg}"),
            Op::I32Store(memarg) => write!(f, "i32.store {memarg}"),
            Op::I64Store(memarg) => write!(f, "i64.store {memarg}"),
            Op::F32Store(memarg) => write!(f, "f32.store {memarg}"),
            Op::F64Store(memarg) => write!(f, "i64.store {memarg}"),
            Op::I32Store8(memarg) => write!(f, "i32.store8 {memarg}"),
            Op::I32Store16(memarg) => write!(f, "i32.store16 {memarg}"),
            Op::I64Store8(memarg) => write!(f, "i64.store8 {memarg}"),
            Op::I64Store16(memarg) => write!(f, "i64.store16 {memarg}"),
            Op::I64Store32(memarg) => write!(f, "i64.store32 {memarg}"),
            Op::I32Const(arg) => write!(f, "i32.const {arg}"),
            Op::I64Const(arg) => write!(f, "i64.const {arg}"),
            Op::F32Const(arg) => write!(f, "f32.const {arg}"),
            Op::F64Const(arg) => write!(f, "f64.const {arg}"),
            Op::I32Eqz => write!(f, "i32.eqz"),
            Op::I32Eq => write!(f, "i32.eq"),
            Op::I32Ne => write!(f, "i32.ne"),
            Op::I32Lts => write!(f, "i32.lts"),
            Op::I32Ltu => write!(f, "i32.ltu"),
            Op::I32Gts => write!(f, "i32.gts"),
            Op::I32Gtu => write!(f, "i32.gtu"),
            Op::I32Leu => write!(f, "i32.leu"),
            Op::I32Les => write!(f, "i32.les"),
            Op::I32Ges => write!(f, "i32.ges"),
            Op::I32Geu => write!(f, "i32.geu"),
            Op::I64Eqz => write!(f, "i64.eqz"),
            Op::I64Eq => write!(f, "i64.eq"),
            Op::I64Ne => write!(f, "i64.ne"),
            Op::I64Lts => write!(f, "i64.lts"),
            Op::I64Ltu => write!(f, "i64.ltu"),
            Op::I64Gts => write!(f, "i64.gts"),
            Op::I64Gtu => write!(f, "i64.gtu"),
            Op::I64Les => write!(f, "i64.les"),
            Op::I64Leu => write!(f, "i64.leu"),
            Op::I64Ges => write!(f, "i64.ges"),
            Op::I64Geu => write!(f, "i64.geu"),
            Op::MemoryCopy => write!(f, "memory.copy"),
            Op::MemoryFill => write!(f, "memory.fill"),
            Op::I32Add => write!(f, "i32.add"),
            Op::I32Sub => write!(f, "i32.sub"),
            Op::I32Mul => write!(f, "i32.mul"),
            Op::I32Divs => write!(f, "i32.div_s"),
            Op::I32Divu => write!(f, "i32.div_u"),
            Op::I32Rems => write!(f, "i32.rem_s"),
            Op::I32Remu => write!(f, "i32.rem_u"),
            Op::I32And => write!(f, "i32.and"),
            Op::I32Or => write!(f, "i32.or"),
            Op::I32Xor => write!(f, "i32.xor"),
            Op::I32Shl => write!(f, "i32.shl"),
            Op::I32Shrs => write!(f, "i32.shrs"),
            Op::I32Shru => write!(f, "i32.shru"),
            Op::I32Rotl => write!(f, "i32.rotl"),
            Op::I32Rotr => write!(f, "i32.rotr"),
            Op::I64Add => write!(f, "i64.add"),
            Op::I64Sub => write!(f, "i64.sub"),
            Op::I64Mul => write!(f, "i64.mul"),
            Op::I64Divs => write!(f, "i64.div_s"),
            Op::I64Divu => write!(f, "i64.div_u"),
            Op::I64Rems => write!(f, "i64.rem_s"),
            Op::I64Remu => write!(f, "i64.rem_u"),
            Op::I64And => write!(f, "i64.and"),
            Op::I64Or => write!(f, " i64.or"),
            Op::I64Xor => write!(f, "i64.xor"),
            Op::I64Shl => write!(f, "i64.shl"),
            Op::I64Shrs => write!(f, "i64.shrs"),
            Op::I64Shru => write!(f, "i64.shru"),
            Op::I64Rotl => write!(f, "i64.rotl"),
            Op::I64Rotr => write!(f, "i64.rotr"),
            Op::MemoryInit { data_id, .. } => {
                write!(f, "memory.init {data_id}")
            }
        }
    }
}
