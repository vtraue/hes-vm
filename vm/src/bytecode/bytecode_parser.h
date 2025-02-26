#pragma once
#include "bytecode_reader.h"
#include "bytecode.h"

typedef struct Bytecode_Parser {
	Arena* arena;
	Bytecode_Type_Section type_section;	

} Bytecode_Parser;

bool bytecode_check_header(Bytecode_Reader* reader);
bool bytecode_check_version(Bytecode_Reader* reader);

