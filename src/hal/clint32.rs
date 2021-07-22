use core::{ops::Add, ptr::{read_volatile, write_volatile}};

use bit_field::BitField;

use crate::{println, sbi::{hart_mask::HartMask, ipi::Ipi, sbiret::SbiRet, timer::Timer}};
pub struct Clint32 {
    base: usize,
    mtimecmp_offset: usize,
    max_hartid: usize,
}

impl Clint32 {
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
            let reg_l = (base.add(self.mtimecmp_offset) as *mut u64).add(hartid) as *mut u32;
            let reg_h = reg_l.add(1);
            write_volatile(reg_l, u32::max_value());
            write_volatile(reg_h, wait_for.get_bits(32..64) as u32);
            write_volatile(reg_l, wait_for.get_bits(0..32) as u32);
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

impl Ipi for Clint32 {
    fn max_hartid(&self) -> usize {
        self.max_hartid
    } 
    fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet {
        for i in 0..=self.max_hartid() {
            if hart_mask.has(i) {
                self.send_soft_irq(i);
            }
        }
        SbiRet::ok(0)
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

impl Timer for Clint32 {
    fn set_timer(&self, stime_value: u64) {
        let hartid = riscv::register::mhartid::read();
        self.set_timer(hartid, stime_value);
    }
}
