#include "bytecode_parser.hpp"

#include <SDL3/SDL_log.h>

#include <string_view>
#include <variant>

#include "../os.h"
#include "../util.h"
#include "bytecode.hpp"
#include "bytecode_reader.hpp"
#include "opcode.hpp"

namespace Bytecode {
std::optional<Type_Id> type_id_from_byte(uint8_t id) {
  if (id > static_cast<uint8_t>(Type_Id::Num_I32) ||
      id < static_cast<uint8_t>(Type_Id::Ref_Externref)) {
    return {};
  }

  return static_cast<Type_Id>(id);
}
std::optional<Section_Id> section_id_from_byte(uint8_t id) {
  if (id < static_cast<uint8_t>(Section_Id::Custom) ||
      id > static_cast<uint8_t>(Section_Id::Data_Count)) {
    return {};
  }

  return static_cast<Section_Id>(id);
}

bool Parser::check_header(Reader& reader) {
  for (int i : {0x0, 0x61, 0x73, 0x6D}) {
    uint8_t val = reader.get<uint8_t>();
    if (val != i) {
      SDL_LogError(1, "Check Header: Got %d, expected %d", val, i);
      return false;
    }
  }
  return true;
}

bool Parser::check_version(Reader& reader) {
  for (int i : {0x01, 0x00, 0x00, 0x00}) {
    uint8_t val = reader.get<uint8_t>();
    if (val != i) {
      return false;
    }
  }
  return true;
}

std::optional<Type_Id> Parser::parse_type_id(Reader& reader) {
  uint8_t id = reader.get<uint8_t>();
  SDL_LogInfo(1, "id: %d", id);

  return type_id_from_byte(id);
}  // namespace Bytecode
std::optional<Section_Id> Parser::parse_section_id(Reader& reader) {
  uint8_t id = reader.get<uint8_t>();
  return section_id_from_byte(id);
}

std::optional<Function_Type> Parser::parse_function_type(Reader& reader) {
  auto param_count = reader.get<uint64_t>();

  std::optional<std::span<Type_Id>> out_param_types = {};
  if (param_count != 0) {
    auto params = this->arena->push<Type_Id>(param_count);

    for (size_t i = 0; i < param_count; i++) {
      auto type_id = this->parse_type_id(reader);
      if (!type_id) {
        SDL_LogError(1, "Invalid type id, index: %ld", i);
        return {};
      }
      params[i] = type_id.value();
    }
    out_param_types = params;
  }

  std::optional<std::span<Type_Id>> out_return_types = {};
  auto return_count = reader.get<uint64_t>();
  if (return_count != 0) {
    auto return_types = this->arena->push<Type_Id>(return_count);
    for (size_t i = 0; i < return_count; i++) {
      auto type_id = this->parse_type_id(reader);
      if (!type_id) {
        SDL_LogError(1, "Invalid type id, index: %ld", i);
        return {};
      }
      return_types[i] = type_id.value();
    }
    out_return_types = return_types;
  }

  return Function_Type{.param_types = out_param_types,
                       .return_types = out_return_types};
}

std::optional<Type_Section> Parser::parse_type_section(
    Reader& reader, [[maybe_unused]] size_t section_size) {
  auto function_type_count = reader.get<uint64_t>();
  std::optional<Type_Section> out_type_section = {};
  if (function_type_count > 0) {
    out_type_section = this->arena->push<Function_Type>(function_type_count);
    for (uint64_t i = 0; i < function_type_count; i++) {
      uint8_t func_type_header = reader.get<uint8_t>();
      if (func_type_header != FUNCTYPE_HEADER) {
        SDL_LogError(
            1, "Malformed function type: Wrong header. Got %d, expected %d",
            func_type_header, FUNCTYPE_HEADER);
        return {};
      }
      auto func_type = this->parse_function_type(reader);
      if (!func_type) {
        SDL_LogError(1, "Unable to parse function type");
      } else {
        out_type_section.value()[i] = func_type.value();
      }
    }
  }
  return out_type_section;
}

std::optional<uint32_t> Parser::parse_type_idx(Reader& reader) {
  auto type_id = reader.get<uint32_t>();
  if (!this->type_section) {
    return {};
  }
  if (type_id > this->type_section.value().size()) {
    SDL_LogError(1, "Invalid type id out of scope");
    return {};
  }
  return type_id;
}

std::optional<Function_Section> Parser::parse_function_section(Reader& reader) {
  auto function_count = reader.get<uint64_t>();
  if (function_count > 0) {
    if (!this->type_section || this->type_section.value().empty()) {
      SDL_LogError(1, "Missing type section");
      return {};
    }
    auto type_ids = this->arena->push<uint32_t>(function_count);
    for (uint64_t i = 0; i < function_count; i++) {
      auto type_id = this->parse_type_idx(reader);
      if (!type_id) {
        return {};
      }
      type_ids[i] = type_id.value();
    }
    return type_ids;
  }
  return {};
}

std::optional<std::string_view> Parser::parse_string(Reader& reader) {
  auto str_len = reader.get<uint64_t>();
  auto string_data = reader.copy_bytes_alloc_zero_term(this->arena, str_len);
  if (!string_data) {
    return {};
  }
  return std::string_view(reinterpret_cast<char*>(string_data.value().data()),
                          string_data.value().size_bytes());
}

std::optional<Export> Parser::parse_export(Reader& reader) {
  auto export_name = this->parse_string(reader);
  if (!export_name) {
    SDL_LogError(1, "Unable to parse export name");
    return {};
  }
  SDL_LogInfo(1, "Export name: %s", export_name.value().data());
  auto desc_type_id = reader.get<uint8_t>();
  if (desc_type_id >= static_cast<uint8_t>(Export_Desc::Type::Enum_Len)) {
    SDL_LogError(1, "Malformed export desc type");
    return {};
  }
  auto desc_type = static_cast<Export_Desc::Type>(desc_type_id);
  uint64_t export_id = 0;

  switch (desc_type) {
    case Export_Desc::Type::Funcidx: {
      if (!this->function_section) {
        SDL_LogError(1, "Malformed module: No function section");
        return {};
      }
      export_id = reader.get<uint64_t>();
      if (export_id > this->function_section.value().size()) {
        SDL_LogError(1, "Export: Function id out of scope: got %ld, max: %ld",
                     export_id, this->function_section.value().size());
        return {};
      }
      return Export{.name = export_name.value(),
                    .desc = Export_Desc{.type = desc_type, .id = export_id}};
    } break;
    default:
      SDL_LogError(1, "Unimplemented");
      return {};
      break;
  }
}

std::optional<Export_Section> Parser::parse_export_section(Reader& reader) {
  auto export_count = reader.get<uint64_t>();
  if (export_count) {
    auto exported = this->arena->push<Export>(export_count);
    for (uint64_t i = 0; i < export_count; i++) {
      auto export_data = this->parse_export(reader);
      if (!export_data) {
        SDL_LogError(1, "Unable to parse export section");
        return {};
        break;
      }
      exported[i] = export_data.value();
    }
    return exported;
  }
  return {};
}

std::optional<Blocktype> Parser::parse_blocktype(Reader& reader) {
  uint8_t blocktype_byte = reader.get<uint8_t>();
  constexpr uint8_t blocktype_empty_id = 0x40;

  if (blocktype_byte == blocktype_empty_id) {
    return std::monostate{};
  } else {
    auto block_valtype = this->parse_type_id(reader);
    if (block_valtype) {
      return block_valtype.value();
    } else {
      return static_cast<Block_Type_Index>(reader.get<int32_t>());
    }
  }
}

/*

bool bytecode_parse_expression(Bytecode_Parser* parser, Bytecode_Reader* reader,
                               Bytecode_Expression* out_expression) {
  os_assert(bytecode_reader_can_read(reader));
  os_assert(out_expression != nullptr);

  uint64_t cap = INITAL_EXPRESSION_MAX_COUNT;
  out_expression->instructions = parser->arena->ptr();
  out_expression->cap = cap;
  out_expression->count = 0;

  uint64_t instruction_index = 0;
  int depth = 0;
  bool code_done = false;
  while (!code_done) {
    uint8_t opcode = bytecode_read_byte(reader);
    parser->arena->write_byte(opcode);

    switch ((Bytecode_Op)opcode) {
      // Keine Parameter:
      case Bytecode_Op_unreachable:
      case Bytecode_Op_nop:
      case Bytecode_Op_else:
      case Bytecode_Op_drop:
      case Bytecode_Op_return:
      case Bytecode_Op_i32_eqz:
      case Bytecode_Op_i32_eq:
      case Bytecode_Op_i32_ne:
      case Bytecode_Op_i32_lt_s:
      case Bytecode_Op_i32_lt_u:
      case Bytecode_Op_i32_gt_s:
      case Bytecode_Op_i32_gt_u:
      case Bytecode_Op_i32_le_s:
      case Bytecode_Op_i32_le_u:
      case Bytecode_Op_i32_ge_s:
      case Bytecode_Op_i32_ge_u:
      case Bytecode_Op_select:
        break;

      // Blocktype:
      case Bytecode_Op_block:
      case Bytecode_Op_loop:
      case Bytecode_Op_if: {
        Bytecode_Instruction_Data_Blocktype block_type = {};
        if (bytecode_parse_blocktype(reader, &block_type)) {
          parser->arena->write(&block_type);
          depth += 1;
        } else {
          return false;
        }
      } break;

      case Bytecode_Op_end: {
        if (depth == 0) {
          code_done = true;
          break;
        }
        depth -= 1;
      } break;
      case Bytecode_Op_br:
      case Bytecode_Op_br_if: {
        uint32_t id = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&id);
      } break;
      case Bytecode_Op_call: {
        uint32_t func_id = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&func_id);
      } break;
      case Bytecode_Op_call_indirect: {
        uint32_t type_id = (uint32_t)bytecode_read_var_uint(reader);
        uint32_t table_id = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&type_id);
        parser->arena->write(&table_id);
      } break;
      case Bytecode_Op_local_get:
      case Bytecode_Op_local_set:
      case Bytecode_Op_local_tee:
      case Bytecode_Op_global_get:
      case Bytecode_Op_global_set: {
        uint32_t id_arg = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&id_arg);
      } break;
      case Bytecode_Op_i32_load:
      case Bytecode_Op_i64_load:
      case Bytecode_Op_f32_load:
      case Bytecode_Op_f64_load:
      case Bytecode_Op_i32_load8_s:
      case Bytecode_Op_i32_store:
      case Bytecode_Op_i64_store:
      case Bytecode_Op_f32_store: {
        uint32_t align = (uint32_t)bytecode_read_var_uint(reader);
        uint32_t offset = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&align);
        parser->arena->write(&offset);
      } break;
      case Bytecode_Op_i32_const: {
        uint32_t const_val = (uint32_t)bytecode_read_var_uint(reader);
        parser->arena->write(&const_val);
      }
      case Bytecode_Op_i64_const: {
        uint64_t const_val = bytecode_read_var_uint(reader);
        parser->arena->write(&const_val);
      } break;
      case Bytecode_Op_select_t: {
        uint64_t count = bytecode_read_var_uint(reader);
        parser->arena->write(&count);
        for (uint64_t i = 0; i < count; i++) {
          Bytecode_Type_Id type_id =
              (Bytecode_Type_Id)bytecode_read_byte(reader);
          parser->arena->write(&type_id);
        }
      } break;
      default:
        SDL_LogError(1, "Parsing Opcode %d not implemented", opcode);
    }
    instruction_index += 1;
  }
  out_expression->count = instruction_index;
  return true;
}
*/

bool Parser::parse_next_section(Reader& reader) {
  auto section_id = this->parse_section_id(reader);
  if (!section_id) {
    SDL_LogError(1, "Malformed section id");
    return false;
  }
  auto section_size = reader.get<uint64_t>();
  if (section_size > 0) {
    switch (section_id.value()) {
      case Section_Id::Custom: {
        SDL_LogInfo(1, "Skipping custom section");
        reader.skip_bytes(section_size);
      } break;
      case Section_Id::Type: {
        SDL_LogInfo(1, "Reading type section");
        this->type_section = this->parse_type_section(reader, section_size);
      } break;
      case Section_Id::Function: {
        SDL_LogInfo(1, "Reading type section");
        this->function_section = this->parse_function_section(reader);
      } break;
      case Section_Id::Export: {
        SDL_LogInfo(1, "Reading export section");
        this->export_section = this->parse_export_section(reader);
      } break;
      default: {
        SDL_LogInfo(1, "Skipping unimpl section");
        reader.skip_bytes(section_size);
      } break;
    }
  }
  return true;
}
bool Parser::parse(Reader& reader) {
  bool section_ok = this->parse_next_section(reader);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  // NOTE:(Joh): um Clang tidy happy zu machen
  if (!this->type_section) {
    os_assert(false);
  }
  os_assert(this->type_section.value().size() == 1);
  auto func = this->type_section.value()[0];
  os_assert(func.param_types.has_value())
      os_assert(func.param_types.value().size() == 2);
  os_assert(func.return_types.value().size() == 1);
  os_assert(func.return_types.value()[0] == Type_Id::Num_I32);

  section_ok = this->parse_next_section(reader);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  os_assert(this->function_section);
  os_assert(this->function_section.value().size() == 1);

  section_ok = this->parse_next_section(reader);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  os_assert(this->export_section);
  os_assert(this->export_section.value().size() == 1);
  SDL_LogInfo(1, "Export fn name %s",
              this->export_section.value()[0].name.data());
  return true;
}
/*
bool bytecode_parse(Arena* aren, Bytecode_Reader* reader) {
  Bytecode_Parser parser = {};
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
  SDL_LogInfo(1, "Export fn name: %s", parser.export_section.exported[0].name);
  return true;
}
*/
}  // namespace Bytecode
