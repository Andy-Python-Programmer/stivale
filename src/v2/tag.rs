#[repr(C, packed)]
pub struct StivaleTagHeader {
    pub identifier: u64,
    pub next: u64,
}

/// If the framebuffer tag was requested through the framebuffer tag header and its supported by the stivale
/// bootloader, this tag is returned to the kernel. This tag provides an interface to the framebuffer.
#[repr(C, packed)]
pub struct StivaleFramebufferTag {
    pub header: StivaleTagHeader,
    /// The address of the framebuffer.
    pub framebuffer_addr: u64,
    /// The total width of the framebuffer in pixels.
    pub framebuffer_width: u16,
    /// The total height of the framebuffer in pixels.
    pub framebuffer_height: u16,
    /// The pitch of the framebuffer in bytes.
    pub framebuffer_pitch: u16,
    /// The amount of bytes-per pixel.
    pub framebuffer_bpp: u16,
    /// Memory model of the framebuffer. If set to one, its RGB and all other values
    /// are undefined.
    pub memory_model: u8,
    /// Size of the red mask in RGB.
    pub red_mask_size: u8,
    /// Shift of the red mask in RGB.
    pub red_mask_shift: u8,
    /// Size of the green mask in RGB.
    pub green_mask_size: u8,
    /// Shift of the green mask in RGB.
    pub green_mask_shift: u8,
    /// Size of the blue mask in RGB.
    pub blue_mask_size: u8,
    /// Shift of the blue mask in RGB.
    pub blue_mask_shift: u8,
}

/// If the terminal tag was requested through the terminal tag header and its supported by the stivale
/// bootloader, this tag is returned to the kernel. This tag provides an interface to the stivale terminal.
#[repr(C, packed)]
pub struct StivaleTerminalTag {
    pub header: StivaleTagHeader,
    pub flags: u32,
    /// The amount of columns in the stivale terminal setup by the bootloader.
    pub cols: u16,
    /// The amount of rows in the stivale terminal setup by the bootloader.
    pub rows: u16,
    /// The virtual address of the `term_write` function, which is used to write to the stivale terminal. For
    /// a more safer way use the [StivaleTerminalTag::term_write]
    pub term_write_addr: u64,
}

impl StivaleTerminalTag {
    /// Returns the terminal write function provided by the terminal stivale tag. This function
    /// returns the transmuted function for you to simplify the process of passing the string as a raw pointer
    /// and passing the string length.
    ///
    /// ## Example
    /// ```rust,no_run
    /// let terminal_tag = stivale_struct.terminal().expect("Terminal tag was provided by the stivale2 bootloader");
    /// let term_write = terminal_tag.term_write();
    ///
    /// term_write("Hello, Stivale!");
    /// term_write("Hello, Rust!")
    /// ```
    ///
    /// ## Safety
    /// This function is **not** thread safe.
    pub fn term_write(&self) -> impl Fn(&str) {
        let __fn_ptr = self.term_write_addr as *const ();
        let __term_func =
            unsafe { core::mem::transmute::<*const (), extern "C" fn(*const i8, u64)>(__fn_ptr) };

        move |txt| {
            __term_func(txt.as_ptr() as *const i8, txt.len() as u64);
        }
    }
}
