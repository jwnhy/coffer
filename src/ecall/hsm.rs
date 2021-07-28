use crate::sbi::{
    hsm::{hart_get_status, hart_start, hart_stop, hart_suspend},
    sbiret::SbiRet,
};

const FID_HART_START: usize = 0x0;
const FID_HART_STOP: usize = 0x1;
const FID_HART_GET_STATUS: usize = 0x2;
const FID_HART_SUSPEND: usize = 0x3;

#[inline]
pub fn handle_ecall_hsm(fid: usize, param0: usize, param1: usize, param2: usize) -> SbiRet {
    match fid {
        FID_HART_START => hart_start(param0, param1, param2),
        FID_HART_STOP => hart_stop(),
        FID_HART_GET_STATUS => hart_get_status(param0),
        FID_HART_SUSPEND => hart_suspend(param0 as u32, param1, param2),
        _ => SbiRet::not_supported(),
    }
}
