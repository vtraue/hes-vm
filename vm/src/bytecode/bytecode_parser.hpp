#pragma once
#include <SDL3/SDL_log.h>

#include <expected>
#include <optional>

#include "bytecode.hpp"

namespace Bytecode {

struct Parser {
  Arena* arena;

  Parser(Arena* arena) : arena(arena) {};

  std::optional<Type_Section> type_section{};
  std::optional<Function_Section> function_section{};
  std::optional<Export_Section> export_section{};
  std::optional<Code_Section> code_section{};

  bool check_header(Reader& reader);

  bool check_version(Reader& reader);

  template <typename T>
  std::optional<T> parse(Reader& reader);

  template <typename T>
  std::optional<std::span<T>> parse_vec(Reader& reader) {
    auto count = reader.get<uint32_t>();
    if (count > 0) {
      auto res = this->arena->push<T>(count);
      for (uint32_t i = 0; i < count; i++) {
        auto data = this->parse<T>(reader);
        if (!data) {
          Os::log("Unable to parse vec\n");
          SDL_LogError(1, "Unable to parse vec");
          return {};
        }
        res[i] = data.value();
      }
      return res;
    }
    return {};
  }

  bool parse_next_section(Reader& reader);
  bool parse(Reader& reader);
};
}  // namespace Bytecode
