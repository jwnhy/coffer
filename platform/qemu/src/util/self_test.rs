use core::ops::Generator;
use core::pin::Pin;

use alloc::boxed::Box;
use riscv::register::mstatus::{self, MPP, Mstatus};
use riscv::register::{self, pmpcfg0};

use crate::memory::memory_layout::MemoryLayout;
use crate::memory::pmp::{pmpaddr_read, pmpcfg_read, pmpcfg_write};
use crate::println;
use crate::memory::pmp::PmpFlags;
use crate::runtime::context::Context;
use crate::runtime::runtime::Runtime;

pub fn test_pmp() {
    for i in 0..16 {
        pmpcfg_write(i, (PmpFlags::READABLE | PmpFlags::WRITABLE | PmpFlags::EXECUTABLE).bits());
        assert_eq!(pmpcfg_read(i), (PmpFlags::READABLE | PmpFlags::WRITABLE | PmpFlags::EXECUTABLE).bits());
    }
}

unsafe extern "C" fn _test_context() {
    loop {
        println!("TEST Context");
        asm!("ebreak");
    }
}

pub fn test_context() {
    let mut ctx = Context::new();
    let sp: [u8; 1024] = [0;1024];

    /* now we can just set what we need and swap up */
    ctx.mstatus.set_mpp(MPP::Machine);

    ctx.sp = &sp as *const u8 as usize;
    ctx.sp = ctx.sp + 1024;
    ctx.mepc = _test_context as usize;
    let mut runtime = Runtime::<()>::new(
        ctx, MemoryLayout{ }, Box::new(|x| {
            unsafe {
                (*x).mepc = (*x).mepc + 2;
            }
            None
        }
                              ));
    runtime.init();
    Pin::new(&mut runtime).resume(());
}
