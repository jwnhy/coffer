use core::ptr::{read_volatile, write_volatile};

use crate::sbi::{hart_mask::HartMask, ipi::Ipi, sbiret::SbiRet, timer::Timer};
pub struct Clint {
    base: usize,
    mtimecmp_offset: usize,
    max_hartid: usize,
}

impl Clint {
    pub fn new(base: usize, mtimecmp_offset: usize, max_hartid: usize) -> Self {
        Self {
            base,
            mtimecmp_offset,
            max_hartid,
        }
    }

    pub fn set_timer(&self, hartid: usize, wait_for: u64) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base.add(self.mtimecmp_offset) as *mut u64).add(hartid);
            write_volatile(reg, wait_for)
        }
    }

    pub fn send_soft_irq(&self, hartid: usize) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base as *mut u32).add(hartid);
            write_volatile(reg, 1);
        }
    }

    pub fn clear_soft_irq(&self, hartid: usize) {
        unsafe {
            let base = self.base as *mut u8;
            let reg = (base as *mut u32).add(hartid);
            write_volatile(reg, 0);
        }
    }
}

impl Ipi for Clint {
    fn max_hartid(&self) -> usize {
        self.max_hartid
    } 
    
    #[inline]
    fn clear_soft_irq(&self, hartid: usize) {
        self.clear_soft_irq(hartid);
    }
    #[inline]
    fn send_soft_irq(&self, hartid: usize) {
        self.send_soft_irq(hartid);
    }
}

impl Timer for Clint {
    fn set_timer(&self, stime_value: u64) {
        let hartid = riscv::register::mhartid::read();
        self.set_timer(hartid, stime_value);
    }
}
