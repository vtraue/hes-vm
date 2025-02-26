#pragma once
#include "arena.h"

bool io_read_entire_file(Arena* arena, const char* path, size_t* file_size, uint8_t** file_data);
