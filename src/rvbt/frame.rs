use crate::util::fdt::XLEN;

#[derive(Debug, Clone)]
pub struct Frame {
    pub fp: usize,
    pub sp: usize,
    pub ra: usize,
}

impl Frame {
    pub fn new(fp: usize, sp: usize, ra: usize) -> Self {
        Self { fp, sp, ra }
    }
}


#[inline(always)]
pub fn trace_from(mut curframe: Frame, action: &dyn Fn(&Frame) -> bool) {
    loop {
        let keep_going = action(&curframe);
        if keep_going {
            unsafe {
                // TODO: decide incr depending on arch
                curframe.ra = *((curframe.fp + XLEN / 8) as *mut usize);
                curframe.sp = curframe.fp;
                curframe.fp = *(curframe.fp as *mut usize);
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
    let (fp, sp, ra): (usize, usize, usize);
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
