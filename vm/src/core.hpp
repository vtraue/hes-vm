#ifndef CORE_HPP
#define CORE_HPP

#include <SDL3/SDL.h>
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_iostream.h>
#include <SDL3/SDL_log.h>

#include <cassert>
#include <concepts>
#include <cstdint>
#include <cstring>
#include <optional>
#include <span>

#include "os.hpp"

struct Arena {
  size_t used;
  std::span<uint8_t> data;

  static Arena* create(size_t cap);
  bool can_write_size(size_t bytes);

  template <typename T>
  std::span<T> push(size_t count);

  uint8_t* ptr();
  void reset();
  void destroy();
  void reserve(size_t size);
  size_t bytes_left();

  template <typename T>
  bool write(std::span<T> data);
  template <typename T>
  bool write(T* data);

  bool write_byte(uint8_t b);
};

template <typename T>
std::span<T> Arena::push(size_t count) {
  assert(this->can_write_size(sizeof(T) * count));

  uint8_t* out_ptr = this->ptr();
  this->used += sizeof(T) * count;
  return std::span<T>((T*)out_ptr, count);
}

template <typename T>
bool Arena::write(std::span<T> data) {
  assert(this->can_write_size(data.size_bytes()))
      assert(data.size_bytes() < this->bytes_left());
  std::memcpy(this->ptr(), data.data(), data.size_bytes());

  this->used += data.size_bytes();
  return true;
}

template <typename T>
bool Arena::write(T* data) {
  assert(this->can_write_size(sizeof(T)));
  assert(sizeof(T) < this->bytes_left());
  std::memcpy(this->ptr(), data, sizeof(T));

  this->used += sizeof(T);
  return true;
}

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

namespace Io {
std::optional<std::span<uint8_t>> read_entire_file_alloc(Arena* arena,
                                                         const char* path) {
  assert(path != nullptr);
  SDL_IOStream* stream = SDL_IOFromFile(path, "rb");
  if (stream == nullptr) {
    SDL_LogError(1, "Unable to read file %s: %s", path, SDL_GetError());
    return {};
  }

  int64_t file_size = SDL_GetIOSize(stream);
  if (file_size < 0) {
    SDL_LogError(1, "Unable to get file size %s: %s", path, SDL_GetError());
    return {};
  }

  std::span<uint8_t> buffer = arena->push<uint8_t>((size_t)file_size);

  size_t bytes_read = SDL_ReadIO(stream, buffer.data(), (size_t)file_size);
  SDL_LogInfo(1, "bytes read: %ld", bytes_read);

  SDL_CloseIO(stream);

  return buffer;
}
}  // namespace Io

namespace Leb128 {
/*
 * Für Erklärungen und die Algorithmen unten siehe:
 * 	https://en.wikipedia.org/wiki/LEB128
 * 	https://webassembly.github.io/spec/core/binary/values.html#integers
 */
template <typename T>
struct Result {
  T num;
  size_t bytes_read;

  static Result empty() { return Result{0, 0}; };
};

template <typename T>
Result<T> read(std::span<uint8_t> buffer);

template <typename T>
Result<T> read(std::span<uint8_t> buffer)
  requires std::signed_integral<T>
{
  int shift_pos = 0;
  int64_t res = 0;
  int result_bit_size = sizeof(T) * 4;
  uint8_t current_byte;
  size_t position = 0;

  do {
    if (position >= buffer.size_bytes()) {
      Result<T>::empty();
    }
    current_byte = buffer[position];
    res |= (int64_t)(current_byte & 0x7f) << shift_pos;

    position += 1;
    shift_pos += 7;
  } while ((current_byte & 0x80) != 0);

  if ((shift_pos < result_bit_size) && (current_byte & 0x40) != 0) {
    res |= (int64_t)(~0 << shift_pos);
  }
  assert(position >= 0);

  return Result{(T)res, position};
}

template <typename T>
Result<T> read(std::span<uint8_t> buffer)
  requires std::unsigned_integral<T>
{
  uint64_t position = 0;
  int shift_pos = 0;
  uint64_t res = 0;
  uint8_t current_byte;

  do {
    if (position >= buffer.size_bytes()) {
      return Result<T>::empty();
    }
    current_byte = buffer[position];

    res |= (uint64_t)(current_byte & 0x7f) << shift_pos;

    position += 1;
    shift_pos += 7;
  } while ((current_byte & 0x80) != 0);

  return Result{(T)(res), position};
}
}  // namespace Leb128

#endif
