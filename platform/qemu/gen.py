for s in ['abbrev', 'addr', 'aranges', 'info', 'line', 'line_str', 'ranges', 'rnglists', 'str', 'str_offsets']:
    print("fn _"+s+"_section() -> <EndianSlice<'static, LittleEndian>> {\n \
    let start = _rvbt_"+s+"_start as usize;\n \
    let end = _rvbt_"+s+"_end as usize;\n \
    let bytes = unsafe { \
        slice::from_raw_parts(start as *const u8, end - start)\n \
    };\n\
    ::new(bytes, LittleEndian)}")
