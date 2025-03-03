#pragma once
#include <stddef.h>
#include <stdint.h>

#include <concepts>
#include <span>
#include <utility>

#include "os.h"
/*
 * Für Erklärungen und die Algorithmen unten siehe:
 * 	https://en.wikipedia.org/wiki/LEB128
 * 	https://webassembly.github.io/spec/core/binary/values.html#integers
 */

namespace Leb128 {
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
    res |= (uint64_t)(current_byte & 0x7f) << shift_pos;

    position += 1;
    shift_pos += 7;
  } while ((current_byte & 0x80) != 0);

  if ((shift_pos < result_bit_size) && (current_byte & 0x40) != 0) {
    res |= (uint64_t)(~0 << shift_pos);
  }
  os_assert(position >= 0);

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
