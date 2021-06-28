use core::fmt;
use core::marker::PhantomData;

/// A module info struct
#[repr(packed)]
pub struct Module {
    start: u64,
    end: u64,
    string: [u8; 128],
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We copy as borrowing from packed structs is an error
        let start = self.start;
        let end = self.end;
        f.debug_struct("Module")
            .field("start", &start)
            .field("end", &end)
            .field("string", &self.string())
            .finish()
    }
}

impl Module {
    /// Get the address where the module starts
    pub fn start_address(&self) -> u64 {
        self.start
    }

    /// Get the address where the module ends
    pub fn end_address(&self) -> u64 {
        self.end
    }

    /// Get the size of the module
    ///
    /// Identical to `module.start_address() + module.end_address()`
    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    /// Get the string passed to the module by the bootloader, if any
    pub fn string(&self) -> Option<&str> {
        crate::string_from_u8(&self.string)
    }
}

/// A module tag describing all the modules
#[repr(packed)]
pub struct ModuleTag {
    _identifier: u64,
    _next: u64,
    module_count: u64,
    module_array: [Module; 0],
}

impl ModuleTag {
    /// Get the count of modules
    pub fn module_count(&self) -> u64 {
        self.module_count
    }

    /// Get an iterator over all the memory regions
    pub fn iter(&self) -> ModuleIter {
        ModuleIter {
            tag: self,
            current: 0,
            _phantom: PhantomData::default(),
        }
    }

    fn array(&self) -> &[Module] {
        unsafe {
            core::slice::from_raw_parts(self.module_array.as_ptr(), self.module_count as usize)
        }
    }
}

/// An iterator over all the loaded modules
#[derive(Clone)]
pub struct ModuleIter<'a> {
    tag: &'a ModuleTag,
    current: u64,
    _phantom: PhantomData<&'a Module>,
}

impl<'a> Iterator for ModuleIter<'a> {
    type Item = &'a Module;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.tag.module_count() {
            let entry = &self.tag.array()[self.current as usize];
            self.current += 1;
            Some(entry)
        } else {
            None
        }
    }
}
