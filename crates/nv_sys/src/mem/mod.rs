use ::core;
use core::{
	alloc, //
	any,
	cmp,
	marker,
	mem,
	pin,
	ptr,
	slice,
};

use crate::{
	host, //
	spec,
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

pub use mem::forget;

pub use mem::drop;

pub use ptr::drop_in_place;

pub use mem::size_of;

pub use mem::align_of;

pub use mem::transmute;

pub use ptr::null_mut as null;

pub use mem::MaybeUninit;

pub use mem::ManuallyDrop;

pub use spec::BlockProtection;

pub use spec::BlockSharing;

pub use spec::AllocatedBlock;

pub use spec::Allocator;

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

#[inline]
pub const fn as_bytes<T>(buf: &[T]) -> &[host::Byte] {
	unsafe { slice::from_raw_parts(buf.as_ptr() as *const host::Byte, buf.len() * mem::size_of::<T>()) }
}

#[inline]
pub const fn as_mut_bytes<T>(buf: &mut [T]) -> &mut [host::Byte] {
	unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut host::Byte, buf.len() * mem::size_of::<T>()) }
}

//
// map() and unmap():
//

impl spec::BlockProtection {
	pub const DEFAULT: Self = Self::ReadAndWrite;
}

impl spec::BlockSharing {
	pub const DEFAULT: Self = Self::Public;
}

#[inline]
pub fn map<T: marker::Sized>(count: usize, prot: spec::BlockProtection, vis: spec::BlockSharing) -> spec::Result<UninitBlock<T>> {
	let raw: *mut T = host::memory_map(count, prot, vis)?;

	spec::Result::Ok(UninitBlock {
		raw,
		count,
	})
}

#[inline]
pub fn unmap<T, B>(block: B) -> spec::Result<()>
where
	T: marker::Sized,
	B: spec::AllocatedBlock<T>,
{
	let (raw, count) = unsafe { block.into_raw() };

	host::memory_unmap(raw, count)
}
