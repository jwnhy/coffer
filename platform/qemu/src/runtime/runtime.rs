use core::ops::{Generator, GeneratorState};

use alloc::boxed::Box;
use riscv::register::mtvec;

use super::super::memory::memory_layout::MemoryLayout;
use super::context::{Context, from_machine, from_user_or_supervisor};

const MSTACK_SIZE: usize = 8 * 1024;

pub struct Runtime<Y> {
    /* registers */
    context: Context,
    /* pmp layout */
    layout: MemoryLayout,
    /* exception handler */
    exception_handler: Box<dyn FnMut(*mut Context) -> Option<Y>>,
}

impl<Y> Runtime<Y> {
    pub fn init(&self) {
        let mut addr = from_user_or_supervisor as usize;
        if addr & 0x2 != 0 {
            addr += 2;
        }
        unsafe {mtvec::write(addr, mtvec::TrapMode::Direct)}
    }

    pub fn new(
        context: Context,
        layout: MemoryLayout,
        exception_handler: Box<dyn FnMut(*mut Context) -> Option<Y>>,
    ) -> Self {
        Runtime {
            context,
            layout,
            exception_handler,
        }
    }
}

impl<Y> Generator for Runtime<Y> {
    /* when `exception_handler` cannot handle, yield */
    type Yield = Y;

    type Return = ();

    fn resume(
        mut self: core::pin::Pin<&mut Self>,
        arg: (),
    ) -> core::ops::GeneratorState<Self::Yield, Self::Return> {
        loop {
            let context_pointer = &mut self.context as *mut Context;
            unsafe { from_machine(context_pointer) };
            if let Some(yield_value) = (self.exception_handler)(context_pointer) {
                return GeneratorState::Yielded(yield_value);
            } else {
                continue;
            }
        }
    }
}
