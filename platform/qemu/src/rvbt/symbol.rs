use core::fmt;

use super::{frame::Frame, init::DEBUG_CTX};
use crate::println;
use alloc::string::{String, ToString};
use if_chain::if_chain;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub file: String,
    pub line: u32,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@L{}:{}", self.name, self.line, self.file)
    }
}

#[inline(always)]
pub fn resolve_frame(frame: &Frame, action: &dyn Fn(&Symbol)) {
    resolve(frame.ra, action)
}

#[inline(always)]
pub fn resolve(addr: u64, action: &dyn Fn(&Symbol)) {
    if_chain! {
        if let Some(ctx) = DEBUG_CTX.lock().as_ref();
        if let Ok(mut frame_iter) = ctx.find_frames(addr);
        then {
            while let Ok(Some(frame)) = frame_iter.next() {
                let name = match frame.function {
                    Some(func) => {
                        func.demangle().ok().map_or("".to_string(), |s| s.to_string())
                    },
                    None => "".to_string()
                };
                let (file, line) = match frame.location {
                    Some(loc) => {
                        (loc.file.unwrap_or("??"), loc.line.unwrap_or(0))
                    },
                    None => ("??", 0)
                };
                action(&Symbol{name, file: file.to_string(), line})
            }
        } else {
            println!("[ERROR] debug context not initialized or frame not found");
        }
    }
}
