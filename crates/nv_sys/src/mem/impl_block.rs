use ::core;
use core::{
	any, //
	default,
	fmt,
	marker,
	ops,
	slice,
};

use crate::sync;

impl<T: marker::Sized> super::AllocatedBlock<T> for super::Block<T> {
	#[inline]
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self {
		unsafe { super::UninitBlock::from_raw(raw, count).assume_init() }
	}

	#[inline]
	unsafe fn into_raw(self) -> (*mut T, usize) {
		(self.raw, self.count)
	}

	unsafe fn overwrite(&mut self, slot: usize, val: T) {
		core::debug_assert!(slot < self.count);

		unsafe {
			self.raw.add(slot).write(val);
		}
	}
}

macro_rules! impl_index {
	($index_type:ty) => {
		impl<T: marker::Sized> ops::Index<$index_type> for super::Block<T> {
			type Output = T;

			#[inline]
			fn index(&self, index: $index_type) -> &Self::Output {
				core::debug_assert!((index as usize) < self.count);

				unsafe { &(*self.raw.add(index as usize)) }
			}
		}

		impl<T: marker::Sized> ops::IndexMut<$index_type> for super::Block<T> {
			#[inline]
			fn index_mut(&mut self, index: $index_type) -> &mut Self::Output {
				core::debug_assert!((index as usize) < self.count);

				unsafe { &mut (*self.raw.add(index as usize)) }
			}
		}

		impl<T: marker::Sized> ops::Index<ops::Range<$index_type>> for super::Block<T> {
			type Output = [T];

			#[inline]
			fn index(&self, range: ops::Range<$index_type>) -> &Self::Output {
				let usize_range = (range.start as usize)..(range.end as usize);

				&self.as_slice()[usize_range]
			}
		}

		impl<T: marker::Sized> ops::IndexMut<ops::Range<$index_type>> for super::Block<T> {
			#[inline]
			fn index_mut(&mut self, range: ops::Range<$index_type>) -> &mut Self::Output {
				let usize_range = (range.start as usize)..(range.end as usize);

				&mut self.as_mut_slice()[usize_range]
			}
		}
	};
}

impl_index!(u8);
impl_index!(u16);
impl_index!(u32);
impl_index!(u64);
impl_index!(usize);

impl<T: marker::Sized> ops::Deref for super::Block<T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { &(*self.raw) }
	}
}

impl<T: marker::Sized> ops::DerefMut for super::Block<T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut (*self.raw) }
	}
}

impl<T: fmt::Display> fmt::Display for super::Block<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { core::write!(f, "{}", *self.raw) }
	}
}

impl<T: default::Default> super::Block<T> {
	pub fn clear(&self) {
		for entry in self.as_mut_slice() {
			*entry = T::default();
		}
	}
}

impl<T: marker::Sized> super::Block<T> {
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.count
	}

	#[inline]
	pub const fn as_slice<'b>(&'b self) -> &'b [T] {
		unsafe { slice::from_raw_parts::<'b, T>(self.raw, self.count) }
	}

	#[inline]
	pub const fn as_mut_slice<'b>(&'b self) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut::<'b, T>(self.raw, self.count) }
	}

	#[inline]
	pub const fn into_shared(self) -> super::SharedBlock<T> {
		super::SharedBlock {
			raw: sync::SpinLock::new(self.raw),
			count: self.count,
		}
	}
}

impl<T> super::Block<T>
where
	T: marker::Sized + 'static,
{
	#[inline]
	pub fn into_opaque(self) -> super::OpaqueBlock {
		unsafe {
			super::OpaqueBlock {
				block: super::AllocatedBlock::from_raw(self.raw as *mut (), self.count),
				id: any::TypeId::of::<T>(),
			}
		}
	}
}
