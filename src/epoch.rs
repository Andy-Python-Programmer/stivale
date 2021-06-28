/// The UNIX epoch info tag passed by the bootloader
#[repr(packed)]
pub struct EpochTag {
    _identifier: u64,
    _next: u64,
    epoch: u64,
}

impl EpochTag {
    /// Get the boot UNIX epoch
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

impl From<&EpochTag> for u64 {
    fn from(tag: &EpochTag) -> Self {
        tag.epoch()
    }
}
