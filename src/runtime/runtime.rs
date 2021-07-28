use core::ops::{Generator, GeneratorState};

use alloc::boxed::Box;
use riscv::register::mtvec::{self, Mtvec};

use crate::println;

use super::super::memory::memory_layout::MemoryLayout;
use super::context::{from_machine, from_user_or_supervisor, Context};

pub struct Runtime<Y> {
    /* registers */
    context: Context,
    /* pmp layout */
    layout: Option<MemoryLayout>,
    /* exception handler */
    exception_handler: Box<dyn FnMut(*mut Context) -> Option<Y>>,
    global_mtvec: Mtvec,
}

impl<Y> Runtime<Y> {
    pub fn new(
        context: Context,
        layout: Option<MemoryLayout>,
        exception_handler: Box<dyn FnMut(*mut Context) -> Option<Y>>,
    ) -> Self {
        Runtime {
            context,
            layout,
            exception_handler,
            global_mtvec: riscv::register::mtvec::read(),
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
        let context_pointer = &mut self.context as *mut Context;
        self.global_mtvec = riscv::register::mtvec::read();
        let addr = from_user_or_supervisor as usize;
        unsafe { mtvec::write(addr, mtvec::TrapMode::Direct) }
        loop {
            unsafe { from_machine(context_pointer) };
            if let Some(yield_value) = (self.exception_handler)(context_pointer) {
                unsafe {
                    mtvec::write(
                        self.global_mtvec.address(),
                        self.global_mtvec.trap_mode().unwrap(),
                    )
                }
                return GeneratorState::Yielded(yield_value);
            } else {
                continue;
            }
        }
    }
}
