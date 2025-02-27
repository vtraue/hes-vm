#include "bytecode_parser.h"

#include <SDL3/SDL_log.h>

#include "../util.h"
#include "bytecode.h"
#include "bytecode_reader.h"
bool bytecode_check_header(Bytecode_Reader* reader) {
  os_assert(reader != nullptr);

  uint8_t header_data[4] = {0x0, 0x61, 0x73, 0x6D};

  for (uint64_t i = 0; i < COUNT_OF(header_data); ++i) {
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

  for (uint64_t i = 0; i < COUNT_OF(version_data); ++i) {
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

bool bytecode_parse_type_id(uint8_t id, Bytecode_Type_Id* out_id) {
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
    for (size_t i = 0; i < func_type.param_count; i++) {
      uint8_t id = bytecode_read_byte(reader);
      if (!bytecode_parse_type_id(id, &func_type.param_types[i])) {
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
    for (size_t i = 0; i < func_type.return_count; i++) {
      uint8_t id = bytecode_read_byte(reader);
      if (!bytecode_parse_type_id(id, &func_type.return_types[i])) {
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
  UNUSED(section_size);
  os_assert(out_type_section != nullptr);
  os_assert(arena != nullptr);
  os_assert(bytecode_reader_can_read(reader));
  uint64_t function_type_count = bytecode_read_var_uint(reader);

  if (function_type_count > 0) {
    out_type_section->type_count = function_type_count;
    out_type_section->function_types =
        arena_push_count(arena, Bytecode_Function_Type, function_type_count);

    for (uint64_t i = 0; i < function_type_count; i++) {
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
bool bytecode_parse_type_idx(Bytecode_Reader* reader, uint64_t type_count,
                             uint32_t* out_type_id) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_type_id != nullptr);
  os_assert(type_count >= 0);
  *out_type_id = (uint32_t)bytecode_read_var_uint(reader);
  if (*out_type_id > (uint32_t)type_count) {
    SDL_LogError(1, "Invalid type id out of scope");
    return false;
  }
  return true;
}

bool bytecode_parse_function_section(
    Arena* arena, Bytecode_Reader* reader, uint64_t type_count,
    Bytecode_Function_Section* out_function_section) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_function_section != nullptr);
  os_assert(arena != nullptr);

  out_function_section->function_count = bytecode_read_var_uint(reader);
  out_function_section->type_idx =
      arena_push_count(arena, uint32_t, out_function_section->function_count);

  if (out_function_section->function_count > 0) {
    if (type_count <= 0) {
      SDL_LogError(1, "Mismatched function / type length");
      return false;
    }

    for (uint64_t i = 0; i < out_function_section->function_count; i++) {
      bool type_ok = bytecode_parse_type_idx(
          reader, type_count, &out_function_section->type_idx[i]);
      if (!type_ok) {
        SDL_LogError(1, "Unable to read type id");
        return false;
      }
    }
  }
  return true;
}

void bytecode_set_section_parsed(Bytecode_Parser* parser,
                                 Bytecode_Section_Id id) {
  parser->main_sections_present |= 1 << (uint16_t)(id);
}

bool bytecode_is_section_parsed(Bytecode_Parser* parser,
                                Bytecode_Section_Id id) {
  return parser->main_sections_present & (1 << (uint16_t)(id));
}

bool bytecode_parse_string(Arena* arena, Bytecode_Reader* reader,
                           uint64_t* out_string_length, char** out_string) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(arena != nullptr);
  os_assert(out_string_length != nullptr);
  os_assert(out_string != nullptr);

  uint64_t str_len = bytecode_read_var_uint(reader);
  char* string_data =
      (char*)bytecode_read_bytes_zero_term(arena, reader, str_len);
  *out_string_length = str_len;
  *out_string = string_data;
  return true;
}

bool bytecode_parse_export(Bytecode_Parser* parser, Bytecode_Reader* reader,
                           Bytecode_Export* out_export) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_export != nullptr);

  uint64_t export_name_len = 0;
  char* export_name = nullptr;

  os_assert(bytecode_parse_string(parser->arena, reader, &export_name_len,
                                  &export_name));
  uint8_t desc_type_id = bytecode_read_byte(reader);
  if (desc_type_id >= (uint8_t)Bytecode_Export_Desc_Type_Enum_Len) {
    SDL_LogError(1, "Malfored export desc type");
    return false;
  }
  Bytecode_Export_Desc_Type desc_type = (Bytecode_Export_Desc_Type)desc_type_id;
  switch (desc_type) {
    case Bytecode_Export_Desc_Type_Funcidx:
      if (!bytecode_is_section_parsed(parser, Bytecode_Section_Id_Function)) {
        SDL_LogError(1, "Malformed module: No function section");
        return false;
      }
      uint64_t func_id = bytecode_read_var_uint(reader);

      if (func_id > parser->function_section.function_count) {
        SDL_LogError(1, "Export: Function id out of scope");
        return false;
      }
      out_export->desc.id = func_id;
      break;
    default:
      SDL_LogError(1, "Unimplemented");
      break;
  }
  out_export->desc.type = desc_type;
  out_export->name = export_name;
  out_export->name_length = export_name_len;

  return true;
}

bool bytecode_parse_export_section(Bytecode_Parser* parser,
                                   Bytecode_Reader* reader,
                                   Bytecode_Export_Section* out_section) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_section != nullptr);
  uint64_t export_count = bytecode_read_var_uint(reader);
  out_section->export_count = export_count;
  out_section->export =
      arena_push_count(parser->arena, Bytecode_Export, export_count);
  if (export_count > 0) {
    for (uint64_t i = 0; i < export_count; i++) {
      os_assert(bytecode_parse_export(parser, reader, &out_section->export[i]));
    }
  }
  return true;
}

bool bytecode_parse_section(Bytecode_Reader* reader, Bytecode_Parser* parser) {
  os_assert(parser->arena != nullptr);
  os_assert(bytecode_reader_can_read(reader));
  Bytecode_Section_Id section_id = 0;
  if (!bytecode_parse_section_id(reader, &section_id)) {
    SDL_LogError(1, "Malformed section id");
    return false;
  }
  uint64_t section_size = bytecode_read_var_uint(reader);
  if (section_size > 0) {
    switch (section_id) {
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
        bytecode_parse_type_section(parser->arena, reader, section_size,
                                    &parser->type_section);

        bytecode_set_section_parsed(parser, Bytecode_Section_Id_Type);
        break;

      case Bytecode_Section_Id_Function:
        SDL_LogInfo(1, "Reading function section");
        if (!bytecode_is_section_parsed(parser, Bytecode_Section_Id_Type)) {
          SDL_LogInfo(1, "Sections declared out of order");
          return false;
        }
        if (!bytecode_parse_function_section(parser->arena, reader,
                                             parser->type_section.type_count,
                                             &parser->function_section)) {
          SDL_LogInfo(1, "Unable to parse function section");
          return false;
        }
        bytecode_set_section_parsed(parser, section_id);
        break;

      case Bytecode_Section_Id_Export:
        SDL_LogInfo(1, "Reading Export Section");
        if (!bytecode_parse_export_section(parser, reader,
                                           &parser->export_section)) {
          SDL_LogError(1, "Unable to read export section");
          return false;
        }
        bytecode_set_section_parsed(parser, section_id);
        break;
    }
  }
  return true;
}

bool bytecode_parse(Arena* arena, Bytecode_Reader* reader) {
  Bytecode_Parser parser = {0};
  parser.arena = arena;
  bool section_ok = bytecode_parse_section(reader, &parser);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  os_assert(bytecode_is_section_parsed(&parser, Bytecode_Section_Id_Type));
  os_assert(parser.type_section.type_count == 1);
  os_assert(parser.type_section.function_types->param_count == 2);
  os_assert(parser.type_section.function_types->return_count == 1);
  os_assert(parser.type_section.function_types->return_types[0] ==
            Bytecode_Type_Id_Num_I32);

  section_ok = bytecode_parse_section(reader, &parser);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section 2");
    return false;
  }
  os_assert(bytecode_is_section_parsed(&parser, Bytecode_Section_Id_Function));
  os_assert(parser.function_section.function_count == 1);

  section_ok = bytecode_parse_section(reader, &parser);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section 3");
    return false;
  }
  os_assert(bytecode_is_section_parsed(&parser, Bytecode_Section_Id_Export));
  os_assert(parser.export_section.export_count == 1);
  SDL_LogInfo(1, "Export fn name: %s", parser.export_section.export[0].name);
  return true;
}
