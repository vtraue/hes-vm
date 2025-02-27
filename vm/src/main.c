#include <SDL3/SDL.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_messagebox.h>
#include <stddef.h>
#include <stdio.h>

#include "arena.h"
#include "bytecode/bytecode.h"
#include "bytecode/bytecode_parser.h"
#include "bytecode/bytecode_reader.h"
#include "io.h"
#include "leb128.h"
#include "os.h"
int main() {
  int leb_numbers[] = {0x45, 0x42, 0x30, 0x6C};
  int64_t res = 0;

  uint64_t ures = 0;
  leb128_read_u64((uint8_t*)leb_numbers, 0, sizeof(leb_numbers), &ures);
  printf("%ld\n", ures);
  SDL_assert(ures == 69);

  leb128_read_i64((uint8_t*)leb_numbers, 0, sizeof(leb_numbers), &res);
  printf("%ld\n", res);
  SDL_assert(res == -59);
  char* cwd = SDL_GetCurrentDirectory();
  printf("dir: %s\n", cwd);
  size_t wasm_file_size = 0;
  uint8_t* wasm_file_data = nullptr;
  Arena* arena = arena_create(MB(5));
  SDL_assert(io_read_entire_file(arena, "test.wasm", &wasm_file_size,
                                 &wasm_file_data));

  Bytecode_Reader reader = {
      .data_size = wasm_file_size,
      .data = wasm_file_data,
  };
  os_assert(bytecode_check_header(&reader));
  os_assert(bytecode_check_version(&reader));
  os_assert(bytecode_parse(arena, &reader));

  arena_destroy(arena);
}
