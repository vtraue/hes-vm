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
#include "arena.hpp"

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
