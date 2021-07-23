pub struct FenceInfo {
    pub start: Option<usize>,
    pub size: Option<usize>,
    pub asid: Option<usize>,
    pub vmid: Option<usize>,
}

impl FenceInfo {
    pub fn is_flush_all(&self) -> bool {
        self.start.is_none() || self.size.is_none() || self.size.unwrap() == usize::max_value()
    }
}
