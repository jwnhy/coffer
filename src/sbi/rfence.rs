use super::sbiret::SbiRet;
use super::fence_info::FenceInfo;

pub trait LocalFence: Send {
    fn local_sfence(&self, finfo: FenceInfo);
    fn local_hfence(&self, finfo: FenceInfo);
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref LOCAL_FENCE: Mutex<Option<Box<dyn LocalFence>>> = Mutex::new(None);
}

pub fn init_rfence<T>(rfence: T)
where
    T: LocalFence + Send + 'static,
{
    *LOCAL_FENCE.lock() = Some(Box::new(rfence));
}


pub(crate) fn probe_rfence() -> SbiRet {
    use super::ipi::IPI;
    if IPI.lock().as_ref().is_some() && LOCAL_FENCE.lock().as_ref().is_some() {
        SbiRet::ok(1)
    } else {
        SbiRet::ok(0)
    }
}
