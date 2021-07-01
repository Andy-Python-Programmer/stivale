//! This module contains the definitions for stivale2 boot protocol. The stivale2 boot protocol is an
//! modern version of the legacy stivale protocol which provides the kernel with most of the features
//! one may need. The stivale2 protocol also supports 32-bit systems.

mod header;
mod tag;
mod utils;

pub use header::*;
pub use tag::*;

#[repr(C, packed)]
pub struct StivaleStruct {
    bootloader_brand: [u8; 64],
    bootloader_version: [u8; 64],
    tags: u64,
}

impl StivaleStruct {
    pub fn bootloader_brand(&self) -> &str {
        utils::string_from_slice(&self.bootloader_brand)
    }

    pub fn bootloader_version(&self) -> &str {
        utils::string_from_slice(&self.bootloader_version)
    }

    pub fn get_tag(&self, identifier: u64) -> Option<u64> {
        let mut current_tag = self.tags as *const StivaleTagHeader;

        while !current_tag.is_null() {
            let tag = unsafe { &*current_tag };

            if tag.identifier == identifier {
                return Some(current_tag as u64);
            }

            current_tag = tag.next as *const StivaleTagHeader;
        }

        None
    }

    pub fn framebuffer(&self) -> Option<&StivaleFramebufferTag> {
        self.get_tag(0x506461d2950408fa)
            .map(|addr| unsafe { &*(addr as *const StivaleFramebufferTag) })
    }

    pub fn terminal(&self) -> Option<&StivaleTerminalTag> {
        self.get_tag(0xc2b3f4c3233b0974)
            .map(|addr| unsafe { &*(addr as *const StivaleTerminalTag) })
    }

    pub fn memory_map(&self) -> Option<&StivaleMemoryMapTag> {
        self.get_tag(0x2187f79e8612de07)
            .map(|addr| unsafe { &*(addr as *const StivaleMemoryMapTag) })
    }

    pub fn epoch(&self) -> Option<&StivaleEpochTag> {
        self.get_tag(0x566a7bed888e1407)
            .map(|addr| unsafe { &*(addr as *const StivaleEpochTag) })
    }

    pub fn frimware(&self) -> Option<&StivaleFirmwareTag> {
        self.get_tag(0x359d837855e3858c)
            .map(|addr| unsafe { &*(addr as *const StivaleFirmwareTag) })
    }

    pub fn efi_system_table(&self) -> Option<&StivaleEfiSystemTableTag> {
        self.get_tag(0x4bc5ec15845b558e)
            .map(|addr| unsafe { &*(addr as *const StivaleEfiSystemTableTag) })
    }

    pub fn kernel_file(&self) -> Option<&StivaleKernelFileTag> {
        self.get_tag(0xe599d90c2975584a)
            .map(|addr| unsafe { &*(addr as *const StivaleKernelFileTag) })
    }

    pub fn kernel_slide(&self) -> Option<&StivaleKernelSlideTag> {
        self.get_tag(0xee80847d01506c57)
            .map(|addr| unsafe { &*(addr as *const StivaleKernelSlideTag) })
    }
}
