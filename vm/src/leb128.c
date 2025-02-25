#include "leb128.h"
#include "os.h"
#include "util.h"
/*
* Für Erklärungen und die Algorithmen unten siehe: 
* 	https://en.wikipedia.org/wiki/LEB128
* 	https://webassembly.github.io/spec/core/binary/values.html#integers
*/

size_t leb128_read_u64(const uint8_t* buffer, uint64_t buffer_position, int64_t buffer_size, uint64_t* result) {
	os_assert(buffer != nullptr);
	os_assert(result != nullptr);
	os_assert(buffer_size != 0);
	os_assert(buffer_position < buffer_size);	

	size_t position = buffer_position; 
	int shift_pos = 0;
	uint64_t res = 0;
	uint8_t current_byte = buffer[position];		

	do {
		if(position >= buffer_size) {
			return 0;
		}
		current_byte = buffer[position]; 

		res |= (uint64_t) (current_byte & 0x7f) << shift_pos;

		position += 1;
		shift_pos += 7;
	} while ((current_byte & 0x80) != 0);

	*result = res;

	return position - buffer_position; 
}

size_t leb128_read_i64(const uint8_t* buffer, uint64_t buffer_position, int64_t buffer_size, int64_t* result) {
	os_assert(buffer != nullptr);
	os_assert(result != nullptr);
	os_assert(buffer_size != 0);
	os_assert(buffer_position < buffer_size);	

	size_t position = buffer_position; 
	int shift_pos = 0;
	uint64_t res = 0;
	int result_bit_size = BIT_SIZE_OF(int64_t); 	
	uint8_t current_byte = buffer[position]; 
	
	do {
		if(position >= buffer_size) {
			return 0;
		}
		current_byte = buffer[position];
		res |= (uint64_t) (current_byte & 0x7f) << shift_pos;


		position += 1;
		shift_pos += 7;
	} while ((current_byte & 0x80) != 0);

	if ((shift_pos < result_bit_size) && (current_byte & 0x40) != 0) {
		res |= (~0 << shift_pos);
	}

	*result = res;
	return position - buffer_position;
}
