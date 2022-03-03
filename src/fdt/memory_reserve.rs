use core::ops::{Index, IndexMut};

use super::header::FdtHeader;
use endiantype::*;

#[repr(C)]
pub struct FdtMemoryReserveEntry {
    address: u64_be,
    size: u64_be,
}

impl FdtMemoryReserveEntry {
    pub fn valid(&self) -> bool {
        !(self.address == 0 && self.size == 0)
    }

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

pub(crate) struct FdtMemoryReserveIter {
    raw_ptr: *const FdtMemoryReserveEntry,
}

impl Iterator for FdtMemoryReserveIter {
    type Item = &'static FdtMemoryReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let result = unsafe { &*self.raw_ptr };
        if result.valid() {
            self.raw_ptr = unsafe { self.raw_ptr.add(1) };
            Some(result)
        } else {
            None
        }
    }
}

impl FdtMemoryReserveIter {
    pub unsafe fn from_ptr(raw_ptr: *const u8) -> Self {
        FdtMemoryReserveIter {
            raw_ptr: raw_ptr as *const FdtMemoryReserveEntry,
        }
    }
}
