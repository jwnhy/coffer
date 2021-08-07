use core::ptr::read;
use riscv::register::mstatus::{self, MPP};
// TODO: This is untested
#[inline]
pub fn align(addr: usize, alignment: usize) -> usize {
    ((addr + (alignment - 1)) & !(alignment - 1))
}
