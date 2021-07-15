#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(arbitrary_enum_discriminant)]
#![feature(generator_trait)]
#![feature(fn_align)]

extern crate alloc;

mod ecall;
mod hal;
mod memory;
mod runtime;
mod rvbt;
mod util;
#[macro_use]
mod sbi;

use crate::{memory::pmp::PmpFlags, sbi::timer::process_timer, util::{fdt::init_sunxi_clint, timer_test::test_timer}};
use alloc::boxed::Box;
use buddy_system_allocator::LockedHeap;
use core::{
    ops::Generator,
    panic::PanicInfo,
    pin::Pin,
    ptr::{slice_from_raw_parts_mut, write_volatile},
};
use ecall::handle_ecall;
use goblin::elf::Elf;
use riscv::register::{mcause::{self, Exception, Interrupt, Trap}, mcounteren, medeleg, mideleg, mie, mstatus::{self, MPP}, satp::{self, Satp}, sie::Sie, sscratch, stvec};
use runtime::{context::Context, runtime::Runtime};
use rvbt::{frame::trace, symbol::resolve_frame};

use crate::{
    memory::memory_layout::Region,
    rvbt::init::debug_init,
    sbi::console_getchar,
    util::{
        fdt::{detect_clint, detect_ns16550a, detect_sifive_uart, detect_sunxi_uart, init_fdt},
        status::{print_machine, print_mstatus, print_mtvec},
    },
};

const HART_STACK_SIZE: usize = 8 * 1024;
const NUM_CORES: usize = 1;
const SBI_STACK_SIZE: usize = NUM_CORES * HART_STACK_SIZE;

#[no_mangle]
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

static DEVICE_TREE: &[u8] = include_bytes!("../board.dtb");

const SBI_HEAP_SIZE: usize = 8 * 1024;

#[no_mangle]
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
        init_fdt(DEVICE_TREE.as_ptr() as usize);
        //init_fdt(dtb);
        //detect_ns16550a();
        //detect_sifive_uart();
        detect_sunxi_uart();
        init_sunxi_clint(0x1400_0000);
        println!("dtb is {:x}", dtb);
        println!("sunxi dtb is {:x}", SUNXI_HEAD.dtb_base);
        println!("sunxi second dtb is {:x}", SUNXI_HEAD.uboot_base);
        println!("Serial is fine");
        //detect_clint();
        //test_pmp();

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
        //detect_ns16550a();
        //detect_sunxi_uart();

        println!("RISC-V TEE in Rust");
        println!("dtb addr: 0x{:x}", dtb);
        let mut rt = kernel_runtime(hartid, 0x4200_0000, 0x4200_0000);
        Pin::new(&mut rt).resume(());
    }
    loop {}
}

fn kernel_runtime(hartid: usize, dtb: usize, kernel_addr: usize) -> Runtime<()> {
    let mut ctx = Context::new();
    ctx.a0 = hartid;
    ctx.a1 = dtb;
    ctx.mepc = kernel_addr;
    ctx.mstatus.set_mpp(MPP::Supervisor);
    ctx.mstatus.set_fs(riscv::register::sstatus::FS::Dirty);
    ctx.mcounteren = 0xffff_ffff;
    ctx.medeleg = 0xb1ff;
    ctx.mideleg = 0x222;
    unsafe {
        riscv::register::mie::set_msoft();
        riscv::register::sscratch::write(0x0);
        riscv::register::satp::write(0x0);
        stvec::write(kernel_addr, riscv::register::mtvec::TrapMode::Direct);
        // TODO: SETUP PLIC
        write_volatile(0x101F_FFFC as *mut u32, 0x1);
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
                e @ _ => println!("unknown exception {:?}@{:x}", e, (*ctx_ptr).mepc),
            }
            None
        }),
    );
    runtime
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
        r0::init_data(&mut _data_start, &mut _data_end, &_flash_data);
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
