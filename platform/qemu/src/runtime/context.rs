use riscv::register::mstatus::Mstatus;

#[repr(C)]
pub struct Context {
    pub ra: usize,  // x0
    pub sp: usize,  // x1
    pub gp: usize,  // x2
    pub tp: usize,  // x3
    pub t0: usize,  // x4
    pub t1: usize,  // x5
    pub t2: usize,  // x6
    pub fp: usize,  // x7
    pub s1: usize,  // x8
    pub a0: usize,  // x9
    pub a1: usize,  // x10
    pub a2: usize,  // x11
    pub a3: usize,  // x12
    pub a4: usize,  // x13
    pub a5: usize,  // x14
    pub a6: usize,  // x15
    pub a7: usize,  // x16
    pub s2: usize,  // x17
    pub s3: usize,  // x18
    pub s4: usize,  // x19
    pub s5: usize,  // x20
    pub s6: usize,  // x21
    pub s7: usize,  // x22
    pub s8: usize,  // x23
    pub s9: usize,  // x24
    pub s10: usize, // x25
    pub s11: usize, // x26
    pub t3: usize,  // x27
    pub t4: usize,  // x28
    pub t5: usize,  // x29
    pub t6: usize,  // x30

    pub mstatus: Mstatus, // x31
    pub mepc: usize,    // x32
    pub msp: usize,     //x33
}

impl Context {
    pub fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/* this function only saves callee-saved registers */
#[cfg(target_arch = "riscv64")]
#[naked]
#[link_section = ".text"]
pub(super) unsafe extern "C" fn from_machine(context: *mut Context) {
    // sp: main machine stack
    // a0.sp: per rt user stack
    asm!("
        addi    sp,     sp,     -15*8
        /* return addr */
        sd      ra,     0*8(sp)
        /* saved register */
        sd      gp,     1*8(sp)
        sd      tp,     2*8(sp)
        sd      s0,     3*8(sp)
        sd      s1,     4*8(sp)
        sd      s2,     5*8(sp)
        sd      s3,     6*8(sp)
        sd      s4,     7*8(sp)
        sd      s5,     8*8(sp)
        sd      s6,     9*8(sp)
        sd      s7,     10*8(sp)
        sd      s8,     11*8(sp)
        sd      s9,     12*8(sp)
        sd      s10,    13*8(sp)
        sd      s11,    14*8(sp)
        j       {to_user_or_supervisor}
        ", to_user_or_supervisor = sym to_user_or_supervisor, options(noreturn))
}

#[cfg(target_arch = "riscv64")]
#[naked]
#[link_section = ".text"]
pub(super) unsafe extern "C" fn from_user_or_supervisor() {
    asm!("
         /* Page 36 of https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf */
         .p2align 2
         /* mscratch = a0
          * a0 = mscratch
          */
         csrrw  a0, mscratch, a0
         sd     ra,     0*8(a0)
         sd     sp,     1*8(a0)
         sd     gp,     2*8(a0)		// x2
         sd     tp,     3*8(a0)		// x3
         sd     t0,     4*8(a0)		// x4
         sd     t1,     5*8(a0)		// x5
         sd     t2,     6*8(a0)		// x6
         sd     fp,     7*8(a0)		// x7
         sd     s1,     8*8(a0)		// x8
	     /* keeps a0 before done */
         sd     a1,     10*8(a0)		// x10
         sd     a2,     11*8(a0)		// x11
         sd     a3,     12*8(a0)		// x12
         sd     a4,     13*8(a0)		// x13
         sd     a5,     14*8(a0)		// x14
         sd     a6,     15*8(a0)		// x15
         sd     a7,     16*8(a0)		// x16
         sd     s2,     17*8(a0)		// x17
         sd     s3,     18*8(a0)		// x18
         sd     s4,     19*8(a0)		// x19
         sd     s5,     20*8(a0)		// x20
         sd     s6,     21*8(a0)		// x21
         sd     s7,     22*8(a0)		// x22
         sd     s8,     23*8(a0)		// x23
         sd     s9,     24*8(a0)		// x24
         sd     s10,    25*8(a0)		// x25
         sd     s11,    26*8(a0)		// x26
         sd     t3,     27*8(a0)		// x27
         sd     t4,     28*8(a0)		// x28
         sd     t5,     29*8(a0)		// x29
         sd     t6,     30*8(a0)		// x30
         csrr   t1,     mstatus
         sd     t1,     31*8(a0)
         csrr   t1,     mepc
         sd     t1,     32*8(a0)
         
         /* mscratch = a0;
          * t1 = mscratch;
          */
         csrrw  t1, mscratch, a0
         sd     t1, 9*8(a0)
         ld      sp,     33*8(a0)
         j      {to_machine}
         ",to_machine = sym to_machine, options(noreturn))
}

#[cfg(target_arch = "riscv64")]
#[naked]
#[link_section = ".text"]
pub(super) unsafe extern "C" fn to_machine() {
    asm!(
        "
        /* return addr */
        ld      ra,     0*8(sp)
        /* saved register */
        ld      gp,     1*8(sp)
        ld      tp,     2*8(sp)
        ld      s0,     3*8(sp)
        ld      s1,     4*8(sp) 
        ld      s2,     5*8(sp)
        ld      s3,     6*8(sp)
        ld      s4,     7*8(sp)
        ld      s5,     8*8(sp)
        ld      s6,     9*8(sp)
        ld      s7,     10*8(sp)
        ld      s8,     11*8(sp)
        ld      s9,     12*8(sp)
        ld      s10,    13*8(sp)
        ld      s11,    14*8(sp)
        addi    sp,     sp,     15*8
        ret
         ",
        options(noreturn)
    )
}

#[cfg(target_arch = "riscv64")]
#[naked]
#[link_section = ".text"]
pub(super) unsafe extern "C" fn to_user_or_supervisor(context: *mut Context) {
    asm!(
        "
        sd      sp,         33*8(a0)
        csrw    mscratch,   a0
        ld      t0,         31*8(a0)
        ld      t1,         32*8(a0)
        csrw    mstatus,    t0
        csrw    mepc,       t1

        ld      ra,         0*8(a0)
        ld      sp,         1*8(a0)
        ld      gp,         2*8(a0)		// x2
        ld      tp,         3*8(a0)		// x3
        ld      t0,         4*8(a0)		// x4
        ld      t1,         5*8(a0)		// x5
        ld      t2,         6*8(a0)		// x6
        ld      fp,         7*8(a0)		// x7
        ld      s1,         8*8(a0)		// x8
		/* keeps a0 before done */
        ld      a1,         10*8(a0)		// x10
        ld      a2,         11*8(a0)		// x11
        ld      a3,         12*8(a0)		// x12
        ld      a4,         13*8(a0)		// x13
        ld      a5,         14*8(a0)		// x14
        ld      a6,         15*8(a0)		// x15
        ld      a7,         16*8(a0)		// x16
        ld      s2,         17*8(a0)		// x17
        ld      s3,         18*8(a0)		// x18
        ld      s4,         19*8(a0)		// x19
        ld      s5,         20*8(a0)		// x20
        ld      s6,         21*8(a0)		// x21
        ld      s7,         22*8(a0)		// x22
        ld      s8,         23*8(a0)		// x23
        ld      s9,         24*8(a0)		// x24
        ld      s10,        25*8(a0)		// x25
        ld      s11,        26*8(a0)		// x26
        ld      t3,         27*8(a0)		// x27
        ld      t4,         28*8(a0)		// x28
        ld      t5,         29*8(a0)		// x29
        ld      t6,         30*8(a0)		// x30
        ld      a0,         9*8(a0)		// x9
        mret
         ",
        options(noreturn)
    )
}
