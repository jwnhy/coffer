use bitflags::*;

bitflags! {
    pub struct PmpFlags: u8 {
        const READABLE =    1 << 0;
        const WRITABLE =    1 << 1;
        const EXECUTABLE =  1 << 2;
        const MODE_TOR =    1 << 3;
        const MODE_NA4 =    2 << 3;
        const MODE_NAPOT =  3 << 3;
        const LOCKED =      1 << 7;
    }
}
struct Region {
    addr: usize,
    size: usize,
    enabled: bool,
    pmp_cfg: PmpFlags,
    pmp_num: usize,
}

impl Region {
    fn to_napot(&self) -> usize {
        if self.size <= 2 {
            self.addr
        } else {
            self.addr | !(1 << (self.size - 2))
        }
    }

    fn to_tor(&self) -> (usize, usize) {
        (self.addr, self.addr + self.size)
    }
}

pub struct MemoryLayout {
    /* at most 16 pmp region is allowed */
    regions: [Region; 16]
}
