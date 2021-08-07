use core::{intrinsics::size_of, ops::Add};

use alloc::vec;
use alloc::vec::Vec;

use crate::{println, util::addr::align};

use super::{
    cstr::CStr,
    endian::{types::u32_be, BigEndian},
    header::FdtHeader,
    prop::FdtProp,
    token::*,
};

#[repr(C)]
pub struct FdtNodeInner {
    magic: u32_be,
    name: CStr,
}

impl FdtNodeInner {
    pub fn name(&self) -> &CStr {
        &self.name
    }

    pub fn size(&self) -> usize {
        let unaligned = size_of::<u32_be>() + self.name.len() + 1;
        align(unaligned, 4)
    }
}

pub struct FdtNode<'a> {
    inner: &'a FdtNodeInner,
    props: Vec<FdtProp<'a>>,
    subnodes: Vec<FdtNode<'a>>,
    end: *const u32,
}

impl<'a, 'b> FdtNode<'a> {
    pub unsafe fn from_header(header: &'b FdtHeader) -> Result<FdtNode<'a>, &'static str> {
        let node_offset = (*header).struct_offset();
        let header_ptr = header.as_ptr();
        Self::from_ptr(header_ptr.add(node_offset), header)
    }

    pub unsafe fn from_ptr(
        node_ptr: *const u8,
        header: &'b FdtHeader,
    ) -> Result<FdtNode<'a>, &'static str> {
        let magic = u32_be::new(*(node_ptr as *const u32));
        if magic != FDT_BEGIN_NODE {
            return Err("[ERROR]: node does not start with FDT_BEGIN_NODE");
        }
        let inner_ptr = node_ptr as *const FdtNodeInner;
        let inner = &*inner_ptr;

        let mut cur = node_ptr.add(inner.size());
        let mut props = vec![];
        let mut subnodes = vec![];
        while *(cur as *const u32) != FDT_END_NODE {
            let token = u32_be::new(*(cur as *const u32));
            assert!(cur as usize % 4 == 0);
            match token {
                FDT_NOP => cur = cur.add(4),
                FDT_PROP => {
                    let prop = FdtProp::from_ptr(cur as *mut u8, header)?;
                    cur = cur.add(prop.size());
                    props.push(prop);
                }
                FDT_BEGIN_NODE => {
                    let node = Self::from_ptr(cur as *const u8, header)?;
                    cur = node.end().add(1) as *const u8;
                    subnodes.push(node);
                }
                FDT_END_NODE => break,
                FDT_NIL => cur = cur.add(4),
                _ => unreachable!(),
            }
        }
        props.shrink_to_fit();
        subnodes.shrink_to_fit();
        Ok(FdtNode {
            inner,
            props,
            subnodes,
            end: align(cur as usize, 4) as *const u32,
        })
    }

    pub fn end(&self) -> *const u32 {
        self.end
    }

    pub fn begin(&self) -> *const u32 {
        self.inner as *const FdtNodeInner as *const u32
    }
}
