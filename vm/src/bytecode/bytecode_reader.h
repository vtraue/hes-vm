#pragma once
#include <stdint.h>
#include <stddef.h>
#include "../arena.h"
typedef struct Bytecode_Reader {
	uint8_t* data;
	size_t data_size;
	size_t current_position;

} Bytecode_Reader;



bool bytecode_reader_can_read(Bytecode_Reader* reader);
void bytecode_reader_skip_bytes(Bytecode_Reader* reader, int64_t offset);
uint8_t* bytecode_bytes_at(Bytecode_Reader* reader);
uint8_t* bytecode_read_bytes(Arena* arena, Bytecode_Reader* reader, size_t count);
uint8_t bytecode_read_byte(Bytecode_Reader* reader);
int64_t bytecode_read_var_int(Bytecode_Reader* reader);
uint64_t bytecode_read_var_uint(Bytecode_Reader* reader);

