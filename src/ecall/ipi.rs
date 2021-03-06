use riscv::register::mstatus::{self, MPP};

use crate::sbi::{
    hart_mask::HartMask,
    ipi::{self, send_ipi_many, IPI_SMODE_EVENT_ID},
    sbiret::SbiRet,
};

pub const FID_SEND_IPI: usize = 0x0;

#[inline]
pub fn handle_ecall_ipi(fid: usize, param0: usize, param1: usize) -> SbiRet {
    match fid {
        FID_SEND_IPI => send_ipi(param0, param1),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn send_ipi(param0: usize, param1: usize) -> SbiRet {
    let hart_mask = unsafe { HartMask::new(param0, param1) };
    send_ipi_many(hart_mask, *IPI_SMODE_EVENT_ID.read())
}
