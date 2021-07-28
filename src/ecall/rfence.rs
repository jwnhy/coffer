use riscv::register::mstatus;

use crate::sbi::{
    hart_mask::HartMask,
    rfence::{remote_fence_i, remote_sfence_vma},
    sbiret::SbiRet,
};

const FID_RFENCE_I: usize = 0x0;
const FID_SFENCE_VMA: usize = 0x1;
const FID_SFENCE_VMA_ASID: usize = 0x2;
const FID_HFENCE_GVMA_VMID: usize = 0x3;
const FID_HFENCE_GVMA: usize = 0x4;
const FID_HFENCE_VVMA_ASID: usize = 0x5;
const FID_HFENCE_VVMA: usize = 0x6;

#[inline]
pub fn handle_ecall_rfence(
    fid: usize,
    param0: usize,
    param1: usize,
    param2: usize,
    param3: usize,
    param4: usize,
) -> SbiRet {
    let hart_mask = unsafe { HartMask::new(param0, param1) };
    match fid {
        FID_RFENCE_I => remote_fence_i(hart_mask),
        FID_SFENCE_VMA => remote_sfence_vma(hart_mask, param2, param3),
        FID_SFENCE_VMA_ASID => SbiRet::not_supported(),
        FID_HFENCE_GVMA_VMID => SbiRet::not_supported(),
        FID_HFENCE_GVMA => SbiRet::not_supported(),
        FID_HFENCE_VVMA_ASID => SbiRet::not_supported(),
        FID_HFENCE_VVMA => SbiRet::not_supported(),
        _ => SbiRet::not_supported(),
    }
}
