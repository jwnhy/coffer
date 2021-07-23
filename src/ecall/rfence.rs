use riscv::register::mstatus;

use crate::sbi::{hart_mask::HartMask, sbiret::SbiRet};


const FID_RFENCE_I: usize = 0x0;
const FID_SFENCE_VMA: usize = 0x1;
const FID_SFENCE_VMA_ASID: usize = 0x2;
const FID_HFENCE_GVMA_VMID: usize = 0x3;
const FID_HFENCE_GVMA: usize = 0x4;
const FID_HFENCE_VVMA_ASID: usize = 0x5;
const FID_HFENCE_VVMA: usize = 0x6;


#[inline]
pub fn handle_ecall_rfence(fid: usize, param0: usize, param1: usize, param2: usize, param3: usize, param4: usize) -> SbiRet{
    let mpp = mstatus::read().mpp();
    let hart_mask = unsafe { HartMask::new(param0, param1, mpp) };
    unimplemented!()
    /*match fid {
        FID_RFENCE_I => remote_fence_i(hart_mask),
        FID_SFENCE_VMA => remote_sfence_vma(hart_mask, param2, param3),
        FID_SFENCE_VMA_ASID => remote_sfence_vma_asid(hart_mask, param2, param3, param4),
        FID_HFENCE_GVMA_VMID => remote_hfence_gvma_vmid(hart_mask, param2, param3, param4),
        FID_HFENCE_GVMA => remote_hfence_gvma(hart_mask, param2, param3),
        FID_HFENCE_VVMA_ASID => remote_hfence_vvma_asid(hart_mask, param2, param3, param4),
        FID_HFENCE_VVMA => remote_hfence_vvma(hart_mask, param2, param3),
        _ => SbiRet::not_supported(),
    } */
}
