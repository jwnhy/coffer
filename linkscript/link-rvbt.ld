	.rvbt_addr : {
		_rvbt_addr_start = .;
		. += _debug_addr_end - _debug_addr_start;
		_rvbt_addr_end = .;
	}
	.rvbt_aranges  : {
		_rvbt_aranges_start = .;
		. += _debug_aranges_end - _debug_aranges_start;
		_rvbt_aranges_end = .;
	}
	.rvbt_line  : {
		_rvbt_line_start = .;
		. += _debug_line_end - _debug_line_start;
		_rvbt_line_end = .;
	}
	.rvbt_line_str  : {
		_rvbt_line_str_start = .;
		. += _debug_line_str_end - _debug_line_str_start;
		_rvbt_line_str_end = .;
	}
	.rvbt_info  : {
    _rvbt_info_start = .;
		. += _debug_info_end - _debug_info_start;
    _rvbt_info_end = .;
  }
  .rvbt_abbrev  : {
    _rvbt_abbrev_start = .;
		. += _debug_abbrev_end - _debug_abbrev_start;
    _rvbt_abbrev_end = .;
  }
  .rvbt_str  : {
    _rvbt_str_start = .;
		. += _debug_str_end - _debug_str_start;
    _rvbt_str_end = .;
  }
	.rvbt_ranges  : {
		_rvbt_ranges_start = .;
		. += _debug_ranges_end - _debug_ranges_start;
		_rvbt_ranges_end = .;
	}
	.rvbt_rnglists  : {
		_rvbt_rnglists_start = .;
		. += _debug_rnglists_end - _debug_rnglists_start;
		_rvbt_rnglists_end = .;
	}
	.rvbt_str_offsets  : {
		_rvbt_str_offsets_start = .;
		. += _debug_str_offsets_end - _debug_str_offsets_start;
		_rvbt_str_offsets_end = .;
	}
	
