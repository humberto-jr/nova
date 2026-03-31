use ::core::{
	marker, //
	ops,
};

impl<U: ?marker::Sized> super::AllocatedBlock<()> for super::UnsizedBlock<U> {
	#[inline(always)]
	unsafe fn from_raw(_: *mut (), _: usize) -> Self {
		crate::panic!("Attempt to create an UnsizedBlock using the AllocatedBlock::from_raw() method")
	}

	#[inline(always)]
	unsafe fn into_raw(self) -> (*mut (), usize) {
		// NOTE: This is important because the deallocate() method of the owner allocator
		// needs to access the original pointer to a type T (raw) before its coercion to
		// an unsized type U (fat).
		(self.raw, self.count)
	}

	#[inline(always)]
	unsafe fn overwrite(&mut self, _: usize, _: ()) {
		crate::panic!("Attempt to write an UnsizedBlock using the AllocatedBlock::overwrite() method")
	}
}

impl<U: ?marker::Sized> ops::Deref for super::UnsizedBlock<U> {
	type Target = U;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe { &(*self.fat) }
	}
}

impl<U: ?marker::Sized> ops::DerefMut for super::UnsizedBlock<U> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut (*self.fat) }
	}
}

impl<U: ?marker::Sized> super::UnsizedBlock<U> {
	#[inline(always)]
	pub const unsafe fn from_raw<T: marker::Sized>(raw: *mut T, fat: *mut U, count: usize) -> Self {
		// SAFETY: Types U and T must have compatible memory layouts,
		// and T must have implemented U if the latter is a trait. If
		// U is [U], count must accurately represent the length.
		Self {
			raw: raw as *mut (),
			fat,
			count,
		}
	}

	#[inline(always)]
	pub fn as_mut_pin(&mut self) -> super::Pin<&mut U> {
		unsafe { super::Pin::new_unchecked(&mut (*self.fat)) }
	}
}
