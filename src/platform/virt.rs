use crate::util::fdt::{detect_ns16550a, init_fdt};

pub fn virt_init(dtb: usize) -> usize {
    init_fdt(dtb);
    detect_ns16550a();
    0x8020_0000
}
