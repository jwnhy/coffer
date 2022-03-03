use super::cstr::CStr;
use super::memory_reserve::FdtMemoryReserveEntry;
use super::node::FdtNode;
use endiantype::*;
const FDT_MAGIC: u32_be = u32_be::from_native(0xd00d_feed);

#[repr(C)]
pub struct FdtHeader {
    magic: u32_be,
    total_size: u32_be,
    off_dt_struct: u32_be,
    off_dt_strings: u32_be,
    off_mem_rsvmap: u32_be,
    version: u32_be,
    last_comp_version: u32_be,
    boot_cpuid_phys: u32_be,
    size_dt_strings: u32_be,
    size_dt_struct: u32_be,
}

impl FdtHeader {
    pub unsafe fn from_ptr(fdt_ptr: *const u8) -> Result<&'static FdtHeader, &'static str> {
        if fdt_ptr.is_null() {
            return Err("[ERROR]: fdt pointer is null");
        }
        let magic = u32_be::new(*(fdt_ptr as *const u32));
        if magic != FDT_MAGIC {
            return Err("[ERROR]: FDT_MAGIC is not 0xd00dfeed");
        }
        let fdt_ref = &*(fdt_ptr as *mut FdtHeader);
        fdt_ref.check()?;
        Ok(fdt_ref)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self as *const FdtHeader as *const u8
    }

    pub fn total_size(&self) -> usize {
        self.total_size.to_native() as usize
    }

    
    pub fn memory_reserve_ptr(&self) -> *const u8{
        unsafe {
            self.as_ptr().add(self.off_mem_rsvmap.to_native() as usize)
        }
    }

    pub fn fdt_node_ptr(&self) -> *const u8{
        unsafe {
            self.as_ptr().add(self.off_dt_struct.to_native() as usize)
        }
    }

    pub fn str_at_offset(&self, offset: usize) -> &'static CStr {
        unsafe { CStr::from_ptr(self.as_ptr().add(self.off_dt_strings.to_native() as usize + offset)) }
    }

    fn check(&self) -> Result<(), &'static str> {
        if self.off_dt_strings + self.size_dt_strings > self.total_size {
            return Err("[ERROR]: fdt dt_strings overflowed");
        }
        if self.off_dt_struct + self.size_dt_struct > self.total_size {
            return Err("[ERROR]: fdt dt_struct overflowed");
        }
        Ok(())
    }
}
