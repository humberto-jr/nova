use ::core;
use core::{
	cell, //
	cmp,
	fmt,
	hint,
	marker,
	ops,
	sync::atomic,
};

use crate::mem;

const WRITER_BIT: u32 = 1 << 31;

//
// SpinLock<T>:
//

pub struct SpinLock<T> {
	state: atomic::AtomicU32,
	data: cell::UnsafeCell<T>,
}

unsafe impl<T: marker::Send> marker::Send for SpinLock<T> {}

unsafe impl<T: marker::Send + marker::Sync> marker::Sync for SpinLock<T> {}

impl<'s, T> SpinLock<T> {
	pub const fn new(value: T) -> Self {
		Self {
			state: atomic::AtomicU32::new(0),
			data: cell::UnsafeCell::new(value),
		}
	}

	#[inline]
	pub fn reader_count(&self) -> u32 {
		self.state.load(atomic::Ordering::Relaxed) & !WRITER_BIT
	}

	#[inline]
	pub fn has_active_writer(&self) -> bool {
		(self.state.load(atomic::Ordering::Relaxed) & WRITER_BIT) != 0
	}

	pub fn shared_read(&'s self) -> ReadOnlyLock<'s, T> {
		loop {
			let state = self.state.load(atomic::Ordering::Acquire);

			if (state & WRITER_BIT) == 0 {
				if self.state.compare_exchange_weak(state, state + 1, atomic::Ordering::AcqRel, atomic::Ordering::Relaxed).is_ok() {
					return ReadOnlyLock {
						lock: self,
					};
				}
			}

			hint::spin_loop();
		}
	}

	pub fn exclusive_write(&'s self) -> ReadAndWriteLock<'s, T> {
		loop {
			if self.state.compare_exchange_weak(0, WRITER_BIT, atomic::Ordering::AcqRel, atomic::Ordering::Relaxed).is_ok() {
				return ReadAndWriteLock {
					lock: self,
				};
			}

			hint::spin_loop();
		}
	}

	pub fn release(mut self) -> T {
		let _ = self.exclusive_write();

		unsafe {
			// SAFETY: The cell is replaced with an uninit copy, but
			// it won't be used and self will immediately drop.
			mem::take_ownership(&mut self.data).into_inner()
		}
	}

	#[inline]
	const fn unwrap(&self) -> &mut T {
		unsafe { &mut *self.data.get() }
	}
}

//
// ReadOnlyLock<'s, T>:
//

pub struct ReadOnlyLock<'s, T> {
	lock: &'s SpinLock<T>,
}

impl<'s, T> ops::Deref for ReadOnlyLock<'s, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.lock.unwrap()
	}
}

impl<'s, T> ops::Drop for ReadOnlyLock<'s, T> {
	#[inline]
	fn drop(&mut self) {
		self.lock.state.fetch_sub(1, atomic::Ordering::Release);
	}
}

impl<'s, T: fmt::Display> fmt::Display for ReadOnlyLock<'s, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		core::write!(f, "{}", **self)
	}
}

impl<'lhs, 'rhs, T: cmp::PartialEq> cmp::PartialEq<ReadOnlyLock<'rhs, T>> for ReadOnlyLock<'lhs, T> {
	#[inline]
	fn eq(&self, other: &ReadOnlyLock<'rhs, T>) -> bool {
		(*self.lock.unwrap()) == (*other.lock.unwrap())
	}

	#[inline]
	fn ne(&self, other: &ReadOnlyLock<'rhs, T>) -> bool {
		!self.eq(other)
	}
}

impl<'s, T> ReadOnlyLock<'s, T> {
	#[inline]
	pub fn unlock(self) {}
}

//
// ReadAndWriteLock<'s, T>:
//

pub struct ReadAndWriteLock<'s, T> {
	lock: &'s SpinLock<T>,
}

impl<'s, T> ops::Deref for ReadAndWriteLock<'s, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &Self::Target {
		self.lock.unwrap()
	}
}

impl<'s, T> ops::DerefMut for ReadAndWriteLock<'s, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.lock.unwrap()
	}
}

impl<'s, T> ops::Drop for ReadAndWriteLock<'s, T> {
	#[inline]
	fn drop(&mut self) {
		self.lock.state.store(0, atomic::Ordering::Release);
	}
}

impl<'s, T: fmt::Display> fmt::Display for ReadAndWriteLock<'s, T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		core::write!(f, "{}", **self)
	}
}

impl<'lhs, 'rhs, T: cmp::PartialEq> cmp::PartialEq<ReadAndWriteLock<'rhs, T>> for ReadAndWriteLock<'lhs, T> {
	// NOTE: Since only one lock with exclusive access to write can exist at a time,
	// this necessarily compares the lock's content from different SpinLocks. The
	// same is valid when comparing ReadAndWriteLocks and ReadOnlyLocks (see below).

	#[inline]
	fn eq(&self, other: &ReadAndWriteLock<'rhs, T>) -> bool {
		(*self.lock.unwrap()) == (*other.lock.unwrap())
	}

	#[inline]
	fn ne(&self, other: &ReadAndWriteLock<'rhs, T>) -> bool {
		!self.eq(other)
	}
}

impl<'lhs, 'rhs, T: cmp::PartialEq> cmp::PartialEq<ReadOnlyLock<'rhs, T>> for ReadAndWriteLock<'lhs, T> {
	#[inline]
	fn eq(&self, other: &ReadOnlyLock<'rhs, T>) -> bool {
		(*self.lock.unwrap()) == (*other.lock.unwrap())
	}

	#[inline]
	fn ne(&self, other: &ReadOnlyLock<'rhs, T>) -> bool {
		!self.eq(other)
	}
}

impl<'s, T> ReadAndWriteLock<'s, T> {
	#[inline]
	pub fn unlock(self) {}
}
