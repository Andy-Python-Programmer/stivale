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

    pub fn command_line(&self) -> Option<&'static StivaleCommandLineTag> {
        self.get_tag(0xe5e76a1b4597a781)
            .map(|addr| unsafe { &*(addr as *const StivaleCommandLineTag) })
    }

    pub fn memory_map(&self) -> Option<&'static StivaleMemoryMapTag> {
        self.get_tag(0x2187f79e8612de07)
            .map(|addr| unsafe { &*(addr as *const StivaleMemoryMapTag) })
    }

    pub fn framebuffer(&self) -> Option<&'static StivaleFramebufferTag> {
        self.get_tag(0x506461d2950408fa)
            .map(|addr| unsafe { &*(addr as *const StivaleFramebufferTag) })
    }

    pub fn edid_info(&self) -> Option<&'static StivaleEdidInfoTag> {
        self.get_tag(0x968609d7af96b845)
            .map(|addr| unsafe { &*(addr as *const StivaleEdidInfoTag) })
    }

    #[allow(deprecated)]
    pub fn mtrr(&self) -> Option<&'static StivaleMtrrTag> {
        self.get_tag(0x6bc1a78ebe871172)
            .map(|addr| unsafe { &*(addr as *const StivaleMtrrTag) })
    }

    pub fn terminal(&self) -> Option<&'static StivaleTerminalTag> {
        self.get_tag(0xc2b3f4c3233b0974)
            .map(|addr| unsafe { &*(addr as *const StivaleTerminalTag) })
    }

    pub fn modules(&self) -> Option<&'static StivaleModuleTag> {
        self.get_tag(0x4b6fe466aade04ce)
            .map(|addr| unsafe { &*(addr as *const StivaleModuleTag) })
    }

    pub fn rsdp(&self) -> Option<&'static StivaleRsdpTag> {
        self.get_tag(0x9e1786930a375e78)
            .map(|addr| unsafe { &*(addr as *const StivaleRsdpTag) })
    }

    pub fn smbios(&self) -> Option<&'static StivaleSmbiosTag> {
        self.get_tag(0x274bd246c62bf7d1)
            .map(|addr| unsafe { &*(addr as *const StivaleSmbiosTag) })
    }

    pub fn epoch(&self) -> Option<&'static StivaleEpochTag> {
        self.get_tag(0x566a7bed888e1407)
            .map(|addr| unsafe { &*(addr as *const StivaleEpochTag) })
    }

    pub fn frimware(&self) -> Option<&'static StivaleFirmwareTag> {
        self.get_tag(0x359d837855e3858c)
            .map(|addr| unsafe { &*(addr as *const StivaleFirmwareTag) })
    }

    pub fn efi_system_table(&self) -> Option<&'static StivaleEfiSystemTableTag> {
        self.get_tag(0x4bc5ec15845b558e)
            .map(|addr| unsafe { &*(addr as *const StivaleEfiSystemTableTag) })
    }

    pub fn kernel_file(&self) -> Option<&'static StivaleKernelFileTag> {
        self.get_tag(0xe599d90c2975584a)
            .map(|addr| unsafe { &*(addr as *const StivaleKernelFileTag) })
    }

    pub fn kernel_slide(&self) -> Option<&'static StivaleKernelSlideTag> {
        self.get_tag(0xee80847d01506c57)
            .map(|addr| unsafe { &*(addr as *const StivaleKernelSlideTag) })
    }

    pub fn smp(&self) -> Option<&'static StivaleSmpTag> {
        self.get_tag(0x34d1d96339647025)
            .map(|addr| unsafe { &*(addr as *const StivaleSmpTag) })
    }

    pub fn pxe_info(&self) -> Option<&'static StivalePxeInfoTag> {
        self.get_tag(0x29d1e96239247032)
            .map(|addr| unsafe { &*(addr as *const StivalePxeInfoTag) })
    }

    pub fn uart(&self) -> Option<&'static StivaleUartTag> {
        self.get_tag(0xb813f9b8dbc78797)
            .map(|addr| unsafe { &*(addr as *const StivaleUartTag) })
    }

    pub fn dev_tree(&self) -> Option<&'static StivaleDeviceTreeTag> {
        self.get_tag(0xabb29bd49a2833fa)
            .map(|addr| unsafe { &*(addr as *const StivaleDeviceTreeTag) })
    }

    pub fn vmap(&self) -> Option<&'static StivaleVMap> {
        self.get_tag(0xb0ed257db18cb58f)
            .map(|addr| unsafe { &*(addr as *const StivaleVMap) })
    }
}
