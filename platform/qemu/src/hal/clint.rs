use core::ptr::{read_volatile, write_volatile};

use crate::sbi::{hart_mask::HartMask,ipi::Ipi, sbiret::SbiRet};
pub struct Clint {
    base: usize,
    mtimecmp_offset: usize,
    mtime_offset: usize,
    max_hart_id: usize,
}

impl Clint {
    pub fn new(base: usize, mtimecmp_offset: usize, mtime_offset: usize, max_hart_id: usize) -> Self {
        Self {
            base,
            mtimecmp_offset,
            mtime_offset,
            max_hart_id,
        }
    }

    pub fn get_mtime(&self) -> u64 {
        unsafe {
            let base = self.base as *mut u8;
            let reg = base.add(self.mtime_offset);
            read_volatile(reg as *mut u64)
        }
    }

    pub fn set_timer(&self, hart_id: usize, wait_for: u64) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base.add(self.mtimecmp_offset) as *mut u64).add(hart_id);
            write_volatile(reg, wait_for)
        }
    }

    pub fn send_soft_irq(&self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base as *mut u32).add(hart_id);
            write_volatile(reg, 1);
        }
    }

    pub fn clear_soft_irq(&self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base as *mut u32).add(hart_id);
            write_volatile(reg, 0);
        }
    }
}

impl Ipi for Clint {
    fn max_hart_id(&self) -> usize {
        self.max_hart_id
    } 
    fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet {
        for i in 0..=self.max_hart_id() {
            if hart_mask.has(i) {
                self.send_soft_irq(i);
            }
        }
        SbiRet::ok(0)
    }
}
