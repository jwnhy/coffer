use core::intrinsics::size_of;

use alloc::format;

use crate::util::addr::align;

use super::{cstr::CStr, endian::types::u32_be, header::FdtHeader, token::FDT_PROP};
#[repr(C)]
struct FdtPropInner {
    magic: u32_be,
    len: u32_be,
    nameoff: u32_be,
}

impl FdtPropInner {
    pub fn len(&self) -> usize {
        self.len.to_native() as usize
    }

    pub fn name_offset(&self) -> usize {
        self.nameoff.to_native() as usize
    }
}

pub struct FdtProp<'a> {
    inner: &'a FdtPropInner,
    name: &'static CStr,
    data: *mut u8,
}

impl<'a> FdtProp<'a> {
    pub unsafe fn from_ptr(prop_ptr: *const u8, header: &FdtHeader) -> Result<FdtProp<'a>, &'static str> {
        let magic = u32_be::new(*(prop_ptr as *const u32)); 
        if magic != FDT_PROP {
            return Err("[ERROR]: prop does not start with FDT_PROP");
        }
        let inner_ptr =  prop_ptr as *mut FdtPropInner;
        let inner = &*inner_ptr;
        let data_ptr = prop_ptr.add(size_of::<FdtPropInner>()) as *mut u8;
        Ok(Self {
            inner,
            name: header.str_at_offset(inner.name_offset()),
            data: data_ptr
        })
    }

    pub fn size(&self) -> usize {
        let unaligned = self.inner.len() + size_of::<FdtPropInner>();
        align(unaligned, 4)
    }
}
