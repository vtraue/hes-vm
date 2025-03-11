#include "arena.hpp"
#include "core.hpp"

Arena* Arena::create(size_t cap) {
  assert(cap != 0);
  size_t reserve_size = (cap + sizeof(Arena));
  auto data = Os::Mem::reserve(reserve_size);
  auto arena = reinterpret_cast<Arena*>(data.data());

  arena->data = data;
  arena->used = sizeof(Arena);
  return arena;
}

bool Arena::can_write_size(size_t size_bytes) {
  if (this->used + size_bytes >= this->data.size_bytes()) {
    return false;
  }
  return true;
}

uint8_t* Arena::ptr() { return this->data.data() + this->used; }

void Arena::reset() { this->used = 0; }

void Arena::destroy() { Os::Mem::unreserve(this->data); }

void Arena::reserve(size_t size_bytes) {
  assert(this->can_write_size(size_bytes));
  this->used += size_bytes;
}

size_t Arena::bytes_left() { return this->data.size_bytes() - this->used; }

bool Arena::write_byte(uint8_t b) {
  if (!this->can_write_size(1)) {
    return false;
  }
  this->data[this->used] = b;
  this->used += 1;
  return true;
}

