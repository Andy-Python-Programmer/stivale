use core::marker::PhantomData;

/// The type of the memory map entry
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemoryMapEntryType {
    /// Usable memory
    Usable = 1,
    /// Memory reserved by the system
    Reserved = 2,
    /// ACPI memory that can be reclaimed
    AcpiReclaimable = 3,
    /// ACPI memory that cannot be reclaimed
    AcpiNvs = 4,
    /// Memory marked as defective (bad RAM)
    BadMemory = 5,
    /// Memory used by the bootloader that can be reclaimed after it's not being used anymore
    BootloaderReclaimable = 0x1000,
    /// Memory containing the kernel and any modules
    Kernel = 0x1001,
}

/// A memory region
#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryMapEntry {
    base: u64,
    length: u64,
    entry_type: MemoryMapEntryType,
    _unused: u32,
}

impl MemoryMapEntry {
    /// Get the address where the memory region starts
    pub fn start_address(&self) -> u64 {
        self.base
    }

    /// Get the address where the memory region ends
    ///
    /// Identical to `entry.start_address() + entry.size()`
    pub fn end_address(&self) -> u64 {
        self.base + self.length
    }

    /// Get the size of the memory region
    pub fn size(&self) -> u64 {
        self.length
    }

    /// Get the type of the memory region
    pub fn entry_type(&self) -> MemoryMapEntryType {
        self.entry_type
    }
}

/// A memory map tag provided by the bootloader
#[repr(packed)]
pub struct MemoryMapTag {
    _identifier: u64,
    _next: u64,
    entries: u64,
    pub entry_array: [MemoryMapEntry; 0],
}

impl MemoryMapTag {
    /// Get the count of memory regions
    pub fn entries(&self) -> u64 {
        self.entries
    }

    /// Get an iterator over all the memory regions
    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter {
            tag: self,
            current: 0,
            _phantom: PhantomData::default(),
        }
    }

    fn array(&self) -> &[MemoryMapEntry] {
        unsafe { core::slice::from_raw_parts(self.entry_array.as_ptr(), self.entries as usize) }
    }
}

/// An iterator over all memory regions
#[derive(Clone)]
pub struct MemoryMapIter<'a> {
    tag: &'a MemoryMapTag,
    current: u64,
    _phantom: PhantomData<&'a MemoryMapEntry>,
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.tag.entries() {
            let entry = &self.tag.array()[self.current as usize];
            self.current += 1;
            Some(entry)
        } else {
            None
        }
    }
}
