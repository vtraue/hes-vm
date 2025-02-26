#pragma once
#include <stddef.h>
#include <stdint.h>
typedef enum Bytecode_Section_Id : uint8_t {
	Bytecode_Section_Id_Custom = 0,
	Bytecode_Section_Id_Type = 1,
	Bytecode_Section_Id_Import = 2,
	Bytecode_Section_Id_Function = 3,
	Bytecode_Section_Id_Table = 4,
	Bytecode_Section_Id_Memory = 5,
	Bytecode_Section_Id_Global = 6,
	Bytecode_Section_Id_Export = 7,
	Bytecode_Section_Id_Start = 8,
	Bytecode_Section_Id_Element = 9,
	Bytecode_Section_Id_Code = 10,
	Bytecode_Section_Id_Data = 11,
	Bytecode_Section_Id_Data_Count = 12,
} Bytecode_Section_Id ;

typedef enum Bytecode_Type_Id : uint8_t {
	Bytecode_Type_Id_Num_I32 = 0x7F,
	Bytecode_Type_Id_Num_I64 = 0x7E,
	Bytecode_Type_Id_Num_F32 = 0x7D,
	Bytecode_Type_Id_Num_F64 = 0x7C,
	Bytecode_Type_Id_Vector = 0x7B,
	Bytecode_Type_Id_Ref_Funcref = 0x70,
	Bytecode_Type_Id_Ref_Externref = 0x6F,
} Bytecode_Type_Id ;

#define FUNCTION_TYPE_MAX_PARAMS 12 
#define FUNCTION_TYPE_MAX_RETURN 12

typedef struct Bytecode_Function_Type {
	size_t param_count;
	Bytecode_Type_Id param_types[FUNCTION_TYPE_MAX_PARAMS];

	size_t return_count;
	Bytecode_Type_Id return_types[FUNCTION_TYPE_MAX_RETURN];
} Bytecode_Function_Type ;
