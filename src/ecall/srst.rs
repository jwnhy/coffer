use core::intrinsics::transmute;

use crate::sbi::{sbiret::SbiRet, srst::{ResetType, system_reset}};


const FID_SYSTEM_RESET: usize = 0x0;
pub fn handle_ecall_srst(fid: usize, param0: usize, param1: usize) -> SbiRet {
    match fid {
        FID_SYSTEM_RESET => system_reset(unsafe {transmute(param0 as u32)}, unsafe {transmute(param1 as u32)}),
        _ => SbiRet::not_supported(),
    }
}
