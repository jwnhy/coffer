use core::convert::Infallible;

use embedded_hal::serial::{Read, Write};
use crate::util::reg::{read_reg, write_reg};

pub struct SunxiUart {
    base: usize,
}

mod offset {
    pub const RBR: usize = 0;
    pub const THR: usize = 0;
    pub const USR: usize = 31;
}

mod mask {
    pub const TXNF: u32 = 1 << 1;
    pub const RXNE: u32 = 1 << 2;
}

impl SunxiUart {
    pub fn new(base: usize) -> Self {
        Self { base }
    }
}

impl Read<u8> for SunxiUart {
    type Error = Infallible;
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {

        let read_ready = unsafe {read_reg::<u32>(self.base, offset::USR * 0x04) & mask::RXNE};
        if read_ready != 0 {
            let word = unsafe { read_reg::<u32>(self.base, offset::RBR) };
            Ok(word as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for SunxiUart {
    type Error = Infallible;
    fn try_write(&mut self, word: u8) -> nb::Result<(), Self::Error> {

        let write_ready = unsafe {read_reg::<u32>(self.base, offset::USR * 0x04) & mask::TXNF};
        if write_ready != 0 {
            unsafe { write_reg::<u32>(self.base, offset::THR, word as u32) };
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        let write_ready = unsafe {read_reg::<u32>(self.base, offset::USR * 0x04) & mask::TXNF};
        if write_ready != 0 {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
