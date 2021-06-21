#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm)]

extern crate alloc;

mod ecall;
mod hal;
mod rvbt;
mod util;
#[macro_use]
mod sbi;

use buddy_system_allocator::LockedHeap;
use core::panic::PanicInfo;
use rvbt::{frame::trace, symbol::resolve_frame};

use crate::{rvbt::init::debug_init, util::fdt::{detect_ns16550a, detect_sifive_uart, init_fdt}};

const HART_STACK_SIZE: usize = 64 * 1024;
const NUM_CORES: usize = 8;
const SBI_STACK_SIZE: usize = NUM_CORES * HART_STACK_SIZE;
#[link_section = ".bss"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

const SBI_HEAP_SIZE: usize = 4096 * 1024;
#[link_section = ".bss"]
static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];
#[global_allocator]
static mut SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}

#[no_mangle]
fn rust_oom() -> ! {
    loop {}
}

#[no_mangle]
fn outer_layer() {
    test_trace();
}

#[no_mangle]
fn test_trace() {
    trace(&mut |frame| {
        resolve_frame(frame, &|symbol| println!("{}", symbol));
        true
    });
}

extern "C" fn main(hartid: usize, dtb: usize) -> ! {
    if hartid == 0 {
        init_heap();
        init_fdt(dtb);
        detect_ns16550a();
        detect_sifive_uart();
        debug_init();
        outer_layer();
        println!("RISC-V TEE in Rust");
        println!("dtb addr: 0x{:x}", dtb);
        let cpu_num = util::fdt::FDT.lock().as_ref().unwrap().cpus().count();
        println!("I have {} cores", cpu_num);
    }
    loop {}
}
fn init_heap() {
    unsafe {
        SBI_HEAP
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, SBI_HEAP_SIZE)
    }
}
#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry() -> ! {
    asm!(
    "
        nop
        la      sp, {stack}
        li      t0, {hart_stack_size}
        addi    t1, a0, 1
    1:  add     sp, sp, t0
        addi    t1, t1, -1
        bnez    t1, 1b
        j       {main}
        ",
    hart_stack_size = const HART_STACK_SIZE,
    stack = sym SBI_STACK,
    main = sym main,
    options(noreturn)
    )
}
