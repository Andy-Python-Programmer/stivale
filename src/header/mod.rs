//! stivale2 header module for letting a stivale2 compliant bootloader find the kernel
//!
//! Contains a stivale2 header struct and various tags allowed by stivale2
//!
//! # Examples
//!
//! Basic header
//! ```ignore
//! static STACK: [u8; 4096] = [0; 4096];
//!
//! #[link_section = ".stivale2hdr"]
//! #[used]
//! static STIVALE_HDR: StivaleHeader = StivaleHeader::new(STACK[0] as *const u8);
//! ```
//!
//! Header with a framebuffer tag
//! ```ignore
//! static STACK: [u8; 4096] = [0; 4096];
//! static FRAMEBUFFER_TAG: HeaderFramebufferTag = HeaderFramebufferTag::new().bpp(24);
//!
//! #[link_section = ".stivale2hdr"]
//! #[used]
//! static STIVALE_HDR: StivaleHeader = StivaleHeader::new(STACK[0] as *const u8).tags((&FRAMEBUFFER_TAG as *const HeaderFramebufferTag).cast());
//! ```

pub mod framebuffer;
pub use framebuffer::*;

pub mod paging;
pub use paging::*;

bitflags! {
    pub struct StivaleHeaderFlags: u64 {
        /// Set if the bootloader should apply kernel address space layout randomization
        const KASLR = 0x1;
    }
}

#[repr(C)]
union StivaleHeaderEntryPoint {
    function: extern "C" fn(stivale_struct_addr: usize) -> !,
    null: u64,
}

/// A stivale2 header for the bootloader
///
/// It must be defined in a static, and it must have the parameters `#[link_section = ".stivale2hdr"]`
/// and `#[used]` so it isn't optimized away and a stivale2 compliant bootloader can find it
#[repr(C, packed)]
#[allow(dead_code)]
pub struct StivaleHeader {
    entry_point: StivaleHeaderEntryPoint,
    stack: *const u8,
    flags: StivaleHeaderFlags,
    tags: *const (),
}

// Send and Sync are okay because we won't be accessing the data on runtime anyways
unsafe impl Send for StivaleHeader {}
unsafe impl Sync for StivaleHeader {}
impl StivaleHeader {
    /// Create a new stivale2 header with a stack
    pub const fn new(stack: *const u8) -> Self {
        StivaleHeader {
            entry_point: StivaleHeaderEntryPoint { null: 0 },
            stack,
            flags: StivaleHeaderFlags::empty(),
            tags: core::ptr::null(),
        }
    }

    /// Set the entry point that a stivale2 compliant bootloader will call
    pub const fn entry_point(
        mut self,
        entry_point: extern "C" fn(stivale_struct_addr: usize) -> !,
    ) -> Self {
        self.entry_point = StivaleHeaderEntryPoint {
            function: entry_point,
        };
        self
    }

    /// Set the stivale2 header flags
    pub const fn flags(mut self, flags: StivaleHeaderFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Set a pointer to a stivale2 header tag
    pub const fn tags(mut self, tag: *const ()) -> Self {
        self.tags = tag;
        self
    }
}
