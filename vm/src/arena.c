#include "arena.h"

#include <stdint.h>

#include "os.h"
// TODO: (joh) Arenas sollten später wachsen können

Arena *arena_create(size_t cap) {
  os_assert(cap != 0);
  size_t reserve_size = (cap + sizeof(Arena));
  uint8_t *data = os_mem_reserve(reserve_size);

  Arena *arena = (Arena *)data;

  arena->data = data + sizeof(Arena);
  arena->cap = cap;
  arena->used = 0;
  return arena;
}

uint8_t *arena_alloc(Arena *arena, size_t size_bytes) {
  os_assert(arena != nullptr);
  os_assert(arena->data != nullptr);
  os_assert(arena->used <= arena->cap);
  if (arena->used + size_bytes >= arena->cap) {
    os_crash_with_message("Arena out of memory!");
  }

  uint8_t *out_ptr = arena->data + arena->used;
  arena->used += size_bytes;
  return out_ptr;
}

void arena_reset(Arena *arena) { arena->used = 0; }

void arena_destroy(Arena *arena) {
  os_mem_unreserve((void *)arena, arena->cap);
}
