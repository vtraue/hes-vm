#pragma once
#include <stdint.h>

#include <optional>

#include "arena.hpp"

namespace Io {
std::optional<std::span<uint8_t>> read_entire_file_alloc(Arena* arena,
                                                         const char* path);
}
