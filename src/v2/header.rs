use super::StivaleStruct;

macro_rules! make_header_tag {
	($(#[$meta:meta])* struct $name:ident: $id:expr;) => {
		make_header_tag!($(#[$meta])* struct $name: $id => {};);
	};

	($(#[$meta:meta])* struct $name:ident: $id:expr => {$($(#[$field_meta:meta])* $field_name:ident : $field_ty:ty = $field_default:expr),*};) => {
        $(#[$meta])*
        #[repr(C, packed)]
        pub struct $name {
			identifier: u64,
			next: *const (),
			$($field_name: $field_ty),*
		}

        #[allow(deprecated)] unsafe impl Send for $name {}
        #[allow(deprecated)] unsafe impl Sync for $name {}

        #[allow(deprecated)] impl $name {
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

			$($(#[$field_meta])* pub const fn $field_name(mut self, value: $field_ty) -> Self {
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

    /// Returns the stack pointer placed in this header.
    pub fn get_stack(&self) -> *const u8 {
        self.stack
    }

    /// Returns the flags stored in this header.
    pub fn get_flags(&self) -> u64 {
        self.flags
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

make_header_tag!(
    /// If this tag is present the bootloader is instructed to initialise a graphical
    /// framebuffer video mode. Omitting this tag will make the bootloader default to a
    /// CGA-compatible text mode, if supported.
    struct StivaleFramebufferHeaderTag: 0x3ecc1bc43d0f7971 => {
        framebuffer_width: u16 = 0,
        framebuffer_height: u16 = 0,
        framebuffer_bpp: u16 = 0,
        _padding: u16 = 0
    };
);

make_header_tag!(
    /// If this tag is present the bootloader is instructed to set up a terminal
    /// for use by the kernel at runtime. See "Terminal struct tag" below. The framebuffer
    /// header tag **must** be specified when passing this header tag, and this tag may inhibit
    /// the WC MTRR framebuffer feature.
    struct StivaleTerminalHeaderTag: 0xa85d499b1823be72 => {
        flags: u64 = 0
    };
);

bitflags::bitflags! {
    /// Bitfield representing the SMP header flags passed to the bootloader.
    pub struct StivaleSmpHeaderTagFlags: u64 {
        /// Instruct the bootloader to use XAPIC.
        const XAPIC = 0;
        /// Instruct the bootloader to use X2APIC, if avaliable.
        const X2APIC = 1;
    }
}

make_header_tag!(
    struct StivaleSmpHeaderTag: 0x1ab015085f3273df => {
        flags: StivaleSmpHeaderTagFlags = StivaleSmpHeaderTagFlags::XAPIC
    };
);

make_header_tag!(
    /// This tag tells the bootloader to, in case a framebuffer was requested, make that framebuffer's
    /// caching type write-combining using x86's MTRR model specific registers. This caching type helps speed
    /// up framebuffer writes on real hardware.
    ///
    /// ## Legacy
    /// This tag is deprecated and considered legacy. Use is discouraged and it may not be supported on newer bootloaders.
    #[deprecated(note = "This tag is deprecated and considered legacy. Use is discouraged and it may not be supported on newer bootloaders.")]
    struct StivaleMtrrHeaderTag: 0x4c7bb07731282e00;
);

make_header_tag!(
    /// If this tag is present the bootloader is instructed to enable upport for 5-level paging, if
    /// available.
    struct Stivale5LevelPagingHeaderTag: 0x932f477032007e8f;
);

make_header_tag!(
    /// If this tag is present the bootloader is instructed to unmap the first page of the virtual address
    /// space before passing control to the kernel, for architectures that support paging.
    struct StivaleUnmapNullHeaderTag: 0x92919432b16fe7e7;
);

make_header_tag!(
    /// This tag tells the bootloader that the kernel has no requirement for a framebuffer
    /// to be initialised. Omitting both the any video header tag and the framebuffer header
    /// tag means "force CGA text mode" (where available), and the bootloader will refuse to
    /// boot the kernel if it fails to fulfill that request.
    struct StivaleAnyVideoTag: 0xc75c9fa92a44c4db => {
        /// 0: prefer linear framebuffer
        ///
        /// 1: prefer no linear framebuffer
        ///    (CGA text mode if available)
        ///
        /// All other values undefined.
        preference: u64 = 0
    };
);

unsafe impl Send for StivaleHeader {}
unsafe impl Sync for StivaleHeader {}
