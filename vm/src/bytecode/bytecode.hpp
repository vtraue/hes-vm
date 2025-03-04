#pragma once
#include <cstddef>
#include <cstdint>
#include <optional>
#include <span>
#include <string_view>

#include "opcode.hpp"
constexpr uint8_t FUNCTYPE_HEADER = 0x60;

namespace Bytecode {
enum class Section_Id : uint8_t {
  Custom = 0,
  Type = 1,
  Import = 2,
  Function = 3,
  Table = 4,
  Memory = 5,
  Global = 6,
  Export = 7,
  Start = 8,
  Element = 9,
  Code = 10,
  Data = 11,
  Data_Count = 12,
};

std::optional<Section_Id> section_id_from_byte(uint8_t id);

enum class Type_Id : uint8_t {
  Num_I32 = 0x7F,
  Num_I64 = 0x7E,
  Num_F32 = 0x7D,
  Num_F64 = 0x7C,
  Vector = 0x7B,
  Ref_Funcref = 0x70,
  Ref_Externref = 0x6F,

};

std::optional<Type_Id> type_id_from_byte(uint8_t id);
struct Export_Desc {
  enum Type : uint8_t {
    Funcidx = 0x00,
    Tableidx = 0x01,
    Memidx = 0x02,
    Globalidx = 0x03,
    Enum_Len = 0x04,
  };
  Type type;
  uint64_t id;
};

struct Export {
  std::string_view name;
  Export_Desc desc;
};

struct Function_Type {
  std::optional<std::span<Type_Id>> param_types;
  std::optional<std::span<Type_Id>> return_types;
};

using Function_Section = std::span<uint32_t>;
using Export_Section = std::span<Export>;
using Type_Section = std::span<Function_Type>;

constexpr int INITAL_EXPRESSION_MAX_COUNT = 255;
using Label_Id = uint32_t;

namespace Imm {
using Block_Type_Index = int32_t;

struct Blocktype {
  enum class Tag : uint8_t { Empty, Val_Type, Type_Index };
  Tag tag;
  union {
    Type_Id val_type;
    int32_t type_id;
  } data;
};

template <typename T>
struct Pair {
  T x;
  T y;
};

struct Br_Table {
  std::span<Label_Id> label_idx;
  Label_Id ln;
};

union Data {
  uint32_t u32;
  int32_t i32;
  int64_t i64;
  uint8_t u8;
  std::span<Type_Id> valtypes;
  Br_Table br_table;
  Pair<uint32_t> pair_u32;
  Blocktype block_type;
};
}  // namespace Imm

struct Instruction {
  Op op;
  uint32_t suffix;
  Imm::Data args;
};

using Expression = std::span<Instruction>;

struct Locals_of_T {
  uint32_t count;
  Type_Id valtype;
};

struct Locals {
  std::span<Locals_of_T> data;

  size_t concat_count() {
    size_t count = 0;
    for (Locals_of_T locals : this->data) {
      count += locals.count;
    }
    return count;
  }

  bool concat(std::span<Type_Id> dest) {
    size_t i = 0;
    for (Locals_of_T locals : this->data) {
      for (uint32_t c = 0; c < locals.count; c++) {
        if (i >= dest.size()) {
          return false;
        }
        dest[i] = locals.valtype;
        i += 1;
      }
    }
    return true;
  }
};

struct Code {
  uint32_t size_bytes;
  std::optional<Locals> locals;
  Expression expr;
};

using Code_Section = std::span<Code>;
}  // namespace Bytecode

/*
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
*/

// TODO: (joh) Spaeter sollte das dynamisch wachsen koennen. Die
// wahrscheinlichkeit, dass wir mehr als 255 Instruktionen in einer Expression
// haben werden ist relativ hoch!
/*
typedef struct Bytecode_Expression {
  uint64_t count;
  uint64_t cap;
  uint64_t size;
  uint8_t* instructions;

} Bytecode_Expression;
*/
/*
typedef struct Bytecode_Func {
        uint64_t locals_count;
        Bytecode_Locals* locals;
}
*/
