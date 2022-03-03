use crate::{sbi::ipi_event::IpiEventOps, util::fdt::XLEN};

use core::arch::asm;
use super::hart_mask::HartMask;
use super::ipi::send_ipi_many;
use super::ipi_event::IpiEvent;
use super::sbiret::SbiRet;
use super::{fence_info::FenceInfo, ipi_event::create_ipi_event};

pub trait LocalFence: Send {
    fn local_sfence(&self, finfo: FenceInfo);
    fn local_hfence(&self, finfo: FenceInfo);
}

use alloc::boxed::Box;
use spin::{Mutex, RwLock};

lazy_static::lazy_static! {
    static ref LOCAL_FENCE: Mutex<Option<Box<dyn LocalFence>>> = Mutex::new(None);
    static ref IPI_RFENCE_I_EVENT: IpiEvent = IpiEvent {
        name: "IPI_RFENCE_I",
        ops: IpiEventOps {
                before: None,
                process: process_rfence_i,
                after: None,
            },
    };
    static ref IPI_SFENCE_VMA_EVENT: IpiEvent = IpiEvent {
        name: "IPI_SFENCE_VMA",
        ops: IpiEventOps {
            before: None,
            process: process_sfence_vma,
            after: None,
        }
    };
}

pub static IPI_RFENCE_I_EVENT_ID: RwLock<usize> = RwLock::new(XLEN);
pub static IPI_SFENCE_VMA_EVENT_ID: RwLock<usize> = RwLock::new(XLEN);

pub fn process_sfence_vma() {
    unsafe { asm!("sfence.vma") };
}

pub(crate) fn remote_sfence_vma(hart_mask: HartMask, start: usize, size: usize) -> SbiRet {
    let event_id = *IPI_SFENCE_VMA_EVENT_ID.read();
    send_ipi_many(hart_mask, event_id)
}

pub fn process_rfence_i() {
    unsafe { asm!("fence.i") };
}

pub(crate) fn remote_fence_i(hart_mask: HartMask) -> SbiRet {
    let event_id = *IPI_RFENCE_I_EVENT_ID.read();
    send_ipi_many(hart_mask, event_id)
}

pub fn init_rfence<T>(rfence: T)
where
    T: LocalFence + Send + 'static,
{
    *LOCAL_FENCE.lock() = Some(Box::new(rfence));
    *IPI_RFENCE_I_EVENT_ID.write() = create_ipi_event(&IPI_RFENCE_I_EVENT);
    *IPI_SFENCE_VMA_EVENT_ID.write() = create_ipi_event(&IPI_SFENCE_VMA_EVENT);
}

pub(crate) fn probe_rfence() -> SbiRet {
    use super::ipi::IPI;
    if IPI.lock().as_ref().is_some() && LOCAL_FENCE.lock().as_ref().is_some() {
        SbiRet::ok(1)
    } else {
        SbiRet::ok(0)
    }
}
