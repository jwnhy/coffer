use super::{hart_mask::HartMask, sbiret::SbiRet};

pub trait Rfence: Send {
    fn remote_fence_i(&mut self, hart_mask: HartMask) -> SbiRet;
    fn remote_sfence_vma(&mut self, hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet;
    fn remote_sfence_vma_asid(&mut self, hart_mask: HartMask, start_addr: usize, size: usize, asid: usize) -> SbiRet;
    fn remote_hfence_gvma_vmid(&mut self, hart_mask: HartMask, start_addr: usize, size: usize, vmid: usize) -> SbiRet;
    fn remote_hfence_gvma(&mut self, hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet;
    fn remote_hfence_vvma_asid(&mut self, hart_mask: HartMask, start_addr: usize, size: usize, asid: usize) -> SbiRet;
    fn remote_hfence_vvma(&mut self, hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet;
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref RFENCE: Mutex<Option<Box<dyn Rfence>>> = Mutex::new(None);
}

pub fn init_rfence<T>(rfence: T)
where
    T: Rfence + Send + 'static,
{
    *RFENCE.lock() = Some(Box::new(rfence));
}

pub(crate) fn remote_fence_i(hart_mask: HartMask) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_fence_i(hart_mask)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_sfence_vma(hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_sfence_vma(hart_mask, start_addr, size)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_sfence_vma_asid(hart_mask: HartMask, start_addr: usize, size: usize, asid: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_sfence_vma_asid(hart_mask, start_addr, size, asid)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_hfence_gvma_vmid(hart_mask: HartMask, start_addr: usize, size: usize, vmid: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_hfence_gvma_vmid(hart_mask, start_addr, size, vmid)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_hfence_gvma(hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_hfence_gvma(hart_mask, start_addr, size)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_hfence_vvma_asid(hart_mask: HartMask, start_addr: usize, size: usize, asid: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_hfence_vvma_asid(hart_mask, start_addr, size, asid)
    } else {
        SbiRet::not_supported()
    }
}

pub(crate) fn remote_hfence_vvma(hart_mask: HartMask, start_addr: usize, size: usize) -> SbiRet {
    if let Some(rfence) = RFENCE.lock().as_mut() {
        rfence.remote_hfence_gvma(hart_mask, start_addr, size)
    } else {
        SbiRet::not_supported()
    }
}

