use core::borrow::Borrow;

use alloc::{boxed::Box, vec::Vec};
use spin::RwLock;

use crate::util::fdt::XLEN;

use super::hart_scratch::IpiScratch;
pub struct IpiEventOps {
    pub before: Option<fn(usize, &mut IpiScratch)>,
    pub after: Option<fn()>,
    pub process: fn(),
}

pub struct IpiEvent {
    pub name: &'static str,
    pub ops: IpiEventOps,
}

lazy_static::lazy_static! {
    static ref IPI_EVENTS: RwLock<[Option<&'static IpiEvent>; XLEN]> = RwLock::new([None; XLEN]);
}

pub fn create_ipi_event(new_event: &'static IpiEvent) -> usize {
    let mut events = IPI_EVENTS.write();
    match events.iter_mut().enumerate().find(|(_, x)| x.is_none()) {
        Some((idx, slot)) => {
            *slot = Some(new_event);
            idx
        }
        None => panic!("[ERROR]: ipi event creation failed, not enough slot, max XLEN"),
    }
}

pub fn destroy_ipi_event(event_id: usize) {
    let mut events = IPI_EVENTS.write();
    events[event_id] = None;
}

pub fn get_ipi_evnet(event_id: usize) -> &'static IpiEvent {
    let events = *IPI_EVENTS.read();
    events[event_id].expect("[ERROR]: no such ipi event")
}
