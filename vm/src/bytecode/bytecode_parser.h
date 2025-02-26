#pragma once
#include "bytecode_reader.h"

bool bytecode_check_header(Bytecode_Reader* reader);
bool bytecode_check_version(Bytecode_Reader* reader);

