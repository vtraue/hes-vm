#include "bytecode_reader.h"

#include "../leb128.h"
#include "../os.h"

bool bytecode_reader_can_read(Bytecode_Reader* reader) {
  return reader != nullptr && reader->current_position >= 0 &&
         reader->current_position < (int64_t)reader->data_size;
}

uint8_t* bytecode_bytes_at(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  return reader->data + reader->current_position;
}

void bytecode_reader_skip_bytes(Bytecode_Reader* reader, uint64_t offset) {
  os_assert(reader != nullptr);
  os_assert((reader->current_position + (int64_t)offset) <
            (int64_t)reader->data_size);
  reader->current_position += offset;
}

bool bytecode_read_bytes_into(Bytecode_Reader* reader, size_t count,
                              size_t buffer_size, uint8_t* buffer) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(count <= buffer_size);
  os_memcpy(buffer, bytecode_bytes_at(reader), count);
  reader->current_position += count;

  return true;
}

uint8_t* bytecode_read_bytes_alloc(Arena* arena, Bytecode_Reader* reader,
                                   size_t count) {
  os_assert(arena != nullptr);
  os_assert(reader != nullptr);
  os_assert(reader->current_position + (int64_t)count <
            (int64_t)(reader->data_size));

  uint8_t* buffer = arena_alloc(arena, count);
  os_assert(bytecode_read_bytes_into(reader, count, count, buffer));

  return buffer;
}

uint8_t* bytecode_read_bytes_zero_term(Arena* arena, Bytecode_Reader* reader,
                                       size_t count) {
  os_assert(arena != nullptr);
  os_assert(reader != nullptr);
  os_assert(reader->current_position + (int64_t)count <
            (int64_t)(reader->data_size));

  uint8_t* buffer = arena_alloc(arena, count + 1);
  os_memcpy(buffer, bytecode_bytes_at(reader), count);

  reader->current_position += count;
  buffer[count + 1] = 0;

  return buffer;
}
uint8_t bytecode_read_byte(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  os_assert(bytecode_reader_can_read(reader));
  uint8_t out_val = *bytecode_bytes_at(reader);
  reader->current_position += 1;
  return out_val;
}

int64_t bytecode_read_var_int(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  int64_t out_int = 0;
  size_t bytes_read = leb128_read_i64(reader->data, reader->current_position,
                                      (int64_t)reader->data_size, &out_int);

  os_assert(bytes_read != 0);
  reader->current_position += bytes_read;
  return out_int;
}

uint64_t bytecode_read_var_uint(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);
  uint64_t out_int = 0;
  size_t bytes_read = leb128_read_u64(reader->data, reader->current_position,
                                      (int64_t)(reader->data_size), &out_int);

  os_assert(bytes_read != 0);
  reader->current_position += bytes_read;
  return out_int;
}
