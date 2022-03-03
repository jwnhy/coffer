#![no_std]
#![no_main]
#![allow(unused)]
#![allow(non_snake_case)]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(generator_trait)]
#![feature(fn_align)]
#![feature(core_intrinsics)]
#![feature(asm_sym)]
#![feature(asm_const)]

extern crate alloc;

mod ecall;
mod fdt;
mod hal;
mod memory;
mod platform;
mod runtime;
mod rvbt;
mod util;
#[macro_use]
mod sbi;

use crate::{
    memory::pmp::PmpFlags,
    sbi::{ipi::process_ipi, timer::process_timer},
};
use alloc::boxed::Box;
use core::{ops::Generator, pin::Pin};
use ecall::handle_ecall;
use platform::generic::generic_init;
use riscv::{asm::wfi, register::{
    mcause::{self, Exception, Interrupt, Trap},
    mstatus::MPP,
    stvec,
}};
use runtime::{context::Context, runtime::Runtime};
use util::banner::print_banner;
use core::arch::asm;
use crate::memory::memory_layout::Region;

pub extern "C" fn main(hartid: usize, dtb: usize) -> ! {
    let hartid = riscv::register::mhartid::read();
    let global_region = Region {
            addr: 0x0,
            size: 56,
            enabled: true,
            pmp_cfg: PmpFlags::EXECUTABLE
                | PmpFlags::READABLE
                | PmpFlags::WRITABLE
                | PmpFlags::MODE_NAPOT,
        };
    global_region.enforce(0);
    if hartid == 0 {
        let jump_addr = generic_init(dtb);
        let mut rt = kernel_runtime(hartid, dtb, jump_addr);
        Pin::new(&mut rt).resume(());
    }
    loop {unsafe {wfi()};}
}

fn kernel_runtime(hartid: usize, dtb: usize, kernel_addr: usize) -> Runtime<()> {
    let mut ctx = Context::new();
    ctx.a0 = hartid;
    ctx.a1 = dtb;
    ctx.mepc = kernel_addr;
    ctx.mstatus.set_mpp(MPP::Supervisor);
    ctx.mstatus.set_fs(riscv::register::sstatus::FS::Dirty);
    ctx.mcounteren = 0xffff_ffff;
    //ctx.medeleg = 0xb1ff;
    //ctx.mideleg = 0x222;
    unsafe {
        riscv::register::mie::set_msoft();
    }
    let runtime = Runtime::<()>::new(
        ctx,
        None,
        Box::new(|ctx_ptr| unsafe {
            let cause = mcause::read().cause();
            match cause {
                Trap::Exception(Exception::SupervisorEnvCall) => {
                    let sbi_ret = handle_ecall(ctx_ptr);
                    (*ctx_ptr).a0 = sbi_ret.error;
                    (*ctx_ptr).a1 = sbi_ret.value;
                    (*ctx_ptr).mepc = (*ctx_ptr).mepc + 4;
                }
                Trap::Interrupt(Interrupt::MachineTimer) => {
                    process_timer();
                }
                Trap::Interrupt(Interrupt::MachineSoft) => {
                    process_ipi();
                }
                e @ _ => println!("unknown exception {:?}@{:x}", e, (*ctx_ptr).mepc),
            }
            None
        }),
    );
    runtime
}
