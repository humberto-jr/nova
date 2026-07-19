use ::core::slice;

use crate::{
	bit, //
	ffi::unix::wayland::client::xdg_shell_protocol as ffi,
	mem,
	spec,
};

pub const SHARED_MEMORY_FILENAME: &str = "/tmp/wayland-shm\0";

//
// WindowState:
//

pub type WindowState = bit::Mask16;

pub const WINDOW_STATE_ACTIVATED_BIT: u8 = 0;
pub const WINDOW_STATE_MAXIMIZED_BIT: u8 = 1;
pub const WINDOW_STATE_FULL_SCREEN_BIT: u8 = 2;
pub const WINDOW_STATE_RESIZING_BIT: u8 = 3;
pub const WINDOW_STATE_TILED_BIT: u8 = 4;
pub const WINDOW_STATE_SHOULD_CLOSE_BIT: u8 = 5;

//
// Extent:
//

#[derive(Debug, Clone, PartialEq)]
pub struct Extent {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

impl Extent {
	#[inline]
	pub const fn zeroed() -> Self {
		Self {
			x: 0,
			y: 0,
			width: 0,
			height: 0,
		}
	}

	#[inline]
	pub const fn size(&self) -> usize {
		(self.width as usize) * (self.height as usize)
	}
}

//
// Helpers:
//

pub fn wl_array_slice<'a, T>(array: *const ffi::wl_array) -> &'a [T] {
	unsafe {
		let array = &(*array);

		let count = array.size / mem::size_of::<T>();

		slice::from_raw_parts(array.data as *const T, count)
	}
}

pub fn allocate_shared_memory(len: usize) -> crate::File {
	let mut file = crate::File::new();

	let _ = file.open(SHARED_MEMORY_FILENAME, spec::FileAccess::Temporary);

	if file.is_open() {
		let _ = file.resize(len);

		file
	} else {
		crate::panic!("Failed to open \"{SHARED_MEMORY_FILENAME}\"");
	}
}

pub fn draw_test_pattern(file: &crate::File, extent: &Extent) {
	let size = extent.size() * super::FORMAT_CHANNEL_COUNT;

	let result = file.map(0, size, spec::BlockProtection::ReadAndWrite, spec::BlockSharing::Public);

	let block = if let spec::Result::Ok(block) = result {
		block
	} else {
		return;
	};

	let raw_buf = block.as_mut_ptr() as *mut u32;

	for y in 0..extent.height {
		for x in 0..extent.width {
			let r = (x * 2) as u32;
			let g = (y * 2) as u32;
			let b = 0x80;

			let offset = ((y * extent.width) + x) as usize;

			unsafe {
				(*raw_buf.add(offset)) = 0xFF000000 | (r << 16) | (g << 8) | b;
			}
		}
	}

	let _ = file.unmap(block);
}
