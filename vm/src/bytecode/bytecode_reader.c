#include "bytecode_reader.h"
#include "../os.h"
#include "../leb128.h"
#include "../util.h"

bool bytecode_can_read(Bytecode_Reader* reader) {
	return reader != nullptr && reader->current_position < reader->data_size; 
}

uint8_t* bytecode_bytes_at(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);
	return reader->data + reader->current_position;
}

void bytecode_seek(Bytecode_Reader* reader, int64_t offset) {
	os_assert(reader != nullptr);
	os_assert(reader->current_position + offset < reader->data_size);
	reader->current_position += offset;
}

uint8_t* bytecode_read_bytes(Arena* arena, Bytecode_Reader* reader, size_t count) {
	os_assert(arena != nullptr);	
	os_assert(reader != nullptr);
	os_assert(reader->current_position + count < reader->data_size); 

	uint8_t* buffer = arena_alloc(arena, count);
	os_memcpy(buffer, bytecode_bytes_at(reader), count); 	

	reader->current_position += count;
	
	return buffer;
}

uint8_t bytecode_read_byte(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);
	os_assert(bytecode_can_read(reader))
	uint8_t out_val = *bytecode_bytes_at(reader);
	reader->current_position += 1;
	return out_val;
}

int64_t bytecode_read_var_int(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);	
	int64_t out_int = 0;
	size_t bytes_read = leb128_read_i64(reader->data, reader->current_position, reader->data_size, &out_int);

	os_assert(bytes_read != 0);
	reader->current_position += bytes_read;
	return out_int;
}

uint64_t bytecode_read_var_uint(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);	
	uint64_t out_int = 0;
	size_t bytes_read = leb128_read_u64(reader->data, reader->current_position, reader->data_size, &out_int);

	os_assert(bytes_read != 0);
	reader->current_position += bytes_read;
	return out_int;
}

bool bytecode_check_header(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);
	
	uint8_t header_data[4] = {0x0, 0x61, 0x73, 0x6D};

	for(int i = 0; i < COUNT_OF(header_data); ++i) {
		uint8_t val = bytecode_read_byte(reader);
		if(val != header_data[i]) {
			return false;
		}
	}
	return true;
}

