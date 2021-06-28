#[repr(C, packed)]
pub struct StivaleTagHeader {
	pub identifier: u64,
	pub next: u64,
}

#[repr(C, packed)]
pub struct StivaleFramebufferTag {
	pub header: StivaleTagHeader,
	pub framebuffer_addr: u64,
	pub framebuffer_width: u16,
	pub framebuffer_height: u16,
	pub framebuffer_pitch: u16,
	pub framebuffer_bpp: u16,
	pub memory_model: u8,
	pub red_mask_size: u8,
	pub red_mask_shift: u8,
	pub green_mask_size: u8,
	pub green_mask_shift: u8,
	pub blue_mask_size: u8,
	pub blue_mask_shift: u8,
}

#[repr(C, packed)]
pub struct StivaleTerminalTag {
	pub header: StivaleTagHeader,
	pub flags: u32,
	pub cols: u16,
	pub rows: u16,
	pub term_write: u64,
}

impl StivaleTerminalTag {
	pub fn term_write(&self, text: &str) {
		let bytes = text.as_bytes();
		let term_write =
			unsafe { core::mem::transmute::<_, extern "C" fn(*const u8, usize)>(self.term_write) };

		term_write(bytes.as_ptr(), bytes.len());
	}
}
