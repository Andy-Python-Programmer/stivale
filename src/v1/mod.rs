//! This module contains the definitions for stivale boot protocol. The stivale boot protocol aims
//! to be a simple to implement protocol which provides the kernel with most of the features one may
//! need in a modern x86_64 context (although 32-bit x86 is also supported).

union StivaleHeaderEntryPoint {
    func: extern "C" fn(&'static StivaleStruct) -> !,
    zero: u16,
}

#[repr(C, packed)]
pub struct StivaleHeader {
    stack: *const u8,
    flags: u16,
    framebuffer_width: u16,
    framebuffer_height: u16,
    framebuffer_bpp: u16,
    entry_point: StivaleHeaderEntryPoint,
}

impl StivaleHeader {
    pub fn new() -> Self {
        Self {
            stack: core::ptr::null(),
            flags: 0x00,
            framebuffer_width: 0x00,
            framebuffer_height: 0x00,
            framebuffer_bpp: 0x00,
            entry_point: StivaleHeaderEntryPoint { zero: 0x00 },
        }
    }

    /// Sets the requested framebuffer width. Only parsed if a graphics mode is requested. If
    /// set to zero, the bootloader would pick the best possible video mode automatically (recommended).
    pub fn framebuffer_width(mut self, framebuffer_width: u16) -> Self {
        self.framebuffer_width = framebuffer_width;
        self
    }

    /// Sets the requested framebuffer height. Only parsed if a graphics mode is requested. If
    /// set to zero, the bootloader would pick the best possible video mode automatically (recommended).
    pub fn framebuffer_height(mut self, framebuffer_height: u16) -> Self {
        self.framebuffer_height = framebuffer_height;
        self
    }

    /// Sets the requested framebuffer bpp. Only parsed if a graphics mode is requested. If
    /// set to zero, the bootloader would pick the best possible video mode automatically (recommended).
    pub fn framebuffer_bpp(mut self, framebuffer_bpp: u16) -> Self {
        self.framebuffer_bpp = framebuffer_bpp;
        self
    }

    /// Sets the stack pointer which will be in ESP/RSP when the kernel is loaded.
    /// It can only be set to NULL for 64-bit kernels. 32-bit kernels are mandated to
    /// provide a vaild stack. 64-bit and 32-bit valid stacks must be at least 256 bytes
    /// in usable space and must be 16 byte aligned addresses.
    pub fn stack(mut self, stack: *const u8) -> Self {
        self.stack = stack;
        self
    }

    /// Sets the entry point address. If not zero, the bootloader would jump to the specified
    /// entry point instead of jumping to the entry point specified the kernel ELF.
    pub fn entry_point(mut self, func: extern "C" fn(&'static StivaleStruct) -> !) -> Self {
        self.entry_point = StivaleHeaderEntryPoint { func };
        self
    }
}

#[repr(C, packed)]
pub struct StivaleStruct {
    // TODO:
}
