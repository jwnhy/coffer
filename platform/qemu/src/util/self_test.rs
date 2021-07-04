use core::ops::Generator; use core::pin::Pin;
use core::sync::atomic::AtomicBool;

use alloc::boxed::Box;
use riscv::register::mstatus::{self, Mstatus, MPP};
use riscv::register::{self, mcause, pmpcfg0};
use riscv::register::mcause::Exception::*;

use crate::memory::memory_layout::{MemoryLayout, Region};
use crate::memory::pmp::PmpFlags;
use crate::memory::pmp::{pmpaddr_read, pmpcfg_read, pmpcfg_write};
use crate::println;
use crate::runtime::context::Context;
use crate::runtime::runtime::Runtime;

use super::status::print_pmp;

/* global flag to indicate exception status */
static mut READ_FLAG: AtomicBool = AtomicBool::new(false);
static mut WRITE_FLAG: AtomicBool = AtomicBool::new(false);
static mut EXEC_FLAG: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch="riscv64")]
unsafe fn _write_test(addr: usize, expected: bool) {
    *WRITE_FLAG.get_mut() = false;
    mstatus::set_mpp(MPP::Supervisor);
    mstatus::set_mprv();
    asm!("
        sd {0}, 0({1})
        ", out(reg)_, in(reg)addr);
    assert_eq!(*WRITE_FLAG.get_mut(), expected);
}

#[cfg(target_arch="riscv64")]
unsafe fn _read_test(addr: usize, expected: bool) {
    *READ_FLAG.get_mut() = false;
    mstatus::set_mpp(MPP::Supervisor);
    mstatus::set_mprv();
    asm!("
        ld {0}, 0({1})
        ", out(reg)_, in(reg)addr);
    assert_eq!(*READ_FLAG.get_mut(), expected);
}

#[cfg(target_arch="riscv64")]
unsafe extern "C" fn _test_pmp(region: &Region) {
    /* first we allow all PMP config */
    let global_region = Region {
        addr: 0x0,
        size: 56,
        enabled: true,
        pmp_cfg: PmpFlags::READABLE
            | PmpFlags::WRITABLE
            | PmpFlags::EXECUTABLE
            | PmpFlags::MODE_NAPOT,
    };

    /* open up PMPs */
    global_region.enforce(15);
    region.enforce(1);

    /* test what we can access */
    let (s, e) = (region.addr_range().min().unwrap(), region.addr_range().max().unwrap());
    for addr in (s-1024..s).step_by(8) {
        _read_test(addr, false);
        _write_test(addr, false);
    }
    for addr in (e+1..e+1024).step_by(8) {
        _read_test(addr, false);
        _write_test(addr, false);
    }
    /* test what we cannot access */
    for addr in region.addr_range() {
        if !region.pmp_cfg.contains(PmpFlags::READABLE) {
            _read_test(addr, true);
        }
        if !region.pmp_cfg.contains(PmpFlags::WRITABLE) {
            _write_test(addr, true);
        }
    }
    asm!("ebreak");
}

pub fn test_pmp() {
    let mut ctx = Context::new();
    let sp: [u8; 8192] = [0; 8192];
    let region = Region {
        addr: 0x8100_0000,
        size: 16,
        enabled: true,
        pmp_cfg: PmpFlags::MODE_TOR,
    };

    /* now we can just set what we need and swap up */
    ctx.mstatus.set_mpp(MPP::Machine);
    ctx.a0 = &region as *const Region as usize;
    ctx.sp = &sp as *const u8 as usize;
    ctx.sp = ctx.sp + 8192;
    ctx.mepc = _test_pmp as usize;
    let mut runtime = Runtime::<()>::new(
        ctx,
        None,
        Box::new(|x| {
            unsafe {
                mstatus::set_mpp(MPP::Machine);
                (*x).mepc = (*x).mepc + 4;
                match mcause::read().cause() {
                    mcause::Trap::Exception(LoadFault) => { *READ_FLAG.get_mut() = true; },
                    mcause::Trap::Exception(StoreFault) => { *WRITE_FLAG.get_mut() = true; },
                    mcause::Trap::Exception(InstructionFault) => { *EXEC_FLAG.get_mut() = true; }
                    mcause::Trap::Exception(Breakpoint) => return Some(()),
                    e @ _ => panic!("[ERROR] unexpected exception: {:?}", e)

                }
            }
            None
        }),
    );
    runtime.init();
    Pin::new(&mut runtime).resume(());
}
