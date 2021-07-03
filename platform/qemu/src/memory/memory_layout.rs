use bit_field::BitField;

use super::pmp::{PmpFlags, pmpaddr_write, pmpcfg_write};
use crate::util::fdt::XLEN;

pub struct Region {
    /* one protected region */
    addr: usize,
    size: usize,
    pub enabled: bool,
    pub pmp_cfg: PmpFlags,
}

impl Region {
    fn to_napot(&self) -> usize {
        if self.size <= 2 {
            self.addr >> 2
        } else {
            let mut pmpaddr = self.addr;
            pmpaddr.set_bit(self.size-3, false);
            pmpaddr.set_bits(0..(self.size-4), (1 << (self.size-3))-1);
            pmpaddr >> 2
        }
    }

    fn to_tor(&self) -> (usize, usize) {
        (self.addr >> 2, (self.addr + self.size) >> 2)
    }

    fn enforce(&self, index: usize) {
        if self.pmp_cfg.contains(PmpFlags::MODE_NAPOT) {
            pmpaddr_write(index, self.to_napot())
        } else {
            let (s, e) = self.to_tor();
            pmpaddr_write(index-1, s);
            pmpaddr_write(index, e);
        }
        pmpcfg_write(index, self.pmp_cfg.bits());
    }
}

pub struct MemoryLayout {
    /* at most 16 pmp region is allowed */
    //regions: [Region; 16]
}
