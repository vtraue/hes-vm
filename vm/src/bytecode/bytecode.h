#pragma once
#include <stddef.h>
#include <stdint.h>

#include "../arena.h"
#include "bytecode_reader.h"
#include "opcode.h"
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
  Bytecode_Export* exported;
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
  Bytecode_Instruction_Blocktype_Type_Empty,
  Bytecode_Instruction_Blocktype_Type_Valtype,
  Bytecode_Instruction_Blocktype_Type_TypeIndex,

} Bytecode_Instruction_Blocktype_Type;

typedef struct Bytecode_Instruction_Data_Blocktype {
  Bytecode_Instruction_Blocktype_Type block_type;
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
  uint32_t count;
  Bytecode_Export_Desc_Type* valtypes;
} Bytecode_Instruction_Data_Vec_Valtype;

typedef struct Bytecode_Instruction_Data_Br_Table {
  Bytecode_Instruction_Data_Vec_Valtype vec;
  uint32_t label_id;
} Bytecode_Instruction_Data_Br_Table;

typedef struct Bytecode_Instruction {
  Bytecode_Op opcode;
  uint32_t suffix;
  union {
    Bytecode_Instruction_Data_Blocktype block;
    Bytecode_Instruction_Data_U32x2 u32x2;
    Bytecode_Instruction_Data_Memarg mem;
    Bytecode_Instruction_Data_Vec_Valtype valtypes;
    Bytecode_Instruction_Data_Br_Table br_table;

    uint32_t u32;
    uint64_t u64;
    float f32;
  } args;
} Bytecode_Instruction;

#define INITAL_EXPRESSION_MAX_COUNT 255
// TODO: (joh) Spaeter sollte das dynamisch wachsen koennen. Die
// wahrscheinlichkeit, dass wir mehr als 255 Instruktionen in einer Expression
// haben werden ist relativ hoch!
typedef struct Bytecode_Expression {
  uint64_t count;
  uint64_t cap;
  uint64_t size;
  uint8_t* instructions;

} Bytecode_Expression;

/*
typedef struct Bytecode_Func {
        uint64_t locals_count;
        Bytecode_Locals* locals;
}
*/
