#pragma once
#include <stddef.h>
#include <stdint.h>

#include <concepts>
#include <optional>
#include <span>
#include <string_view>
#include <type_traits>

#include "../arena.hpp"
#include "../leb128.hpp"
// TODO: Error-Handling!
namespace Bytecode {
struct Reader {
  std::span<uint8_t> data;
  int64_t current_position;

  static Reader from_buffer(std::span<uint8_t> buffer);
  bool can_read();
  uint8_t* ptr();
  void skip_bytes(size_t count);
  bool copy_bytes_into(size_t count, std::span<uint8_t> dest);
  std::optional<std::span<uint8_t>> copy_bytes_alloc(Arena* arena,
                                                     size_t count);
  std::optional<std::span<uint8_t>> copy_bytes_alloc_zero_term(Arena* arena,
                                                               size_t count);

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
    os_assert((size_t)this->current_position < this->data.size_bytes());
    auto leb_result = Leb128::read<T>(this->bytes());
    this->current_position += leb_result.bytes_read;

    return leb_result.num;
  }

  template <>
  uint8_t get<uint8_t>() {
    os_assert((size_t)this->current_position + 1 < this->data.size_bytes());
    uint8_t data = this->data[(size_t)current_position];
    this->current_position += 1;

    return data;
  }
};

/*
        bool bytecode_reader_can_read(Bytecode_Reader* reader);
        void bytecode_reader_skip_bytes(Bytecode_Reader* reader, uint64_t
   offset); uint8_t* bytecode_bytes_at(Bytecode_Reader* reader); uint8_t*
   bytecode_read_bytes_alloc(Arena* arena, Bytecode_Reader* reader, size_t
   count); bool bytecode_read_bytes_into(Bytecode_Reader* reader, size_t count,
                                                                                                                                size_t buffer_size, uint8_t* buffer);
        uint8_t* bytecode_read_bytes_zero_term(Arena* arena, Bytecode_Reader*
   reader, size_t count);

        uint8_t bytecode_read_byte(Bytecode_Reader* reader);
        int64_t bytecode_read_var_int(Bytecode_Reader* reader);
        uint64_t bytecode_read_var_uint(Bytecode_Reader* reader);
*/
}  // namespace Bytecode
