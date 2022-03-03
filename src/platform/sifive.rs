use crate::{hal::Tlb, println, sbi::rfence::init_rfence, util::fdt::{detect_clint, detect_sifive_uart, init_fdt}};

pub fn sifive_init(dtb: usize) -> usize {
    init_fdt(dtb);
    detect_sifive_uart();
    detect_clint();
    init_rfence(Tlb {});
    0x8020_0000
}
