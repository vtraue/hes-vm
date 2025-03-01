#pragma once
#include <SDL3/SDL.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#define os_assert(x) SDL_assert(x);
void os_crash_with_message(char* msg);
uint8_t* os_mem_reserve(size_t size);
void os_mem_unreserve(uint8_t* ptr, size_t size);
void* os_memcpy(void* dst, const void* src, size_t len);
bool buffer_copy(void* dest, size_t dest_size, void* src, size_t count);
