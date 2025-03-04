#include "bytecode_reader.hpp"

#include <SDL3/SDL_log.h>

#include "../os.h"

namespace Bytecode {
bool Reader::can_read() {
  return this->current_position >= 0 &&
         this->current_position < (int64_t)this->data.size_bytes();
}

uint8_t* Reader::ptr() {
  return this->data
      .subspan((size_t)this->current_position,
               this->data.size() - (size_t)this->current_position)
      .data();
}

void Reader::skip_bytes(uint64_t offset) {
  os_assert((this->current_position + (int64_t)offset) <
            (int64_t)this->data.size_bytes());
  this->current_position += (int64_t)offset;
}

bool Reader::copy_bytes_into(size_t count, std::span<uint8_t> dest) {
  os_assert(count <= dest.size_bytes());
  os_memcpy(dest.data(), this->ptr(), count);
  this->current_position += (int64_t)count;
  return true;
}

std::optional<std::span<uint8_t>> Reader::copy_bytes_alloc(Arena* arena,
                                                           size_t count) {
  os_assert(this->current_position + (int64_t)count <
            (int64_t)(this->data.size_bytes()));
  std::span<uint8_t> buffer = arena->push<uint8_t>(count);
  this->copy_bytes_into(count, buffer);
  return buffer;
}

std::optional<std::span<uint8_t>> Reader::copy_bytes_alloc_zero_term(
    Arena* arena, size_t count) {
  os_assert(this->current_position + (int64_t)count <
            (int64_t)(this->data.size_bytes()));
  std::span<uint8_t> buffer = arena->push<uint8_t>(count + 1);
  this->copy_bytes_into(count, buffer);
  buffer[count] = 0;
  return buffer;
}

Reader Reader::from_buffer(std::span<uint8_t> buffer) {
  return Reader{.data = buffer, .current_position = 0};
}

}  // namespace Bytecode

/*
uint8_t bytecode_read_byte(Bytecode_Reader* reader) {}

int64_t bytecode_read_var_int(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  int64_t out_int = 0;
  size_t bytes_read = leb128_read_i64(reader->data, reader->current_position,
                                      (int64_t)reader->data_size, &out_int);

  os_assert(bytes_read != 0);
  reader->current_position += bytes_read;
  return out_int;
}

uint64_t bytecode_read_var_uint(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  uint64_t out_int = 0;
  size_t bytes_read = leb128_read_u64(reader->data, reader->current_position,
                                      (int64_t)(reader->data_size), &out_int);

  os_assert(bytes_read != 0);
  reader->current_position += bytes_read;
  return out_int;
}
*/
