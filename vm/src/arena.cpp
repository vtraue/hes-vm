#include "arena.hpp"

#include <cstdint>
#include <span>

#include "os.h"
// TODO: (joh) Arenas sollten später wachsen können

Arena *Arena::create(size_t cap) {
  os_assert(cap != 0);
  size_t reserve_size = (cap + sizeof(Arena));
  uint8_t *data = os_mem_reserve(reserve_size);

  auto *arena = reinterpret_cast<Arena *>(data);

  arena->data = data + sizeof(Arena);
  arena->cap = cap;
  arena->used = 0;
  return arena;
}

bool Arena::can_write_size(size_t size_bytes) {
  os_assert(this->used <= this->cap);
  if (this->used + size_bytes >= this->cap) {
    os_crash_with_message("Arena out of memory!");
    return false;
  }
  return true;
}

uint8_t *Arena::ptr() { return this->data + this->used; }

void Arena::reset() { this->used = 0; }

void Arena::destroy() { os_mem_unreserve((uint8_t *)this, this->cap); }

void Arena::reserve(size_t size_bytes) {
  if (this->used + size_bytes >= this->cap) {
    os_crash_with_message("Arena out of memory!");
    return;
  }
  this->used += size_bytes;
}

size_t Arena::bytes_left() { return this->cap - this->used; }

bool Arena::write_byte(uint8_t b) {
  if (!this->can_write_size(1)) {
    return false;
  }
  *this->ptr() = b;
  this->used += 1;
  return true;
}
