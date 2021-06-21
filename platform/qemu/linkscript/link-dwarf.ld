SECTIONS {
	.debug_addr 0 : {
		_debug_addr_start = .;
    KEEP (*(.debug_addr)) *(.debug_addr)
		_debug_addr_end = .;
	}
	.debug_aranges 0 : {
		_debug_aranges_start = .;
    KEEP (*(.debug_aranges)) *(.debug_aranges)
		_debug_aranges_end = .;
	}
	.debug_line 0 : {
		_debug_line_start = .;
    KEEP (*(.debug_line)) *(.debug_line)
		_debug_line_end = .;
	}
	.debug_line_str 0 : {
		_debug_line_str_start = .;
    KEEP (*(.debug_line_str)) *(.debug_line_str)
		_debug_line_str_end = .;
	}
	  .debug_info 0 : {
    _debug_info_start = .;
    KEEP (*(.debug_info .gnu.linkonce.wi.*)) *(.debug_info .gnu.linkonce.wi.*)
    _debug_info_end = .;
  }
  .debug_abbrev 0 : {
    _debug_abbrev_start = .;
    KEEP (*(.debug_abbrev)) *(.debug_abbrev)
    _debug_abbrev_end = .;
  }
  .debug_str 0 : {
    _debug_str_start = .;
    KEEP (*(.debug_str)) *(.debug_str)
    _debug_str_end = .;
  }
	.debug_ranges 0 : {
		_debug_ranges_start = .;
    KEEP (*(.debug_ranges)) *(.debug_ranges)
		_debug_ranges_end = .;
	}
	.debug_rnglists 0 : {
		_debug_rnglists_start = .;
    KEEP (*(.debug_rnglists)) *(.debug_rnglists)
		_debug_rnglists_end = .;
	}
	.debug_str_offsets 0 : {
		_debug_str_offsets_start = .;
    KEEP (*(.debug_str_offsets)) *(.debug_str_offsets)
		_debug_str_offsets_end = .;
	}
}
