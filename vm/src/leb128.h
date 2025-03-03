#pragma once
#include <stddef.h>
#include <stdint.h>

// TODO:(joh): leb128 -> u32

/*
 * Lese ein leb128 encodiertes signed int aus buffer an der Stelle
 * buffer_position mit der Größe buffer_size. Rückgabewert: Anzahl der gelesenen
 * Bytes
 */
size_t leb128_read_i64(const uint8_t* buffer, int64_t buffer_position,
                       int64_t buffer_size, int64_t* result);

/*
 * Lese ein leb128 encodiertes unsigned int aus buffer an der Stelle
 * buffer_position mit der Größe buffer_size. Rückgabewert: Anzahl der gelesenen
 * Bytes
 */
size_t leb128_read_u64(const uint8_t* buffer, int64_t buffer_position,
                       int64_t buffer_size, uint64_t* result);
