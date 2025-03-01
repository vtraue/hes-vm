#pragma once
#include "mem.h"
#include "os.h"
typedef struct Arena Arena;

/*
struct Arena_Region {
        Arena* next;

        size_t used;
        size_t cap;
        uint8_t* data;
};
*/
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
