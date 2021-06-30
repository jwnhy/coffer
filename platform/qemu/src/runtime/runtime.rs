use super::context::Context;
use super::super::memory::memory_layout::MemoryLayout;

const MSTACK_SIZE: usize = 8 * 1024;

pub struct Runtime {
    context: Context,
    mstack: [u8; MSTACK_SIZE],
    layout: MemoryLayout,
}
