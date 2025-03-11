#pragma once
#include <cassert>
#include <cstdint>
#include <cstring>
#include <span>

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

