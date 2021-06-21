use crate::sbi::{ipi::send_ipi_many, sbiret::SbiRet};

const FID_SEND_IPI: usize = 0x0;

#[inline]
pub fn handle_ecall_ipi(fid: usize, param0: usize, param1: usize) -> SbiRet {
    match fid {
        FID_SEND_IPI => send_ipi(param0, param1),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn send_ipi(hart_mask_arr: usize, hart_mask_base: usize) -> SbiRet {
   unimplemented!() 
}
