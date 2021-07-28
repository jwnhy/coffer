use core::ptr::write_volatile;

use crate::{
    println,
    util::fdt::{detect_sunxi_uart, init_fdt, init_sunxi_clint},
};

#[repr(C)]
struct SunxiHead {
    pub jump_inst: u32,
    pub magic: [u8; 8],
    pub dtb_base: u32,
    pub uboot_base: u32,
    pub res3: u32,
    pub res4: u32,
    pub res5: [u8; 8],
    pub res6: [u8; 8],
    pub opensbi_base: u32,
}

#[no_mangle]
#[link_section = ".head_data"]
static SUNXI_HEAD: SunxiHead = SunxiHead {
    jump_inst: 0x4000_006f, // j 0x4000_0400
    magic: *b"opensbi\0",
    uboot_base: 0,
    dtb_base: 0,
    res3: 0,
    res4: 0,
    res5: [0; 8],
    res6: [0; 8],
    opensbi_base: 0x4000_0000,
};

static DEVICE_TREE: &[u8] = include_bytes!("../../dtb/sunxi.dtb");

pub fn sunxi_init(dtb: usize) -> usize {
    init_fdt(DEVICE_TREE.as_ptr() as usize);
    detect_sunxi_uart();
    init_sunxi_clint(0x1400_0000);
    // TODO: SETUP PLIC
    unsafe { write_volatile(0x101F_FFFC as *mut u32, 0x1) };
    0x4200_0000
}
