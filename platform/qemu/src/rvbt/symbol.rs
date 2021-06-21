use core::slice;

use addr2line::{Context, Location};
use alloc::{boxed::Box, string::String};
use gimli::{
    DebugAbbrev, DebugAddr, DebugAranges, DebugInfo, DebugLine, DebugLineStr, DebugRanges,
    DebugRngLists, DebugStr, DebugStrOffsets, EndianSlice, LittleEndian,
};
use if_chain::if_chain;
use spin::Mutex;

use crate::{println, sbi::init_console_fused_serial};

extern "C" {
    fn _rvbt_abbrev_start();
    fn _rvbt_abbrev_end();
    fn _rvbt_addr_start();
    fn _rvbt_addr_end();
    fn _rvbt_aranges_start();
    fn _rvbt_aranges_end();
    fn _rvbt_info_start();
    fn _rvbt_info_end();
    fn _rvbt_line_start();
    fn _rvbt_line_end();
    fn _rvbt_line_str_start();
    fn _rvbt_line_str_end();
    fn _rvbt_ranges_start();
    fn _rvbt_ranges_end();
    fn _rvbt_rnglists_start();
    fn _rvbt_rnglists_end();
    fn _rvbt_str_start();
    fn _rvbt_str_end();
    fn _rvbt_str_offsets_start();
    fn _rvbt_str_offsets_end();
}

fn _abbrev_section() -> DebugAbbrev<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_abbrev_start as usize;
    let end = _rvbt_abbrev_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugAbbrev::new(bytes, LittleEndian)
}

fn _addr_section() -> DebugAddr<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_addr_start as usize;
    let end = _rvbt_addr_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugAddr::from(EndianSlice::new(bytes, LittleEndian))
}
fn _aranges_section() -> DebugAranges<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_aranges_start as usize;
    let end = _rvbt_aranges_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugAranges::new(bytes, LittleEndian)
}
fn _info_section() -> DebugInfo<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_info_start as usize;
    let end = _rvbt_info_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugInfo::new(bytes, LittleEndian)
}
fn _line_section() -> DebugLine<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_line_start as usize;
    let end = _rvbt_line_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugLine::new(bytes, LittleEndian)
}
fn _line_str_section() -> DebugLineStr<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_line_str_start as usize;
    let end = _rvbt_line_str_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugLineStr::from(EndianSlice::new(bytes, LittleEndian))
}
fn _ranges_section() -> DebugRanges<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_ranges_start as usize;
    let end = _rvbt_ranges_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugRanges::new(bytes, LittleEndian)
}
fn _rnglists_section() -> DebugRngLists<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_rnglists_start as usize;
    let end = _rvbt_rnglists_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugRngLists::new(bytes, LittleEndian)
}
fn _str_section() -> DebugStr<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_str_start as usize;
    let end = _rvbt_str_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugStr::new(bytes, LittleEndian)
}
fn _str_offsets_section() -> DebugStrOffsets<EndianSlice<'static, LittleEndian>> {
    let start = _rvbt_str_offsets_start as usize;
    let end = _rvbt_str_offsets_end as usize;
    let bytes = unsafe { slice::from_raw_parts(start as *const u8, end - start) };
    DebugStrOffsets::from(EndianSlice::new(bytes, LittleEndian))
}

lazy_static::lazy_static!{
    static ref DEBUG_CTX: Mutex<Option<Box<Context<EndianSlice<'static, LittleEndian>>>>> = Mutex::new(None);
}

pub fn find_location(addr: u64) {
    if_chain!{
        if let Some(ctx) = DEBUG_CTX.lock().as_ref();
        if let Ok(Some(loc))= ctx.find_location(addr);
        then{
            println!("File: {:?} Line: {:?} Column: {:?}", loc.file, loc.line, loc.column);
        } 
    }
}

pub fn find_frame(addr: u64) {
    if_chain!{
        if let Some(ctx) = DEBUG_CTX.lock().as_ref();
        if let Ok(mut frame_iter) = ctx.find_frames(addr);
        then {
            while let Ok(Some(frame)) = frame_iter.next() {
                println!("Function: {:?}", frame.function.unwrap().demangle().unwrap());
            }
        }
    }
}

pub fn parse_info() {
    *DEBUG_CTX.lock() = Some(Box::new(Context::from_sections(
        _abbrev_section(),
        _addr_section(),
        _aranges_section(),
        _info_section(),
        _line_section(),
        _line_str_section(),
        _ranges_section(),
        _rnglists_section(),
        _str_section(),
        _str_offsets_section(),
        EndianSlice::new(&[], LittleEndian),
    ).unwrap()));
}
