use crate::util::fdt::XLEN;

use super::{
    hart_mask::HartMask, hart_scratch::get_hart_scratch, ipi_event::get_ipi_evnet, sbiret::SbiRet,
};

pub trait Ipi: Send {
    fn send_ipi_many(&self, hart_mask: HartMask) -> SbiRet;
    fn max_hartid(&self) -> usize;
    fn clear_soft_irq(&self, hartid: usize);
    fn send_soft_irq(&self, hartid: usize);
}

use alloc::boxed::Box;
use bit_field::BitField;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref IPI: Mutex<Option<Box<dyn Ipi>>> = Mutex::new(None);
}

pub fn init_ipi<T>(ipi: T)
where
    T: Ipi + Send + 'static,
{
    *IPI.lock() = Some(Box::new(ipi));
}

pub(crate) fn send_ipi_many(hart_mask: HartMask) -> SbiRet {
    if let Some(ipi) = IPI.lock().as_ref() {
        ipi.send_ipi_many(hart_mask)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn clear_ipi(hartid: usize) {
    if let Some(ipi) = IPI.lock().as_ref() {
        ipi.clear_soft_irq(hartid);
    }
}

pub(crate) fn send_ipi(hartid: usize, event_id: usize) {
    if let Some(ipi) = IPI.lock().as_ref() {
        let mut remote_scratch = get_hart_scratch(hartid).lock();
        let ipi_event = get_ipi_evnet(event_id).expect("[ERROR]: no such event");
        if let Some(before) = ipi_event.ops.before {
            (before)(hartid, &mut remote_scratch.ipi_scratch)
        }
        remote_scratch.ipi_scratch.trigger(event_id);
        ipi.send_soft_irq(hartid);
        if let Some(after) = ipi_event.ops.after {
            (after)()
        }
    }
}

pub(crate) fn process_ipi() {
    let hartid = riscv::register::mhartid::read();
    let mut scratch = get_hart_scratch(hartid).lock();
    for event_id in 0..XLEN {
        if scratch.ipi_scratch.is_triggered(event_id) {
            if let Some(event) = get_ipi_evnet(event_id) {
                (event.ops.process)();
            } else {
                panic!("[ERROR]: no such event")
            }
        }
    }
    clear_ipi(hartid);
    scratch.ipi_scratch.clear_triggered();
}

pub(crate) fn probe_ipi() -> SbiRet {
    if let Some(_) = IPI.lock().as_ref() {
        SbiRet::ok(1)
    } else {
        SbiRet::ok(0)
    }
}
