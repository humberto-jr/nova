use ::core;
use core::{
	default, //
	fmt,
	marker,
	ops,
	slice,
};

//
// SharedBlock<T>:
//

impl<T: marker::Sized> super::AllocatedBlock<T> for super::SharedBlock<T> {
	#[inline]
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self {
		unsafe { super::Block::from_raw(raw, count).into_shared() }
	}

	#[inline]
	unsafe fn into_raw(self) -> (*mut T, usize) {
		(self.raw.release(), self.count)
	}

	unsafe fn overwrite(&mut self, slot: usize, val: T) {
		core::debug_assert!(slot < self.count);

		unsafe {
			self.raw.exclusive_write().add(slot).write(val);
		}
	}
}

impl<T: marker::Sized> super::SharedBlock<T> {
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.count
	}
}

impl<T> super::SharedBlock<T> {
	#[inline]
	pub fn shared_read<'b>(&'b self) -> super::ReadOnlyBlock<'b, T> {
		super::ReadOnlyBlock {
			raw: self.raw.shared_read(),
			count: self.count,
		}
	}

	#[inline]
	pub fn exclusive_write<'b>(&'b self) -> super::ReadAndWriteBlock<'b, T> {
		super::ReadAndWriteBlock {
			raw: self.raw.exclusive_write(),
			count: self.count,
		}
	}

	#[inline]
	pub fn into_block(self) -> super::Block<T> {
		super::Block {
			raw: self.raw.release(),
			count: self.count,
		}
	}
}

//
// ReadOnlyBlock<'b, T>:
//

impl<'b, T: marker::Sized> ops::Deref for super::ReadOnlyBlock<'b, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { &(**self.raw) }
	}
}

impl<'b, T: fmt::Display> fmt::Display for super::ReadOnlyBlock<'b, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { core::write!(f, "{}", **self.raw) }
	}
}

impl<'b, T: marker::Sized> super::ReadOnlyBlock<'b, T> {
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.count
	}

	#[inline]
	pub fn as_slice(&'b self) -> &'b [T] {
		unsafe { slice::from_raw_parts::<'b, T>(*self.raw, self.count) }
	}
}

//
// ReadAndWriteBlock<'b, T>:
//

impl<'b, T: marker::Sized> ops::Deref for super::ReadAndWriteBlock<'b, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		unsafe { &(**self.raw) }
	}
}

impl<'b, T: marker::Sized> ops::DerefMut for super::ReadAndWriteBlock<'b, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut (**self.raw) }
	}
}

impl<'b, T: fmt::Display> fmt::Display for super::ReadAndWriteBlock<'b, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe { core::write!(f, "{}", **self.raw) }
	}
}

impl<'b, T: default::Default> super::ReadAndWriteBlock<'b, T> {
	pub fn clear(&mut self) {
		for entry in self.as_mut_slice() {
			*entry = T::default();
		}
	}
}

impl<'b, T: marker::Sized> super::ReadAndWriteBlock<'b, T> {
	#[inline]
	pub const fn capacity(&self) -> usize {
		self.count
	}

	#[inline]
	pub fn as_slice(&'b self) -> &'b [T] {
		unsafe { slice::from_raw_parts::<'b, T>(*self.raw, self.count) }
	}

	#[inline]
	pub fn as_mut_slice(&'b self) -> &'b mut [T] {
		unsafe { slice::from_raw_parts_mut::<'b, T>(*self.raw, self.count) }
	}
}
