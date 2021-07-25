use bit_field::BitField;


#[derive(Debug, Clone)]
pub struct HartMask {
    mask: usize,
    base: usize,
}

impl HartMask {
    pub unsafe fn new(mask: usize, base: usize) -> Self {
        HartMask {
            mask,
            base,
        }
    }

    pub fn has(&self, hartid: usize) -> bool {
        // TODO: add maximum assertion here
        if self.base == usize::MAX {
            return true;
        }
        if hartid < self.base {
            return false;
        }
        self.mask.get_bit(hartid - self.base)
    }
}
