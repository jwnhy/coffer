use crate::sbi::{sbiret::SbiRet, timer::set_timer};

pub const FID_SET_TIMER: usize = 0x0;

#[inline]
pub fn handle_ecall_timer(fid: usize, param0: usize) -> SbiRet {
    match fid {
        FID_SET_TIMER => set_timer(param0 as u64),
        _ => SbiRet::not_supported(),
    }
}
