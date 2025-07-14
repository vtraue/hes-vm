#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod op {
    use crate::{
        leb::Leb,
        reader::{BytecodeReader, FromBytecode, ParserError, ValueType},
    };
    use byteorder::ReadBytesExt;
    use core::fmt;
    pub enum Blocktype {
        Empty,
        Value(ValueType),
        TypeIndex(u32),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Blocktype {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Blocktype::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
                Blocktype::Value(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Value", &__self_0)
                }
                Blocktype::TypeIndex(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TypeIndex", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Blocktype {
        #[inline]
        fn clone(&self) -> Blocktype {
            let _: ::core::clone::AssertParamIsClone<ValueType>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Blocktype {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Blocktype {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Blocktype {
        #[inline]
        fn eq(&self, other: &Blocktype) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (Blocktype::Value(__self_0), Blocktype::Value(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (Blocktype::TypeIndex(__self_0), Blocktype::TypeIndex(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Blocktype {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<ValueType>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    impl FromBytecode for Blocktype {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let value = Leb::read_s33(reader)?;
            if value > 0 {
                Ok(Self::TypeIndex(value.try_into().unwrap()))
            } else {
                let b: u8 = (value * -1) as u8;
                match value * -1 {
                    0x40 => Ok(Self::Empty),
                    0x6F..=0x7F => Ok(Self::Value(b.try_into()?)),
                    _ => Err(ParserError::InvalidBlocktype(value)),
                }
            }
        }
    }
    impl fmt::Display for Blocktype {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Blocktype::Empty => f.write_fmt(format_args!("")),
                Blocktype::Value(value_type) => f.write_fmt(format_args!("-> <{0}>", value_type)),
                Blocktype::TypeIndex(id) => f.write_fmt(format_args!("{0}", id)),
            }
        }
    }
    pub struct Memarg {
        pub offset: u32,
        pub align: u32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Memarg {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Memarg",
                "offset",
                &self.offset,
                "align",
                &&self.align,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Memarg {
        #[inline]
        fn clone(&self) -> Memarg {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Memarg {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Memarg {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Memarg {
        #[inline]
        fn eq(&self, other: &Memarg) -> bool {
            self.offset == other.offset && self.align == other.align
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Memarg {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
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
            f.write_fmt(format_args!("{0} {1}", self.offset, self.align))
        }
    }
    pub enum Op {
        Unreachable,
        Nop,
        Block(Blocktype),
        Loop(Blocktype),
        If { bt: Blocktype, jmp: usize },
        Else(isize),
        End,
        Br { label: usize, jmp: usize },
        BrIf { label: usize, jmp: usize },
        Return,
        Call(usize),
        CallIndirect { table: usize, type_id: usize },
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
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Op {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Op::Unreachable => ::core::fmt::Formatter::write_str(f, "Unreachable"),
                Op::Nop => ::core::fmt::Formatter::write_str(f, "Nop"),
                Op::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Block", &__self_0)
                }
                Op::Loop(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Loop", &__self_0)
                }
                Op::If {
                    bt: __self_0,
                    jmp: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "If", "bt", __self_0, "jmp", &__self_1,
                ),
                Op::Else(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Else", &__self_0)
                }
                Op::End => ::core::fmt::Formatter::write_str(f, "End"),
                Op::Br {
                    label: __self_0,
                    jmp: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "Br", "label", __self_0, "jmp", &__self_1,
                ),
                Op::BrIf {
                    label: __self_0,
                    jmp: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "BrIf", "label", __self_0, "jmp", &__self_1,
                ),
                Op::Return => ::core::fmt::Formatter::write_str(f, "Return"),
                Op::Call(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Call", &__self_0)
                }
                Op::CallIndirect {
                    table: __self_0,
                    type_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CallIndirect",
                    "table",
                    __self_0,
                    "type_id",
                    &__self_1,
                ),
                Op::Drop => ::core::fmt::Formatter::write_str(f, "Drop"),
                Op::Select(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Select", &__self_0)
                }
                Op::LocalGet(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "LocalGet", &__self_0)
                }
                Op::LocalSet(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "LocalSet", &__self_0)
                }
                Op::LocalTee(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "LocalTee", &__self_0)
                }
                Op::GlobalGet(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "GlobalGet", &__self_0)
                }
                Op::GlobalSet(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "GlobalSet", &__self_0)
                }
                Op::I32Load(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Load", &__self_0)
                }
                Op::I64Load(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load", &__self_0)
                }
                Op::F32Load(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F32Load", &__self_0)
                }
                Op::F64Load(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F64Load", &__self_0)
                }
                Op::I32Load8s(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Load8s", &__self_0)
                }
                Op::I32Load8u(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Load8u", &__self_0)
                }
                Op::I32Load16s(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Load16s", &__self_0)
                }
                Op::I32Load16u(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Load16u", &__self_0)
                }
                Op::I64Load8s(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load8s", &__self_0)
                }
                Op::I64Load8u(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load8u", &__self_0)
                }
                Op::I64Load16s(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load16s", &__self_0)
                }
                Op::I64Load16u(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load16u", &__self_0)
                }
                Op::I64Load32s(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load32s", &__self_0)
                }
                Op::I64Load32u(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Load32u", &__self_0)
                }
                Op::I32Store(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Store", &__self_0)
                }
                Op::I64Store(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Store", &__self_0)
                }
                Op::F32Store(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F32Store", &__self_0)
                }
                Op::F64Store(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F64Store", &__self_0)
                }
                Op::I32Store8(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Store8", &__self_0)
                }
                Op::I32Store16(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Store16", &__self_0)
                }
                Op::I64Store8(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Store8", &__self_0)
                }
                Op::I64Store16(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Store16", &__self_0)
                }
                Op::I64Store32(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Store32", &__self_0)
                }
                Op::I32Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I32Const", &__self_0)
                }
                Op::I64Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "I64Const", &__self_0)
                }
                Op::F32Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F32Const", &__self_0)
                }
                Op::F64Const(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F64Const", &__self_0)
                }
                Op::I32Eqz => ::core::fmt::Formatter::write_str(f, "I32Eqz"),
                Op::I32Eq => ::core::fmt::Formatter::write_str(f, "I32Eq"),
                Op::I32Ne => ::core::fmt::Formatter::write_str(f, "I32Ne"),
                Op::I32Lts => ::core::fmt::Formatter::write_str(f, "I32Lts"),
                Op::I32Ltu => ::core::fmt::Formatter::write_str(f, "I32Ltu"),
                Op::I32Gts => ::core::fmt::Formatter::write_str(f, "I32Gts"),
                Op::I32Gtu => ::core::fmt::Formatter::write_str(f, "I32Gtu"),
                Op::I32Leu => ::core::fmt::Formatter::write_str(f, "I32Leu"),
                Op::I32Les => ::core::fmt::Formatter::write_str(f, "I32Les"),
                Op::I32Ges => ::core::fmt::Formatter::write_str(f, "I32Ges"),
                Op::I32Geu => ::core::fmt::Formatter::write_str(f, "I32Geu"),
                Op::I64Eqz => ::core::fmt::Formatter::write_str(f, "I64Eqz"),
                Op::I64Eq => ::core::fmt::Formatter::write_str(f, "I64Eq"),
                Op::I64Ne => ::core::fmt::Formatter::write_str(f, "I64Ne"),
                Op::I64Lts => ::core::fmt::Formatter::write_str(f, "I64Lts"),
                Op::I64Ltu => ::core::fmt::Formatter::write_str(f, "I64Ltu"),
                Op::I64Gts => ::core::fmt::Formatter::write_str(f, "I64Gts"),
                Op::I64Gtu => ::core::fmt::Formatter::write_str(f, "I64Gtu"),
                Op::I64Les => ::core::fmt::Formatter::write_str(f, "I64Les"),
                Op::I64Leu => ::core::fmt::Formatter::write_str(f, "I64Leu"),
                Op::I64Ges => ::core::fmt::Formatter::write_str(f, "I64Ges"),
                Op::I64Geu => ::core::fmt::Formatter::write_str(f, "I64Geu"),
                Op::I32Add => ::core::fmt::Formatter::write_str(f, "I32Add"),
                Op::I32Sub => ::core::fmt::Formatter::write_str(f, "I32Sub"),
                Op::I32Mul => ::core::fmt::Formatter::write_str(f, "I32Mul"),
                Op::I32Divs => ::core::fmt::Formatter::write_str(f, "I32Divs"),
                Op::I32Divu => ::core::fmt::Formatter::write_str(f, "I32Divu"),
                Op::I32Rems => ::core::fmt::Formatter::write_str(f, "I32Rems"),
                Op::I32Remu => ::core::fmt::Formatter::write_str(f, "I32Remu"),
                Op::I32And => ::core::fmt::Formatter::write_str(f, "I32And"),
                Op::I32Or => ::core::fmt::Formatter::write_str(f, "I32Or"),
                Op::I32Xor => ::core::fmt::Formatter::write_str(f, "I32Xor"),
                Op::I32Shl => ::core::fmt::Formatter::write_str(f, "I32Shl"),
                Op::I32Shrs => ::core::fmt::Formatter::write_str(f, "I32Shrs"),
                Op::I32Shru => ::core::fmt::Formatter::write_str(f, "I32Shru"),
                Op::I32Rotl => ::core::fmt::Formatter::write_str(f, "I32Rotl"),
                Op::I32Rotr => ::core::fmt::Formatter::write_str(f, "I32Rotr"),
                Op::I64Add => ::core::fmt::Formatter::write_str(f, "I64Add"),
                Op::I64Sub => ::core::fmt::Formatter::write_str(f, "I64Sub"),
                Op::I64Mul => ::core::fmt::Formatter::write_str(f, "I64Mul"),
                Op::I64Divs => ::core::fmt::Formatter::write_str(f, "I64Divs"),
                Op::I64Divu => ::core::fmt::Formatter::write_str(f, "I64Divu"),
                Op::I64Rems => ::core::fmt::Formatter::write_str(f, "I64Rems"),
                Op::I64Remu => ::core::fmt::Formatter::write_str(f, "I64Remu"),
                Op::I64And => ::core::fmt::Formatter::write_str(f, "I64And"),
                Op::I64Or => ::core::fmt::Formatter::write_str(f, "I64Or"),
                Op::I64Xor => ::core::fmt::Formatter::write_str(f, "I64Xor"),
                Op::I64Shl => ::core::fmt::Formatter::write_str(f, "I64Shl"),
                Op::I64Shrs => ::core::fmt::Formatter::write_str(f, "I64Shrs"),
                Op::I64Shru => ::core::fmt::Formatter::write_str(f, "I64Shru"),
                Op::I64Rotl => ::core::fmt::Formatter::write_str(f, "I64Rotl"),
                Op::I64Rotr => ::core::fmt::Formatter::write_str(f, "I64Rotr"),
                Op::MemoryCopy => ::core::fmt::Formatter::write_str(f, "MemoryCopy"),
                Op::MemoryFill => ::core::fmt::Formatter::write_str(f, "MemoryFill"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Op {
        #[inline]
        fn clone(&self) -> Op {
            let _: ::core::clone::AssertParamIsClone<Blocktype>;
            let _: ::core::clone::AssertParamIsClone<usize>;
            let _: ::core::clone::AssertParamIsClone<isize>;
            let _: ::core::clone::AssertParamIsClone<Option<ValueType>>;
            let _: ::core::clone::AssertParamIsClone<Memarg>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            let _: ::core::clone::AssertParamIsClone<f64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Op {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Op {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Op {
        #[inline]
        fn eq(&self, other: &Op) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (Op::Block(__self_0), Op::Block(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::Loop(__self_0), Op::Loop(__arg1_0)) => __self_0 == __arg1_0,
                    (
                        Op::If {
                            bt: __self_0,
                            jmp: __self_1,
                        },
                        Op::If {
                            bt: __arg1_0,
                            jmp: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (Op::Else(__self_0), Op::Else(__arg1_0)) => __self_0 == __arg1_0,
                    (
                        Op::Br {
                            label: __self_0,
                            jmp: __self_1,
                        },
                        Op::Br {
                            label: __arg1_0,
                            jmp: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (
                        Op::BrIf {
                            label: __self_0,
                            jmp: __self_1,
                        },
                        Op::BrIf {
                            label: __arg1_0,
                            jmp: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (Op::Call(__self_0), Op::Call(__arg1_0)) => __self_0 == __arg1_0,
                    (
                        Op::CallIndirect {
                            table: __self_0,
                            type_id: __self_1,
                        },
                        Op::CallIndirect {
                            table: __arg1_0,
                            type_id: __arg1_1,
                        },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (Op::Select(__self_0), Op::Select(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::LocalGet(__self_0), Op::LocalGet(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::LocalSet(__self_0), Op::LocalSet(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::LocalTee(__self_0), Op::LocalTee(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::GlobalGet(__self_0), Op::GlobalGet(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::GlobalSet(__self_0), Op::GlobalSet(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Load(__self_0), Op::I32Load(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load(__self_0), Op::I64Load(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F32Load(__self_0), Op::F32Load(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F64Load(__self_0), Op::F64Load(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Load8s(__self_0), Op::I32Load8s(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Load8u(__self_0), Op::I32Load8u(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Load16s(__self_0), Op::I32Load16s(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Load16u(__self_0), Op::I32Load16u(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load8s(__self_0), Op::I64Load8s(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load8u(__self_0), Op::I64Load8u(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load16s(__self_0), Op::I64Load16s(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load16u(__self_0), Op::I64Load16u(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load32s(__self_0), Op::I64Load32s(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Load32u(__self_0), Op::I64Load32u(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Store(__self_0), Op::I32Store(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Store(__self_0), Op::I64Store(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F32Store(__self_0), Op::F32Store(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F64Store(__self_0), Op::F64Store(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Store8(__self_0), Op::I32Store8(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Store16(__self_0), Op::I32Store16(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Store8(__self_0), Op::I64Store8(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Store16(__self_0), Op::I64Store16(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Store32(__self_0), Op::I64Store32(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I32Const(__self_0), Op::I32Const(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::I64Const(__self_0), Op::I64Const(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F32Const(__self_0), Op::F32Const(__arg1_0)) => __self_0 == __arg1_0,
                    (Op::F64Const(__self_0), Op::F64Const(__arg1_0)) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    impl Op {
        pub fn needs_end_terminator(&self) -> bool {
            match self {
                Op::Block(_) | Op::Loop(_) | Op::If { bt: _, jmp: _ } => true,
                _ => false,
            }
        }
        pub fn is_const(&self) -> bool {
            match self {
                Self::I32Const(_) | Self::I64Const(_) | Self::GlobalGet(_) => true,
                _ => false,
            }
        }
        pub fn is_terminator(&self) -> bool {
            match self {
                Self::End => true,
                _ => false,
            }
        }
        pub fn continues(&self, depth: u32) -> (u32, bool) {
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
            return match self {
                Op::If { bt: _, jmp: _ } => true,
                _ => false,
            } || match self {
                Op::Else(_) => true,
                _ => false,
            };
        }
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
                0x43 => ::core::panicking::panic("not yet implemented"),
                0x44 => ::core::panicking::panic("not yet implemented"),
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
                0xFC => ::core::panicking::panic("not yet implemented"),
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "Unimplemented Opcode {0:0X}",
                        opcode
                    ));
                }
            };
            Ok(instr)
        }
    }
    impl fmt::Display for Op {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Op::Unreachable => f.write_fmt(format_args!("unreachable")),
                Op::Nop => f.write_fmt(format_args!("nop")),
                Op::Block(blocktype) => f.write_fmt(format_args!("block {0}", blocktype)),
                Op::Loop(blocktype) => f.write_fmt(format_args!("loop {0}", blocktype)),
                Op::If { bt, jmp } => f.write_fmt(format_args!("if {0} (jmp: {1})", bt, jmp)),
                Op::Else(jmp) => f.write_fmt(format_args!("else (delta ip: {0}", jmp)),
                Op::End => f.write_fmt(format_args!("end")),
                Op::Br { label, jmp } => f.write_fmt(format_args!("br {0} (jmp: {1})", label, jmp)),
                Op::BrIf { label, jmp } => {
                    f.write_fmt(format_args!("br_if {0} (jmp: {1})", label, jmp))
                }
                Op::Return => f.write_fmt(format_args!("return")),
                Op::Call(func_id) => f.write_fmt(format_args!("call {0}", func_id)),
                Op::CallIndirect { table, type_id } => {
                    f.write_fmt(format_args!("call_indirect {0} {1}", table, type_id))
                }
                Op::Drop => f.write_fmt(format_args!("drop")),
                Op::Select(_) => f.write_fmt(format_args!("select")),
                Op::LocalGet(id) => f.write_fmt(format_args!("local.get {0}", id)),
                Op::LocalSet(id) => f.write_fmt(format_args!("local.set {0}", id)),
                Op::LocalTee(id) => f.write_fmt(format_args!("local.tee {0}", id)),
                Op::GlobalGet(id) => f.write_fmt(format_args!("global.get {0}", id)),
                Op::GlobalSet(id) => f.write_fmt(format_args!("global.set {0}", id)),
                Op::I32Load(memarg) => f.write_fmt(format_args!("i32.load {0}", memarg)),
                Op::I64Load(memarg) => f.write_fmt(format_args!("i64.load {0}", memarg)),
                Op::F32Load(memarg) => f.write_fmt(format_args!("f32.load {0}", memarg)),
                Op::F64Load(memarg) => f.write_fmt(format_args!("f64.load {0}", memarg)),
                Op::I32Load8s(memarg) => f.write_fmt(format_args!("i32.load8s {0}", memarg)),
                Op::I32Load8u(memarg) => f.write_fmt(format_args!("i32.load8u {0}", memarg)),
                Op::I32Load16s(memarg) => f.write_fmt(format_args!("i32.load16s {0}", memarg)),
                Op::I32Load16u(memarg) => f.write_fmt(format_args!("i32.load16u {0}", memarg)),
                Op::I64Load8s(memarg) => f.write_fmt(format_args!("i64.load8s {0}", memarg)),
                Op::I64Load8u(memarg) => f.write_fmt(format_args!("i64.load8u {0}", memarg)),
                Op::I64Load16s(memarg) => f.write_fmt(format_args!("i64.load16s {0}", memarg)),
                Op::I64Load16u(memarg) => f.write_fmt(format_args!("i64.load16u {0}", memarg)),
                Op::I64Load32s(memarg) => f.write_fmt(format_args!("i64.load32s {0}", memarg)),
                Op::I64Load32u(memarg) => f.write_fmt(format_args!("i64.load32u {0}", memarg)),
                Op::I32Store(memarg) => f.write_fmt(format_args!("i32.store {0}", memarg)),
                Op::I64Store(memarg) => f.write_fmt(format_args!("i64.store {0}", memarg)),
                Op::F32Store(memarg) => f.write_fmt(format_args!("f32.store {0}", memarg)),
                Op::F64Store(memarg) => f.write_fmt(format_args!("i64.store {0}", memarg)),
                Op::I32Store8(memarg) => f.write_fmt(format_args!("i32.store8 {0}", memarg)),
                Op::I32Store16(memarg) => f.write_fmt(format_args!("i32.store16 {0}", memarg)),
                Op::I64Store8(memarg) => f.write_fmt(format_args!("i64.store8 {0}", memarg)),
                Op::I64Store16(memarg) => f.write_fmt(format_args!("i64.store16 {0}", memarg)),
                Op::I64Store32(memarg) => f.write_fmt(format_args!("i64.store32 {0}", memarg)),
                Op::I32Const(arg) => f.write_fmt(format_args!("i32.const {0}", arg)),
                Op::I64Const(arg) => f.write_fmt(format_args!("i64.const {0}", arg)),
                Op::F32Const(arg) => f.write_fmt(format_args!("f32.const {0}", arg)),
                Op::F64Const(arg) => f.write_fmt(format_args!("f64.const {0}", arg)),
                Op::I32Eqz => f.write_fmt(format_args!("i32.eqz")),
                Op::I32Eq => f.write_fmt(format_args!("i32.eq")),
                Op::I32Ne => f.write_fmt(format_args!("i32.ne")),
                Op::I32Lts => f.write_fmt(format_args!("i32.lts")),
                Op::I32Ltu => f.write_fmt(format_args!("i32.ltu")),
                Op::I32Gts => f.write_fmt(format_args!("i32.gts")),
                Op::I32Gtu => f.write_fmt(format_args!("i32.gtu")),
                Op::I32Leu => f.write_fmt(format_args!("i32.leu")),
                Op::I32Les => f.write_fmt(format_args!("i32.les")),
                Op::I32Ges => f.write_fmt(format_args!("i32.ges")),
                Op::I32Geu => f.write_fmt(format_args!("i32.geu")),
                Op::I64Eqz => f.write_fmt(format_args!("i64.eqz")),
                Op::I64Eq => f.write_fmt(format_args!("i64.eq")),
                Op::I64Ne => f.write_fmt(format_args!("i64.ne")),
                Op::I64Lts => f.write_fmt(format_args!("i64.lts")),
                Op::I64Ltu => f.write_fmt(format_args!("i64.ltu")),
                Op::I64Gts => f.write_fmt(format_args!("i64.gts")),
                Op::I64Gtu => f.write_fmt(format_args!("i64.gtu")),
                Op::I64Les => f.write_fmt(format_args!("i64.les")),
                Op::I64Leu => f.write_fmt(format_args!("i64.leu")),
                Op::I64Ges => f.write_fmt(format_args!("i64.ges")),
                Op::I64Geu => f.write_fmt(format_args!("i64.geu")),
                Op::MemoryCopy => f.write_fmt(format_args!("memory.copy")),
                Op::MemoryFill => f.write_fmt(format_args!("memory.fill")),
                Op::I32Add => f.write_fmt(format_args!("i32.add")),
                Op::I32Sub => f.write_fmt(format_args!("i32.sub")),
                Op::I32Mul => f.write_fmt(format_args!("i32.mul")),
                Op::I32Divs => f.write_fmt(format_args!("i32.div_s")),
                Op::I32Divu => f.write_fmt(format_args!("i32.div_u")),
                Op::I32Rems => f.write_fmt(format_args!("i32.rem_s")),
                Op::I32Remu => f.write_fmt(format_args!("i32.rem_u")),
                Op::I32And => f.write_fmt(format_args!("i32.and")),
                Op::I32Or => f.write_fmt(format_args!("i32.or")),
                Op::I32Xor => f.write_fmt(format_args!("i32.xor")),
                Op::I32Shl => f.write_fmt(format_args!("i32.shl")),
                Op::I32Shrs => f.write_fmt(format_args!("i32.shrs")),
                Op::I32Shru => f.write_fmt(format_args!("i32.shru")),
                Op::I32Rotl => f.write_fmt(format_args!("i32.rotl")),
                Op::I32Rotr => f.write_fmt(format_args!("i32.rotr")),
                Op::I64Add => f.write_fmt(format_args!("i64.add")),
                Op::I64Sub => f.write_fmt(format_args!("i64.sub")),
                Op::I64Mul => f.write_fmt(format_args!("i64.mul")),
                Op::I64Divs => f.write_fmt(format_args!("i64.div_s")),
                Op::I64Divu => f.write_fmt(format_args!("i64.div_u")),
                Op::I64Rems => f.write_fmt(format_args!("i64.rem_s")),
                Op::I64Remu => f.write_fmt(format_args!("i64.rem_u")),
                Op::I64And => f.write_fmt(format_args!("i64.and")),
                Op::I64Or => f.write_fmt(format_args!(" i64.or")),
                Op::I64Xor => f.write_fmt(format_args!("i64.xor")),
                Op::I64Shl => f.write_fmt(format_args!("i64.shl")),
                Op::I64Shrs => f.write_fmt(format_args!("i64.shrs")),
                Op::I64Shru => f.write_fmt(format_args!("i64.shru")),
                Op::I64Rotl => f.write_fmt(format_args!("i64.rotl")),
                Op::I64Rotr => f.write_fmt(format_args!("i64.rotr")),
            }
        }
    }
}
pub mod reader {
    use crate::{
        leb::{Leb, LebError},
        op::Op,
    };
    use byteorder::ReadBytesExt;
    use core::fmt::{self, Display};
    use itertools::Itertools;
    use parser_derive::FromBytecode;
    use std::{
        io::{Cursor, Read, Seek, SeekFrom},
        iter::repeat,
        ops::Range,
        string::FromUtf8Error,
        usize,
    };
    use thiserror::Error;
    const TYPE_MAGIC: u8 = 0x60;
    pub enum ParserError {
        #[error("Unable to read from reader: {0}")]
        Io(#[from] std::io::Error),
        #[error(
            "Invalid LEB encoding: {0}\n
        See: https://webassembly.github.io/spec/core/binary/values.html#integers"
        )]
        Leb(#[from] LebError),
        #[error(
            "Invalid wasm header. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module"
        )]
        InvalidHeader([u8; 4]),
        #[error(
            "Invalid wasm version. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module"
        )]
        InvalidVersion([u8; 4]),
        #[error(
            "Invalid value type. 
        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/types.html#number-types"
        )]
        InvalidValueTypeId(u8),
        #[error("Invalid function type encoding. Expected: 0x60 got: {0}")]
        InvalidFunctionTypeEncoding(u8),
        #[error("Invalid bool encoding. Expected: 0x60 got: {0}")]
        InvalidBool(u8),
        #[error("Invalid blocktype encoding: Got {0}")]
        InvalidBlocktype(i64),
        #[error("Invalid limits encoding: Got {0}, expected either 0x00 or 0x01")]
        InvalidLimitsEncoding(u8),
        #[error("Invalid Import Type: Got {0}, expected 0x00, 0x01, 0x02 or 0x03")]
        InvalidImportType(u8),
        #[error("Unable to decode string: {0}")]
        InvalidUtf(#[from] FromUtf8Error),
        #[error("Invalid Export Type Encoding: Got {0}, expected 0x00, 0x01, 0x02 or 0x03")]
        InvalidExportDesc(u8),
        #[error("Invalid Data Mode Encoding: Got {0}, expected 0, 1 or 2")]
        InvalidDataMode(u32),
        #[error("Invalid section id: Got {0}, expected 0..11")]
        InvalidSectionId(u8),
        #[error("Unable to parse wat source code: {0}")]
        WatParseError(#[from] wat::Error),
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for ParserError {
        fn source(&self) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
            use ::thiserror::__private::AsDynError as _;
            #[allow(deprecated)]
            match self {
                ParserError::Io { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                ParserError::Leb { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                ParserError::InvalidHeader { .. } => ::core::option::Option::None,
                ParserError::InvalidVersion { .. } => ::core::option::Option::None,
                ParserError::InvalidValueTypeId { .. } => ::core::option::Option::None,
                ParserError::InvalidFunctionTypeEncoding { .. } => ::core::option::Option::None,
                ParserError::InvalidBool { .. } => ::core::option::Option::None,
                ParserError::InvalidBlocktype { .. } => ::core::option::Option::None,
                ParserError::InvalidLimitsEncoding { .. } => ::core::option::Option::None,
                ParserError::InvalidImportType { .. } => ::core::option::Option::None,
                ParserError::InvalidUtf { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                ParserError::InvalidExportDesc { .. } => ::core::option::Option::None,
                ParserError::InvalidDataMode { .. } => ::core::option::Option::None,
                ParserError::InvalidSectionId { .. } => ::core::option::Option::None,
                ParserError::WatParseError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for ParserError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use ::thiserror::__private::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                ParserError::Io(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("Unable to read from reader: {0}", __display0),
                                )
                        }
                    }
                }
                ParserError::Leb(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid LEB encoding: {0}\n\n        See: https://webassembly.github.io/spec/core/binary/values.html#integers",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidHeader(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid wasm header. \n        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidVersion(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid wasm version. \n        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/modules.html#binary-module",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidValueTypeId(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid value type. \n        Got: {0:?}\nSee: https://webassembly.github.io/spec/core/binary/types.html#number-types",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidFunctionTypeEncoding(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid function type encoding. Expected: 0x60 got: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidBool(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid bool encoding. Expected: 0x60 got: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidBlocktype(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid blocktype encoding: Got {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidLimitsEncoding(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid limits encoding: Got {0}, expected either 0x00 or 0x01",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidImportType(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid Import Type: Got {0}, expected 0x00, 0x01, 0x02 or 0x03",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidUtf(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("Unable to decode string: {0}", __display0),
                                )
                        }
                    }
                }
                ParserError::InvalidExportDesc(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid Export Type Encoding: Got {0}, expected 0x00, 0x01, 0x02 or 0x03",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidDataMode(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid Data Mode Encoding: Got {0}, expected 0, 1 or 2",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::InvalidSectionId(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid section id: Got {0}, expected 0..11",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                ParserError::WatParseError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Unable to parse wat source code: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
            }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes
    )]
    #[automatically_derived]
    impl ::core::convert::From<std::io::Error> for ParserError {
        fn from(source: std::io::Error) -> Self {
            ParserError::Io { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes
    )]
    #[automatically_derived]
    impl ::core::convert::From<LebError> for ParserError {
        fn from(source: LebError) -> Self {
            ParserError::Leb { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes
    )]
    #[automatically_derived]
    impl ::core::convert::From<FromUtf8Error> for ParserError {
        fn from(source: FromUtf8Error) -> Self {
            ParserError::InvalidUtf { 0: source }
        }
    }
    #[allow(
        deprecated,
        unused_qualifications,
        clippy::elidable_lifetime_names,
        clippy::needless_lifetimes
    )]
    #[automatically_derived]
    impl ::core::convert::From<wat::Error> for ParserError {
        fn from(source: wat::Error) -> Self {
            ParserError::WatParseError { 0: source }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParserError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ParserError::Io(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
                }
                ParserError::Leb(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Leb", &__self_0)
                }
                ParserError::InvalidHeader(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "InvalidHeader", &__self_0)
                }
                ParserError::InvalidVersion(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidVersion",
                        &__self_0,
                    )
                }
                ParserError::InvalidValueTypeId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidValueTypeId",
                        &__self_0,
                    )
                }
                ParserError::InvalidFunctionTypeEncoding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidFunctionTypeEncoding",
                        &__self_0,
                    )
                }
                ParserError::InvalidBool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "InvalidBool", &__self_0)
                }
                ParserError::InvalidBlocktype(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidBlocktype",
                        &__self_0,
                    )
                }
                ParserError::InvalidLimitsEncoding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidLimitsEncoding",
                        &__self_0,
                    )
                }
                ParserError::InvalidImportType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidImportType",
                        &__self_0,
                    )
                }
                ParserError::InvalidUtf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "InvalidUtf", &__self_0)
                }
                ParserError::InvalidExportDesc(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidExportDesc",
                        &__self_0,
                    )
                }
                ParserError::InvalidDataMode(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidDataMode",
                        &__self_0,
                    )
                }
                ParserError::InvalidSectionId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidSectionId",
                        &__self_0,
                    )
                }
                ParserError::WatParseError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "WatParseError", &__self_0)
                }
            }
        }
    }
    impl ParserError {
        pub fn is_eof(&self) -> bool {
            if let Self::Io(e) = self {
                e.kind() == std::io::ErrorKind::UnexpectedEof
            } else {
                false
            }
        }
    }
    pub trait BytecodeReader: Read + Seek + Sized {
        fn parse<T: FromBytecode>(&mut self) -> Result<T, ParserError> {
            T::from_reader(self)
        }
    }
    impl<T: Read + Seek> BytecodeReader for T {}
    pub trait FromBytecode: Sized {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError>;
    }
    impl<T: FromBytecode> FromBytecode for Vec<T> {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            parse_vec(reader)
        }
    }
    impl FromBytecode for u32 {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Leb::read_u32(reader)?)
        }
    }
    impl FromBytecode for u64 {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Leb::read_u64(reader)?)
        }
    }
    impl FromBytecode for usize {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Leb::read_u32(reader)? as usize)
        }
    }
    impl FromBytecode for bool {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            match reader.read_u8()? {
                0 => Ok(false),
                1 => Ok(true),
                num => Err(ParserError::InvalidBool(num)),
            }
        }
    }
    pub struct WithPosition<T> {
        pub data: T,
        pub position: Range<usize>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for WithPosition<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "WithPosition",
                "data",
                &self.data,
                "position",
                &&self.position,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::default::Default> ::core::default::Default for WithPosition<T> {
        #[inline]
        fn default() -> WithPosition<T> {
            WithPosition {
                data: ::core::default::Default::default(),
                position: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for WithPosition<T> {
        #[inline]
        fn clone(&self) -> WithPosition<T> {
            WithPosition {
                data: ::core::clone::Clone::clone(&self.data),
                position: ::core::clone::Clone::clone(&self.position),
            }
        }
    }
    #[automatically_derived]
    impl<T> ::core::marker::StructuralPartialEq for WithPosition<T> {}
    #[automatically_derived]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for WithPosition<T> {
        #[inline]
        fn eq(&self, other: &WithPosition<T>) -> bool {
            self.data == other.data && self.position == other.position
        }
    }
    impl<T> WithPosition<T> {
        pub fn new(data: T, position: Range<usize>) -> Self {
            Self { data, position }
        }
        pub fn as_ref(&self) -> WithPosition<&T> {
            WithPosition::new(&self.data, self.position.clone())
        }
    }
    impl<T: FromBytecode> FromBytecode for WithPosition<T> {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            parse_with_pos(reader)
        }
    }
    impl<T: FromBytecode> From<(T, Range<usize>)> for WithPosition<T> {
        fn from(value: (T, Range<usize>)) -> Self {
            WithPosition::new(value.0, value.1)
        }
    }
    pub fn read_with_pos<R: Read + Seek, T: Sized, F>(
        reader: &mut R,
        read_op: F,
    ) -> Result<WithPosition<T>, ParserError>
    where
        F: FnOnce(&mut R) -> T,
    {
        let start = reader.seek(SeekFrom::Current(0))? as usize;
        let data = read_op(reader);
        let end = reader.seek(SeekFrom::Current(0))? as usize;
        let range = start..end - start;
        Ok(WithPosition::new(data, range))
    }
    pub fn try_read_with_pos<R: Read + Seek, T: Sized, F>(
        reader: &mut R,
        read_op: F,
    ) -> Result<WithPosition<T>, ParserError>
    where
        F: FnOnce(&mut R) -> Result<T, ParserError>,
    {
        let start = reader.seek(SeekFrom::Current(0))? as usize;
        let data = read_op(reader)?;
        let end = reader.seek(SeekFrom::Current(0))? as usize;
        let range = start..end - start;
        Ok(WithPosition::new(data, range))
    }
    pub fn parse_with_pos<R: BytecodeReader, T: FromBytecode>(
        reader: &mut R,
    ) -> Result<WithPosition<T>, ParserError> {
        try_read_with_pos(reader, |r| r.parse())
    }
    pub fn iter_vec<R: BytecodeReader, T: FromBytecode>(
        reader: &mut R,
    ) -> Result<impl Iterator<Item = Result<T, ParserError>>, ParserError> {
        let count = Leb::read_u32(reader)?;
        Ok((0..count).map(|_| reader.parse::<T>()))
    }
    pub fn iter_vec_with_pos<R: BytecodeReader, T: FromBytecode>(
        reader: &mut R,
    ) -> Result<impl Iterator<Item = Result<WithPosition<T>, ParserError>>, ParserError> {
        let count = Leb::read_u32(reader)?;
        Ok((0..count).map(|_| parse_with_pos(reader)))
    }
    pub fn parse_vec<R: BytecodeReader, T: FromBytecode>(
        reader: &mut R,
    ) -> Result<Vec<T>, ParserError> {
        iter_vec(reader)?.collect()
    }
    pub fn parse_vec_pos<R: BytecodeReader, T: FromBytecode>(
        reader: &mut R,
    ) -> Result<WithPosition<Vec<WithPosition<T>>>, ParserError> {
        let start = reader.seek(SeekFrom::Current(0))? as usize;
        let data = iter_vec_with_pos(reader)?.collect::<Result<Vec<WithPosition<T>>, _>>()?;
        let end = reader.seek(SeekFrom::Current(0))? as usize;
        let range = start..end - start;
        Ok(WithPosition::new(data, range))
    }
    pub fn parse_string<R: BytecodeReader>(reader: &mut R) -> Result<String, ParserError> {
        let len = reader.parse::<usize>()?;
        let mut buffer = ::alloc::vec::from_elem(0, len);
        reader.read_exact(&mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
    pub fn parse_data_with_pos<R: BytecodeReader>(
        reader: &mut R,
    ) -> Result<WithPosition<Vec<u8>>, ParserError> {
        try_read_with_pos(reader, |r| {
            let data_size: usize = r.parse()?;
            let mut buffer = ::alloc::vec::from_elem(0, data_size);
            r.read_exact(&mut buffer)?;
            Ok(buffer)
        })
    }
    pub fn iter_const_expr<R: BytecodeReader>(
        reader: &mut R,
    ) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>> {
        repeat(0)
            .map(|_| reader.parse::<WithPosition<Op>>())
            .take_while(|op| op.as_ref().is_ok_and(|op| !op.data.is_terminator()) || op.is_err())
    }
    pub fn iter_expr<R: BytecodeReader>(
        reader: &mut R,
    ) -> impl Iterator<Item = Result<WithPosition<Op>, ParserError>> {
        (0..)
            .map(|_| reader.parse::<WithPosition<Op>>())
            .scan(0, |depth, op| {
                let cont = op.as_ref().is_ok_and(|op| {
                    let (new_depth, should_terminate) = op.data.continues(*depth);
                    *depth = new_depth;
                    should_terminate
                });
                if cont || op.is_err() { Some(op) } else { None }
            })
    }
    impl FromBytecode for String {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(parse_string(reader)?)
        }
    }
    pub struct Header {
        header: Range<usize>,
        version: Range<usize>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Header {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Header",
                "header",
                &self.header,
                "version",
                &&self.version,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Header {
        #[inline]
        fn clone(&self) -> Header {
            Header {
                header: ::core::clone::Clone::clone(&self.header),
                version: ::core::clone::Clone::clone(&self.version),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Header {
        #[inline]
        fn default() -> Header {
            Header {
                header: ::core::default::Default::default(),
                version: ::core::default::Default::default(),
            }
        }
    }
    pub const WASM_HEADER_MAGIC: &[u8; 4] = b"\0asm";
    pub const WASM_HEADER_VERSION: &[u8; 4] = &[1, 0, 0, 0];
    impl FromBytecode for Header {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let header = try_read_with_pos(reader, |reader| {
                let mut header: [u8; 4] = [0; 4];
                reader.read_exact(&mut header)?;
                Ok(header)
            })?;
            if header.data != *WASM_HEADER_MAGIC {
                return Err(ParserError::InvalidHeader(header.data));
            }
            let version = try_read_with_pos(reader, |reader| {
                let mut version: [u8; 4] = [0; 4];
                reader.read_exact(&mut version)?;
                if version != *WASM_HEADER_VERSION {
                    Err(ParserError::InvalidVersion(version))
                } else {
                    Ok(version)
                }
            })?;
            Ok(Header {
                header: header.position,
                version: version.position,
            })
        }
    }
    pub fn read_wasm_header(reader: &mut impl BytecodeReader) -> Result<Header, ParserError> {
        reader.parse()
    }
    #[repr(u8)]
    pub enum ValueType {
        I32 = 0x7F,
        I64 = 0x7E,
        F32 = 0x7D,
        F64 = 0x7C,
        Funcref = 0x70,
        Externref = 0x6F,
        Vectype = 0x7B,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ValueType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ValueType::I32 => "I32",
                    ValueType::I64 => "I64",
                    ValueType::F32 => "F32",
                    ValueType::F64 => "F64",
                    ValueType::Funcref => "Funcref",
                    ValueType::Externref => "Externref",
                    ValueType::Vectype => "Vectype",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ValueType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ValueType {
        #[inline]
        fn eq(&self, other: &ValueType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ValueType {
        #[inline]
        fn partial_cmp(&self, other: &ValueType) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ValueType {}
    #[automatically_derived]
    impl ::core::clone::Clone for ValueType {
        #[inline]
        fn clone(&self) -> ValueType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ValueType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl Display for ValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = match self {
                ValueType::I32 => "i32",
                ValueType::I64 => "i64",
                ValueType::F32 => "f32",
                ValueType::F64 => "f64",
                ValueType::Funcref => "funcref",
                ValueType::Externref => "externref",
                ValueType::Vectype => "vec",
            };
            f.write_fmt(format_args!("{0}", str))
        }
    }
    impl std::convert::TryFrom<u8> for ValueType {
        type Error = ParserError;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0x7F => Ok(Self::I32),
                0x7E => Ok(Self::I64),
                0x7D => Ok(Self::F32),
                0x7C => Ok(Self::F64),
                0x70 => Ok(Self::Funcref),
                0x6F => Ok(Self::Externref),
                0x7B => Ok(Self::Vectype),
                _ => Err(ParserError::InvalidValueTypeId(value)),
            }
        }
    }
    impl std::convert::TryFrom<i8> for ValueType {
        type Error = ParserError;
        fn try_from(value: i8) -> std::result::Result<Self, Self::Error> {
            let value = value as u8;
            value.try_into()
        }
    }
    impl ValueType {
        pub fn is_num(&self) -> bool {
            match self {
                ValueType::I32 | ValueType::I64 | ValueType::F32 | ValueType::F64 => true,
                _ => false,
            }
        }
        pub fn is_vec(&self) -> bool {
            match self {
                ValueType::Vectype => true,
                _ => false,
            }
        }
        pub fn is_ref(&self) -> bool {
            match self {
                ValueType::Funcref | ValueType::Externref => true,
                _ => false,
            }
        }
        pub fn bit_width(&self) -> Option<usize> {
            match self {
                ValueType::I32 => Some(32),
                ValueType::I64 => Some(64),
                ValueType::F32 => Some(32),
                ValueType::F64 => Some(64),
                ValueType::Funcref => None,
                ValueType::Externref => None,
                ValueType::Vectype => Some(128),
            }
        }
    }
    impl FromBytecode for ValueType {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            reader.read_u8()?.try_into()
        }
    }
    pub struct Type {
        pub params: WithPosition<Vec<WithPosition<ValueType>>>,
        pub results: WithPosition<Vec<WithPosition<ValueType>>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Type",
                "params",
                &self.params,
                "results",
                &&self.results,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Type {
        #[inline]
        fn default() -> Type {
            Type {
                params: ::core::default::Default::default(),
                results: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Type {
        #[inline]
        fn clone(&self) -> Type {
            Type {
                params: ::core::clone::Clone::clone(&self.params),
                results: ::core::clone::Clone::clone(&self.results),
            }
        }
    }
    impl FromBytecode for Type {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let magic = reader.read_u8()?;
            if magic != TYPE_MAGIC {
                Err(ParserError::InvalidFunctionTypeEncoding(magic))
            } else {
                Ok(Self {
                    params: reader.parse()?,
                    results: reader.parse()?,
                })
            }
        }
    }
    impl Type {
        pub fn iter_params(&self) -> impl Iterator<Item = &ValueType> {
            self.params.data.iter().map(|v| &v.data)
        }
        pub fn iter_results(&self) -> impl Iterator<Item = &ValueType> {
            self.results.data.iter().map(|v| &v.data)
        }
        pub fn get_param(&self, id: usize) -> Option<ValueType> {
            self.params.data.get(id).map(|id| id.data)
        }
        pub fn get_result(&self, id: usize) -> Option<ValueType> {
            self.results.data.get(id).map(|id| id.data)
        }
    }
    impl Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let p = self.iter_params();
            let r = self.iter_results();
            f.write_fmt(format_args!(
                "({0}) -> ({1})",
                p.format(", "),
                r.format(", ")
            ))
        }
    }
    pub struct GlobalType {
        pub t: WithPosition<ValueType>,
        pub mutable: WithPosition<bool>,
    }
    impl FromBytecode for GlobalType {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                t: FromBytecode::parse(reader)?,
                mutable: FromBytecode::parse(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GlobalType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "GlobalType",
                "t",
                &self.t,
                "mutable",
                &&self.mutable,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GlobalType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GlobalType {
        #[inline]
        fn eq(&self, other: &GlobalType) -> bool {
            self.t == other.t && self.mutable == other.mutable
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GlobalType {
        #[inline]
        fn clone(&self) -> GlobalType {
            GlobalType {
                t: ::core::clone::Clone::clone(&self.t),
                mutable: ::core::clone::Clone::clone(&self.mutable),
            }
        }
    }
    impl GlobalType {
        pub fn is_mut(&self) -> bool {
            self.mutable.data
        }
    }
    impl Display for GlobalType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut_str = if self.is_mut() { "mut" } else { "" };
            f.write_fmt(format_args!("{0} {1}", mut_str, self.t.data))
        }
    }
    pub struct ConstExpr {
        expr: Vec<WithPosition<Op>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConstExpr {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "ConstExpr", "expr", &&self.expr)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ConstExpr {
        #[inline]
        fn clone(&self) -> ConstExpr {
            ConstExpr {
                expr: ::core::clone::Clone::clone(&self.expr),
            }
        }
    }
    impl FromBytecode for ConstExpr {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(ConstExpr {
                expr: iter_const_expr(reader).collect::<Result<Vec<_>, _>>()?,
            })
        }
    }
    pub struct Global {
        pub t: WithPosition<GlobalType>,
        pub init_expr: WithPosition<ConstExpr>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Global {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Global",
                "t",
                &self.t,
                "init_expr",
                &&self.init_expr,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Global {
        #[inline]
        fn clone(&self) -> Global {
            Global {
                t: ::core::clone::Clone::clone(&self.t),
                init_expr: ::core::clone::Clone::clone(&self.init_expr),
            }
        }
    }
    impl FromBytecode for Global {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                t: reader.parse()?,
                init_expr: reader.parse()?,
            })
        }
    }
    impl Global {
        pub fn value_type(&self) -> ValueType {
            self.t.data.t.data
        }
        pub fn iter_init_expr(&self) -> impl Iterator<Item = &Op> {
            self.init_expr.data.expr.iter().map(|v| &v.data)
        }
    }
    impl Display for Global {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "{0} = {1}",
                self.t.data,
                self.iter_init_expr().format(" ,"),
            ))
        }
    }
    pub struct Limits {
        pub min: WithPosition<u32>,
        pub max: Option<WithPosition<u32>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Limits {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "Limits", "min", &self.min, "max", &&self.max,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Limits {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Limits {
        #[inline]
        fn eq(&self, other: &Limits) -> bool {
            self.min == other.min && self.max == other.max
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Limits {
        #[inline]
        fn clone(&self) -> Limits {
            Limits {
                min: ::core::clone::Clone::clone(&self.min),
                max: ::core::clone::Clone::clone(&self.max),
            }
        }
    }
    impl Limits {
        pub fn in_range(&self, i: i32) -> bool {
            if self.min.data as i32 > i {
                return false;
            }
            if let Some(WithPosition {
                data: max,
                position: _,
            }) = &self.max
            {
                if i > *max as i32 || *max < self.min.data {
                    return false;
                }
            }
            true
        }
    }
    impl FromBytecode for Limits {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            match reader.read_u8()? {
                0x00 => Ok(Self {
                    min: reader.parse()?,
                    max: None,
                }),
                0x01 => Ok(Self {
                    min: reader.parse()?,
                    max: Some(reader.parse()?),
                }),
                num => Err(ParserError::InvalidLimitsEncoding(num)),
            }
        }
    }
    impl Display for Limits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.max {
                Some(WithPosition {
                    data: m,
                    position: _,
                }) => f.write_fmt(format_args!("({0}..{1})", self.min.data, m)),
                None => f.write_fmt(format_args!("({0}..)", self.min.data)),
            }
        }
    }
    pub enum ImportDesc {
        TypeIdx(usize),
        TableType(Limits),
        MemType(Limits),
        GlobalType(GlobalType),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ImportDesc {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ImportDesc::TypeIdx(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TypeIdx", &__self_0)
                }
                ImportDesc::TableType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TableType", &__self_0)
                }
                ImportDesc::MemType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MemType", &__self_0)
                }
                ImportDesc::GlobalType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "GlobalType", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ImportDesc {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ImportDesc {
        #[inline]
        fn eq(&self, other: &ImportDesc) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ImportDesc::TypeIdx(__self_0), ImportDesc::TypeIdx(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ImportDesc::TableType(__self_0), ImportDesc::TableType(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ImportDesc::MemType(__self_0), ImportDesc::MemType(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ImportDesc::GlobalType(__self_0), ImportDesc::GlobalType(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ImportDesc {
        #[inline]
        fn clone(&self) -> ImportDesc {
            match self {
                ImportDesc::TypeIdx(__self_0) => {
                    ImportDesc::TypeIdx(::core::clone::Clone::clone(__self_0))
                }
                ImportDesc::TableType(__self_0) => {
                    ImportDesc::TableType(::core::clone::Clone::clone(__self_0))
                }
                ImportDesc::MemType(__self_0) => {
                    ImportDesc::MemType(::core::clone::Clone::clone(__self_0))
                }
                ImportDesc::GlobalType(__self_0) => {
                    ImportDesc::GlobalType(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl FromBytecode for ImportDesc {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let id = reader.read_u8()?;
            match id {
                0x00 => Ok(Self::TypeIdx(reader.parse()?)),
                0x01 => Ok(Self::TableType(reader.parse()?)),
                0x02 => Ok(Self::MemType(reader.parse()?)),
                0x03 => Ok(Self::GlobalType(reader.parse()?)),
                _ => Err(ParserError::InvalidImportType(id)),
            }
        }
    }
    impl Display for ImportDesc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ImportDesc::TypeIdx(i) => f.write_fmt(format_args!("{0}", i)),
                ImportDesc::TableType(limits) => f.write_fmt(format_args!("table {0}", limits)),
                ImportDesc::MemType(limits) => f.write_fmt(format_args!("mem {0}", limits)),
                ImportDesc::GlobalType(global_type) => {
                    f.write_fmt(format_args!("{0}", global_type))
                }
            }
        }
    }
    pub struct ImportIdent {
        pub module: WithPosition<String>,
        pub name: WithPosition<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ImportIdent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ImportIdent",
                "module",
                &self.module,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ImportIdent {
        #[inline]
        fn clone(&self) -> ImportIdent {
            ImportIdent {
                module: ::core::clone::Clone::clone(&self.module),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    impl Display for ImportIdent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "module: {0}, name: {1}",
                self.module.data, self.name.data
            ))
        }
    }
    impl FromBytecode for ImportIdent {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                module: reader.parse()?,
                name: reader.parse()?,
            })
        }
    }
    pub struct Import {
        pub ident: WithPosition<ImportIdent>,
        pub desc: WithPosition<ImportDesc>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Import {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Import",
                "ident",
                &self.ident,
                "desc",
                &&self.desc,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Import {
        #[inline]
        fn clone(&self) -> Import {
            Import {
                ident: ::core::clone::Clone::clone(&self.ident),
                desc: ::core::clone::Clone::clone(&self.desc),
            }
        }
    }
    impl FromBytecode for Import {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                ident: reader.parse()?,
                desc: reader.parse()?,
            })
        }
    }
    impl Display for Import {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("({0}): {1}", self.ident.data, self.desc.data))
        }
    }
    pub struct Locals {
        pub n: u32,
        pub t: ValueType,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Locals {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "Locals", "n", &self.n, "t", &&self.t,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Locals {
        #[inline]
        fn clone(&self) -> Locals {
            Locals {
                n: ::core::clone::Clone::clone(&self.n),
                t: ::core::clone::Clone::clone(&self.t),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Locals {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Locals {
        #[inline]
        fn eq(&self, other: &Locals) -> bool {
            self.n == other.n && self.t == other.t
        }
    }
    impl FromBytecode for Locals {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                n: reader.parse()?,
                t: reader.parse()?,
            })
        }
    }
    impl Locals {
        pub fn flat_iter(&self) -> impl Iterator<Item = ValueType> {
            (0..self.n).map(|_| self.t)
        }
    }
    impl Display for Locals {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.flat_iter()
                .try_for_each(|v| f.write_fmt(format_args!("{0}\n", v)))
        }
    }
    pub enum ExportDesc {
        FuncId(usize),
        TableId(usize),
        MemId(usize),
        GlobalId(usize),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ExportDesc {
        #[inline]
        fn clone(&self) -> ExportDesc {
            match self {
                ExportDesc::FuncId(__self_0) => {
                    ExportDesc::FuncId(::core::clone::Clone::clone(__self_0))
                }
                ExportDesc::TableId(__self_0) => {
                    ExportDesc::TableId(::core::clone::Clone::clone(__self_0))
                }
                ExportDesc::MemId(__self_0) => {
                    ExportDesc::MemId(::core::clone::Clone::clone(__self_0))
                }
                ExportDesc::GlobalId(__self_0) => {
                    ExportDesc::GlobalId(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExportDesc {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ExportDesc::FuncId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "FuncId", &__self_0)
                }
                ExportDesc::TableId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TableId", &__self_0)
                }
                ExportDesc::MemId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MemId", &__self_0)
                }
                ExportDesc::GlobalId(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "GlobalId", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ExportDesc {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ExportDesc {
        #[inline]
        fn eq(&self, other: &ExportDesc) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (ExportDesc::FuncId(__self_0), ExportDesc::FuncId(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ExportDesc::TableId(__self_0), ExportDesc::TableId(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ExportDesc::MemId(__self_0), ExportDesc::MemId(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (ExportDesc::GlobalId(__self_0), ExportDesc::GlobalId(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    impl ExportDesc {
        pub fn new(export_type: u8, id: usize) -> Result<Self, ParserError> {
            match export_type {
                0x00 => Ok(Self::FuncId(id)),
                0x01 => Ok(Self::TableId(id)),
                0x02 => Ok(Self::MemId(id)),
                0x03 => Ok(Self::GlobalId(id)),
                _ => Err(ParserError::InvalidExportDesc(export_type)),
            }
        }
    }
    impl FromBytecode for ExportDesc {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let t = reader.read_u8()?;
            let id = reader.parse::<usize>()?;
            Self::new(t, id)
        }
    }
    impl Display for ExportDesc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ExportDesc::FuncId(id) => f.write_fmt(format_args!("func id: {0}", id)),
                ExportDesc::TableId(id) => f.write_fmt(format_args!("table id: {0}", id)),
                ExportDesc::MemId(id) => f.write_fmt(format_args!("mem id {0}", id)),
                ExportDesc::GlobalId(id) => f.write_fmt(format_args!("global id {0}", id)),
            }
        }
    }
    pub struct Export {
        pub name: WithPosition<String>,
        pub desc: WithPosition<ExportDesc>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Export {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Export",
                "name",
                &self.name,
                "desc",
                &&self.desc,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Export {
        #[inline]
        fn clone(&self) -> Export {
            Export {
                name: ::core::clone::Clone::clone(&self.name),
                desc: ::core::clone::Clone::clone(&self.desc),
            }
        }
    }
    impl FromBytecode for Export {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                name: reader.parse()?,
                desc: reader.parse()?,
            })
        }
    }
    impl fmt::Display for Export {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0}: {1}", self.name.data, self.desc.data))
        }
    }
    pub enum Data {
        Active {
            mem_id: usize,
            expr: WithPosition<Vec<WithPosition<Op>>>,
            data: WithPosition<Vec<u8>>,
        },
        Passive(WithPosition<Vec<u8>>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Data {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Data::Active {
                    mem_id: __self_0,
                    expr: __self_1,
                    data: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f, "Active", "mem_id", __self_0, "expr", __self_1, "data", &__self_2,
                ),
                Data::Passive(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Passive", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Data {
        #[inline]
        fn clone(&self) -> Data {
            match self {
                Data::Active {
                    mem_id: __self_0,
                    expr: __self_1,
                    data: __self_2,
                } => Data::Active {
                    mem_id: ::core::clone::Clone::clone(__self_0),
                    expr: ::core::clone::Clone::clone(__self_1),
                    data: ::core::clone::Clone::clone(__self_2),
                },
                Data::Passive(__self_0) => Data::Passive(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl Data {
        fn parse_active<R: BytecodeReader>(
            reader: &mut R,
            mem_id: usize,
        ) -> Result<Data, ParserError> {
            let expr = try_read_with_pos(reader, |r| {
                iter_const_expr(r).collect::<Result<Vec<_>, _>>()
            })?;
            let buffer = parse_data_with_pos(reader)?;
            Ok(Self::Active {
                mem_id,
                expr,
                data: buffer,
            })
        }
    }
    impl FromBytecode for Data {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            match reader.parse::<u32>()? {
                0 => Data::parse_active(reader, 0),
                1 => Ok(Self::Passive(parse_data_with_pos(reader)?)),
                2 => {
                    let id: usize = reader.parse()?;
                    Data::parse_active(reader, id)
                }
                n => Err(ParserError::InvalidDataMode(n)),
            }
        }
    }
    pub struct Expression {
        data: Vec<WithPosition<Op>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Expression", "data", &&self.data)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Expression {
        #[inline]
        fn clone(&self) -> Expression {
            Expression {
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    impl Display for Expression {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "{0}",
                self.data.iter().map(|op| op.data).format("\n")
            ))
        }
    }
    impl FromBytecode for Expression {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            {
                ::std::io::_print(format_args!("Reading expression...\n"));
            };
            Ok(Self {
                data: iter_expr(reader).collect::<Result<Vec<_>, _>>()?,
            })
        }
    }
    pub struct Function {
        pub size: usize,
        pub locals: WithPosition<Vec<WithPosition<Locals>>>,
        pub code: WithPosition<Expression>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Function {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Function",
                "size",
                &self.size,
                "locals",
                &self.locals,
                "code",
                &&self.code,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Function {
        #[inline]
        fn clone(&self) -> Function {
            Function {
                size: ::core::clone::Clone::clone(&self.size),
                locals: ::core::clone::Clone::clone(&self.locals),
                code: ::core::clone::Clone::clone(&self.code),
            }
        }
    }
    impl FromBytecode for Function {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            Ok(Self {
                size: reader.parse()?,
                locals: reader.parse()?,
                code: reader.parse()?,
            })
        }
    }
    impl Display for Function {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("Locals\n"))?;
            self.locals
                .data
                .iter()
                .try_for_each(|l| f.write_fmt(format_args!("{0}\n", l.data)))?;
            f.write_fmt(format_args!("Code:\n {0}\n", self.code.data))
        }
    }
    impl Function {
        pub fn iter_locals(&self) -> impl Iterator<Item = ValueType> {
            self.locals
                .data
                .iter()
                .map(|l| l.data.flat_iter())
                .flatten()
        }
        pub fn get_local(&self, id: usize) -> Option<ValueType> {
            let mut locals = self.iter_locals();
            locals.nth(id)
        }
        pub fn get_op_mut(&mut self, index: usize) -> Option<&mut Op> {
            self.code.data.data.get_mut(index).map(|op| &mut op.data)
        }
    }
    pub struct CustomSection {
        pub name: WithPosition<String>,
        pub data: WithPosition<Vec<u8>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CustomSection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CustomSection",
                "name",
                &self.name,
                "data",
                &&self.data,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CustomSection {
        #[inline]
        fn clone(&self) -> CustomSection {
            CustomSection {
                name: ::core::clone::Clone::clone(&self.name),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    impl CustomSection {
        pub fn init(
            reader: &mut impl BytecodeReader,
            section_size: usize,
        ) -> Result<Self, ParserError> {
            let name: WithPosition<String> = reader.parse()?;
            let data_size = section_size - name.position.clone().count();
            let data = try_read_with_pos(reader, |r| {
                let mut buffer = ::alloc::vec::from_elem(0, data_size);
                r.read_exact(&mut buffer)?;
                Ok(buffer)
            })?;
            Ok(Self { name, data })
        }
    }
    pub enum SectionId {
        Type = 1,
        Import = 2,
        Function = 3,
        Table = 4,
        Memory = 5,
        Global = 6,
        Export = 7,
        Start = 8,
        DataCount = 9,
        Code = 10,
        Data = 11,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SectionId {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    SectionId::Type => "Type",
                    SectionId::Import => "Import",
                    SectionId::Function => "Function",
                    SectionId::Table => "Table",
                    SectionId::Memory => "Memory",
                    SectionId::Global => "Global",
                    SectionId::Export => "Export",
                    SectionId::Start => "Start",
                    SectionId::DataCount => "DataCount",
                    SectionId::Code => "Code",
                    SectionId::Data => "Data",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SectionId {}
    #[automatically_derived]
    impl ::core::clone::Clone for SectionId {
        #[inline]
        fn clone(&self) -> SectionId {
            *self
        }
    }
    pub type Types = Vec<WithPosition<Type>>;
    pub type Imports = Vec<WithPosition<Import>>;
    pub type Functions = Vec<WithPosition<usize>>;
    pub type Tables = Vec<WithPosition<Limits>>;
    pub type Memories = Vec<WithPosition<Limits>>;
    pub type Globals = Vec<WithPosition<Global>>;
    pub type Exports = Vec<WithPosition<Export>>;
    pub type Start = u32;
    pub type DataCount = u32;
    pub type Code = Vec<WithPosition<Function>>;
    pub type ModuleData = Vec<WithPosition<Data>>;
    pub enum SectionData {
        Type(Types),
        Import(Imports),
        Function(Functions),
        Table(Tables),
        Memory(Memories),
        Global(Globals),
        Export(Exports),
        Start(Start),
        DataCount(DataCount),
        Code(Code),
        Data(ModuleData),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SectionData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SectionData::Type(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Type", &__self_0)
                }
                SectionData::Import(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Import", &__self_0)
                }
                SectionData::Function(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Function", &__self_0)
                }
                SectionData::Table(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Table", &__self_0)
                }
                SectionData::Memory(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Memory", &__self_0)
                }
                SectionData::Global(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Global", &__self_0)
                }
                SectionData::Export(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Export", &__self_0)
                }
                SectionData::Start(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Start", &__self_0)
                }
                SectionData::DataCount(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DataCount", &__self_0)
                }
                SectionData::Code(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Code", &__self_0)
                }
                SectionData::Data(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Data", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SectionData {
        #[inline]
        fn clone(&self) -> SectionData {
            match self {
                SectionData::Type(__self_0) => {
                    SectionData::Type(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Import(__self_0) => {
                    SectionData::Import(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Function(__self_0) => {
                    SectionData::Function(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Table(__self_0) => {
                    SectionData::Table(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Memory(__self_0) => {
                    SectionData::Memory(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Global(__self_0) => {
                    SectionData::Global(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Export(__self_0) => {
                    SectionData::Export(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Start(__self_0) => {
                    SectionData::Start(::core::clone::Clone::clone(__self_0))
                }
                SectionData::DataCount(__self_0) => {
                    SectionData::DataCount(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Code(__self_0) => {
                    SectionData::Code(::core::clone::Clone::clone(__self_0))
                }
                SectionData::Data(__self_0) => {
                    SectionData::Data(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl SectionData {
        pub fn init<R: BytecodeReader>(reader: &mut R, id: u8) -> Result<Self, ParserError> {
            match id {
                0x01 => Ok(Self::Type(reader.parse()?)),
                0x02 => Ok(Self::Import(reader.parse()?)),
                0x03 => Ok(Self::Function(reader.parse()?)),
                0x04 => Ok(Self::Table(reader.parse()?)),
                0x05 => Ok(Self::Memory(reader.parse()?)),
                0x06 => Ok(Self::Global(reader.parse()?)),
                0x07 => Ok(Self::Export(reader.parse()?)),
                0x08 => Ok(Self::Start(reader.parse()?)),
                0x09 => Ok(Self::DataCount(reader.parse()?)),
                0x0A => Ok(Self::Code(reader.parse()?)),
                0x0B => Ok(Self::Data(reader.parse()?)),
                num => Err(ParserError::InvalidSectionId(num)),
            }
        }
    }
    pub struct Section {
        pub id: u8,
        pub size: usize,
        pub data: WithPosition<SectionDataOrCustom>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Section {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Section",
                "id",
                &self.id,
                "size",
                &self.size,
                "data",
                &&self.data,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Section {
        #[inline]
        fn clone(&self) -> Section {
            Section {
                id: ::core::clone::Clone::clone(&self.id),
                size: ::core::clone::Clone::clone(&self.size),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    impl Section {
        pub fn new_section(id: u8, size: usize, data: WithPosition<SectionData>) -> Self {
            Self {
                id,
                size,
                data: WithPosition::new(SectionDataOrCustom::Section(data.data), data.position),
            }
        }
        pub fn new_custom(
            reader: &mut impl BytecodeReader,
            id: u8,
            size: usize,
        ) -> Result<Self, ParserError> {
            let section = try_read_with_pos(reader, |r| CustomSection::init(r, size))?;
            Ok(Self {
                id,
                size,
                data: WithPosition::new(
                    SectionDataOrCustom::Custom(section.data),
                    section.position,
                ),
            })
        }
    }
    pub enum SectionDataOrCustom {
        Section(SectionData),
        Custom(CustomSection),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SectionDataOrCustom {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SectionDataOrCustom::Section(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Section", &__self_0)
                }
                SectionDataOrCustom::Custom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Custom", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SectionDataOrCustom {
        #[inline]
        fn clone(&self) -> SectionDataOrCustom {
            match self {
                SectionDataOrCustom::Section(__self_0) => {
                    SectionDataOrCustom::Section(::core::clone::Clone::clone(__self_0))
                }
                SectionDataOrCustom::Custom(__self_0) => {
                    SectionDataOrCustom::Custom(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl FromBytecode for Section {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let id = reader.read_u8()?;
            let size = reader.parse::<usize>()?;
            match id {
                0x00 => Section::new_custom(reader, id, size),
                _ => {
                    let data = try_read_with_pos(reader, |r| SectionData::init(r, id))?;
                    Ok(Section::new_section(id, size, data))
                }
            }
        }
    }
    pub fn parse_until_eof<T: FromBytecode>(
        reader: &mut impl BytecodeReader,
    ) -> impl Iterator<Item = Result<T, ParserError>> {
        (0..)
            .map(|_| reader.parse())
            .take_while(|op| op.as_ref().is_err_and(|e| !e.is_eof()) || op.is_ok())
    }
    pub fn iter_sections(
        reader: &mut impl BytecodeReader,
    ) -> impl Iterator<Item = Result<WithPosition<Section>, ParserError>> {
        parse_until_eof(reader)
    }
    pub struct SortedImports {
        pub functions: Vec<(usize, usize)>,
        pub tables: Vec<(usize, Limits)>,
        pub mems: Vec<(usize, Limits)>,
        pub globals: Vec<(usize, GlobalType)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SortedImports {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "SortedImports",
                "functions",
                &self.functions,
                "tables",
                &self.tables,
                "mems",
                &self.mems,
                "globals",
                &&self.globals,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SortedImports {
        #[inline]
        fn default() -> SortedImports {
            SortedImports {
                functions: ::core::default::Default::default(),
                tables: ::core::default::Default::default(),
                mems: ::core::default::Default::default(),
                globals: ::core::default::Default::default(),
            }
        }
    }
    impl SortedImports {
        pub fn add(&mut self, import: &Import, id: usize) {
            match &import.desc.data {
                ImportDesc::TypeIdx(t_id) => self.functions.push((id, *t_id)),
                ImportDesc::TableType(limits) => self.tables.push((id, limits.clone())),
                ImportDesc::MemType(limits) => self.mems.push((id, limits.clone())),
                ImportDesc::GlobalType(gt) => self.globals.push((id, gt.clone())),
            }
        }
    }
    type MaybeAt<T> = Option<WithPosition<T>>;
    pub struct Bytecode {
        pub header: Header,
        pub types: MaybeAt<Types>,
        pub imports: MaybeAt<Imports>,
        pub functions: MaybeAt<Functions>,
        pub tables: MaybeAt<Tables>,
        pub memories: MaybeAt<Memories>,
        pub globals: MaybeAt<Globals>,
        pub exports: MaybeAt<Exports>,
        pub start: MaybeAt<Start>,
        pub data_count: MaybeAt<DataCount>,
        pub code: MaybeAt<Code>,
        pub data: MaybeAt<ModuleData>,
        pub custom_sections: Vec<WithPosition<CustomSection>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Bytecode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "header",
                "types",
                "imports",
                "functions",
                "tables",
                "memories",
                "globals",
                "exports",
                "start",
                "data_count",
                "code",
                "data",
                "custom_sections",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.header,
                &self.types,
                &self.imports,
                &self.functions,
                &self.tables,
                &self.memories,
                &self.globals,
                &self.exports,
                &self.start,
                &self.data_count,
                &self.code,
                &self.data,
                &&self.custom_sections,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Bytecode", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Bytecode {
        #[inline]
        fn default() -> Bytecode {
            Bytecode {
                header: ::core::default::Default::default(),
                types: ::core::default::Default::default(),
                imports: ::core::default::Default::default(),
                functions: ::core::default::Default::default(),
                tables: ::core::default::Default::default(),
                memories: ::core::default::Default::default(),
                globals: ::core::default::Default::default(),
                exports: ::core::default::Default::default(),
                start: ::core::default::Default::default(),
                data_count: ::core::default::Default::default(),
                code: ::core::default::Default::default(),
                data: ::core::default::Default::default(),
                custom_sections: ::core::default::Default::default(),
            }
        }
    }
    impl Bytecode {
        fn add_section(&mut self, section: WithPosition<SectionData>) {
            match section.data {
                SectionData::Type(data) => {
                    self.types = Some(WithPosition::new(data, section.position));
                }
                SectionData::Import(data) => {
                    self.imports = Some(WithPosition::new(data, section.position));
                }
                SectionData::Function(data) => {
                    self.functions = Some(WithPosition::new(data, section.position));
                }
                SectionData::Table(data) => {
                    self.tables = Some(WithPosition::new(data, section.position));
                }
                SectionData::Memory(data) => {
                    self.memories = Some(WithPosition::new(data, section.position));
                }
                SectionData::Global(data) => {
                    self.globals = Some(WithPosition::new(data, section.position));
                }
                SectionData::Export(data) => {
                    self.exports = Some(WithPosition::new(data, section.position));
                }
                SectionData::Start(data) => {
                    self.start = Some(WithPosition::new(data, section.position));
                }
                SectionData::DataCount(data) => {
                    self.data_count = Some(WithPosition::new(data, section.position));
                }
                SectionData::Code(data) => {
                    self.code = Some(WithPosition::new(data, section.position));
                }
                SectionData::Data(data) => {
                    self.data = Some(WithPosition::new(data, section.position));
                }
            };
        }
        fn add_custom_section(&mut self, section: WithPosition<CustomSection>) {
            self.custom_sections.push(section);
        }
    }
    impl FromBytecode for Bytecode {
        fn from_reader<R: BytecodeReader>(reader: &mut R) -> Result<Self, ParserError> {
            let mut module: Bytecode = Default::default();
            module.header = reader.parse()?;
            for section in iter_sections(reader) {
                let section = section?;
                let section_data = section.data;
                let pos = section.position;
                match section_data.data.data {
                    SectionDataOrCustom::Section(section_data) => {
                        module.add_section(WithPosition::new(section_data, pos))
                    }
                    SectionDataOrCustom::Custom(custom_section) => {
                        module.add_custom_section(WithPosition::new(custom_section, pos))
                    }
                }
            }
            Ok(module)
        }
    }
    impl Bytecode {
        pub fn get_type(&self, id: usize) -> Option<&Type> {
            Some(&self.types.as_ref()?.data.get(id)?.data)
        }
        pub fn get_import(&self, id: usize) -> Option<&Import> {
            Some(&self.imports.as_ref()?.data.get(id)?.data)
        }
        pub fn get_function(&self, id: usize) -> Option<&usize> {
            Some(&self.functions.as_ref()?.data.get(id)?.data)
        }
        pub fn get_table(&self, id: usize) -> Option<&Limits> {
            Some(&self.tables.as_ref()?.data.get(id)?.data)
        }
        pub fn get_memory(&self, id: usize) -> Option<&Limits> {
            Some(&self.memories.as_ref()?.data.get(id)?.data)
        }
        pub fn get_global(&self, id: usize) -> Option<&Global> {
            Some(&self.globals.as_ref()?.data.get(id)?.data)
        }
        pub fn get_export(&self, id: usize) -> Option<&Export> {
            Some(&self.exports.as_ref()?.data.get(id)?.data)
        }
        pub fn get_code(&self, id: usize) -> Option<&Function> {
            Some(&self.code.as_ref()?.data.get(id)?.data)
        }
        pub fn get_data(&self, id: usize) -> Option<&Data> {
            Some(&self.data.as_ref()?.data.get(id)?.data)
        }
        pub fn get_type_pos(&self, id: usize) -> Option<WithPosition<&Type>> {
            Some(self.types.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_import_pos(&self, id: usize) -> Option<WithPosition<&Import>> {
            Some(self.imports.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_function_pos(&self, id: usize) -> Option<WithPosition<&usize>> {
            Some(self.functions.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_table_pos(&self, id: usize) -> Option<WithPosition<&Limits>> {
            Some(self.tables.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_memory_pos(&self, id: usize) -> Option<WithPosition<&Limits>> {
            Some(self.memories.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_global_pos(&self, id: usize) -> Option<WithPosition<&Global>> {
            Some(self.globals.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_export_pos(&self, id: usize) -> Option<WithPosition<&Export>> {
            Some(self.exports.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_code_pos(&self, id: usize) -> Option<WithPosition<&Function>> {
            Some(self.code.as_ref()?.data.get(id)?.as_ref())
        }
        pub fn get_data_pos(&self, id: usize) -> Option<WithPosition<&Data>> {
            Some(self.data.as_ref()?.data.get(id)?.as_ref())
        }
    }
    impl Bytecode {
        pub fn sort_imports(&self) -> Option<SortedImports> {
            if let Some(imports) = &self.imports {
                let mut sorted: SortedImports = Default::default();
                imports
                    .data
                    .iter()
                    .enumerate()
                    .for_each(|(id, i)| sorted.add(&i.data, id));
                Some(sorted)
            } else {
                None
            }
        }
    }
    pub fn parse_binary(reader: &mut impl BytecodeReader) -> Result<Bytecode, ParserError> {
        reader.parse()
    }
    pub fn parse_wat(code: &str) -> Result<Bytecode, ParserError> {
        let data = wat::parse_str(code)?;
        let mut reader = Cursor::new(data);
        parse_binary(&mut reader)
    }
}
pub mod leb {
    use byteorder::ReadBytesExt;
    use std::io::Read;
    use thiserror::Error;
    pub enum LebError {
        #[error("Unable to read from reader: {0}")]
        Io(std::io::Error),
        #[error("Invalid leb")]
        InvalidLeb,
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for LebError {}
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for LebError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use ::thiserror::__private::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                LebError::Io(_0) => match (_0.as_display(),) {
                    (__display0,) => __formatter
                        .write_fmt(format_args!("Unable to read from reader: {0}", __display0)),
                },
                LebError::InvalidLeb {} => __formatter.write_str("Invalid leb"),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LebError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                LebError::Io(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
                }
                LebError::InvalidLeb => ::core::fmt::Formatter::write_str(f, "InvalidLeb"),
            }
        }
    }
    impl From<std::io::Error> for LebError {
        fn from(value: std::io::Error) -> Self {
            Self::Io(value)
        }
    }
    pub struct Leb {}
    impl Leb {
        #[inline]
        pub fn read_u32(reader: &mut impl Read) -> Result<u32, LebError> {
            let byte = reader.read_u8()?;
            if (byte & 0x80) == 0 {
                Ok(u32::from(byte))
            } else {
                Self::read_u32_big(reader, byte)
            }
        }
        fn read_u32_big(reader: &mut impl Read, first: u8) -> Result<u32, LebError> {
            let mut result = (first & 0x7F) as u32;
            let mut shift = 7;
            loop {
                let byte = reader.read_u8()?;
                result |= ((byte & 0x7F) as u32) << shift;
                if shift >= 25 && (byte >> (32 - shift)) != 0 {
                    return Err(LebError::InvalidLeb);
                }
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            Ok(result)
        }
        pub fn read_u64(reader: &mut impl Read) -> Result<u64, LebError> {
            let byte = u64::from(reader.read_u8()?);
            if (byte & 0x80) == 0 {
                Ok(byte)
            } else {
                Self::read_u64_big(reader, byte)
            }
        }
        fn read_u64_big(reader: &mut impl Read, byte: u64) -> Result<u64, LebError> {
            let mut result = byte & 0x7F;
            let mut shift = 7;
            loop {
                let byte = u64::from(reader.read_u8()?);
                result |= (byte & 0x7F) << shift;
                if shift >= 57 && (byte >> (64 - shift)) != 0 {
                    return Err(LebError::InvalidLeb);
                }
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            Ok(result)
        }
        #[inline]
        pub fn read_var_i32(reader: &mut impl Read) -> Result<i32, LebError> {
            let byte = reader.read_u8()?;
            if (byte & 0x80) == 0 {
                Ok(((byte as i32) << 25) >> 25)
            } else {
                Self::read_i32_big(reader, byte)
            }
        }
        fn read_i32_big(reader: &mut impl Read, byte: u8) -> Result<i32, LebError> {
            let mut result = (byte & 0x7F) as i32;
            let mut shift = 7;
            loop {
                let byte = reader.read_u8()?;
                result |= ((byte & 0x7F) as i32) << shift;
                if shift >= 25 {
                    let continuation_bit = (byte & 0x80) != 0;
                    let sign_and_unused_bit = (byte << 1) as i8 >> (32 - shift);
                    if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                        return Err(LebError::InvalidLeb);
                    }
                    return Ok(result);
                }
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            let ashift = 32 - shift;
            Ok((result << ashift) >> ashift)
        }
        pub fn read_s33(reader: &mut impl Read) -> Result<i64, LebError> {
            let byte = reader.read_u8()?;
            if (byte & 0x80) == 0 {
                return Ok(((byte as i8) << 1) as i64 >> 1);
            }
            let mut result = (byte & 0x7F) as i64;
            let mut shift = 7;
            loop {
                let byte = reader.read_u8()?;
                result |= ((byte & 0x7F) as i64) << shift;
                if shift >= 25 {
                    let continuation_bit = (byte & 0x80) != 0;
                    let sign_and_unused_bit = (byte << 1) as i8 >> (33 - shift);
                    if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                        return Err(LebError::InvalidLeb);
                    }
                    return Ok(result);
                }
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            let ashift = 64 - shift;
            Ok((result << ashift) >> ashift)
        }
        pub fn read_var_i64(reader: &mut impl Read) -> Result<i64, LebError> {
            let mut result: i64 = 0;
            let mut shift = 0;
            loop {
                let byte = reader.read_u8()?;
                result |= i64::from(byte & 0x7F) << shift;
                if shift >= 57 {
                    let continuation_bit = (byte & 0x80) != 0;
                    let sign_and_unused_bit = ((byte << 1) as i8) >> (64 - shift);
                    if continuation_bit || (sign_and_unused_bit != 0 && sign_and_unused_bit != -1) {
                        return Err(LebError::InvalidLeb);
                    }
                    return Ok(result);
                }
                shift += 7;
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            let ashift = 64 - shift;
            Ok((result << ashift) >> ashift)
        }
    }
}
pub mod info {
    use crate::reader::{Bytecode, GlobalType, Limits, SortedImports, ValueType};
    pub enum IncludeMode {
        Internal,
        Exported,
        Imported,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IncludeMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    IncludeMode::Internal => "Internal",
                    IncludeMode::Exported => "Exported",
                    IncludeMode::Imported => "Imported",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IncludeMode {
        #[inline]
        fn clone(&self) -> IncludeMode {
            match self {
                IncludeMode::Internal => IncludeMode::Internal,
                IncludeMode::Exported => IncludeMode::Exported,
                IncludeMode::Imported => IncludeMode::Imported,
            }
        }
    }
    pub enum FunctionType {
        Internal {
            code_id: usize,
            export_id: Option<usize>,
        },
        Imported {
            import_id: usize,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FunctionType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                FunctionType::Internal {
                    code_id: __self_0,
                    export_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Internal",
                    "code_id",
                    __self_0,
                    "export_id",
                    &__self_1,
                ),
                FunctionType::Imported {
                    import_id: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Imported",
                    "import_id",
                    &__self_0,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FunctionType {
        #[inline]
        fn clone(&self) -> FunctionType {
            match self {
                FunctionType::Internal {
                    code_id: __self_0,
                    export_id: __self_1,
                } => FunctionType::Internal {
                    code_id: ::core::clone::Clone::clone(__self_0),
                    export_id: ::core::clone::Clone::clone(__self_1),
                },
                FunctionType::Imported {
                    import_id: __self_0,
                } => FunctionType::Imported {
                    import_id: ::core::clone::Clone::clone(__self_0),
                },
            }
        }
    }
    pub struct Function {
        type_id: usize,
        t: FunctionType,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Function {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Function",
                "type_id",
                &self.type_id,
                "t",
                &&self.t,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Function {
        #[inline]
        fn clone(&self) -> Function {
            Function {
                type_id: ::core::clone::Clone::clone(&self.type_id),
                t: ::core::clone::Clone::clone(&self.t),
            }
        }
    }
    impl Function {
        pub fn new_internal(type_id: usize, code_id: usize, export_id: Option<usize>) -> Self {
            Function {
                type_id,
                t: FunctionType::Internal { code_id, export_id },
            }
        }
        pub fn new_imported(type_id: usize, import_id: usize) -> Self {
            Function {
                type_id,
                t: FunctionType::Imported { import_id },
            }
        }
    }
    pub enum GlobalInfo {
        Internal {
            global_id: usize,
            export_id: Option<usize>,
        },
        Imported {
            import_id: usize,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GlobalInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                GlobalInfo::Internal {
                    global_id: __self_0,
                    export_id: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Internal",
                    "global_id",
                    __self_0,
                    "export_id",
                    &__self_1,
                ),
                GlobalInfo::Imported {
                    import_id: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Imported",
                    "import_id",
                    &__self_0,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GlobalInfo {
        #[inline]
        fn clone(&self) -> GlobalInfo {
            match self {
                GlobalInfo::Internal {
                    global_id: __self_0,
                    export_id: __self_1,
                } => GlobalInfo::Internal {
                    global_id: ::core::clone::Clone::clone(__self_0),
                    export_id: ::core::clone::Clone::clone(__self_1),
                },
                GlobalInfo::Imported {
                    import_id: __self_0,
                } => GlobalInfo::Imported {
                    import_id: ::core::clone::Clone::clone(__self_0),
                },
            }
        }
    }
    pub struct Global {
        pub t: ValueType,
        pub mutable: bool,
        pub info: GlobalInfo,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Global {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Global",
                "t",
                &self.t,
                "mutable",
                &self.mutable,
                "info",
                &&self.info,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Global {
        #[inline]
        fn clone(&self) -> Global {
            Global {
                t: ::core::clone::Clone::clone(&self.t),
                mutable: ::core::clone::Clone::clone(&self.mutable),
                info: ::core::clone::Clone::clone(&self.info),
            }
        }
    }
    impl Global {
        pub fn new_imported(global_type: &GlobalType, import_id: usize) -> Self {
            let info = GlobalInfo::Imported { import_id };
            Global {
                t: global_type.t.data,
                mutable: global_type.mutable.data,
                info,
            }
        }
    }
    pub enum MemoryInfo {
        Internal { export_id: Option<usize> },
        Imported { import_id: usize },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MemoryInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MemoryInfo::Internal {
                    export_id: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Internal",
                    "export_id",
                    &__self_0,
                ),
                MemoryInfo::Imported {
                    import_id: __self_0,
                } => ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Imported",
                    "import_id",
                    &__self_0,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MemoryInfo {
        #[inline]
        fn clone(&self) -> MemoryInfo {
            match self {
                MemoryInfo::Internal {
                    export_id: __self_0,
                } => MemoryInfo::Internal {
                    export_id: ::core::clone::Clone::clone(__self_0),
                },
                MemoryInfo::Imported {
                    import_id: __self_0,
                } => MemoryInfo::Imported {
                    import_id: ::core::clone::Clone::clone(__self_0),
                },
            }
        }
    }
    pub struct Memory {
        limits: Limits,
        info: MemoryInfo,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Memory {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Memory",
                "limits",
                &self.limits,
                "info",
                &&self.info,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Memory {
        #[inline]
        fn clone(&self) -> Memory {
            Memory {
                limits: ::core::clone::Clone::clone(&self.limits),
                info: ::core::clone::Clone::clone(&self.info),
            }
        }
    }
    impl Memory {
        pub fn new_imported(import_id: usize, limits: Limits) -> Memory {
            Memory {
                limits,
                info: MemoryInfo::Imported { import_id },
            }
        }
    }
    pub struct BytecodeInfo {
        imports: Option<SortedImports>,
        functions: Vec<Function>,
        globals: Vec<Global>,
        memories: Vec<Memory>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BytecodeInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "BytecodeInfo",
                "imports",
                &self.imports,
                "functions",
                &self.functions,
                "globals",
                &self.globals,
                "memories",
                &&self.memories,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BytecodeInfo {
        #[inline]
        fn default() -> BytecodeInfo {
            BytecodeInfo {
                imports: ::core::default::Default::default(),
                functions: ::core::default::Default::default(),
                globals: ::core::default::Default::default(),
                memories: ::core::default::Default::default(),
            }
        }
    }
    impl BytecodeInfo {
        pub fn new(bytecode: &Bytecode) -> Self {
            let mut info: BytecodeInfo = Default::default();
            if let Some(imports) = bytecode.sort_imports() {
                info.functions.extend(
                    imports
                        .functions
                        .iter()
                        .map(|(id, t_id)| Function::new_imported(*id, *t_id)),
                );
                info.globals.extend(
                    imports
                        .globals
                        .iter()
                        .map(|(id, gt)| Global::new_imported(gt, *id)),
                );
                info.memories.extend(
                    imports
                        .mems
                        .iter()
                        .map(|(id, limits)| Memory::new_imported(*id, limits.clone())),
                );
                info.imports = Some(imports);
            }
            info
        }
        pub fn has_memory(&self) -> bool {
            self.memories.len() > 0
        }
    }
}
