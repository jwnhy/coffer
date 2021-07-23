use crate::sbi::fence_info::FenceInfo;
use crate::sbi::rfence::LocalFence;
pub struct Tlb;

#[naked]
#[no_mangle]
extern "C" fn __tlb_flush_all() {
   unsafe { asm!("sfence.vma", options(noreturn)) }
}

impl LocalFence for Tlb {
    fn local_sfence(&self, finfo: FenceInfo) {
        if finfo.is_flush_all() {
            __tlb_flush_all();
        }
        let start = finfo.start.unwrap();
        let size = finfo.size.unwrap();
        if finfo.asid.is_some() {
            for addr in start..start+size {
                unsafe {asm!("sfence.vma {0}, {1}", in(reg) addr, in(reg) finfo.asid.unwrap())}
            }
        }
    }

    fn local_hfence(&self, finfo: FenceInfo) {
        todo!()
    }
}
