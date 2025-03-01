#pragma once
#include "mem.h"
#include "os.h"
typedef struct Arena Arena;

struct Arena {
  size_t used;
  size_t cap;
  uint8_t* data;
};

Arena* arena_create(size_t cap);

uint8_t* arena_alloc(Arena* arena, size_t size_bytes);
void arena_reset(Arena* arena);
void arena_destroy(Arena* arena);
// char* arena_sprintf(Arena* arena, const char *format, ...);
#define arena_push_count(arena, T, count) \
  (T*)(arena_alloc(arena, sizeof(T) * count))

uint8_t* arena_get_ptr(Arena* arena);
void arena_reserve(Arena* arena, size_t size_bytes);

size_t arena_get_mem_left(Arena* arena);
bool arena_write_bytes(Arena* arena, uint8_t* source, size_t size_bytes);

bool arena_write_byte(Arena* arena, uint8_t b);
#define arena_write(arena, ptr) \
  arena_write_bytes(arena, (uint8_t*)ptr, sizeof(*ptr))
#define arena_copy_struct(arena, ptr) \
  arena_write_bytes(arena, (uint8_t*)ptr, sizeof(*ptr))
