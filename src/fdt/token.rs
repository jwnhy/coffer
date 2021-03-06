use endiantype::*;
pub const FDT_BEGIN_NODE: u32_be = u32_be::from_native(0x0000_0001);
pub const FDT_END_NODE: u32_be = u32_be::from_native(0x0000_0002);
pub const FDT_PROP: u32_be = u32_be::from_native(0x0000_0003);
pub const FDT_NOP: u32_be = u32_be::from_native(0x0000_0004);
pub const FDT_END: u32_be = u32_be::from_native(0x0000_0009);
pub const FDT_NIL: u32_be = u32_be::from_native(0);

pub(crate) fn skip_nop(addr: *const u8) -> *const u8 {
    let mut cur = addr as *const u32_be;
    unsafe {
        while *cur == FDT_NOP {
            cur = cur.add(1);
        }
    }
    cur as *const u8
}

pub(crate) fn skip_end_node(addr: *const u8) -> *const u8 {
    let mut cur = addr as *const u32_be;
    unsafe {
        while *cur == FDT_END_NODE {
            cur = cur.add(1);
        }
    }
    cur as *const u8
}

#[inline]
pub(crate) fn align(addr: usize, alignment: usize) -> usize {
    ((addr + (alignment - 1)) & !(alignment - 1))
}
