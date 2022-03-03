use core::{fmt::{self, Display}, iter::FromIterator};

use alloc::string::String;

#[repr(transparent)]
pub struct CStr {
    dummy: u8,
}

impl CStr {
    pub unsafe fn from_ptr(str_ptr: *const u8) -> &'static Self {
        &*(str_ptr as *const CStr)
    }
    
    pub fn len(&self) -> usize {
        let mut cnt = 0;
        let mut cur = self as *const CStr as *const u8;
        unsafe {
            while *cur != '\0' as u8 {
                cnt += 1;
                cur = cur.add(1);
            }
        }
        cnt
    }
}

impl Display for CStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cur = self as *const CStr as *const u8;
        unsafe {
            while *cur != '\0' as u8 {
                write!(f, "{}", *cur as char);
                cur = cur.add(1);
            }
        }
        Ok(())
    }
}

