#include "bytecode_parser.h"

#include <SDL3/SDL_log.h>

#include "../util.h"
#include "bytecode.h"
#include "bytecode_reader.h"
bool bytecode_check_header(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);

  uint8_t header_data[4] = {0x0, 0x61, 0x73, 0x6D};

  for (int i = 0; i < COUNT_OF(header_data); ++i) {
    uint8_t val = bytecode_read_byte(reader);
    if (val != header_data[i]) {
      return false;
    }
  }
  return true;
}

bool bytecode_check_version(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);

  uint8_t version_data[4] = {0x01, 0x00, 0x00, 0x00};

  for (int i = 0; i < COUNT_OF(version_data); ++i) {
    uint8_t val = bytecode_read_byte(reader);
    if (val != version_data[i]) {
      return false;
    }
  }
  return true;
}
bool bytecode_parse_section_id(Bytecode_Reader* reader,
                               Bytecode_Section_Id* out_id) {
  uint8_t id = bytecode_read_byte(reader);
  if (id < 0 || id >= Bytecode_Section_Id_Enum_Max) {
    return false;
  }
  *out_id = id;
  return true;
}

bool bytecode_parse_type_id(Bytecode_Reader* reader, uint8_t id,
                            Bytecode_Type_Id* out_id) {
  if (id > Bytecode_Type_Id_Num_I32 || id < Bytecode_Type_Id_Ref_Externref) {
    return false;
  }
  *out_id = id;
  return true;
}

//?: (joh) Sollen wir die 0x60 hier mitlesen oder lieber doch aussen?
bool bytecode_parse_function_type(Bytecode_Reader* reader,
                                  Bytecode_Function_Type* out_function_type) {
  Bytecode_Function_Type func_type = {0};
  os_assert(reader != nullptr);
  uint64_t param_count = bytecode_read_var_uint(reader);

  if (param_count != 0) {
    func_type.param_count = param_count;
    if (func_type.param_count >= FUNCTION_TYPE_MAX_PARAMS) {
      SDL_LogError(1, "Parameter Count exceeds max params");
      return false;
    }
    for (int i = 0; i < func_type.param_count; i++) {
      uint8_t id = bytecode_read_byte(reader);
      if (!bytecode_parse_type_id(reader, id, &func_type.param_types[i])) {
        SDL_LogError(1, "Invalid type id: %d",
                     (uint8_t)func_type.param_types[i]);
        return false;
      }
    }
  }

  uint64_t return_count = bytecode_read_var_uint(reader);

  if (return_count != 0) {
    func_type.return_count = return_count;
    if (func_type.return_count >= FUNCTION_TYPE_MAX_RETURN) {
      SDL_LogError(1, "Return Val Count exceeds max return ");
      return false;
    }
    for (int i = 0; i < func_type.return_count; i++) {
      uint8_t id = bytecode_read_byte(reader);
      if (!bytecode_parse_type_id(reader, id, &func_type.return_types[i])) {
        SDL_LogError(1, "Invalid type id");
        return false;
      }
    }
  }

  *out_function_type = func_type;

  return true;
}

bool bytecode_parse_type_section(Arena* arena, Bytecode_Reader* reader,
                                 size_t section_size,
                                 Bytecode_Type_Section* out_type_section) {
  os_assert(out_type_section != nullptr);
  os_assert(arena != nullptr);
  os_assert(bytecode_reader_can_read(reader)) uint64_t function_type_count =
      bytecode_read_var_uint(reader);
  if (function_type_count > 0) {
    out_type_section->type_count = function_type_count;
    out_type_section->function_types =
        arena_push_count(arena, Bytecode_Function_Type, function_type_count);

    for (int i = 0; i < function_type_count; i++) {
      uint8_t func_type_header = bytecode_read_byte(reader);
      if (func_type_header != FUNCTYPE_HEADER) {
        SDL_LogError(
            1, "Malformed function type: Wrong header. Got %d, expected %d",
            func_type_header, FUNCTYPE_HEADER);
        return false;
      }
      bool func_type_ok = bytecode_parse_function_type(
          reader, &out_type_section->function_types[i]);
      if (!func_type_ok) {
        SDL_LogError(1, "Unable to parse function type");
      }
    }
  }
  return true;
}
bool bytecode_parse_type_idx(Bytecode_Reader* reader, int type_count,
                             int64_t* out_type_id) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_type_id != nullptr);

  *out_type_id = bytecode_read_var_uint(reader);
  if (*out_type_id > type_count) {
    SDL_LogError(1, "Invalid type id out of scope");
    return false;
  }
  return true;
}

/*
bool bytecode_parse_function_section(Arena* arena, Bytecode_Reader* reader,
Bytecode_Function_Section* out_function_section) {
        os_assert(bytecode_reader_can_read(reader));
        os_assert(out_function_section != nullptr);
        os_assert(arena != nullptr);

}
*/
bool bytecode_parse_section(Arena* arena, Bytecode_Reader* reader,
                            Bytecode_Section* out_section) {
  os_assert(arena != nullptr);
  os_assert(out_section != nullptr);
  os_assert(bytecode_reader_can_read(reader));
  if (!bytecode_parse_section_id(reader, &out_section->id)) {
    SDL_LogError(1, "Malformed section id");
    return false;
  }
  uint64_t section_size = bytecode_read_var_uint(reader);
  if (section_size > 0) {
    switch (out_section->id) {
      case Bytecode_Section_Id_Custom:
        SDL_LogInfo(1, "Skipping custom section");
        bytecode_reader_skip_bytes(reader, section_size);
        break;
      default:
        SDL_LogInfo(1, "Skipping unimplemented section");
        bytecode_reader_skip_bytes(reader, section_size);
        break;
      case Bytecode_Section_Id_Type:
        SDL_LogInfo(1, "Reading type section");
        bytecode_parse_type_section(arena, reader, section_size,
                                    &out_section->data.type_section);
        break;
    }
  }
  return true;
}
