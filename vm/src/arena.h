#pragma once
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include "os.h"
#include "mem.h"
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
