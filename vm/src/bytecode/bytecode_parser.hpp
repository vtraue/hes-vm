#pragma once
#include <optional>

#include "bytecode.hpp"
#include "bytecode_reader.hpp"

namespace Bytecode {
struct Parser {
  Arena* arena;

  Parser(Arena* arena) : arena(arena) {};

  std::optional<Type_Section> type_section{};
  std::optional<Function_Section> function_section{};
  std::optional<Export_Section> export_section{};

  bool check_header(Reader& reader);
  bool check_version(Reader& reader);
  std::optional<Type_Id> parse_type_id(Reader& reader);
  std::optional<Section_Id> parse_section_id(Reader& reader);
  std::optional<Function_Type> parse_function_type(Reader& reader);
  std::optional<Type_Section> parse_type_section(
      Reader& reader, [[maybe_unused]] size_t section_size);
  std::optional<uint32_t> parse_type_idx(Reader& reader);
  std::optional<Function_Section> parse_function_section(Reader& reader);
  std::optional<std::string_view> parse_string(Reader& reader);
  std::optional<Export> parse_export(Reader& reader);
  std::optional<Export_Section> parse_export_section(Reader& reader);
  std::optional<Blocktype> parse_blocktype(Reader& reader);
  bool parse_next_section(Reader& reader);
  bool parse(Reader& reader);
};

/*
typedef struct Bytecode_Parser {
        Arena* arena;
        Bytecode_Type_Section type_section;
        Bytecode_Function_Section function_section;
        Bytecode_Export_Section export_section;
        uint16_t main_sections_present;
} Bytecode_Parser;

bool bytecode_check_header(Bytecode_Reader* reader);
bool bytecode_check_version(Bytecode_Reader* reader);
bool bytecode_parse_section(Bytecode_Reader* reader, Bytecode_Parser* parser);
bool bytecode_parse(Arena* arena, Bytecode_Reader* reader);

void bytecode_set_section_parsed(Bytecode_Parser* parser,
                                                                                                                                 Bytecode_Section_Id id);
bool bytecode_is_section_parsed(Bytecode_Parser* parser,
                                                                                                                                Bytecode_Section_Id id);
*/
}  // namespace Bytecode
