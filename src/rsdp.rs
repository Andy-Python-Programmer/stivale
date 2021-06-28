/// The ACPI RSDP info tag passed by the bootloader
#[repr(C, packed)]
pub struct RSDPTag {
    _identifier: u64,
    _next: u64,
    rsdp: u64,
}

impl RSDPTag {
    /// Get the RSDP address
    pub fn rsdp(&self) -> u64 {
        self.rsdp
    }
}

impl From<&RSDPTag> for u64 {
    fn from(tag: &RSDPTag) -> Self {
        tag.rsdp()
    }
}
