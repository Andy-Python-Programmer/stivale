#![no_std]
#![feature(const_fn_fn_ptr_basics)]
#![warn(clippy::all)]

//! A crate for parsing qloader2 and tomatboot's stivale2 structures
//!
//! The `header` module contains a header struct and tags for letting a bootloader know
//! that a kernel is stivale2 compliant
//!
//! `StivaleStructure` is loaded with `load`, with an address that's passed in RDI on x86_64
//! (the first function parameter on an `extern "C" fn`)

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This crate only supports the x86_64 architecture");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("This crate only supports 64-bit architectures");

#[macro_use]
extern crate bitflags;

pub mod header;
pub use header::*;

pub mod epoch;
pub mod firmware;
pub mod framebuffer;
pub mod rsdp;
pub mod terminal;

use epoch::EpochTag;
use firmware::FirmwareTag;
use framebuffer::FramebufferTag;
use rsdp::RSDPTag;

pub mod memory;
pub mod module;

use memory::MemoryMapTag;
use module::ModuleTag;
use terminal::TerminalTag;

pub(crate) fn string_from_u8(data: &[u8]) -> Option<&str> {
    use core::{slice, str};
    if data[0] == 0 {
        None
    } else {
        let mut strlen = 0;
        while strlen < data.len() && data[strlen] != 0 {
            strlen += 1;
        }

        unsafe {
            Some(str::from_utf8_unchecked(slice::from_raw_parts(
                (&data[0]) as *const u8,
                strlen,
            )))
        }
    }
}

/// Load the stivale2 structure from an address
///
/// The structure pointer is passed in the EDI register
///
/// # Safety
/// This function will cause undefined behavior when a non-stivale2 compliant
/// bootloader boots the kernel
///
/// This can be avoided by creating a custom entry point that's not the
/// ELF entry point, and setting that in the stivale2 header
///
/// # Examples
///
/// ```ignore
/// let mut stivale_struct_ptr: u64 = 0;
/// unsafe { asm!("mov $2, %rdi" : "=r"(stivale_struct_ptr)) };
/// let stivale_struct = unsafe { stivale::load(stivale_struct_ptr as usize) };
/// ```
///
/// ```ignore
/// fn kernel_main(stivale_struct_ptr: usize) {
///     let stivale_struct = unsafe { stivale::load(stivale_struct_ptr) };
/// }
/// ```
pub unsafe fn load(address: usize) -> StivaleStructure {
    let inner = &*(address as *const StivaleStructureInner);
    StivaleStructure { inner }
}

/// The stivale2 structure containing all the tags passed by the bootloader
pub struct StivaleStructure {
    inner: *const StivaleStructureInner,
}

#[repr(C, packed)]
pub struct StivaleStructureInner {
    bootloader_brand: [u8; 64],
    bootloader_version: [u8; 64],
    tags: u64,
}

impl StivaleStructure {
    fn inner(&self) -> &StivaleStructureInner {
        unsafe { &*self.inner }
    }

    fn get_tag(&self, identifier: u64) -> Option<u64> {
        let mut next: *const EmptyStivaleTag = self.inner().tags as *const EmptyStivaleTag;
        while !next.is_null() {
            let tag = unsafe { &*next };
            if tag.identifier == identifier {
                return Some(next as u64);
            }
            next = tag.next as *const EmptyStivaleTag;
        }
        None
    }

    /// Get the bootloader brand that booted the kernel, if any
    pub fn bootloader_brand(&self) -> Option<&str> {
        string_from_u8(&self.inner().bootloader_brand)
    }

    /// Get the bootloader version, if any
    pub fn bootloader_version(&self) -> Option<&str> {
        string_from_u8(&self.inner().bootloader_version)
    }

    /// Get the video framebuffer info tag
    pub fn framebuffer(&self) -> Option<&'static FramebufferTag> {
        self.get_tag(0x506461d2950408fa)
            .map(|tag| unsafe { &*(tag as *const FramebufferTag) })
    }

    /// Get the ACPI RSDP structure pointer
    pub fn rsdp(&self) -> Option<&'static RSDPTag> {
        self.get_tag(0x9e1786930a375e78)
            .map(|tag| unsafe { &*(tag as *const RSDPTag) })
    }

    /// Get the current UNIX epoch during boot
    pub fn epoch(&self) -> Option<&'static EpochTag> {
        self.get_tag(0x566a7bed888e1407)
            .map(|tag| unsafe { &*(tag as *const EpochTag) })
    }

    /// Get the firmware tag passed by the bootloader
    pub fn firmware(&self) -> Option<&'static FirmwareTag> {
        self.get_tag(0x359d837855e3858c)
            .map(|tag| unsafe { &*(tag as *const FirmwareTag) })
    }

    /// Get the memory map tag
    pub fn memory_map(&self) -> Option<&'static MemoryMapTag> {
        self.get_tag(0x2187f79e8612de07)
            .map(|tag| unsafe { &*(tag as *const MemoryMapTag) })
    }

    /// Get the terminal tag
    pub fn terminal(&self) -> Option<&'static TerminalTag> {
        self.get_tag(0xc2b3f4c3233b0974)
            .map(|tag| unsafe { &*(tag as *const TerminalTag) })
    }

    /// Get the module tag
    pub fn module(&self) -> Option<&'static ModuleTag> {
        self.get_tag(0x4b6fe466aade04ce)
            .map(|tag| unsafe { &*(tag as *const ModuleTag) })
    }
}

struct EmptyStivaleTag {
    identifier: u64,
    next: u64,
}
