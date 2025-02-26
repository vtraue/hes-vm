#include "bytecode_parser.h"
#include "../util.h"
#include "bytecode.h" 
#include "bytecode_reader.h"
#include <SDL3/SDL_log.h>
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

bool bytecode_check_version(Bytecode_Reader* reader) {
	os_assert(reader != nullptr);
	
	uint8_t version_data[4] = {0x01, 0x00, 0x00, 0x00};

	for(int i = 0; i < COUNT_OF(version_data); ++i) {
		uint8_t val = bytecode_read_byte(reader);
		if(val != version_data[i]) {
			return false;
		}
	}
	return true;
}

//?: (joh) Sollen wir die 0x60 hier mitlesen oder lieber doch aussen? 
bool bytecode_parse_function_type(Bytecode_Reader* reader, Bytecode_Function_Type* out_function_type) {
	Bytecode_Function_Type func_type = {0};
	os_assert(reader != nullptr);
	uint64_t param_count = bytecode_read_var_uint(reader);

	if(param_count != 0) {
		func_type.param_count = param_count;
		if(func_type.param_count >= FUNCTION_TYPE_MAX_PARAMS){
			SDL_LogError(1, "Parameter Count exceeds max params");
			return false;
		}
		for(int i = 0; i < func_type.param_count; i++) {
			func_type.param_types[i] = (Bytecode_Type_Id)(bytecode_read_byte(reader));
		}
	}

	uint64_t return_count = bytecode_read_var_uint(reader);

	if(return_count != 0) {
		func_type.return_count = return_count;
		if(func_type.return_count >= FUNCTION_TYPE_MAX_RETURN){
			SDL_LogError(1, "Return Val Count exceeds max return ");
			return false;
		}
		for(int i = 0; i < func_type.return_count; i++) {
			func_type.return_types[i] = (Bytecode_Type_Id)(bytecode_read_byte(reader));
		}
	}

	*out_function_type = func_type;

	return true;
}

