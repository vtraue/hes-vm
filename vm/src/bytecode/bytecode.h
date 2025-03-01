#pragma once
#include <stddef.h>
#include <stdint.h>

#include "../arena.h"
#include "bytecode_reader.h"
#define FUNCTYPE_HEADER 0x60

typedef enum Bytecode_Section_Id : uint8_t {
  Bytecode_Section_Id_Custom = 0,
  Bytecode_Section_Id_Type = 1,
  Bytecode_Section_Id_Import = 2,
  Bytecode_Section_Id_Function = 3,
  Bytecode_Section_Id_Table = 4,
  Bytecode_Section_Id_Memory = 5,
  Bytecode_Section_Id_Global = 6,
  Bytecode_Section_Id_Export = 7,
  Bytecode_Section_Id_Start = 8,
  Bytecode_Section_Id_Element = 9,
  Bytecode_Section_Id_Code = 10,
  Bytecode_Section_Id_Data = 11,
  Bytecode_Section_Id_Data_Count = 12,
  Bytecode_Section_Id_Enum_Max = 13,
} Bytecode_Section_Id;

typedef enum Bytecode_Type_Id : uint8_t {
  Bytecode_Type_Id_Num_I32 = 0x7F,
  Bytecode_Type_Id_Num_I64 = 0x7E,
  Bytecode_Type_Id_Num_F32 = 0x7D,
  Bytecode_Type_Id_Num_F64 = 0x7C,
  Bytecode_Type_Id_Vector = 0x7B,
  Bytecode_Type_Id_Ref_Funcref = 0x70,
  Bytecode_Type_Id_Ref_Externref = 0x6F,
} Bytecode_Type_Id;

typedef enum Bytecode_Export_Desc_Type : uint8_t {
  Bytecode_Export_Desc_Type_Funcidx = 0x00,
  Bytecode_Export_Desc_Type_Tableidx = 0x01,
  Bytecode_Export_Desc_Type_Memidx = 0x02,
  Bytecode_Export_Desc_Type_Globalidx = 0x03,
  Bytecode_Export_Desc_Type_Enum_Len = 0x04,
} Bytecode_Export_Desc_Type;

typedef struct Bytecode_Export_Desc {
  Bytecode_Export_Desc_Type type;
  uint64_t id;
} Bytecode_Export_Desc;

typedef struct Bytecode_Export {
  char* name;
  uint64_t name_length;

  Bytecode_Export_Desc desc;
} Bytecode_Export;

typedef struct Bytecode_Export_Section {
  uint64_t export_count;
  Bytecode_Export* export;
} Bytecode_Export_Section;

#define FUNCTION_TYPE_MAX_PARAMS 12
#define FUNCTION_TYPE_MAX_RETURN 12

typedef struct Bytecode_Function_Type {
  size_t param_count;
  Bytecode_Type_Id param_types[FUNCTION_TYPE_MAX_PARAMS];

  size_t return_count;
  Bytecode_Type_Id return_types[FUNCTION_TYPE_MAX_RETURN];
} Bytecode_Function_Type;

typedef struct Bytecode_Type_Section {
  size_t type_count;
  Bytecode_Function_Type* function_types;
} Bytecode_Type_Section;

typedef struct Bytecode_Function_Section {
  uint64_t function_count;
  uint32_t* type_idx;
} Bytecode_Function_Section;

typedef struct Bytecode_Section {
  Bytecode_Section_Id id;

  union {
    Bytecode_Type_Section type_section;
  } data;
} Bytecode_Section;

typedef struct Bytecode_Locals {
  uint64_t count;
  Bytecode_Type_Id valtype;
} Bytecode_Locals;

typedef enum Bytecode_Instruction_Blocktype_Type {
  Empty,
  Valtype,
  TypeIndex,

} Bytecode_Instruction_Blocktype_Type;

typedef struct Bytecode_Instruction_Data_Blocktype {
  Bytecode_Type_Id block_type;
  union {
    uint8_t valtype;
    uint32_t type_index;
  } data;
} Bytecode_Instruction_Data_Blocktype;

typedef struct Bytecode_Instruction_Data_U32x2 {
  uint32_t a;
  uint32_t b;
} Bytecode_Instruction_Data_U32x2;

typedef struct Bytecode_Instruction_Data_Memarg {
  uint32_t align;
  uint32_t offset;
} Bytecode_Instruction_Data_Memarg;

typedef struct Bytecode_Instruction_Data_Vec_Valtype {
  uint64_t count;
  Bytecode_Export_Desc_Type* valtypes;
} Bytecode_Instruction_Data_Vec_Valtype;

typedef enum Bytecode_Instruction_Type {
  Bytecode_Op_unreachable,
  Bytecode_Op_nop,
  Bytecode_Op_block,
  Bytecode_Op_loop,
  Bytecode_Op_if,
  Bytecode_Op_else,
  Bytecode_Op_end,
  Bytecode_Op_br,
  Bytecode_Op_br_if,
  Bytecode_Op_br_table,
  Bytecode_Op_return,
  Bytecode_Op_call,
  Bytecode_Op_call_indirect,
  Bytecode_Op_drop,
  Bytecode_Op_select,
  Bytecode_Op_select_t,
  Bytecode_Op_local_get,
  Bytecode_Op_local_set,
  Bytecode_Op_local_tee,
  Bytecode_Op_global_get,
  Bytecode_Op_global_set,
  Bytecode_Op_i32_load,
  Bytecode_Op_i64_load,
  Bytecode_Op_f32_load,
  Bytecode_Op_f64_load,
  Bytecode_Op_i32_load8_s,
  Bytecode_Op_i32_store,
  Bytecode_Op_i64_store,
  Bytecode_Op_f32_store,
  Bytecode_Op_i32_const,
  Bytecode_Op_i64_const,
  Bytecode_Op_i32_eqz,
  Bytecode_Op_i32_eq,
  Bytecode_Op_i32_ne,
  Bytecode_Op_i32_lt_s,
  Bytecode_Op_i32_lt_u,
  Bytecode_Op_i32_gt_s,
  Bytecode_Op_i32_gt_u,
  Bytecode_Op_i32_le_s,
  Bytecode_Op_i32_le_u,
  Bytecode_Op_i32_ge_s,
  Bytecode_Op_i32_ge_u,
  Bytecode_Op_ref_null,
  Bytecode_Op_ref_is_null,
  Bytecode_Op_ref_func,
} Bytecode_Instruction_Type;

typedef struct Bytecode_Instruction {
  Bytecode_Instruction_Type type;
  union {
    Bytecode_Instruction_Data_Blocktype block;
    Bytecode_Instruction_Data_U32x2 u32x2;
    Bytecode_Instruction_Data_Memarg mem;
    Bytecode_Instruction_Data_Vec_Valtype valtypes;
    uint32_t u32;
    uint64_t u64;
    float f32;
  } args;
} Bytecode_Instruction;

/*
typedef struct Bytecode_Func {
        uint64_t locals_count;
        Bytecode_Locals* locals;
}
*/
