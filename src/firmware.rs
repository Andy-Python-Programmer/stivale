bitflags! {
    pub struct FirmwareFlags: u64 {
        /// Set if the kernel was booted from a BIOS bootloader, clear if booted from an UEFI bootloader
        const BIOS_BOOT = 0x1;
    }
}

/// The system firmware info tag passed by the bootloader
#[repr(packed)]
pub struct FirmwareTag {
    _identifier: u64,
    _next: u64,
    flags: FirmwareFlags,
}

impl FirmwareTag {
    /// Get the firmware and boot flags
    pub fn flags(&self) -> FirmwareFlags {
        self.flags
    }
}
