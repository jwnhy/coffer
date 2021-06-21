#[derive(Debug, Clone)]
pub struct Frame {
    pub fp: u64,
    pub sp: u64,
    pub ra: u64,
}

impl Frame {
    pub fn new(fp: u64, sp: u64, ra: u64) -> Self {
        Self { fp, sp, ra }
    }
}

#[cfg(target_arch="riscv64")]
const XLEN: u64 = 8;
#[cfg(target_arch="riscv32")]
const XLEN: u64 = 4;

#[inline(always)]
pub fn trace_from(mut curframe: Frame, action: &dyn Fn(&Frame) -> bool) {
    loop {
        let keep_going = action(&curframe);
        if keep_going {
            unsafe {
                // TODO: decide incr depending on arch
                curframe.ra = *((curframe.fp + XLEN) as *mut u64);
                curframe.sp = curframe.fp;
                curframe.fp = *(curframe.fp as *mut u64);
                if curframe.ra == 0 || curframe.fp == 0 {
                    break;
                }
            }
        } else {
            break;
        }
    }
}

#[inline(always)]
pub fn trace(action: &dyn Fn(&Frame) -> bool) {
    let (fp, sp, ra): (u64, u64, u64);
    unsafe {
        asm!("
        mv {0}, s0
        mv {1}, x2
        mv {2}, x1
        ", out(reg) fp, out(reg) sp, out(reg) ra);
    }
    let curframe = Frame::new(fp, sp, ra);
    trace_from(curframe, action)
}
