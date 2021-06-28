#![feature(const_fn_fn_ptr_basics)]
#![no_std]

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
}
