use core::u16;

/// The header terminal tag, which if present instructs the stivale bootloader to set up a terminal for
/// the kernel at run time. The framebuffer header tag must be specified when passing this header tag. See
/// the documentation of [TerminalTag] for more information.
#[repr(C, packed)]
pub struct HeaderTerminalTag {
    identifier: u64,
    next: *const (),
    flags: u32,
}

impl HeaderTerminalTag {
    /// Creates a new header terminal tag. See the structure-level documentation for more information.
    #[inline]
    pub fn new() -> Self {
        Self {
            identifier: 0xa85d499b1823be72,
            next: core::ptr::null(),
            flags: 0x00,
        }
    }

    /// Add another tag to the stivale header.
    pub const fn next(mut self, tag: *const ()) -> Self {
        self.next = tag;
        self
    }
}

// Send and Sync are okay because we won't be accessing the header data on runtime.
unsafe impl Send for HeaderTerminalTag {}
unsafe impl Sync for HeaderTerminalTag {}

/// If the terminal tag was requested through the terminal tag header and its supported by the stivale
/// bootloader, this tag is returned to the kernel. This tag provides an interface to the stivale terminal.
#[repr(C, packed)]
pub struct TerminalTag {
    identifier: u64,
    next: u64,
    flags: u32,
    cols: u16,
    rows: u16,
    term_write: u64,
}

impl TerminalTag {
    /// Returns the amount of rows in the stivale terminal setup by the stivale bootloader.
    #[inline]
    pub fn rows(&self) -> u16 {
        self.rows
    }

    /// Returns the amount of columns in the stivale terminal setup by the stivale bootloader.
    #[inline]
    pub fn cols(&self) -> u16 {
        self.cols
    }

    /// Returns the terminal write function provided by the terminal stivale tag. This function
    /// returns the transmuted function for you to simplify the process of passing the string as a raw pointer
    /// and passing the string length.
    ///
    /// ## Example
    /// ```rust,norun
    /// let terminal_tag = stivale_struct.terminal();
    /// let term_write = terminal_tag.get_term_write_func();
    ///
    /// term_write("Hello, Stivale!");
    /// term_write("Hello, Rust!")
    /// ```
    ///
    /// ## Saftey
    /// This function is **not** thread safe.
    pub fn get_term_write_func(&self) -> impl Fn(&str) {
        let __fn_ptr = self.term_write as *const ();
        let __term_func =
            unsafe { core::mem::transmute::<*const (), extern "C" fn(*const i8, u64)>(__fn_ptr) };

        move |txt| {
            __term_func(txt.as_ptr() as *const i8, txt.len() as u64);
        }
    }
}
