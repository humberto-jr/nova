use ::core;
use core::{
	alloc, //
	any,
	clone,
	cmp,
	marker,
	mem,
	pin,
	ptr,
};

use crate::{
	ffi::libc, //
	sync,
};

//
// Useful re-exports:
//

pub use pin::Pin;

pub use mem::zeroed;

pub use mem::replace;

pub use mem::take;

pub use mem::swap;

pub use mem::size_of;

pub use mem::align_of;

pub use mem::transmute;

pub use ptr::null_mut as null;

pub use mem::MaybeUninit;

//
// AllocatedBlock<T>:
//

pub trait AllocatedBlock<T: ?marker::Sized> {
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self;

	unsafe fn into_raw(self) -> (*mut T, usize);

	unsafe fn overwrite(&mut self, slot: usize, val: T);
}

//
// AllocatorResult<T>:
//

mod impl_allocator_result;

pub enum AllocatorResult<T> {
	None,

	Allocated(UninitBlock<T>),

	Deallocated,

	OutOfMemory,

	SizeLimitExceeded,

	UnsupportedAlignment,

	AllocatorUnavailable,
}

//
// Allocator:
//

pub trait Allocator: clone::Clone {
	fn allocate<T: marker::Sized>(&mut self, count: usize) -> AllocatorResult<T>;

	fn reallocate<T: marker::Sized>(&mut self, count: usize, block: impl AllocatedBlock<T>) -> AllocatorResult<T>;

	fn deallocate<T: marker::Sized>(&mut self, block: impl AllocatedBlock<T>) -> AllocatorResult<()>;
}

//
// UninitBlock<T>:
//

mod impl_uninit_block;

#[derive(PartialEq)]
pub struct UninitBlock<T: marker::Sized> {
	raw: *mut T,
	count: usize,
}

//
// Block<T>:
//

mod impl_block;

#[derive(PartialEq)]
pub struct Block<T: marker::Sized> {
	raw: *mut T,
	count: usize,
}

//
// SharedBlock<T>:
//

mod impl_shared_block;

pub struct SharedBlock<T: marker::Sized> {
	raw: sync::SpinLock<*mut T>,
	count: usize,
}

pub struct ReadOnlyBlock<'b, T: marker::Sized> {
	raw: sync::ReadOnlyLock<'b, *mut T>,
	count: usize,
}

pub struct ReadAndWriteBlock<'b, T: marker::Sized> {
	raw: sync::ReadAndWriteLock<'b, *mut T>,
	count: usize,
}

//
// OpaqueBlock:
//

mod impl_opaque_block;

#[derive(PartialEq)]
pub struct OpaqueBlock {
	block: Block<()>,
	id: any::TypeId,
}

//
// UnsizedBlock<U>:
//

mod impl_unsized_block;

pub struct UnsizedBlock<U: ?marker::Sized> {
	raw: *mut (),
	fat: *mut U,
	count: usize,
}

#[macro_export]
macro_rules! coerce_unsized_block {
	($unsized_type:ty, $block:expr) => {
		unsafe {
			use ::core::alloc::boxed::Box;
			use nv_sys::mem::AllocatedBlock;
			use nv_sys::mem::UnsizedBlock;

			let (raw, count) = $block.into_raw();
			let boxed: Box<$unsized_type> = Box::from_raw(raw);

			UnsizedBlock::<$unsized_type>::from_raw(raw, Box::leak(boxed), count)
		}
	};
}

//
// Low-level utils:
//

#[inline]
pub const fn layout<T>() -> (usize, usize) {
	let layout = alloc::Layout::new::<T>();

	(layout.size(), layout.align())
}

#[inline]
pub const fn posix_min_alignment() -> usize {
	alloc::Layout::new::<*const ()>().align()
}

#[inline]
pub fn posix_adjusted_alignment<T>() -> usize {
	let (_, align) = layout::<T>();

	cmp::max(align, posix_min_alignment())
}

#[inline]
pub fn is_aligned<T>(raw: *const T) -> bool {
	if raw.is_null() {
		false
	} else {
		let (_, align) = layout::<T>();

		((raw as usize) % align) == 0
	}
}

#[inline]
pub const fn align_up(addr: usize, align: usize) -> usize {
	core::debug_assert!(align.is_power_of_two());

	(addr + (align - 1)) & !(align - 1)
}

#[inline]
pub const unsafe fn take_ownership<T>(val: &mut T) -> T {
	unsafe { replace(val, uninit()) }
}

#[inline]
pub const unsafe fn uninit<T>() -> T {
	unsafe { mem::MaybeUninit::uninit().assume_init() }
}

#[inline]
pub const fn uninit_array<T, const MAX_LEN: usize>() -> [mem::MaybeUninit<T>; MAX_LEN] {
	[const { mem::MaybeUninit::uninit() }; MAX_LEN]
}

pub fn aligned_malloc<T: marker::Sized>(count: usize) -> AllocatorResult<T> {
	if count == 0 {
		return AllocatorResult::None;
	}

	let mut raw: *mut libc::c_void = null();

	let info = unsafe {
		let (size, _) = layout::<T>();

		let align = posix_adjusted_alignment::<T>();

		libc::posix_memalign(&mut raw, align, size * count)
	};

	match info {
		libc::EINVAL => AllocatorResult::UnsupportedAlignment,

		libc::ENOMEM => AllocatorResult::OutOfMemory,

		_ => {
			let raw = raw as *mut T;

			AllocatorResult::Allocated(UninitBlock {
				raw,
				count,
			})
		},
	}
}

pub fn aligned_realloc<T, B>(new_count: usize, old_block: B) -> AllocatorResult<T>
where
	T: marker::Sized,
	B: AllocatedBlock<T>,
{
	let result = aligned_malloc::<T>(new_count);

	let new_block = if let AllocatorResult::Allocated(block) = result {
		block
	} else {
		let _ = free(old_block);

		return result;
	};

	unsafe {
		let (old_block_raw, old_block_count) = old_block.into_raw();

		let max_len = cmp::min(old_block_count, new_block.count);

		let new_buf = new_block.raw as *mut libc::c_void;

		let old_buf = old_block_raw as *mut libc::c_void;

		let (size, _) = layout::<T>();

		let _ = libc::memcpy(new_buf, old_buf, max_len * size);

		libc::free(old_buf);
	}

	AllocatorResult::Allocated(new_block)
}

pub fn free<T, B>(block: B) -> AllocatorResult<()>
where
	T: ?marker::Sized,
	B: AllocatedBlock<T>,
{
	unsafe {
		let (raw, _) = block.into_raw();

		libc::free(raw as *mut libc::c_void);

		AllocatorResult::Deallocated
	}
}
