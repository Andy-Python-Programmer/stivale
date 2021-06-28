/// A stivale2 header tag that asks the bootloader for 5-level paging, if supported
#[repr(packed)]
#[allow(dead_code)]
pub struct Header5LevelPagingTag {
    identifier: u64,
    next: *const (),
}

// Send and Sync are okay because we won't be accessing the data on runtime anyways
unsafe impl Send for Header5LevelPagingTag {}
unsafe impl Sync for Header5LevelPagingTag {}
impl Header5LevelPagingTag {
    /// Create a new header tag that marks the kernel for 5-level paging
    pub const fn new() -> Self {
        Header5LevelPagingTag {
            identifier: 0x932f477032007e8f,
            next: core::ptr::null(),
        }
    }

    /// Add another tag to the stivale2 header
    pub const fn next(mut self, tag: *const ()) -> Self {
        self.next = tag;
        self
    }
}
