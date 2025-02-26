#include "io.h"

#include <SDL3/SDL.h>
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_iostream.h>
#include <SDL3/SDL_log.h>

#include "arena.h"

bool io_read_entire_file(Arena* arena, const char* path, size_t* out_file_size,
                         uint8_t** out_file_data) {
  os_assert(path != nullptr);
  os_assert(out_file_size != nullptr);
  os_assert(out_file_data != nullptr);

  SDL_IOStream* stream = SDL_IOFromFile(path, "rb");
  if (stream == nullptr) {
    SDL_LogError(1, "Unable to read file %s: %s", path, SDL_GetError());
    return false;
  }

  int64_t file_size = SDL_GetIOSize(stream);
  if (file_size < 0) {
    SDL_LogError(1, "Unable to get file size %s: %s", path, SDL_GetError());
    return false;
  }

  uint8_t* buffer = arena_alloc(arena, file_size);

  size_t bytes_read = SDL_ReadIO(stream, buffer, file_size);
  SDL_LogInfo(1, "bytes read: %ld", bytes_read);

  SDL_CloseIO(stream);
  *out_file_size = file_size;
  *out_file_data = buffer;

  return true;
}
