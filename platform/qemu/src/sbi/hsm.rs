use super::sbiret::SbiRet;

#[repr(u8)]
pub enum HartState {
    Started = 0,
    Stopped,
    StartPending,
    StopPending,
    Suspended,
    SuspendedPening,
    ResumePending
}

pub trait Hsm : Send {
    fn hart_start(&mut self, hartid: usize, start_addr: usize, opaque: usize) -> SbiRet;
    fn hart_stop(&mut self) -> SbiRet;
    fn hart_get_status(&mut self, hartid: usize) -> SbiRet;
    fn hart_suspend(&mut self, suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet;
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static!{
    static ref HSM: Mutex<Option<Box<dyn Hsm>>> = Mutex::new(None);
}

pub fn init_hsm<T>(hsm: T)
where T: Send + Hsm + 'static
{
    *HSM.lock() = Some(Box::new(hsm));
}

pub(crate) fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet{
    if let Some(hsm) = HSM.lock().as_mut() {
        hsm.hart_start(hartid, start_addr, opaque)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn hart_stop() -> SbiRet {
    if let Some(hsm) = HSM.lock().as_mut() {
        hsm.hart_stop()
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn hart_get_status(hartid: usize) -> SbiRet {
    if let Some(hsm) = HSM.lock().as_mut() {
        hsm.hart_get_status(hartid)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    if let Some(hsm) = HSM.lock().as_mut() {
        hsm.hart_suspend(suspend_type, resume_addr, opaque)
    } else {
        SbiRet::not_supported()
    }
}
