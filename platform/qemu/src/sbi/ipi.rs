use super::{hart_mask::HartMask, sbiret::SbiRet};
pub trait Ipi: Send {
   fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet; 
   fn max_hart_id(&self) -> usize;
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static!{
    static ref IPI: Mutex<Option<Box<dyn Ipi>>> = Mutex::new(None);
}

pub fn init_ipi<T>(ipi: T)
where T: Ipi + Send + 'static
{
    *IPI.lock() = Some(Box::new(ipi));
}

pub fn send_ipi_many(hart_mask: HartMask) -> SbiRet {
    if let Some(ipi) = IPI.lock().as_ref() {
        ipi.send_ipi_many(hart_mask)
    } else {
        SbiRet::not_supported()
    }
}
