#include "bytecode_parser.hpp"

#include <SDL3/SDL_log.h>

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

std::optional<Imm::Blocktype> Parser::parse_blocktype(Reader& reader) {
  uint8_t blocktype_byte = reader.get<uint8_t>();
  constexpr uint8_t blocktype_empty_id = 0x40;
  Imm::Blocktype res = {};
  if (blocktype_byte == blocktype_empty_id) {
    res.tag = Imm::Blocktype::Tag::Empty;
  } else {
    auto block_valtype = this->parse_type_id(reader);
    if (block_valtype) {
      res.tag = Imm::Blocktype::Tag::Val_Type;
      res.data.val_type = block_valtype.value();
    } else {
      res.tag = Imm::Blocktype::Tag::Type_Index;
      res.data.type_id = reader.get<int32_t>();
    }
  }
  return res;
}

bool Parser::parse_next_section(Reader& reader) {
  auto section_id = this->parse_section_id(reader);
  if (!section_id) {
    SDL_LogError(1, "Malformed section id");
    return false;
  }
  SDL_LogInfo(1, "Reading Section: %hhu",
              static_cast<uint8_t>(section_id.value()));
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
        SDL_LogInfo(1, "Reading function section");
        this->function_section = this->parse_function_section(reader);
      } break;
      case Section_Id::Export: {
        SDL_LogInfo(1, "Reading export section");
        this->export_section = this->parse_export_section(reader);
      } break;
      case Section_Id::Code: {
        SDL_LogInfo(1, "Reading code section");
        this->code_section = this->parse_code_section(reader);
      } break;
      default: {
        SDL_LogInfo(1, "Skipping unimpl section");
        reader.skip_bytes(section_size);
      } break;
    }
  }
  return true;
}
std::optional<Code> Parser::parse_code(Reader& reader) {
  auto size = reader.get<uint32_t>();
  SDL_LogInfo(1, "Code size in bytes: %d", size);

  auto locals = this->parse_locals(reader);
  auto expression = this->parse_expression(reader);
  if (!expression) {
    SDL_LogError(1, "Unable to read expression");
    return {};
  }
  Code out_code{
      .size_bytes = size, .locals = locals, .expr = expression.value()};
  return out_code;
}

std::optional<Code_Section> Parser::parse_code_section(Reader& reader) {
  auto count = reader.get<uint32_t>();
  auto buffer = this->arena->push<Code>(count);
  SDL_LogInfo(1, "Code: Found %d Code entries", count);

  if (count > 0) {
    for (uint32_t i = 0; i < count; i++) {
      auto code = this->parse_code(reader);
      if (!code) {
        SDL_LogError(1, "Unable to parse code");
        return {};
      }
      buffer[i] = code.value();
    }
    return buffer;
  }
  return {};
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
  os_assert(this->type_section.value().size() == 3);

  section_ok = this->parse_next_section(reader);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  os_assert(this->function_section);
  os_assert(this->function_section.value().size() == 3);

  /*
section_ok = this->parse_next_section(reader);
if (!section_ok) {
SDL_LogError(1, "Unable to parse section");
return false;
}
os_assert(this->export_section);
os_assert(this->export_section.value().size() == 1);
SDL_LogInfo(1, "Export fn name %s",
        this->export_section.value()[0].name.data());

  */
  section_ok = this->parse_next_section(reader);
  if (!section_ok) {
    SDL_LogError(1, "Unable to parse section");
    return false;
  }
  os_assert(this->code_section);
  SDL_LogInfo(1, "Done!");
  return true;
}

std::optional<Locals> Parser::parse_locals(Reader& reader) {
  auto count = reader.get<uint32_t>();
  SDL_LogInfo(1, "Locals Count: %d", count);
  if (count <= 0) {
    return {};
  }
  auto locals = this->arena->push<Locals_of_T>(count);

  for (uint32_t i = 0; i < count; i++) {
    auto n = reader.get<uint32_t>();
    auto t = this->parse_type_id(reader);
    if (!t) {
      SDL_LogError(1, "Unable to parse type id");
      return {};
    }
    locals[i] = Locals_of_T{.count = n, .valtype = t.value()};
  }

  Locals res = {.data = locals};
  return res;
}

std::optional<Expression> Parser::parse_expression(Reader& reader) {
  int depth = 0;
  bool code_done = false;
  auto instr_buffer_ptr = reinterpret_cast<Instruction*>(this->arena->ptr());
  size_t instruction_count = 0;
  while (!code_done) {
    SDL_LogInfo(1, "Next instruction");
    auto opcode = reader.get<uint8_t>();
    SDL_LogInfo(1, "Reading op: %04x", opcode);
    auto op = static_cast<Op>(opcode);
    Instruction instr = {};
    instr.op = op;

    switch (op) {
      case Op::Unreachable:
      case Op::Nop:
      case Op::Else:
      case Op::Drop:
      case Op::Return:
      case Op::I32_eqz:
      case Op::I32_eq:
      case Op::I32_ne:
      case Op::I32_lt_s:
      case Op::I32_lt_u:
      case Op::I32_gt_s:
      case Op::I32_gt_u:
      case Op::I32_le_s:
      case Op::I32_le_u:
      case Op::I32_ge_s:
      case Op::I32_ge_u:
      case Op::I32_add:
      case Op::Select:
        break;

      case Op::Block:
      case Op::Loop:
      case Op::If: {
        auto blocktype = this->parse_blocktype(reader);
        if (!blocktype) {
          SDL_LogError(1, "Unable to parse blocktype");
          return {};
        }
        instr.args.block_type = blocktype.value();
        depth += 1;
      } break;
      case Op::Local_get:
      case Op::Local_set:
      case Op::Local_tee:
      case Op::Global_get:
      case Op::Global_set:
      case Op::Br:
      case Op::Call:
      case Op::Br_if: {
        instr.args.u32 = reader.get<uint32_t>();
      } break;
      case Op::I32_load:
      case Op::I64_load:
      case Op::F32_load:
      case Op::F64_load:
      case Op::I32_load8_s:
      case Op::I32_store:
      case Op::I64_store:
      case Op::F32_store:
      case Op::Call_indirect: {
        instr.args.pair_u32.x = reader.get<uint32_t>();
        instr.args.pair_u32.y = reader.get<uint32_t>();
      } break;
      case Op::I64_const: {
        instr.args.i64 = reader.get<int64_t>();
      } break;
      case Op::I32_const: {
        instr.args.i32 = reader.get<int32_t>();
      } break;
      case Op::Select_t: {
        auto arg_count = reader.get<uint32_t>();
        auto args = this->arena->push<Type_Id>(arg_count);
        for (uint32_t i = 0; i < arg_count; ++i) {
          auto type_id = this->parse_type_id(reader);
          if (!type_id) {
            SDL_LogError(1, "Unable to parse type id");
            return {};
          }
          args[i] = type_id.value();
        }
      } break;
      case Op::End: {
        if (depth > 0) {
          depth -= 1;
        } else {
          code_done = true;
        }
      } break;
      default:
        SDL_LogError(1, "Cant parse opcode");
        return {};
    }
    this->arena->write(&instr);
    instruction_count += 1;
  }
  return std::span<Instruction>(instr_buffer_ptr, instruction_count);
}
}  // namespace Bytecode
