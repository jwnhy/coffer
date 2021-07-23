use super::{hart_mask::HartMask, hart_scratch::get_hart_scratch, ipi_event::{create_ipi_event, get_ipi_evnet, IpiEvent, IpiEventOps}, rfence, sbiret::SbiRet};
use crate::util::fdt::XLEN;
use alloc::boxed::Box;
use spin::{Mutex, RwLock};

pub trait Ipi: Send {
    fn max_hartid(&self) -> usize;
    fn clear_soft_irq(&self, hartid: usize);
    fn send_soft_irq(&self, hartid: usize);
}

lazy_static::lazy_static! {
    pub(super) static ref IPI: Mutex<Option<Box<dyn Ipi>>> = Mutex::new(None);
    static ref IPI_SMODE_EVENT: IpiEvent = IpiEvent {
        name: "IPI_SMODE",
        ops: IpiEventOps {
            before: None,
            process: process_ipi_smode,
            after: None,
        },
    };
}

pub static IPI_SMODE_EVENT_ID: RwLock<usize> = RwLock::new(XLEN);

pub fn init_ipi<T>(ipi: T)
where
    T: Ipi + Send + 'static,
{
    *IPI.lock() = Some(Box::new(ipi));
    *IPI_SMODE_EVENT_ID.write() = create_ipi_event(&IPI_SMODE_EVENT);
}

fn process_ipi_smode() {
    unsafe { riscv::register::mip::set_ssoft() };
}

pub(crate) fn send_ipi_smode(hart_mask: HartMask) {
    let smode_event_id = *IPI_SMODE_EVENT_ID.read();
    send_ipi_many(hart_mask, smode_event_id);
}

pub(crate) fn send_ipi_many(hart_mask: HartMask, event_id: usize) -> SbiRet {
    if let Some(ipi) = IPI.lock().as_ref() {
        for i in 0..=ipi.max_hartid() {
            if hart_mask.has(i) {
                send_ipi(i, event_id);
            }
        }
        SbiRet::ok(0)
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
        let ipi_event = get_ipi_evnet(event_id);
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
            let event = get_ipi_evnet(event_id);
            (event.ops.process)();
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
