use super::{hart_mask, sbiret::SbiRet};
pub trait Timer: Send {
    fn set_timer(&self, stime_value: u64);
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static!{
    static ref TIMER: Mutex<Option<Box<dyn Timer>>> = Mutex::new(None);
}

pub fn init_timer<T>(timer: T)
where T: Timer + Send + 'static
{
    *TIMER.lock() = Some(Box::new(timer));
}

pub(crate) fn set_timer(stime_value: u64) -> SbiRet {
    if let Some(timer) = TIMER.lock().as_mut() {
        timer.set_timer(stime_value);
        SbiRet::ok(0)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn probe_timer() -> SbiRet {
    if let Some(_) = TIMER.lock().as_ref() {
        SbiRet::ok(1)
    } else {
        SbiRet::ok(0)
    }
}