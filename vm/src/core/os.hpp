#ifndef OS_HPP
#define OS_HPP
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <span>
#include <string_view>

namespace Os::Mem {
constexpr size_t KB(size_t s) { return s << 10; }
constexpr size_t MB(size_t s) { return s << 20; }

std::span<uint8_t> reserve(size_t size);
void unreserve(std::span<uint8_t> data);
}  // namespace Os::Mem

namespace Os {
void log(std::string_view string);
}

#endif
