use crate::runtime::context::Context;
use crate::sbi::sbiret::SbiRet;
use crate::sbi::*;

use self::base::handle_ecall_base;
use self::hsm::handle_ecall_hsm;
use self::ipi::{handle_ecall_ipi, FID_SEND_IPI};
use self::rfence::handle_ecall_rfence;
use self::srst::handle_ecall_srst;
use self::timer::{handle_ecall_timer, FID_SET_TIMER};

mod base;
mod hsm;
mod ipi;
mod rfence;
mod srst;
mod timer;

pub fn handle_ecall(ctx: *mut Context) -> SbiRet {
    let (ext, fid, p0, p1, p2, p3, p4) = unsafe {
        (
            (*ctx).a7,
            (*ctx).a6,
            (*ctx).a0,
            (*ctx).a1,
            (*ctx).a2,
            (*ctx).a3,
            (*ctx).a4,
        )
    };
    match ext {
        EXT_BASE => handle_ecall_base(fid, p0),
        EXT_TIME => handle_ecall_timer(fid, p0),
        EXT_IPI => handle_ecall_ipi(fid, p0, p1),
        EXT_RFENCE => handle_ecall_rfence(fid, p0, p1, p2, p3, p4),
        EXT_HSM => handle_ecall_hsm(fid, p0, p1, p2),
        EXT_SRST => handle_ecall_srst(fid, p0, p1),
        LEGACY_TIMER => handle_ecall_timer(FID_SET_TIMER, p0).legacy_void(p0, p1),
        LEGACY_GETCHAR => SbiRet {
            error: console_getchar() as usize,
            value: p1,
        },
        LEGACY_PUTCHAR => {
            console_putchar(p0 as u8);
            SbiRet {
                error: p0,
                value: p1,
            }
        }
        LEGACY_SEND_IPI => handle_ecall_ipi(FID_SEND_IPI, p0, 0x0),
        LEGACY_RFENCE_I => SbiRet {
            error: p0,
            value: p1,
        },
        LEGACY_SFENCE_VMA => SbiRet {
            error: p0,
            value: p1,
        },
        LEGACY_SFENCE_VMA_ASID => SbiRet {
            error: p0,
            value: p1,
        },
        LEGACY_SHUTDOWN => SbiRet {
            error: p0,
            value: p1,
        },
        _ => SbiRet::not_supported(),
    }
}
