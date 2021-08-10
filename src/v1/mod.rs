//! This module contains the definitions for stivale boot protocol. The stivale boot protocol aims
//! to be a simple to implement protocol which provides the kernel with most of the features one may
//! need in a modern x86_64 context (although 32-bit x86 is also supported).

mod utils;

use core::marker::PhantomData;

union StivaleHeaderEntryPoint {
    func: extern "C" fn(&'static StivaleStruct) -> !,
    zero: u16,
}

bitflags::bitflags! {
    pub struct StivaleHeaderFlags: u16 {
        /// If set, the bootloader will be instructed to use graphics
        /// framebuffer mode. Else text mode will be selected.
        const FRAMEBUFFER_MODE = 1 << 0;
        /// If set, level 5 paging will be requested to the bootloader
        /// (only if avaliable). Else standard level 4 paging will be used.
        ///
        /// ## 32-bit
        /// This bit is ignored for 32-bit kernels.
        const LEVEL_5_PAGING = 1 << 1;
        /// Formerly used to indicate whether to enable KASLR,
        /// this flag is now reserved as KASLR is enabled in the
        /// bootloader configuration instead. Presently
        /// reserved and unused.
        const KASLR = 1 << 2;
        /// If set, all pointers, except otherwise noted,
        /// are to be offset to the higher half. That is,
        /// their value will be their physical address plus
        /// `0xffff800000000000` with 4-level paging or
        /// `0xff00000000000000` with 5-level paging on x86_64.
        /// Success for this feature can be tested by checking
        /// whether the stivale struct pointer argument passed
        /// to the entry point function is in the higher
        /// half or not.
        const HIGHER_HALF = 1 << 3;
        const NULL = 0x00;
    }
}

#[repr(C)]
pub struct StivaleHeader {
    stack: *const u8,
    flags: StivaleHeaderFlags,
    framebuffer_width: u16,
    framebuffer_height: u16,
    framebuffer_bpp: u16,
    entry_point: StivaleHeaderEntryPoint,
}

impl StivaleHeader {
    pub fn new() -> Self {
        Self {
            stack: core::ptr::null(),
            flags: StivaleHeaderFlags::empty(),
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

    /// Sets the provided stivale header flags. See the documentation of [StivaleHeaderFlags]
    /// for more information.
    pub fn flags(mut self, flags: StivaleHeaderFlags) -> Self {
        self.flags = flags;
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

/// Structure representing a module, containing the information of a module that
/// the bootloader loaded alongside the kernel.
#[repr(C)]
pub struct StivaleModule {
    /// Address where this module has been loaded.
    pub start: u64,
    /// End address of this module.
    pub end: u64,
    /// ASCII 0-terminated string passed to the module as specified in
    /// the config file.
    pub string: [u8; 128],
}

impl StivaleModule {
    /// Returns the size of this module.
    #[inline]
    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    /// Returns the ASCII 0-terminated string passed to the module as specified in the config file
    /// as a rust string.
    #[inline]
    pub fn as_str(&self) -> &str {
        self::utils::string_from_slice(&self.string)
    }
}

/// Iterator over all the modules that were loaded.
#[derive(Clone)]
pub struct StivaleModuleIter<'a> {
    /// A reference to the stivale structure.
    sref: &'a StivaleStruct,
    /// The index of the module entry that we are about to index.
    current: u64,
    phantom: PhantomData<&'a StivaleModule>,
}

impl<'a> Iterator for StivaleModuleIter<'a> {
    type Item = &'a StivaleModule;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.sref.module_len {
            let entry = &self.sref.modules_as_slice()[self.current as usize];
            self.current += 1;

            Some(entry)
        } else {
            None
        }
    }
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

#[repr(C)]
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

impl StivaleMemoryMapEntry {
    /// Returns the end address of this memory region.
    #[inline]
    pub fn end_address(&self) -> u64 {
        self.base + self.length
    }

    /// Returns the entry type of this memory region. External function is required
    /// as reference the entry_type packed field is not aligned.
    #[inline]
    pub fn entry_type(&self) -> StivaleMemoryMapEntryType {
        self.entry_type
    }
}

/// Iterator over all the memory regions provided by the stivale bootloader.
#[derive(Clone)]
pub struct StivaleMemoryMapIter<'a> {
    /// A reference to the stivale structure.
    sref: &'a StivaleStruct,
    /// The index of the memory map entry that we are about to index.
    current: u64,
    phantom: PhantomData<&'a StivaleMemoryMapEntry>,
}

impl<'a> Iterator for StivaleMemoryMapIter<'a> {
    type Item = &'a StivaleMemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.sref.memory_map_len {
            let entry = &self.sref.memory_map_as_slice()[self.current as usize];
            self.current += 1;

            Some(entry)
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct StivaleStruct {
    /// Address of the null-terminated command line.
    pub command_line: u64,
    /// Pointer to the memory map array.
    pub memory_map_array: [StivaleMemoryMapEntry; 0],
    /// Length of the memory map entries.
    pub memory_map_len: u64,

    /// Address of the framebuffer if avaliable else its set to zero.
    pub framebuffer_addr: u64,
    /// The framebuffer pitch in bytes.
    pub framebuffer_pitch: u16,
    /// Width of the framebuffer in pixels.
    pub framebuffer_width: u16,
    /// Height of the framebuffer in pixels.
    pub framebuffer_height: u16,
    /// The framebuffer bits per pixels.
    pub framebuffer_bpp: u16,

    /// Address of the RSDP ACPI structure.
    pub rsdp_adddres: u64,

    /// The length of modules that the stivale bootloader loaded according to the
    /// config.
    pub module_len: u64,
    /// Pointer to the modules array.
    pub modules: [StivaleModule; 0],

    /// UNIX epoch at boot, which is read from system RTC.
    pub unix_epoch: u64,
    pub flags: u64,

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
    _padding: u8,

    /// 32-bit SMBIOS entry point address. Set to 0 if unavailable.
    pub smbios_entry_32: u64,
    /// 64-bit SMBIOS entry point address. Set to 0 if unavailable.
    pub smbios_entry_64: u64,
}

impl StivaleStruct {
    /// Return's the modules array pointer as a rust slice.
    pub fn modules_as_slice(&self) -> &[StivaleModule] {
        unsafe { core::slice::from_raw_parts(self.modules.as_ptr(), self.module_len as usize) }
    }

    /// Returns an iterator over all the modules that were loaded.
    pub fn modules_iter(&self) -> StivaleModuleIter {
        StivaleModuleIter {
            sref: self,
            current: 0,
            phantom: PhantomData::default(),
        }
    }

    /// Return's memory map entries pointer as a rust slice.
    pub fn memory_map_as_slice(&self) -> &[StivaleMemoryMapEntry] {
        unsafe {
            core::slice::from_raw_parts(
                self.memory_map_array.as_ptr(),
                self.memory_map_len as usize,
            )
        }
    }

    /// Returns an iterator over all the memory regions.
    pub fn memory_map_iter(&self) -> StivaleMemoryMapIter {
        StivaleMemoryMapIter {
            sref: self,
            current: 0x00,
            phantom: PhantomData::default(),
        }
    }
}
