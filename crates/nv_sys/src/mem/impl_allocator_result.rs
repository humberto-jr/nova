use ::core;
use core::{
	any, //
	clone,
	fmt,
	marker,
};

impl<T> fmt::Display for super::AllocatorResult<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let typename = any::type_name::<T>();

		match self {
			Self::None => core::write!(f, "AllocatorResult::None"),

			Self::Allocated(_) => core::write!(f, "AllocatorResult::Allocated({typename})"),

			Self::Deallocated => core::write!(f, "AllocatorResult::Deallocated"),

			Self::OutOfMemory => core::write!(f, "AllocatorResult::OutOfMemory"),

			Self::SizeLimitExceeded => core::write!(f, "AllocatorResult::SizeLimitExceeded"),

			Self::UnsupportedAlignment => core::write!(f, "AllocatorResult::UnsupportedAlignment"),

			Self::AllocatorUnavailable => core::write!(f, "AllocatorResult::AllocatorUnavailable"),
		}
	}
}

impl<T: marker::Sized> super::AllocatorResult<T> {
	#[inline]
	pub const fn none() -> Self {
		Self::None
	}

	#[inline]
	pub const fn is_allocated(&self) -> bool {
		core::matches!(self, Self::Allocated(_))
	}

	pub fn unwrap(&mut self) -> super::UninitBlock<T> {
		if let Self::Allocated(block) = super::replace(self, Self::None) {
			block
		} else {
			core::panic!("Unwrapping {self}")
		}
	}
}

impl<T> super::AllocatorResult<T>
where
	T: marker::Sized + clone::Clone,
{
	pub fn init_and_unwrap(&mut self, list: &[T]) -> super::Block<T> {
		if let Self::Allocated(block) = super::replace(self, Self::None) {
			block.init(list)
		} else {
			core::panic!("Unwrapping {self}")
		}
	}
}
