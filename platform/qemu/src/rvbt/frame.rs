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

#[inline(always)]
pub fn trace_from(mut curframe: Frame, action: &mut dyn FnMut(&Frame) -> bool) {
    loop {
        let keep_going = action(&curframe);
        if keep_going {
            unsafe {
                curframe.ra = *((curframe.fp + 8) as *mut u64);
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
pub fn trace(action: &mut dyn FnMut(&Frame) -> bool) {
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
