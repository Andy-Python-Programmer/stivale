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
    pub entry_array: [StivaleMemoryMapEntry; 0],
}

impl StivaleMemoryMapTag {
    /// Return's the slice of memory map entries.
    pub fn as_slice(&self) -> &[StivaleMemoryMapEntry] {
        unsafe { core::slice::from_raw_parts(self.entry_array.as_ptr(), self.entries_len as usize) }
    }

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
            let entry = &self.sref.as_slice()[self.current as usize];
            self.current += 1;

            Some(entry)
        } else {
            None
        }
    }
}

/// This tag is used to get the current UNIX epoch, as per RTC.
#[repr(C, packed)]
pub struct StivaleEpochTag {
    pub header: StivaleTagHeader,
    /// UNIX epoch at boot, which is read from system RTC.
    pub epoch: u64,
}

bitflags::bitflags! {
    /// Bitfield representing the firmware and boot flags passed by the bootloader.
    pub struct StivaleFirmwareTagFlags: u64 {
        /// The kernel was booted in UEFI mode.
        const UEFI = 0x00;
        /// The kernel was booted in a legacy BIOS mode.
        const BIOS = 0x01;
    }
}

/// This tag is used to get the info about the firmware.
#[repr(C, packed)]
pub struct StivaleFirmwareTag {
    pub header: StivaleTagHeader,
    /// Flags telling about the firmware and boot flags passed by the bootloader.
    pub flags: StivaleFirmwareTagFlags,
}

/// This tag is used to get a pointer to the EFI system table if available.
#[repr(C, packed)]
pub struct StivaleEfiSystemTableTag {
    pub header: StivaleTagHeader,
    /// Address of the EFI system table.
    pub system_table_addr: u64,
}

/// This tag is used to get the kernel with a pointer to a copy the raw executable
/// file of the kernel that the bootloader loaded.
#[repr(C, packed)]
pub struct StivaleKernelFileTag {
    pub header: StivaleTagHeader,
    /// Address of the raw kernel file.
    pub kernel_file_addr: u64,
}

/// This tag is used to get the slide that the bootloader applied over the kernel's load
/// address as a positive offset.
#[repr(C, packed)]
pub struct StivalekernelSlideTag {
    pub header: StivaleTagHeader,
    /// The kernel slide. See structure-level documentation for more information.
    pub kernel_slide: u64,
}
