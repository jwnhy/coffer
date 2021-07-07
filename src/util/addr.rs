use riscv::register::mstatus::{self, MPP};
use core::ptr::read;
// TODO: This is untested
#[inline]
pub unsafe fn vaddr_deref(vaddr: *const usize, mode: MPP) -> usize  {
    let prev_mode = mstatus::read().mpp();
    mstatus::set_mpp(mode);
    let val = read(vaddr);
    mstatus::set_mpp(prev_mode);
    val
}
