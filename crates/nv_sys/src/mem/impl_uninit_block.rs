use ::core;
use core::{
	clone, //
	default,
	marker,
};

impl<T: marker::Sized> super::AllocatedBlock<T> for super::UninitBlock<T> {
	#[inline]
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self {
		Self {
			raw,
			count,
		}
	}

	#[inline]
	unsafe fn into_raw(self) -> (*mut T, usize) {
		(self.raw, self.count)
	}

	unsafe fn overwrite(&mut self, slot: usize, val: T) {
		unsafe {
			// TODO: We may consider an unchecked version of the overwrite method so that
			// we do not need to pay the price of checking boundaries at every call if it
			// has been guaranteed to be a valid access by the caller.
			core::debug_assert!(slot < self.count);

			self.raw.add(slot).write(val);
		}
	}
}

impl<T: default::Default> super::UninitBlock<T> {
	pub fn clear(mut self) -> super::Block<T> {
		unsafe {
			for slot in 0..self.count {
				super::AllocatedBlock::overwrite(&mut self, slot, T::default());
			}
		}

		super::Block {
			raw: self.raw,
			count: self.count,
		}
	}
}

impl<T: marker::Sized> super::UninitBlock<T> {
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.count
	}

	#[inline]
	pub const unsafe fn assume_init(self) -> super::Block<T> {
		super::Block {
			raw: self.raw,
			count: self.count,
		}
	}
}

impl<T> super::UninitBlock<T>
where
	T: marker::Sized + clone::Clone,
{
	pub fn init(mut self, list: &[T]) -> super::Block<T> {
		core::debug_assert!(list.len() == self.count);

		unsafe {
			for slot in 0..list.len() {
				super::AllocatedBlock::overwrite(&mut self, slot, list[slot].clone());
			}
		}

		super::Block {
			raw: self.raw,
			count: self.count,
		}
	}
}
