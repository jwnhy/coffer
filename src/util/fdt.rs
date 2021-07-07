use alloc::boxed::Box;
use fdt::{node::FdtNode, Fdt, FdtError};
use if_chain::if_chain;
use spin::Mutex;

#[cfg(target_arch = "riscv64")]
pub const XLEN: usize = 8;
#[cfg(target_arch = "riscv32")]
pub const XLEN: usize = 4;

use crate::{hal::{Clint, Ns16550a, SifiveUart, SunxiUart}, println, sbi::{init_console_embedded_serial, ipi::init_ipi, timer::init_timer}};

lazy_static::lazy_static! {
    pub static ref FDT: Mutex<Option<Box<Fdt<'static>>>> = Mutex::new(None);
}

pub fn init_fdt(fdt_addr: usize) -> Result<(), FdtError> {
    unsafe {
        let fdt = Fdt::from_ptr(fdt_addr as *const u8)?;
        *FDT.lock() = Some(Box::new(fdt));
    }
    Ok(())
}

pub fn detect_clint() {
    if_chain!{
        if let Some(fdt) = FDT.lock().as_ref();
        if let Some(node) = fdt.find_compatible(&["riscv,clint0"]);
        if let Some(mut reg_list) = node.reg();
        if let Some(reg) = reg_list.next();
        then {
            let base = reg.starting_address;
            let cpucnt = fdt.cpus().count();
            let clint = Clint::new(base as usize, 0x4000, 0xbff8, cpucnt);
            init_ipi(clint);
            let clint = Clint::new(base as usize, 0x4000, 0xbff8, cpucnt);
            init_timer(clint);
        } else {
            panic!("init clint failed");
        }
    } 
}

pub fn detect_sunxi_uart() {
    if_chain! {
            if let Some(fdt) = FDT.lock().as_ref();
            if let Some(node) = fdt.find_compatible(&["allwinner,sun20i-uart"]);
            if let Some(mut reg_list) = node.reg();
            if let Some(reg) = reg_list.next();
            then {
                let base = reg.starting_address;
                let serial = SunxiUart::new(base as usize);
                init_console_embedded_serial(serial);
            }

    }
}

pub fn detect_sifive_uart() {
    if_chain! {
            if let Some(fdt) = FDT.lock().as_ref();
            if let Some(node) = fdt.find_compatible(&["sifive,uart0"]);
            if let Some(mut reg_list) = node.reg();
            if let Some(reg) = reg_list.next();
            then {
                let base = reg.starting_address;
                let serial = SifiveUart::new(base as usize, 0, 115200);
                init_console_embedded_serial(serial);
            }

    }
}

pub fn detect_ns16550a() {
    if_chain! {
            if let Some(fdt) = FDT.lock().as_ref();
            if let Some(node) = fdt.find_compatible(&["ns16550a"]);
            if let Some(mut reg_list) = node.reg();
            if let Some(reg) = reg_list.next();
            if let Some(clock) = node.property("clock-frequency");
            if let Some(clk) = clock.as_usize();
            then {
                let base = reg.starting_address;
                let serial = Ns16550a::new(base as usize, 0, clk as u64, 115200);
                init_console_embedded_serial(serial)
            }
    }
}

pub fn detect_hart() -> usize {
    if let Some(fdt) = FDT.lock().as_ref() {
        fdt.cpus().count()
    } else {
        1
    }
}
