use core::ops::Range;

use bit_field::BitField;

use super::pmp::{PmpFlags, pmpaddr_write, pmpcfg_write};
use crate::util::fdt::XLEN;

#[repr(C)]
pub struct Region {
    /* one protected region */
    pub addr: usize,
    pub size: usize,
    pub enabled: bool,
    pub pmp_cfg: PmpFlags,
}

impl Region {
    pub fn addr_range(&self) -> Range<usize> {
        if self.pmp_cfg.contains(PmpFlags::MODE_NA4) || self.pmp_cfg.contains(PmpFlags::MODE_NAPOT) {
            Range{ start: self.addr, end: self.addr + (1 << self.size)}
        } else {
            Range{ start: self.addr, end: self.addr + self.size }
        }
    }

    fn to_napot(&self) -> usize {
        if self.size < 2 || self.size > 56 {
            panic!("[ERROR] invalid pmp napot value");
        }
        if self.size == 2 {
            self.addr >> 2
        } else {
            let mut pmpaddr = self.addr;
            pmpaddr = pmpaddr >> 2;
            pmpaddr.set_bit(self.size-3, false);
            let k = 1usize << (self.size-3)-1;
            pmpaddr.set_bits(0..(self.size-3), (1 << (self.size-3))-1);
            pmpaddr
        }
    }

    fn to_tor(&self) -> (usize, usize) {
        (self.addr >> 2, (self.addr + self.size) >> 2)
    }

    pub fn enforce(&self, index: usize) {
        if !self.enabled {
            return
        }

        if self.pmp_cfg.contains(PmpFlags::MODE_NA4) || self.pmp_cfg.contains(PmpFlags::MODE_NAPOT) {
            pmpaddr_write(index, self.to_napot());
        } else {
            let (s, e) = self.to_tor();
            pmpaddr_write(index-1, s);
            pmpaddr_write(index, e);
        }
        pmpcfg_write(index, self.pmp_cfg.bits());
    }

    pub fn exempt(&self, index: usize) {
        pmpcfg_write(index, 0x0);
        pmpaddr_write(index, 0x0);
    }
}

pub struct MemoryLayout {
    /* at most 16 pmp region is allowed */
    regions: [Region; 16]
}

impl MemoryLayout {
    pub fn enforce(&self) {
        for (i, region) in self.regions.iter().enumerate() {
            region.enforce(i);
        }
    }

    pub fn exempt(&self) {
        for (i, region) in self.regions.iter().enumerate() {
            region.exempt(i);
        }
    }
}
