pub mod console;
pub mod ipi;
pub mod ipi_event;
pub mod sbiret;
pub mod rfence;
pub mod timer;
pub mod hsm;
pub mod srst;

pub mod hart_mask;
pub mod hart_scratch;

pub use console::*;
pub const SBI_SPEC_MAJOR: usize = 0;
pub const SBI_SPEC_MINOR: usize = 2;
pub const COFFER_IMPL_ID: usize = 6;
pub const COFFER_VERSION: usize = 0;

pub const EXT_BASE: usize = 0x10;
pub const EXT_TIME: usize = 0x5449_4D45;
pub const EXT_IPI: usize = 0x73_5049;
pub const EXT_RFENCE: usize = 0x5246_4E43;
pub const EXT_HSM: usize = 0x48_534D;
pub const EXT_SRST: usize = 0x5352_5354;
pub const EXT_PMU: usize = 0x50_4D55;

pub const LEGACY_TIMER: usize = 0x0;
pub const LEGACY_PUTCHAR: usize = 0x1;
pub const LEGACY_GETCHAR: usize = 0x2;
pub const LEGACY_CLEAR_IPI: usize = 0x3;
pub const LEGACY_SEND_IPI: usize = 0x4;
pub const LEGACY_RFENCE_I: usize = 0x5;
pub const LEGACY_SFENCE_VMA: usize = 0x6;
pub const LEGACY_SFENCE_VMA_ASID: usize = 0x7;
pub const LEGACY_SHUTDOWN: usize = 0x8;
