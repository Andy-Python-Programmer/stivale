/// A stivale2 header framebuffer tag
#[repr(packed)]
#[allow(dead_code)]
pub struct HeaderFramebufferTag {
    identifier: u64,
    next: *const (),
    width: u16,
    height: u16,
    bpp: u16,
}

// Send and Sync are okay because we won't be accessing the data on runtime.
unsafe impl Send for HeaderFramebufferTag {}
unsafe impl Sync for HeaderFramebufferTag {}

impl HeaderFramebufferTag {
    /// Create a new header framebuffer tag that will have the bootloader determine the best
    /// resolution and bpp values
    pub const fn new() -> Self {
        HeaderFramebufferTag {
            identifier: 0x3ecc1bc43d0f7971,
            next: core::ptr::null(),
            width: 0,
            height: 0,
            bpp: 0,
        }
    }

    /// Set the requested framebuffer resolution
    ///
    /// Either width or height can be set to 0 to let the bootloader pick the best resolution
    pub const fn resolution(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the requested framebuffer bits per pixel
    pub const fn bpp(mut self, bpp: u16) -> Self {
        self.bpp = bpp;
        self
    }

    /// Add another tag to the stivale2 header
    pub const fn next(mut self, tag: *const ()) -> Self {
        self.next = tag;
        self
    }
}
