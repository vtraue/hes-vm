use alloc::{alloc::Allocator, boxed::Box, fmt};

use crate::types::ValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memarg {
    pub offset: u32,
    pub align: u32,
}

/*
impl FromBytecode for Memarg {
    fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
        Ok(Memarg {
            align: reader.parse()?,
            offset: reader.parse()?,
        })
    }
}
*/
impl fmt::Display for Memarg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.offset, self.align)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrTableEntry {
    pub label: usize,
    pub jump: isize,
}
impl fmt::Display for BrTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.label, self.jump)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocktype {
    Empty,
    Value(ValueType),
    TypeIndex(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Unreachable,
    Nop,
    Block(Blocktype),

    Loop(Blocktype),
    If {
        bt: Blocktype,
        jmp: isize,
    },
    Else(isize),
    End(bool),
    Br {
        label: usize,
        jmp: isize,
    },
    BrIf {
        label: usize,
        jmp: isize,
    },
    BrTable {
        //labels: &'bump [BrTableEntry],
        labels: usize,
        default: BrTableEntry,
    },
    Return,
    Call(usize),
    CallIndirect {
        table: usize,
        type_id: isize,
    },
    RefNull,
    RefIsNull,
    RefFunc,
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

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

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

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    I32WrapI64,
    I64ExtendI32s,
    I64ExtendI32u,
    I32Extend8s,
    I32Extend16s,
    I64Extend8s,
    I64Extend16s,
    I64Extend32s,

    I32TruncSatF32s,
    I32TruncSatF32u,
    I32TruncSatF64s,
    I32TruncSatF64u,
    I64TruncSatF32s,
    I64TruncSatF32u,
    I64TruncSatF64s,
    I64TruncSatF64u,

    MemoryCopy {
        extra_1: usize,
        extra_2: usize,
    },
    MemoryFill {
        extra: usize,
    },
    MemoryInit {
        data_id: usize,
        extra: usize,
    }, //TODO: (joh): Float ops
    MemoryGrow {
        extra: usize,
    },
    TableGet(u32),
    TableSet(u32),
    TableInit {
        elem_id: u32,
        table_id: u32,
    },
    ElemDrop(u32),
    TableCopy {
        table_id_1: u32,
        table_id_2: u32,
    },
    TableGrow(u32),
    TableSize(u32),
    TableFill(u32),
}
