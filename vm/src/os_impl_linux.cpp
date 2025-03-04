#include <sys/mman.h>
#include <unistd.h>

#include <cassert>
#include <cstdio>
#include <span>
#include <string_view>

#include "os.hpp"

namespace Os::Mem {
std::span<uint8_t> reserve(size_t size) {
  assert(size > 0);
  void* out_ptr = mmap(nullptr, size, PROT_READ | PROT_WRITE,
                       MAP_ANON | MAP_PRIVATE, -1, 0);
  assert(out_ptr != MAP_FAILED);
  assert(out_ptr != nullptr);
  return {reinterpret_cast<uint8_t*>(out_ptr), size};
}

void unreserve(std::span<uint8_t> data) {
  assert(munmap(reinterpret_cast<void*>(data.data()), data.size_bytes()) == 0);
}
}  // namespace Os::Mem

namespace Os::Log {
void log(std::string_view string) {
  assert(write(STDOUT_FILENO, reinterpret_cast<const void*>((string.data())),
               string.size()) != -1);
}
}  // namespace Os::Log
