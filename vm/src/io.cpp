#include "io.hpp"

#include <SDL3/SDL.h>
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_iostream.h>
#include <SDL3/SDL_log.h>

#include <cstdint>
#include <optional>
#include <span>

#include "arena.hpp"
#include "os.h"
std::optional<std::span<uint8_t>> Io::read_entire_file_alloc(Arena* arena,
                                                             const char* path) {
  os_assert(path != nullptr);
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
