#pragma once
#include <stdint.h>
#include <stddef.h>

//TODO:(joh): leb128 -> u32
size_t leb128_read_i64(const uint8_t* buffer, uint64_t buffer_position, int64_t buffer_size, int64_t* result);
size_t leb128_read_u64(const uint8_t* buffer, uint64_t buffer_position, int64_t buffer_size, uint64_t* result);

