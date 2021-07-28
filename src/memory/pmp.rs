use bitflags::*;
use riscv::register::*;

bitflags! {
    #[repr(C)]
    pub struct PmpFlags: u8 {
        const READABLE =    1 << 0;
        const WRITABLE =    1 << 1;
        const EXECUTABLE =  1 << 2;
        const MODE_TOR =    1 << 3;
        const MODE_NA4 =    2 << 3;
        const MODE_NAPOT =  3 << 3;
        const LOCKED =      1 << 7;
    }
}

#[cfg(target_arch = "riscv64")]
pub(crate) fn pmpcfg_read(index: usize) -> u8 {
    match index {
        0..=7 => (pmpcfg0::read() >> (8 * index)) as u8,
        8..=15 => (pmpcfg2::read() >> (8 * (index - 8))) as u8,
        _ => panic!("pmp does not exist"),
    }
}

#[cfg(target_arch = "riscv64")]
pub(crate) fn pmpcfg_write(index: usize, value: u8) {
    use bit_field::BitField;
    match index {
        0..=7 => {
            let range = index * 8..(index + 1) * 8;
            let mut reg_value = pmpcfg0::read();
            reg_value.set_bits(range, value as usize);
            pmpcfg0::write(reg_value);
        }
        8..=15 => {
            let range = (index - 8) * 8..(index - 7) * 8;
            let mut reg_value = pmpcfg0::read();
            reg_value.set_bits(range, value as usize);
            pmpcfg2::write(reg_value);
        }
        _ => panic!("pmp does not exist"),
    }
}

#[cfg(target_arch = "riscv64")]
pub(crate) fn pmpaddr_read(index: usize) -> usize {
    match index {
        0 => pmpaddr0::read(),
        1 => pmpaddr1::read(),
        2 => pmpaddr2::read(),
        3 => pmpaddr3::read(),
        4 => pmpaddr4::read(),
        5 => pmpaddr5::read(),
        6 => pmpaddr6::read(),
        7 => pmpaddr7::read(),
        8 => pmpaddr8::read(),
        9 => pmpaddr9::read(),
        10 => pmpaddr10::read(),
        11 => pmpaddr11::read(),
        12 => pmpaddr12::read(),
        13 => pmpaddr13::read(),
        14 => pmpaddr14::read(),
        15 => pmpaddr15::read(),
        _ => panic!("pmp does not exist"),
    }
}

#[cfg(target_arch = "riscv64")]
pub(crate) fn pmpaddr_write(index: usize, value: usize) {
    match index {
        0 => pmpaddr0::write(value),
        1 => pmpaddr1::write(value),
        2 => pmpaddr2::write(value),
        3 => pmpaddr3::write(value),
        4 => pmpaddr4::write(value),
        5 => pmpaddr5::write(value),
        6 => pmpaddr6::write(value),
        7 => pmpaddr7::write(value),
        8 => pmpaddr8::write(value),
        9 => pmpaddr9::write(value),
        10 => pmpaddr10::write(value),
        11 => pmpaddr11::write(value),
        12 => pmpaddr12::write(value),
        13 => pmpaddr13::write(value),
        14 => pmpaddr14::write(value),
        15 => pmpaddr15::write(value),
        _ => panic!("pmp does not exist"),
    }
}
