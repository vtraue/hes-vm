#include <SDL3/SDL.h>
#include <SDL3/SDL_filesystem.h>
#include <SDL3/SDL_log.h>
#include <SDL3/SDL_messagebox.h>
#include <SDL3/SDL_oldnames.h>
#include <SDL3/SDL_stdinc.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "arena.hpp"
#include "bytecode/bytecode.hpp"
#include "bytecode/bytecode_parser.hpp"
#include "bytecode/bytecode_reader.hpp"
#include "io.hpp"
#include "leb128.hpp"
#include "mem.h"
#include "os.h"
int main() {
  constexpr int leb_numbers[] = {0x45, 0x42, 0x30, 0x6C};
  std::span<uint8_t> leb_buffer =
      std::span<uint8_t>((uint8_t*)leb_numbers, sizeof(leb_numbers));

  int64_t res = 0;
  SDL_SetLogPriorities(SDL_LOG_PRIORITY_INFO);

  auto ures = Leb128::read<uint64_t>(leb_buffer);
  printf("%ld\n", ures.num);
  SDL_assert(ures.num == 69);

  res = Leb128::read<int64_t>(leb_buffer).num;
  printf("%ld\n", res);
  SDL_assert(res == -59);
  char* cwd = SDL_GetCurrentDirectory();
  printf("dir: %s\n", cwd);
  SDL_free(cwd);
  Arena* arena = Arena::create(MB(5));
  auto test_file = Io::read_entire_file_alloc(arena, "test.wasm");
  SDL_assert(test_file);

  auto reader = Bytecode::Reader::from_buffer(test_file.value());

  auto parser = Bytecode::Parser(arena);

  os_assert(parser.check_header(reader));
  os_assert(parser.check_version(reader));
  os_assert(parser.parse(reader));

  arena->destroy();
}
