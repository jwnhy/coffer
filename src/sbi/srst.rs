use super::sbiret::SbiRet;
#[repr(u32)]
pub enum ResetType {
    Shutdown = 0x0000_0000,
    ColdReboot = 0x0000_0001,
    WarmReboot = 0x0000_0002,
}

#[repr(u32)]
pub enum ResetReason {
    NoReason = 0x0000_0000,
    SystemFailure = 0x0000_0001,
}

pub trait Srst :Send {
    fn system_reset(&mut self, reset_type: ResetType, reset_reason: ResetReason) -> SbiRet;
}

use alloc::boxed::Box;
use spin::Mutex;


lazy_static::lazy_static!{
    static ref SRST: Mutex<Option<Box<dyn Srst>>> = Mutex::new(None);
}

pub fn init_srst<T>(srst: T)
where T: Srst + Send + 'static
{
    *SRST.lock() = Some(Box::new(srst));
}

pub(crate) fn system_reset(reset_type: ResetType, reset_reason: ResetReason) -> SbiRet {
    if let Some(srst) = SRST.lock().as_mut() {
        srst.system_reset(reset_type, reset_reason)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn probe_srst() -> SbiRet {
    if let Some(_) = SRST.lock().as_ref() {
        SbiRet::ok(1)
    } else {
        SbiRet::ok(0)
    }
}
