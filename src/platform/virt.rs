use crate::{hal::Tlb, sbi::rfence::init_rfence, util::fdt::{detect_clint, detect_ns16550a, init_fdt}};

pub fn virt_init(dtb: usize) -> usize {
    init_fdt(dtb);
    detect_ns16550a();
    detect_clint();
    init_rfence(Tlb{});
    0x8020_0000
}
