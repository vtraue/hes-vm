#include <cstdint>
#include <span>

struct Arena {
  size_t used;
  std::span<uint8_t> data;

  static Arena* create(size_t cap);
  bool can_write_size(size_t bytes);

  template <typename T>
  std::span<T> push(size_t count);

  uint8_t* ptr();
  void reset();
  void destroy();
  void reserve(size_t size);
  size_t bytes_left();

  template <typename T>
  bool write(std::span<T> data);
  template <typename T>
  bool write(T* data);

  bool write_byte(uint8_t b);
};

