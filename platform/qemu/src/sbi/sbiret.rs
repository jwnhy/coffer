#[repr(C)]
pub struct SbiRet {
    error: usize,
    value: usize,
}

mod sbi_error {
    pub const SUCCESS: usize = 0;
    pub const FAILED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-1));
    pub const NOT_SUPPORTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-2));
    pub const INVALID_PARAM: usize = usize::from_ne_bytes(isize::to_ne_bytes(-3));
    pub const DENIED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-4));
    pub const INVALID_ADDRESS: usize = usize::from_ne_bytes(isize::to_ne_bytes(-5));
    pub const ALREADY_AVAILABLE: usize = usize::from_ne_bytes(isize::to_ne_bytes(-6));
    pub const ALREADY_STARTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-7));
    pub const ALREADY_STOPPED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-8));
}

impl SbiRet {
    pub fn ok(value: usize) -> SbiRet {
        SbiRet {
            error: sbi_error::SUCCESS,
            value,
        }
    }

    pub fn not_supported() -> SbiRet {
        SbiRet {
            error: sbi_error::NOT_SUPPORTED,
            value: 0,
        }
    }
    pub fn invalid_param() -> SbiRet {
        SbiRet {
            error: sbi_error::INVALID_PARAM,
            value: 0,
        }
    }
    pub fn denied() -> SbiRet {
        SbiRet {
            error: sbi_error::DENIED,
            value: 0,
        }
    }
    pub fn invalid_address() -> SbiRet {
        SbiRet {
            error: sbi_error::INVALID_ADDRESS,
            value: 0,
        }
    }
    pub fn already_available() -> SbiRet {
        SbiRet {
            error: sbi_error::ALREADY_AVAILABLE,
            value: 0,
        }
    }
    pub fn already_started() -> SbiRet {
        SbiRet {
            error: sbi_error::ALREADY_STARTED,
            value: 0,
        }
    }
    pub fn already_stopped() -> SbiRet {
        SbiRet {
            error: sbi_error::ALREADY_STOPPED,
            value: 0,
        }
    }
}
