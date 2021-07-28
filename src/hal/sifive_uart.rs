use core::convert::Infallible;

use crate::util::reg::{read_reg, write_reg};
use embedded_hal::serial::{Read, Write};

pub struct SifiveUart {
    base: usize,
}

mod offset {
    pub const TXDATA: usize = 0;
    pub const RXDATA: usize = 1;
    pub const TXCTRL: usize = 2;
    pub const RXCTRL: usize = 3;
    pub const IE: usize = 4;
    pub const IP: usize = 5;
    pub const DIV: usize = 6;
}

mod mask {
    pub const RXEMPTY: u32 = 0x8000_0000;
    pub const TXFULL: u32 = 0x8000_0000;
    pub const TXEN: u32 = 0x1;
    pub const RXEN: u32 = 0x1;
}

fn _min_clk_div(clk: u64, target_hz: u64) -> u64 {
    let quotient = (clk + target_hz - 1) / target_hz;
    if quotient == 0 {
        return 0;
    } else {
        return quotient - 1;
    }
}

impl SifiveUart {
    pub fn new(base: usize, clk: u64, baud: u64) -> Self {
        unsafe {
            if clk > 0 {
                let div = _min_clk_div(clk, baud);
                write_reg::<u32>(base, offset::DIV * 0x4, div as u32);
            }
            write_reg::<u32>(base, offset::IE * 0x4, 0);
            write_reg(base, offset::TXCTRL * 0x4, mask::TXEN);
            write_reg(base, offset::RXCTRL * 0x4, mask::RXEN)
        }
        Self { base }
    }
}

impl Read<u8> for SifiveUart {
    type Error = Infallible;
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        let word = unsafe { read_reg::<u32>(self.base, offset::RXDATA * 0x4) };
        if word & mask::RXEMPTY == 0 {
            Ok(word as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for SifiveUart {
    type Error = Infallible;
    fn try_write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let full = unsafe { read_reg::<u32>(self.base, offset::TXDATA * 0x4) & mask::TXFULL };
        if full == 0 {
            unsafe { write_reg::<u32>(self.base, offset::TXDATA * 0x4, word as u32) };
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
