use core::panic::PanicInfo;

use buddy_system_allocator::LockedHeap;
use crate::main;
use crate::platform::sunxi::sunxi_init;
use crate::platform::virt::virt_init;
use crate::println;


const HART_STACK_SIZE: usize = 8 * 1024;
const NUM_CORES: usize = 1;
const SBI_STACK_SIZE: usize = NUM_CORES * HART_STACK_SIZE;

#[no_mangle]
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];


const SBI_HEAP_SIZE: usize = 8 * 1024;

#[no_mangle]
#[link_section = ".bss.uninit"]
static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];

#[global_allocator]
static SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}

#[no_mangle]
fn rust_oom() -> ! {
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

pub fn generic_init(dtb: usize) -> usize {
    init_bss();
    init_heap();
    match () {
        #[cfg(feature = "sunxi")]
        () => sunxi_init(dtb),
        #[cfg(feature = "virt")]
        () => virt_init(dtb),
        #[cfg(feature = "sifive")]
        () => sifive_init(dtb),
        _ => unreachable!(),
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
