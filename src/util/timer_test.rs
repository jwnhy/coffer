use core::{ops::Generator, pin::Pin, ptr::write_volatile};

use alloc::boxed::Box;
use bit_field::BitField;
use riscv::register::{mcause, mstatus::{self, MPP}};

use crate::{
    println,
    runtime::{context::Context, runtime::Runtime},
    sbi::timer::set_timer,
};

static mut TICKS: usize = 0;

#[cfg(target_arch = "riscv64")]
unsafe extern "C" fn _test_timer() {
    use core::ptr::read_volatile;
    let mut i = 0;
    loop {
        let mip = riscv::register::mip::read();
        let mie = riscv::register::mie::read();
        let mstatus = riscv::register::mstatus::read();
        println!("mip: {:?}, mie: {:?}, mstatus: {:?}", mip.mtimer(), mie.mtimer(), mstatus.mie());
        let l = read_volatile(0x1400_4000 as *const u32) as u64;
        let h = read_volatile(0x1400_4004 as *const u32) as u64;
        println!("{:x}, {:x}, {:x}, {:x}", riscv::register::time::read64(), (h<<32)+l, i, TICKS);
        i += 1;
    }
}

fn _set_timer(stimer_val: u64) {
    unsafe {
        write_volatile(0x1400_4000 as *mut u32, u32::max_value());
        write_volatile(0x1400_4004 as *mut u32, stimer_val.get_bits(32..64) as u32);
        write_volatile(0x1400_4000 as *mut u32, stimer_val.get_bits(0..32) as u32);
    }
}

pub fn test_timer() {
    let mut ctx = Context::new();
    let sp: [u8; 8192] = [0; 8192];
    ctx.mstatus.set_mpp(MPP::Machine);
    ctx.mstatus.set_mpie(true);
    ctx.sp = &sp as *const u8 as usize;
    ctx.sp = ctx.sp + 8192;
    ctx.mepc = _test_timer as usize;
    _set_timer(riscv::register::time::read64() + 100_0000);
    unsafe {
        riscv::register::mie::set_mtimer();
    }
    let mut runtime = Runtime::<()>::new(
        ctx,
        None,
        Box::new(|x| unsafe {
            match mcause::read().cause() {
                mcause::Trap::Interrupt(mcause::Interrupt::MachineTimer) => {
                    _set_timer(riscv::register::time::read64() + 100_0000);
                    TICKS = TICKS + 1;
                }
                e @ _ => panic!("[ERROR] unexpected exception@{:x}: {:?}", (*x).mepc, e),
            }
            None
        }),
    );
    Pin::new(&mut runtime).resume(());
}
