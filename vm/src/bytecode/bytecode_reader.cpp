#include <SDL3/SDL_log.h>

#include "bytecode.hpp"

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
void Reader::skip_bytes(size_t offset) {
  assert((this->current_position + (int64_t)offset) <
         (int64_t)this->data.size_bytes());
  this->current_position += (int64_t)offset;
}

bool Reader::copy_bytes_into(size_t count, std::span<uint8_t> dest) {
  assert(count <= dest.size_bytes());
  std::memcpy(dest.data(), this->ptr(), count);
  this->current_position += (int64_t)count;
  return true;
}

std::optional<std::span<uint8_t>> Reader::copy_bytes_alloc(Arena* arena,
                                                           size_t count) {
  assert(this->current_position + (int64_t)count <
         (int64_t)(this->data.size_bytes()));
  std::span<uint8_t> buffer = arena->push<uint8_t>(count);
  this->copy_bytes_into(count, buffer);
  return buffer;
}
std::optional<std::span<uint8_t>> Reader::copy_bytes_alloc_zero_term(
    Arena* arena, size_t count) {
  assert(this->current_position + (int64_t)count <
         (int64_t)(this->data.size_bytes()));
  std::span<uint8_t> buffer = arena->push<uint8_t>(count + 1);
  this->copy_bytes_into(count, buffer);
  buffer[count] = 0;
  return buffer;
}

std::span<uint8_t> Reader::bytes() {
  return this->data.subspan((size_t)this->current_position,
                            this->data.size() - (size_t)this->current_position);
}

template <>
uint8_t Reader::get<uint8_t>() {
  assert((size_t)this->current_position + 1 <= this->data.size_bytes());
  uint8_t data = this->data[(size_t)current_position];
  this->current_position += 1;

  return data;
}

}  // namespace Bytecode
