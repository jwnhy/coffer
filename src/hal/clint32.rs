use core::{ops::Add, ptr::{read_volatile, write_volatile}};

use bit_field::BitField;

use crate::{println, sbi::{hart_mask::HartMask, ipi::Ipi, sbiret::SbiRet, timer::Timer}};
pub struct Clint32 {
    base: usize,
    mtimecmp_offset: usize,
    max_hart_id: usize,
}

impl Clint32 {
    pub fn new(base: usize, mtimecmp_offset: usize, max_hart_id: usize) -> Self {
        Self {
            base,
            mtimecmp_offset,
            max_hart_id,
        }
    }

    pub fn set_timer(&self, hart_id: usize, wait_for: u64) {
        unsafe {
            let base = self.base as *mut u8;
            let reg_l = (base.add(self.mtimecmp_offset) as *mut u64).add(hart_id) as *mut u32;
            let reg_h = reg_l.add(1);
            write_volatile(reg_l, u32::max_value());
            write_volatile(reg_h, wait_for.get_bits(32..64) as u32);
            write_volatile(reg_l, wait_for.get_bits(0..32) as u32);
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

impl Ipi for Clint32 {
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

impl Timer for Clint32 {
    fn set_timer(&self, stime_value: u64) {
        let hart_id = riscv::register::mhartid::read();
        self.set_timer(hart_id, stime_value);
    }
}
