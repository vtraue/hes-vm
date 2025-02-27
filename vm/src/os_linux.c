#include <SDL3/SDL.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_messagebox.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>

#include "os.h"

uint8_t* os_mem_reserve(size_t size) {
  os_assert(size > 0);
  // TODO: (joh) Reservieren ohne commit mit PROT_NONE
  void* out_ptr =
      mmap(0, size, PROT_READ | PROT_WRITE, MAP_ANON | MAP_PRIVATE, -1, 0);
  os_assert(out_ptr != MAP_FAILED);
  os_assert(out_ptr != nullptr);

  return (uint8_t*)out_ptr;
}

void os_mem_unreserve(uint8_t* ptr, size_t size) {
  os_assert(ptr != nullptr);
  os_assert(size > 0);

  os_assert(munmap((void*)ptr, size) == 0);
}

void os_crash_with_message(char* message) {
  fprintf(stderr, "PANIC: %s\n", message);
  SDL_ShowSimpleMessageBox(SDL_MESSAGEBOX_ERROR, "Critical Error!", message,
                           nullptr);
  exit(1);
}

void* os_memcpy(void* dst, const void* src, size_t len) {
  return __builtin_memcpy(dst, src, len);
}

int os_strcmp(const char* str1, const char* str2) {
  return __builtin_strcmp(str1, str2);
}

/*
size_t os_get_file_size(const char* path) {
        SDL_PathInfo file_info = {0};
        SDL_GetPathInfo(path, &file_info);

}

*/
