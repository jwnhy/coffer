use core::ops::Add;

use super::cstr::CStr;
use super::endian::types::*;
use super::endian::{BigEndian, LittleEndian};
const FDT_MAGIC: u32_be = u32_be::from_native(0xd00d_feed);

#[repr(C)]
struct FdtHeaderInner {
    magic: u32_be,
    total_size: u32_be,
    off_dt_struct: u32_be,
    off_dt_strings: u32_be,
    off_mem_rsvmap: u32_be,
    version: u32_be,
    last_comp_version: u32_be,
    boot_cpuid_phys: u32_be,
    size_dt_strings: u32_be,
    size_dt_struct: u32_be
}

pub struct FdtHeader<'a> {
    inner: &'a FdtHeaderInner
}

impl<'a> FdtHeader<'a> {
    pub unsafe fn from_ptr(fdt_ptr: *const u8) -> Result<FdtHeader<'a>, &'static str> {
        if fdt_ptr.is_null() {
            return Err("[ERROR]: fdt pointer is null");
        }
        let magic = u32_be::new(*(fdt_ptr as *const u32));
        if magic != FDT_MAGIC {
            return Err("[ERROR]: FDT_MAGIC is not 0xd00dfeed");
        }
        let fdt_ref = &*(fdt_ptr as *mut FdtHeaderInner);
        let fdt_head = FdtHeader{inner: fdt_ref};
        fdt_head.check()?;
        Ok(fdt_head)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner as *const FdtHeaderInner as *const u8
    }

    pub fn memory_reserve_offset(&self) -> usize {
        self.inner.off_mem_rsvmap.to_native() as usize
    }

    pub fn strings_offset(&self) -> usize {
        self.inner.off_dt_strings.to_native() as usize
    }

    pub fn struct_offset(&self) -> usize {
        self.inner.off_dt_struct.to_native() as usize
    }

    pub fn str_at_offset(&self, offset: usize) -> &'static CStr {
        unsafe {CStr::from_ptr(self.as_ptr().add(self.strings_offset() + offset))}
    }

    fn check(&self) -> Result<(), &'static str> {
        let head = self.inner;
        if head.off_dt_strings + head.size_dt_strings > head.total_size {
            return Err("[ERROR]: fdt dt_strings overflowed");
        }
        if head.off_dt_struct + head.size_dt_struct > head.total_size {
            return Err("[ERROR]: fdt dt_struct overflowed");
        }
        Ok(())
    }
}
