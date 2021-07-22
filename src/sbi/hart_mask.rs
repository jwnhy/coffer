use core::mem::size_of;

use riscv::register::mstatus::MPP;

use crate::util::addr::vaddr_deref;

#[derive(Debug, Clone)]
pub struct HartMask {
    mask_arr: *const usize,
    base: usize,
    mode: MPP,
}

impl HartMask {
    pub unsafe fn new(vaddr: usize, base: usize, mode: MPP) -> Self {
        HartMask {
            mask_arr: vaddr as *const usize,
            base,
            mode,
        }
    }

    pub fn has(&self, hartid: usize) -> bool {
        // TODO: add maximum assertion here
        if self.base == usize::MAX {
            return true;
        }
        if hartid < self.base {
            return false;
        }
        let usize_bits = size_of::<usize>() * 8;
        let idx = hartid - self.base;
        let (i, j) = (idx / usize_bits, idx % usize_bits);
        let mask = unsafe {
            vaddr_deref(self.mask_arr.add(i), self.mode)
        };
        mask & (1 << j) != 0
    }
}
