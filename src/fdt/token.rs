use super::endian::types::*;
pub const FDT_BEGIN_NODE: u32_be = u32_be::from_native(0x0000_0001);
pub const FDT_END_NODE: u32_be = u32_be::from_native(0x0000_0002);
pub const FDT_PROP: u32_be = u32_be::from_native(0x0000_0003);
pub const FDT_NOP: u32_be = u32_be::from_native(0x0000_0004);
pub const FDT_END: u32_be = u32_be::from_native(0x0000_0009);
pub const FDT_NIL: u32_be = u32_be::from_native(0);
