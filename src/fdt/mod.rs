use core::ops::Add;

use self::{header::FdtHeader, memory_reserve::FdtMemoryReserveBlock, node::FdtNode};

pub mod header;
pub mod memory_reserve;
pub mod node;
pub mod prop;
mod endian;
mod cstr;
mod token;

pub struct Fdt<'a, 'b> {
    header: FdtHeader<'a>,
    memory_reserve: FdtMemoryReserveBlock,
    root: FdtNode<'b>
}

impl<'a, 'b> Fdt<'a,'b> {
    pub unsafe fn from_ptr(fdt_ptr: *const u8) -> Result<Self, &'static str>{
        let header = FdtHeader::from_ptr(fdt_ptr)?;
        let memory_reserve = FdtMemoryReserveBlock::from_header(&header)?;
        let root = FdtNode::from_header(&header)?;
        Ok(Fdt::<'a, 'b> {
            header,
            memory_reserve,
            root,
        })
    }

    pub fn header(&self) -> &FdtHeader {
        &self.header
    }

    pub fn header_mut(&mut self) -> &'a mut FdtHeader {
        &mut self.header
    }

    pub fn memory_reserve(&self) -> &FdtMemoryReserveBlock {
        &self.memory_reserve
    }

    pub fn memory_reserve_mut(&mut self) -> &mut FdtMemoryReserveBlock {
        &mut self.memory_reserve
    }

}
