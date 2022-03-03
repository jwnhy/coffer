use core::slice;

use self::{
    header::FdtHeader,
    memory_reserve::{FdtMemoryReserveEntry, FdtMemoryReserveIter},
    node::{FdtNode, FdtNodeIter},
};

mod cstr;
pub mod header;
pub mod memory_reserve;
pub mod node;
pub mod prop;
mod token;

pub struct Fdt {
    header: &'static FdtHeader,
    inner_buffer: &'static [u8]
}

impl Fdt {
    pub unsafe fn from_ptr(fdt_ptr: *const u8) -> Result<Self, &'static str> {
        let header = FdtHeader::from_ptr(fdt_ptr)?;
        Ok(Fdt { header, inner_buffer: slice::from_raw_parts_mut(fdt_ptr as *mut _, header.total_size()) })
    }

    pub fn header(&self) -> &FdtHeader {
        &self.header
    }

    pub fn memory_reserve_iter(&self) -> impl Iterator<Item = &'static FdtMemoryReserveEntry> {
        unsafe { FdtMemoryReserveIter::from_ptr(self.header().memory_reserve_ptr()) }
    }

    pub fn node_iter(&self) -> impl Iterator<Item = &'static FdtNode> {
        unsafe { FdtNodeIter::from_ptr(self.header.fdt_node_ptr()) }
    }
}
