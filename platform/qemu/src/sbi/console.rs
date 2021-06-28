use embedded_hal::serial::{Read, Write};
use nb::block;
pub trait Console: Send {
    fn getchar(&mut self) -> u8;
    fn putchar(&mut self, ch: u8);
    // TODO: Error handling
}

struct EmbeddedSerial<T> {
    inner: T,
}

impl<T> EmbeddedSerial<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Console for EmbeddedSerial<T>
where
    T: Read<u8> + Write<u8> + Send,
{
    fn getchar(&mut self) -> u8 {
        block!(self.inner.try_read()).ok().unwrap()
    }
    fn putchar(&mut self, ch: u8) {
        block!(self.inner.try_write(ch)).ok();
        block!(self.inner.try_flush()).ok();
        // TODO: Add Buffer.
    }
}

struct FusedSerial<T, R>(T, R);

impl<T, R> Console for FusedSerial<T, R>
where
    T: Write<u8> + Send + 'static,
    R: Read<u8> + Send + 'static,
{
    fn getchar(&mut self) -> u8 {
        block!(self.1.try_read()).ok().unwrap()
    }
    fn putchar(&mut self, ch: u8) {
        block!(self.0.try_write(ch)).ok();
        block!(self.0.try_flush()).ok();
        // TODO: Add Buffer.
    }
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref CONSOLE: Mutex<Option<Box<dyn Console>>> = Mutex::new(None);
}

pub fn init_console_embedded_serial<T>(serial: T)
where
    T: Read<u8> + Write<u8> + Send + 'static,
{
    lazy_static::initialize(&CONSOLE);
    let serial = EmbeddedSerial::new(serial);
    *CONSOLE.lock() = Some(Box::new(serial));
}

pub fn init_console_fused_serial<T, R>(tx: T, rx: R)
where
    T: Write<u8> + Send + 'static,
    R: Read<u8> + Send + 'static,
{
    let serial = FusedSerial(tx, rx);
    *CONSOLE.lock() = Some(Box::new(serial));
}

pub fn console_putchar(ch: u8) {
    if let Some(serial) = CONSOLE.lock().as_mut() {
        serial.putchar(ch)
    }
}

pub fn console_getchar() -> u8 {
    if let Some(serial) = CONSOLE.lock().as_mut() {
        serial.getchar()
    } else {
        0
    }
}

use core::fmt;
struct Stdout;
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(serial) = CONSOLE.lock().as_mut() {
            for byte in s.as_bytes() {
                serial.putchar(*byte)
            }
        }
        Ok(())
    }
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Stdout.write_fmt(args).unwrap();
}

#[macro_export(local_inner_macro)]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::sbi::console::_print(core::format_args!($($arg)*));
    });
}

#[macro_export(local_inner_macro)]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::console::_print(core::format_args!(core::concat!($fmt, "\r\n") $(, $($arg)+)?));
    }
}
