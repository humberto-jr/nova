use ::core;
use core::{
	any, //
	marker,
	slice,
};

use crate::sync;

// NOTE: Since the OpaqueBlock does not constrain a concrete
// type in any form, it cannot implement Deref and DerefMut.

impl<T> super::AllocatedBlock<T> for super::OpaqueBlock
where
	T: marker::Sized + 'static,
{
	#[inline(always)]
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self {
		unsafe { super::Block::from_raw(raw, count).into_opaque() }
	}

	#[inline(always)]
	unsafe fn into_raw(self) -> (*mut T, usize) {
		(self.block.raw as *mut T, self.block.count)
	}

	#[inline(always)]
	unsafe fn overwrite(&mut self, slot: usize, val: T) {
		core::debug_assert!(slot < self.block.count);

		unsafe {
			(self.block.raw as *mut T).add(slot).write(val);
		}
	}
}

impl super::OpaqueBlock {
	#[inline(always)]
	pub const fn capacity(&self) -> usize {
		self.block.count
	}

	#[inline(always)]
	pub const fn type_id(&self) -> any::TypeId {
		self.id
	}

	#[inline(always)]
	pub const unsafe fn as_slice_unchecked<'b, T>(&'b self) -> &'b [T] {
		unsafe { slice::from_raw_parts::<'b, T>(self.block.raw as *const T, self.block.count) }
	}

	#[inline(always)]
	pub const unsafe fn as_mut_slice_unchecked<'b, T>(&'b self) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut::<'b, T>(self.block.raw as *mut T, self.block.count) }
	}

	#[inline(always)]
	pub const unsafe fn into_block_unchecked<T>(self) -> super::Block<T> {
		super::Block {
			raw: self.block.raw as *mut T,
			count: self.block.count,
		}
	}

	#[inline(always)]
	pub const unsafe fn into_shared_unchecked<T>(self) -> super::SharedBlock<T> {
		super::SharedBlock {
			raw: sync::SpinLock::new(self.block.raw as *mut T),
			count: self.block.count,
		}
	}

	#[inline(always)]
	pub fn as_slice<'b, T: 'static>(&'b self) -> &'b [T] {
		core::debug_assert!(any::TypeId::of::<T>() == self.id);

		unsafe { self.as_slice_unchecked() }
	}

	#[inline(always)]
	pub fn as_mut_slice<'b, T: 'static>(&'b self) -> &'b mut [T] {
		core::debug_assert!(any::TypeId::of::<T>() == self.id);

		unsafe { self.as_mut_slice_unchecked() }
	}

	#[inline(always)]
	pub fn into_block<T: 'static>(self) -> super::Block<T> {
		core::debug_assert!(any::TypeId::of::<T>() == self.id);

		unsafe { self.into_block_unchecked() }
	}

	#[inline(always)]
	pub fn into_shared<T: 'static>(self) -> super::SharedBlock<T> {
		core::debug_assert!(any::TypeId::of::<T>() == self.id);

		unsafe { self.into_shared_unchecked() }
	}
}
