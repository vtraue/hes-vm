#pragma once
#include "bytecode_reader.h"
#include "bytecode.h"

typedef struct Bytecode_Parser {
	Arena* arena;
	Bytecode_Type_Section type_section;	
	Bytecode_Function_Section function_section;
	uint16_t main_sections_present;
} Bytecode_Parser;

bool bytecode_check_header(Bytecode_Reader* reader);
bool bytecode_check_version(Bytecode_Reader* reader);
bool bytecode_parse_section(Bytecode_Reader* reader, Bytecode_Parser* parser);
bool bytecode_parse(Arena* arena, Bytecode_Reader* reader);

void bytecode_set_section_parsed(Bytecode_Parser* parser, Bytecode_Section_Id id);
bool bytecode_is_section_parsed(Bytecode_Parser* parser, Bytecode_Section_Id id);
