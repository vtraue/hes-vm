#pragma once
#include <cstdint>
namespace Bytecode {
enum class Op : uint8_t {
  Unreachable = 0x00,
  Nop = 0x01,
  Block = 0x02,
  Loop = 0x03,
  If = 0x04,
  Else = 0x05,
  End = 0x0B,
  Br = 0x0C,
  Br_if = 0x0D,
  Br_table = 0x0E,
  Return = 0x0F,
  Call = 0x10,
  Call_indirect = 0x11,
  Drop = 0x1A,
  Select = 0x1B,
  Select_t = 0x1C,
  Local_get = 0x20,
  Local_set = 0x21,
  Local_tee = 0x22,
  Global_get = 0x23,
  Global_set = 0x24,
  I32_load = 0x28,
  I64_load = 0x29,
  F32_load = 0x2A,
  F64_load = 0x2B,
  I32_load8_s = 0x2C,
  I32_store = 0x36,
  I64_store = 0x37,
  F32_store = 0x38,
  I32_const = 0x41,
  I64_const = 0x42,
  I32_eqz = 0x45,
  I32_eq = 0x46,
  I32_ne = 0x47,
  I32_lt_s = 0x48,
  I32_lt_u = 0x49,
  I32_gt_s = 0x4A,
  I32_gt_u = 0x4B,
  I32_le_s = 0x4C,
  I32_le_u = 0x4D,
  I32_ge_s = 0x4E,
  I32_ge_u = 0x4F,
  Ref_null = 0xD0,
  Ref_is_null = 0xD1,
  Ref_func = 0xF2,
  I32_add = 0x6A,

};
}
