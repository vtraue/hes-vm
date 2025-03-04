#ifndef BYTECODE_HPP
#define BYTECODE_HPP

#include "../core.hpp"
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

using Type_Index = uint32_t;
using Function_Section = std::span<Type_Index>;
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

struct Reader {
  std::span<uint8_t> data;
  int64_t current_position;

  static Reader from_buffer(std::span<uint8_t> buffer) {
    return Reader{.data = buffer, .current_position = 0};
  }
  bool can_read() {
    return this->current_position >= 0 &&
           this->current_position < (int64_t)this->data.size_bytes();
  }

  uint8_t* ptr() {
    return this->data
        .subspan((size_t)this->current_position,
                 this->data.size() - (size_t)this->current_position)
        .data();
  }

  void skip_bytes(size_t offset) {
    assert((this->current_position + (int64_t)offset) <
           (int64_t)this->data.size_bytes());
    this->current_position += (int64_t)offset;
  }

  bool copy_bytes_into(size_t count, std::span<uint8_t> dest) {
    assert(count <= dest.size_bytes());
    std::memcpy(dest.data(), this->ptr(), count);
    this->current_position += (int64_t)count;
    return true;
  }

  std::optional<std::span<uint8_t>> copy_bytes_alloc(Arena* arena,
                                                     size_t count) {
    assert(this->current_position + (int64_t)count <
           (int64_t)(this->data.size_bytes()));
    std::span<uint8_t> buffer = arena->push<uint8_t>(count);
    this->copy_bytes_into(count, buffer);
    return buffer;
  }
  std::optional<std::span<uint8_t>> copy_bytes_alloc_zero_term(Arena* arena,
                                                               size_t count) {
    assert(this->current_position + (int64_t)count <
           (int64_t)(this->data.size_bytes()));
    std::span<uint8_t> buffer = arena->push<uint8_t>(count + 1);
    this->copy_bytes_into(count, buffer);
    buffer[count] = 0;
    return buffer;
  }

  std::span<uint8_t> bytes() {
    return this->data.subspan(
        (size_t)this->current_position,
        this->data.size() - (size_t)this->current_position);
  }

  template <typename T>
  T get();

  template <typename T>
  T get()
    requires std::integral<T>
  {
    assert((size_t)this->current_position <= this->data.size_bytes());
    auto leb_result = Leb128::read<T>(this->bytes());
    this->current_position += leb_result.bytes_read;

    return leb_result.num;
  }

  template <>
  uint8_t get<uint8_t>() {
    assert((size_t)this->current_position + 1 <= this->data.size_bytes());
    uint8_t data = this->data[(size_t)current_position];
    this->current_position += 1;

    return data;
  }
};

}  // namespace Bytecode
#endif
