use ::core::{
	default, //
	ops,
	task,
};

use crate::{
	mem, //
	os,
	sync,
};

//
// ThreadTaskMember<T>:
//

pub enum ThreadTaskMember<T> {
	None,
	Set(T),
}

impl<T> ThreadTaskMember<T> {
	pub fn take(&mut self) -> T {
		if let Self::Set(member) = mem::replace(self, Self::None) {
			member
		} else {
			crate::panic!("Attempt to take ownership over an empty thread task member; this is a bug");
		}
	}
}

//
// ThreadTask<Fn, In, Out>:
//

pub struct ThreadTask<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	routine: ThreadTaskMember<Fn>,
	inputs: ThreadTaskMember<In>,
	result: sync::SpinLock<ThreadTaskMember<Out>>,
}

impl<Fn, In, Out> default::Default for ThreadTask<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	#[inline]
	fn default() -> Self {
		Self {
			routine: ThreadTaskMember::None,
			inputs: ThreadTaskMember::None,
			result: sync::SpinLock::new(ThreadTaskMember::None),
		}
	}
}

impl<Fn, In, Out> ThreadTask<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	#[inline]
	pub fn set_routine(&mut self, routine: Fn, args: In) {
		self.routine = ThreadTaskMember::Set(routine);
		self.inputs = ThreadTaskMember::Set(args);
	}

	pub fn execute(&mut self) {
		let routine = self.routine.take();

		let args = self.inputs.take();

		let result = (routine)(args);

		let mut lock = self.result.exclusive_write();

		*lock = ThreadTaskMember::Set(result);
	}

	#[inline]
	pub fn take_result(&self) -> Out {
		self.result.exclusive_write().take()
	}
}

//
// DispatchHandle:
//

pub struct DispatchHandle(u16, super::Descriptor);

impl DispatchHandle {
	#[inline]
	pub const fn invalid() -> Self {
		Self(0, super::INVALID_DESCRIPTOR)
	}

	#[inline]
	pub const fn new(slot: usize, source: super::Descriptor) -> Self {
		Self(slot as _, source)
	}

	#[inline]
	pub const fn as_index(&self) -> usize {
		self.0 as _
	}

	#[inline]
	pub const fn as_descriptor(&self) -> super::Descriptor {
		self.1
	}

	#[inline]
	pub const fn is_valid(&self) -> bool {
		self.1 != super::INVALID_DESCRIPTOR
	}
}

//
// DispatchMask and DispatchSlot:
//

pub type DispatchMask = u32;

pub enum DispatchSlot {
	None,
	Locked,
	Idle(super::Descriptor, DispatchMask),
	Waiting(super::Descriptor, DispatchMask, task::Waker),
}

impl DispatchSlot {
	pub const fn get_events(&self) -> DispatchMask {
		match self {
			Self::None | Self::Locked => 0,

			Self::Idle(_, events) | Self::Waiting(_, events, _) => *events,
		}
	}

	pub fn update_waker(&mut self, waker: task::Waker) {
		match self {
			Self::None | Self::Locked => {},

			Self::Idle(source, events) => {
				*self = Self::Waiting(*source, *events, waker);
			},

			Self::Waiting(source, events, _) => {
				*self = Self::Waiting(*source, *events, waker);
			},
		}
	}

	pub fn wake(&mut self) {
		if let Self::Waiting(source, events, waker) = self {
			waker.wake_by_ref();
			*self = Self::Idle(*source, *events);
		}
	}

	pub fn take(&mut self) -> (super::Descriptor, DispatchMask) {
		match mem::replace(self, Self::None) {
			Self::None | Self::Locked => (super::INVALID_DESCRIPTOR, 0),

			Self::Idle(source, events) => (source, events),

			Self::Waiting(source, events, _) => (source, events),
		}
	}
}

//
// DispatchList:
//

pub const INVALID_DISPATCH_SLOT: usize = usize::MAX;

pub struct DispatchList {
	list: [sync::SpinLock<DispatchSlot>; os::MAX_DISPATCHER_WAKER_COUNT],
}

impl DispatchList {
	#[inline]
	pub const fn empty() -> Self {
		Self {
			list: [const { sync::SpinLock::new(DispatchSlot::None) }; os::MAX_DISPATCHER_WAKER_COUNT],
		}
	}

	pub fn find_free_slot_and_lock(&mut self) -> usize {
		for slot in 0..os::MAX_DISPATCHER_WAKER_COUNT {
			let mut lock = self.list[slot].exclusive_write();

			if let DispatchSlot::None = *lock {
				*lock = DispatchSlot::Locked;

				return slot;
			}
		}

		INVALID_DISPATCH_SLOT
	}

	pub fn unlock(&mut self, slot: usize) {
		let mut lock = self.list[slot].exclusive_write();

		if let DispatchSlot::Locked = *lock {
			*lock = DispatchSlot::None;
		}
	}

	pub fn has(&self, handle: &DispatchHandle) -> bool {
		let lock = self.list[handle.as_index()].shared_read();

		match *lock {
			DispatchSlot::None | DispatchSlot::Locked => false,

			DispatchSlot::Idle(source, _) | DispatchSlot::Waiting(source, _, _) => handle.as_descriptor() == source,
		}
	}

	#[inline]
	pub fn insert(&mut self, handle: &DispatchHandle, events: DispatchMask, waker: task::Waker) {
		let mut lock = self.list[handle.as_index()].exclusive_write();

		*lock = DispatchSlot::Waiting(handle.as_descriptor(), events, waker);
	}

	#[inline]
	pub fn remove(&mut self, handle: &DispatchHandle) -> (super::Descriptor, DispatchMask) {
		self.list[handle.as_index()].exclusive_write().take()
	}

	#[inline]
	pub fn get_events(&self, handle: &DispatchHandle) -> DispatchMask {
		self.list[handle.as_index()].shared_read().get_events()
	}

	#[inline]
	pub fn update_waker(&mut self, handle: &DispatchHandle, waker: task::Waker) {
		self.list[handle.as_index()].exclusive_write().update_waker(waker);
	}

	#[inline]
	pub fn wake(&mut self, handle: &DispatchHandle) {
		self.list[handle.as_index()].exclusive_write().wake();
	}
}
