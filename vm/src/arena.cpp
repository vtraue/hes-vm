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
bool arena_can_write_size(Arena *arena, size_t size_bytes) {
  os_assert(arena != nullptr);
  os_assert(arena->data != nullptr);
  os_assert(arena->used <= arena->cap);
  if (arena->used + size_bytes >= arena->cap) {
    os_crash_with_message("Arena out of memory!");
    return false;
  }
  return true;
}
uint8_t *arena_alloc(Arena *arena, size_t size_bytes) {
  if (!arena_can_write_size(arena, size_bytes)) {
    return nullptr;
  }
  uint8_t *out_ptr = arena_get_ptr(arena);
  arena->used += size_bytes;
  return out_ptr;
}

void arena_reset(Arena *arena) { arena->used = 0; }

void arena_destroy(Arena *arena) {
  os_mem_unreserve((uint8_t *)arena, arena->cap);
}

uint8_t *arena_get_ptr(Arena *arena) { return arena->data + arena->used; }

void arena_reserve(Arena *arena, size_t size_bytes) {
  if (arena->used + size_bytes >= arena->cap) {
    os_crash_with_message("Arena out of memory!");
    return;
  }
  arena->used += size_bytes;
}

size_t arena_get_mem_left(Arena *arena) { return arena->cap - arena->used; }

bool arena_write_byte(Arena *arena, uint8_t b) {
  if (!arena_can_write_size(arena, 1)) {
    return false;
  }
  *arena_get_ptr(arena) = b;
  arena->used += 1;
  return true;
}

bool arena_write_bytes(Arena *arena, uint8_t *source, size_t size_bytes) {
  if (!arena_can_write_size(arena, 1)) {
    return false;
  }

  if (arena->used + size_bytes >= arena->cap) {
    os_crash_with_message("Arena out of memory!");
    return false;
  }

  if (!buffer_copy(arena_get_ptr(arena), arena_get_mem_left(arena), source,
                   size_bytes)) {
    return false;
  }
  arena->used += size_bytes;
  return true;
}
