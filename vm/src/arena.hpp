#pragma once

#include <stdint.h>

#include <span>

#include "os.h"

struct Arena {
  size_t used;
  size_t cap;
  uint8_t* data;

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
  os_assert(this->can_write_size(sizeof(T) * count));

  uint8_t* out_ptr = this->ptr();
  this->used += sizeof(T) * count;
  return std::span<T>((T*)out_ptr, count);
}

template <typename T>
bool Arena::write(std::span<T> data) {
  if (!this->can_write_size(data.size_bytes())) {
    os_crash_with_message("Arena out of memory!");
    return false;
  }

  if (!buffer_copy(this->ptr(), this->bytes_left(), data.data(),
                   data.size_bytes())) {
    return false;
  }
  this->used += data.size_bytes();
  return true;
}

template <typename T>
bool Arena::write(T* data) {
  if (!this->can_write_size(sizeof(T))) {
    os_crash_with_message("Arena out of memory!");
    return false;
  }

  if (!buffer_copy(this->ptr(), this->bytes_left(), data, sizeof(T))) {
    return false;
  }
  this->used += sizeof(T);
  return true;
}
