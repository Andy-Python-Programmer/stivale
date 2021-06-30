use core::marker::PhantomData;

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

/// This tag is used to get the location of the ACPI RSDP structure in memory.
#[repr(C, packed)]
pub struct StivaleRsdpTag {
    pub header: StivaleTagHeader,
    /// Pointer to the ACPI RSDP structure.
    pub rsdp: u64,
}

/// The type of a memory map entry. The entries are guaranteed to be sorted by base address,
/// lowest to highest.
///
/// ## Alignment
/// Usable and bootloader reclaimable entries are guaranteed to be 4096 byte aligned for both
/// base and length. Usable and bootloader reclaimable entries are **guaranteed** not to overlap with
/// any other entry.
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StivaleMemoryMapEntryType {
    /// Usable memory.
    Usable = 1,
    /// Memory reserved by the system.
    Reserved = 2,
    /// ACPI memory that can be reclaimed.
    AcpiReclaimable = 3,
    /// ACPI memory that cannot be reclaimed.
    AcpiNvs = 4,
    /// Memory marked as defective (bad RAM).
    BadMemory = 5,
    /// Memory used by the bootloader that can be reclaimed after it's not being used anymore.
    BootloaderReclaimable = 0x1000,
    /// Memory containing the kernel and any modules.
    Kernel = 0x1001,
    /// Memory containing the framebuffer.
    Framebuffer = 0x1002,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct StivaleMemoryMapEntry {
    /// Physical address of base of the memory section.
    pub base: u64,
    /// Length of this memory section.
    pub length: u64,
    /// The type of this memory map entry.
    pub entry_type: StivaleMemoryMapEntryType,

    padding: u32,
}

#[repr(C, packed)]
pub struct StivaleMemoryMapTag {
    pub header: StivaleTagHeader,
    /// Total length of the memory map entries.
    pub entries_len: u64,
    /// Slice of the memory map entries.
    pub entry_array: [StivaleMemoryMapEntry],
}

impl StivaleMemoryMapTag {
    /// Returns an iterator over all the memory regions.
    pub fn iter(&self) -> StivaleMemoryMapIter {
        StivaleMemoryMapIter {
            sref: self,
            current: 0x00,
            phantom: PhantomData::default(),
        }
    }
}

/// Iterator over all the memory memory regions provided by the stivale bootloader.
#[derive(Clone)]
pub struct StivaleMemoryMapIter<'a> {
    /// A reference to the stivale memory map tag.
    sref: &'a StivaleMemoryMapTag,
    /// The index of the memory map entry that we are about to index.
    current: u64,
    phantom: PhantomData<&'a StivaleMemoryMapEntry>,
}

impl<'a> Iterator for StivaleMemoryMapIter<'a> {
    type Item = &'a StivaleMemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.sref.entries_len {
            let entry = &self.sref.entry_array[self.current as usize];
            self.current += 1;

            Some(entry)
        } else {
            None
        }
    }
}
