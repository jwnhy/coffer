use embedded_hal::serial::{Read, Write};

use crate::util::reg::{read_reg, write_reg};
pub struct Ns16550a {
    base: usize,
    // TODO: make use of shift
    shift: usize,
}

mod offset {
    pub const RBR: usize = 0;
    pub const THR: usize = 0;

    pub const IER: usize = 1;
    pub const FCR: usize = 2;
    pub const LCR: usize = 3;
    pub const MCR: usize = 4;
    pub const LSR: usize = 5;

    pub const DLL: usize = 0;
    pub const DLH: usize = 1;
}

mod mask {
    pub const DR: u8 = 0x1;
    pub const PE: u8 = 0x1 << 1;
    pub const OE: u8 = 0x1 << 2;
    pub const FE: u8 = 0x1 << 3;
    pub const THRE: u8 = 0x1 << 5;
}

pub enum Error {
    Overrun(u8),
    Framing(u8),
    Parity(u8),
}

impl Ns16550a {
    pub fn new(base: usize, shift: usize, clk: u64, baud: u64) -> Self {
        unsafe {
            write_reg::<u8>(base, offset::LCR, 0x80);
            // baud = clk / (16 * latch)
            let latch = clk / (16 * baud);
            // latch = (DLH >> 8) + DLL
            write_reg::<u8>(base, offset::DLL, latch as u8);
            write_reg::<u8>(base, offset::DLH, (latch >> 8) as u8);
            // 8 bit transfer & !DLAB
            write_reg::<u8>(base, offset::LCR, 0x3);
            // no modem control
            write_reg::<u8>(base, offset::MCR, 0);
            // no interrupt
            write_reg::<u8>(base, offset::IER, 0);
            // FIFO
            write_reg::<u8>(base, offset::FCR, 0);
        }
        Self { base, shift }
    }
}

impl Ns16550a {
    fn ok(&self) -> Result<u8, Error> {
        let lsr = unsafe { read_reg::<u8>(self.base, offset::LSR) };
        if lsr & (mask::OE | mask::PE | mask::FE) == 0 {
            return Ok(lsr);
        }
        let overrun_bit = lsr & mask::OE;
        if overrun_bit != 0 {
            return Err(Error::Overrun(lsr));
        }
        let parity_bit = lsr & mask::PE;
        if parity_bit != 0 {
            return Err(Error::Parity(lsr));
        }
        let frame_bit = lsr & mask::FE;
        if frame_bit != 0 {
            return Err(Error::Framing(lsr));
        }
        unreachable!()
    }
}

impl Read<u8> for Ns16550a {
    type Error = Error;
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        let lsr = self.ok()?;
        let data_ready = lsr & mask::DR;
        if data_ready != 0 {
            let word = unsafe { read_reg::<u8>(self.base, offset::RBR) };
            Ok(word)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for Ns16550a {
    type Error = Error;
    fn try_write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let write_ready = unsafe { read_reg::<u8>(self.base, offset::LSR) & mask::THRE };
        if write_ready != 0 {
            unsafe { write_reg::<u8>(self.base, offset::THR, word) };
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        let write_ready = unsafe { read_reg::<u8>(self.base, offset::LSR) } & mask::THRE;
        if write_ready != 0 {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
