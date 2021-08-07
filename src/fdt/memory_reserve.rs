use core::ops::{Index, IndexMut};

use super::{
    endian::{types::u64_be, BigEndian},
    header::FdtHeader,
};

#[repr(C)]
pub struct FdtMemoryReserveEntry {
    address: u64_be,
    size: u64_be,
}

impl FdtMemoryReserveEntry {
    pub fn address(&self) -> usize {
        self.address.to_native() as usize
    }

    pub fn size(&self) -> usize {
        self.size.to_native() as usize
    }

    pub fn set_address(&mut self, new_addr: u64) {
        self.address = BigEndian::<u64>::from_native(new_addr);
    }

    pub fn set_size(&mut self, new_size: u64) {
        self.size = BigEndian::<u64>::from_native(new_size);
    }
}

pub struct FdtMemoryReserveBlock {
    raw_ptr: *mut FdtMemoryReserveEntry,
    size: usize,
}

pub struct FdtMemoryReserveIter<'a> {
    idx: usize,
    rsv_block: &'a FdtMemoryReserveBlock,
}

impl<'a> Iterator for FdtMemoryReserveIter<'a> {
    type Item = &'a FdtMemoryReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.rsv_block.size {
            None
        } else {
            let result = unsafe { &*self.rsv_block.raw_ptr.add(self.idx) };
            self.idx += 1;
            Some(result)
        }
    }
}

impl<'a> Iterator for FdtMemoryReserveIterMut<'a> {
    type Item = &'a mut FdtMemoryReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.rsv_block.size {
            None
        } else {
            let result = unsafe { &mut *self.rsv_block.raw_ptr.add(self.idx) };
            self.idx += 1;
            Some(result)
        }
    }
}

pub struct FdtMemoryReserveIterMut<'a> {
    idx: usize,
    rsv_block: &'a mut FdtMemoryReserveBlock,
}

impl FdtMemoryReserveBlock {
    pub unsafe fn from_header(header: &FdtHeader) -> Result<FdtMemoryReserveBlock, &'static str> {
        let rsv_offset = (*header).memory_reserve_offset();
        let header_ptr = header.as_ptr();
        Self::from_ptr(header_ptr.add(rsv_offset))
    }

    pub unsafe fn from_ptr(rsv_ptr: *const u8) -> Result<FdtMemoryReserveBlock, &'static str> {
        let mut rsv_cur = rsv_ptr as *const FdtMemoryReserveEntry;
        let mut rsv_size = 0;
        let mut rsv_entry = &*rsv_cur;
        while rsv_entry.address != 0 && rsv_entry.size != 0 {
            rsv_size += 1;
            rsv_cur = rsv_cur.add(1);
            rsv_entry = &*rsv_cur;
        }
        Ok(FdtMemoryReserveBlock {
            raw_ptr: rsv_ptr as *mut FdtMemoryReserveEntry,
            size: rsv_size,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &FdtMemoryReserveEntry> {
        FdtMemoryReserveIter {
            idx: 0,
            rsv_block: self,
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut FdtMemoryReserveEntry> {
        FdtMemoryReserveIterMut {
            idx: 0,
            rsv_block: self,
        }
    }
}

impl<'a> Index<usize> for FdtMemoryReserveBlock {
    type Output = FdtMemoryReserveEntry;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size);
        unsafe { &*self.raw_ptr.add(index) }
    }
}

impl<'a> IndexMut<usize> for FdtMemoryReserveBlock {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size);
        unsafe { &mut *self.raw_ptr.add(index) }
    }
}
