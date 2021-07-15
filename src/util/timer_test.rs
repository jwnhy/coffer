use core::{ops::Generator, pin::Pin};

use alloc::boxed::Box;
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

    set_timer(riscv::register::time::read64() + 100_0000);
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

pub fn test_timer() {
    let mut ctx = Context::new();
    let sp: [u8; 8192] = [0; 8192];
    ctx.mstatus.set_mpp(MPP::Machine);
    ctx.mstatus.set_mpie(true);
    ctx.sp = &sp as *const u8 as usize;
    ctx.sp = ctx.sp + 8192;
    ctx.mepc = _test_timer as usize;
    ctx.mie = 1 << 7;
    set_timer(riscv::register::time::read64() + 100_0000);
    unsafe {
        riscv::register::mip::clear_mtimer();
    }
    let mut runtime = Runtime::<()>::new(
        ctx,
        None,
        Box::new(|x| unsafe {
            match mcause::read().cause() {
                mcause::Trap::Interrupt(mcause::Interrupt::MachineTimer) => {
                    set_timer(riscv::register::time::read64() + 100_0000);
                    (*x).mstatus.set_mie(true);
                    (*x).mie = 1 << 7;
                    TICKS = TICKS + 1;
                    riscv::register::mip::clear_mtimer();
                }
                e @ _ => panic!("[ERROR] unexpected exception@{:x}: {:?}", (*x).mepc, e),
            }
            None
        }),
    );
    Pin::new(&mut runtime).resume(());
}
