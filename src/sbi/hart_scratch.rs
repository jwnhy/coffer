use core::intrinsics::atomic_xchg;

use crate::util::fdt::detect_hart;
use alloc::vec::Vec;
use bit_field::BitField;
use spin::{Mutex, RwLock};

pub struct HartScratch {
    pub ipi_scratch: IpiScratch,
}

impl HartScratch {
    pub fn new() -> Self {
        return Self {
            ipi_scratch: IpiScratch::new(),
        };
    }
}

static mut HART_SCRATCH: Vec<Mutex<HartScratch>> = Vec::new();

pub fn init_hart_scratch() {
    /* need init fdt before this */
    let hart_cnt = detect_hart();
    unsafe {
        for _ in 0..hart_cnt {
            HART_SCRATCH.push(Mutex::new(HartScratch::new()))
        }
    }
}

pub fn get_hart_scratch(hartid: usize) -> &'static Mutex<HartScratch> {
    unsafe { &HART_SCRATCH[hartid] }
}

pub struct IpiScratch {
    ipi_triggered: usize,
}

impl IpiScratch {
    pub fn new() -> Self {
        Self { ipi_triggered: 0x0 }
    }

    #[inline]
    pub fn is_triggered(&self, event_id: usize) -> bool {
        self.ipi_triggered.get_bit(event_id)
    }

    #[inline]
    pub fn clear_triggered(&mut self) {
        self.ipi_triggered = 0;
    }

    #[inline]
    pub fn trigger(&mut self, event_id: usize) {
        self.ipi_triggered.set_bit(event_id, true);
        unsafe { asm!("fence w, w") };
    }
}
