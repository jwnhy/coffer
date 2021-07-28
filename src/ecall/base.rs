use riscv::register::{marchid, mimpid, mvendorid};

use crate::sbi::hsm::probe_hsm;
use crate::sbi::ipi::probe_ipi;
use crate::sbi::rfence::probe_rfence;
use crate::sbi::srst::probe_srst;
use crate::sbi::{
    sbiret::SbiRet, timer::probe_timer, COFFER_IMPL_ID, COFFER_VERSION, SBI_SPEC_MAJOR,
    SBI_SPEC_MINOR,
};

const FID_BASE_GET_SPEC_VERSION: usize = 0x0;
const FID_BASE_GET_SBI_IMPL_ID: usize = 0x1;
const FID_BASE_GET_SBI_IMPL_VERSION: usize = 0x2;
const FID_BASE_PROBE_EXTENSION: usize = 0x3;
const FID_BASE_GET_MVENDORID: usize = 0x4;
const FID_BASE_GET_MARCHID: usize = 0x5;
const FID_BASE_GET_MIMPID: usize = 0x6;

#[inline]
pub fn handle_ecall_base(fid: usize, param0: usize) -> SbiRet {
    match fid {
        FID_BASE_GET_SPEC_VERSION => get_spec_version(),
        FID_BASE_GET_SBI_IMPL_ID => get_sbi_impl_id(),
        FID_BASE_GET_SBI_IMPL_VERSION => get_sbi_impl_version(),
        FID_BASE_PROBE_EXTENSION => probe_extension(param0),
        FID_BASE_GET_MVENDORID => get_mvendorid(),
        FID_BASE_GET_MARCHID => get_marchid(),
        FID_BASE_GET_MIMPID => get_mimpid(),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn get_spec_version() -> SbiRet {
    SbiRet::ok((SBI_SPEC_MAJOR << 24) | SBI_SPEC_MINOR)
}

#[inline]
fn get_sbi_impl_id() -> SbiRet {
    SbiRet::ok(COFFER_IMPL_ID)
}

#[inline]
fn get_sbi_impl_version() -> SbiRet {
    SbiRet::ok(COFFER_VERSION)
}

#[inline]
fn probe_extension(ext_id: usize) -> SbiRet {
    match ext_id {
        EXT_BASE => SbiRet::ok(1),
        EXT_TIMER => probe_timer(),
        EXT_IPI => probe_ipi(),
        EXT_HSM => probe_hsm(),
        EXT_SRST => probe_srst(),
        EXT_RFENCE => probe_rfence(),
        _ => SbiRet::ok(0),
    }
}

#[inline]
fn get_mvendorid() -> SbiRet {
    SbiRet::ok(mvendorid::read().map(|x| x.bits()).unwrap_or(0))
}

#[inline]
fn get_marchid() -> SbiRet {
    SbiRet::ok(marchid::read().map(|x| x.bits()).unwrap_or(0))
}

#[inline]
fn get_mimpid() -> SbiRet {
    SbiRet::ok(mimpid::read().map(|x| x.bits()).unwrap_or(0))
}
