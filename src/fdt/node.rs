use super::{
    cstr::CStr,
    header::FdtHeader,
    prop::{FdtProp, FdtPropIter},
    token::*,
};
use crate::{fdt::token::align, print, println};
use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
use core::{intrinsics::size_of, ops::Add};
use endiantype::*;

#[repr(C)]
pub struct FdtNode {
    magic: u32_be,
    name: CStr,
}

impl FdtNode {
    pub fn name(&self) -> &CStr {
        &self.name
    }

    pub fn size(&self) -> usize {
        let unaligned = size_of::<u32_be>() + self.name.len() + 1;
        align(unaligned, 4)
    }
}

impl FdtNode {
    pub unsafe fn from_ptr(node_ptr: *const u8) -> Result<&'static FdtNode, &'static str> {
        let node_ptr = skip_nop(node_ptr);
        let magic = u32_be::new(*(node_ptr as *const u32));
        if magic != FDT_BEGIN_NODE {
            return Err("[ERROR]: node does not start with FDT_BEGIN_NODE");
        }
        let node_ptr = node_ptr as *const FdtNode;
        Ok(&*node_ptr)
    }

    pub fn prop_iter(&self) -> impl Iterator<Item = &'static FdtProp> {
        unsafe { FdtPropIter::from_ptr((self as *const FdtNode as *const u8).add(self.size())) }
    }

    pub fn prop_size(&self) -> usize {
        let mut prop_size = 0;
        for prop in self.prop_iter() {
            prop_size += prop.size();
        }
        prop_size
    }
}

pub struct FdtNodeIter {
    raw_ptr: *const u8,
    path: Vec<&'static CStr>,
}

impl Iterator for FdtNodeIter {
    type Item = &'static FdtNode;

    fn next(&mut self) -> Option<Self::Item> {
        let result = unsafe { FdtNode::from_ptr(self.raw_ptr) };
        if let Ok(node) = result {
            self.path.push(node.name());
            self.raw_ptr = unsafe { self.raw_ptr.add(node.size()).add(node.prop_size()) };
            self.raw_ptr = skip_nop(self.raw_ptr);
            unsafe {
                while *(self.raw_ptr as *const u32_be) == FDT_END_NODE {
                    self.raw_ptr = self.raw_ptr.add(4);
                }
            }
            Some(node)
        } else {
            None
        }
    }
}

impl FdtNodeIter {
    pub unsafe fn from_ptr(raw_ptr: *const u8) -> Self {
        FdtNodeIter {
            raw_ptr,
            path: vec![],
        }
    }
}
