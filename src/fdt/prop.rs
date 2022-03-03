use core::intrinsics::size_of;
use alloc::format;
use endiantype::*;
use super::{
    cstr::CStr,
    header::FdtHeader,
    token::{align, skip_nop, FDT_PROP},
};

#[repr(C)]
pub struct FdtProp {
    magic: u32_be,
    len: u32_be,
    nameoff: u32_be,
}

impl FdtProp {
    pub fn len(&self) -> usize {
        self.len.to_native() as usize
    }

    pub fn name_offset(&self) -> usize {
        self.nameoff.to_native() as usize
    }
}

pub(crate) struct FdtPropIter {
    raw_ptr: *const u8,
}

impl Iterator for FdtPropIter {
    type Item = &'static FdtProp;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_ptr = skip_nop(self.raw_ptr);
        let result = unsafe { FdtProp::from_ptr(self.raw_ptr) };
        if let Ok(prop) = result {
            self.raw_ptr = unsafe { self.raw_ptr.add(prop.size()) };
            Some(prop)
        } else {
            None
        }
    }
}

impl FdtPropIter {
    pub unsafe fn from_ptr(raw_ptr: *const u8) -> Self {
        FdtPropIter { raw_ptr }
    }
}

impl FdtProp {
    pub unsafe fn from_ptr(prop_ptr: *const u8) -> Result<&'static FdtProp, &'static str> {
        let prop_ptr = skip_nop(prop_ptr);
        let magic = u32_be::new(*(prop_ptr as *const u32));
        if magic != FDT_PROP {
            return Err("[ERROR]: prop does not start with FDT_PROP");
        }
        let prop_ptr = prop_ptr as *mut FdtProp;
        Ok(&*prop_ptr)
    }

    pub fn size(&self) -> usize {
        let unaligned = self.len() + size_of::<FdtProp>();
        align(unaligned, 4)
    }
}
