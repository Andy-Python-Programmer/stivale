use super::StivaleStruct;

macro_rules! make_header_tag {
	($name:ident, $id:expr) => {
		make_header_tag!($name, $id, {});
	};
	($name:ident, $id:expr, {$($field_name:ident : $field_ty:ty = $field_default:expr),*}) => {
		#[allow(dead_code)]
		pub struct $name {
			identifier: u64,
			next: *const (),
			$($field_name: $field_ty),*
		}

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl $name {
			pub const fn new() -> Self {
				Self {
					identifier: $id,
					next: core::ptr::null(),
					$($field_name: $field_default),*
				}
			}

			pub const fn next(mut self, next: *const ()) -> Self {
				self.next = next;
				self
			}

			$(pub const fn $field_name(mut self, value: $field_ty) -> Self {
				self.$field_name = value;
				self
			})*
		}
	};
}

union StivaleHeaderEntryPoint {
    func: extern "C" fn(&'static StivaleStruct) -> !,
    zero: u64,
}

#[repr(C, packed)]
pub struct StivaleHeader {
    entry_point: StivaleHeaderEntryPoint,
    stack: *const u8,
    flags: u64,
    tags: *const (),
}

impl StivaleHeader {
    pub const fn new() -> Self {
        Self {
            entry_point: StivaleHeaderEntryPoint { zero: 0 },
            stack: core::ptr::null(),
            flags: 0,
            tags: core::ptr::null(),
        }
    }

    pub const fn entry_point(mut self, func: extern "C" fn(&'static StivaleStruct) -> !) -> Self {
        self.entry_point = StivaleHeaderEntryPoint { func };
        self
    }

    pub const fn stack(mut self, stack: *const u8) -> Self {
        self.stack = stack;
        self
    }

    pub const fn flags(mut self, flags: u64) -> Self {
        self.flags = flags;
        self
    }

    pub const fn tags(mut self, tags: *const ()) -> Self {
        self.tags = tags;
        self
    }
}

make_header_tag!(StivaleFramebufferHeaderTag, 0x3ecc1bc43d0f7971, {
    framebuffer_width: u16 = 0,
    framebuffer_height: u16 = 0,
    framebuffer_bpp: u16 = 0
});

make_header_tag!(StivaleTerminalHeaderTag, 0xa85d499b1823be72, {
    flags: u64 = 0
});

unsafe impl Send for StivaleHeader {}
unsafe impl Sync for StivaleHeader {}
