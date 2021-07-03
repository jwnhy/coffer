#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(arbitrary_enum_discriminant)]
#![feature(generator_trait)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate alloc;

mod memory;
mod runtime;
mod ecall;
mod hal;
mod rvbt;
mod util;
#[macro_use]
mod sbi;

use buddy_system_allocator::LockedHeap;
use core::panic::PanicInfo;
use rvbt::{frame::trace, symbol::resolve_frame};

use crate::{rvbt::init::debug_init, sbi::console_getchar, util::{fdt::{detect_ns16550a, detect_sifive_uart, detect_sunxi_uart, init_fdt}, self_test::{test_context, test_pmp}, status::{print_machine, print_mstatus, print_mtvec}}};

const HART_STACK_SIZE: usize = 8 * 1024;
const NUM_CORES: usize = 2;
const SBI_STACK_SIZE: usize = NUM_CORES * HART_STACK_SIZE;
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

const SBI_HEAP_SIZE: usize = 64 * 1024;
#[link_section = ".bss.uninit"]
static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];

#[global_allocator]
static SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

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

#[link_section = ".head_data"]
static SUNXI_HEAD: SunxiHead = SunxiHead {
    jump_inst: 0x4000_006f, // j 0x4000_0400
    magic: *b"opensbi\0",
    dtb_base: 0,
    uboot_base: 0,
    res3: 0,
    res4: 0,
    res5: [0; 8],
    res6: [0; 8],
    opensbi_base: 0x4000_0000,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}

#[no_mangle]
fn rust_oom() -> ! {
    loop {}
}


extern "C" fn main(hartid: usize, dtb: usize) -> ! {
    let hartid = riscv::register::mhartid::read();
    if hartid == 0 {
        init_bss();
        init_heap();
        init_fdt(dtb);
        detect_sifive_uart();
        test_context();

        test_pmp();
        print_machine();
        //detect_ns16550a();
        //detect_sunxi_uart();
        //debug_init();

        println!("RISC-V TEE in Rust");
        println!("dtb addr: 0x{:x}", dtb);
        let cpu_num = util::fdt::FDT.lock().as_ref().unwrap().cpus().count();
        println!("I have {} cores", cpu_num);
    }
    loop {}
}

fn init_bss() {
    extern "C" {
        static mut _bss_start: u32;
        static mut _bss_end: u32;
        static mut _data_start: u32;
        static mut _data_end: u32;
        static _flash_data: u32;
    }
    unsafe {
        r0::zero_bss(&mut _bss_start, &mut _bss_end);
        r0::init_data(&mut _data_start,&mut _data_end , &_flash_data);
    }
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
        /* flush the instruction cache */
	    fence.i
	    /* Reset all registers except ra, a0, a1 and a2 */
	    li sp, 0
	    li gp, 0
	    li tp, 0
	    li t0, 0
	    li t1, 0
	    li t2, 0
	    li s0, 0
	    li s1, 0
	    li a3, 0
	    li a4, 0
	    li a5, 0
	    li a6, 0
	    li a7, 0
	    li s2, 0
	    li s3, 0
	    li s4, 0
	    li s5, 0
	    li s6, 0
	    li s7, 0
	    li s8, 0
	    li s9, 0
	    li s10, 0
	    li s11, 0
	    li t3, 0
	    li t4, 0
	    li t5, 0
	    li t6, 0
	    csrw mscratch, 0

        
        nop
        la      sp, {stack}
        li      t0, {hart_stack_size}
        csrr    a0, mhartid
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
