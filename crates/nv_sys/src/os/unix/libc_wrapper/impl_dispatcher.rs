use ::core::{
	ops, //
	task,
};

use crate::{
	os, //
	os::unix,
	os::unix::utils,
	spec,
};

use super::ffi;

impl ops::Drop for super::Dispatcher {
	#[inline]
	fn drop(&mut self) {
		let _ = unsafe { ffi::close(self.handle) };
	}
}

impl spec::Dispatcher for super::Dispatcher {
	type Event = spec::DispatchEvent;

	type Handle = utils::DispatchHandle;

	type Descriptor = ffi::c_int;

	fn register(&mut self, list: &[Self::Event], source: &Self::Descriptor, waker: task::Waker) -> spec::Result<Self::Handle> {
		let slot = self.waker_list.find_free_slot_and_lock();

		if slot == usize::MAX {
			return spec::Result::Err(spec::Error::DispatcherNotReady);
		}

		let handle = Self::Handle::new(slot, *source);

		let mut poll = ffi::epoll_event {
			events: DEFAULT_EVENT_BITS,
			u64: encode_handle(&handle),
		};

		for event in list {
			match event {
				Self::Event::None => {},

				Self::Event::Readable => {
					poll.events |= ffi::EPOLLIN as u32;
				},

				Self::Event::Writable => {
					poll.events |= ffi::EPOLLOUT as u32;
				},

				Self::Event::Error => {
					poll.events |= ffi::EPOLLERR as u32;
				},

				Self::Event::Closed => {
					// NOTE: EPOLLHUP and EPOLLRDHUP are included by default.
				},

				Self::Event::TimerExpired => {
					poll.events |= ffi::EPOLLIN as u32;
				},

				Self::Event::WindowInput => {
					poll.events |= ffi::EPOLLIN as u32;
				},

				Self::Event::UnixSignal(_) => {
					// NOTE: Unix signals cannot be directly waited on using epoll.
					// But you can use the signalfd API to turn signals into file
					// descriptors.
					poll.events |= ffi::EPOLLIN as u32;
				},

				Self::Event::WindowsCtrlSignal(_) => {},
			}
		}

		let info = unsafe { ffi::epoll_ctl(self.handle, ffi::EPOLL_CTL_ADD, handle.as_descriptor(), &mut poll) };

		if info != super::POSIX_FAILURE_FLAG {
			self.waker_list.insert(&handle, poll.events, waker);

			spec::Result::Ok(handle)
		} else {
			self.waker_list.unlock(slot);

			spec::Result::Err(spec::Error::Unknown)
		}
	}

	fn update_waker(&mut self, handle: &Self::Handle, waker: task::Waker) -> spec::Result<()> {
		if self.waker_list.has(handle) {
			let events = self.waker_list.get_events(handle);

			let mut poll = ffi::epoll_event {
				events,
				u64: encode_handle(handle),
			};

			let info = unsafe { ffi::epoll_ctl(self.handle, ffi::EPOLL_CTL_MOD, handle.as_descriptor(), &mut poll) };

			if info != super::POSIX_FAILURE_FLAG {
				self.waker_list.update_waker(handle, waker);

				spec::Result::Ok(())
			} else {
				spec::Result::Err(spec::Error::Unknown)
			}
		} else {
			spec::Result::Err(spec::Error::DispatcherHandleUnknown)
		}
	}

	fn unregister(&mut self, handle: &Self::Handle) -> spec::Result<()> {
		if self.waker_list.has(handle) {
			let mut dummy_poll = DUMMY_EVENT_POLL;

			let _ = self.waker_list.remove(handle);

			let _ = unsafe { ffi::epoll_ctl(self.handle, ffi::EPOLL_CTL_DEL, handle.as_descriptor(), &mut dummy_poll) };

			spec::Result::Ok(())
		} else {
			spec::Result::Err(spec::Error::DispatcherHandleUnknown)
		}
	}

	fn wait_and_dispatch(&mut self, ms_timeout: unix::Time) -> u32 {
		let mut event_list = [DUMMY_EVENT_POLL; os::MAX_DISPATCHER_WAKER_COUNT];

		let info = unsafe { ffi::epoll_wait(self.handle, event_list.as_mut_ptr(), event_list.len() as _, ms_timeout as _) };

		if (info != super::POSIX_FAILURE_FLAG) && (info > 0) {
			let count = info as usize;

			for slot in 0..count {
				// NOTE: Since we reconstruct the handle from the data provided by the kernel,
				// which in turn came from the registration step, we assume it is always valid.
				let handle = decode_handle(event_list[slot].u64);

				self.waker_list.wake(&handle);

				if (event_list[slot].events & HANG_UP_EVENT_BITS) > 0 {
					let _ = self.unregister(&handle);
				}
			}

			count as _
		} else {
			0
		}
	}
}

impl super::Dispatcher {
	#[inline]
	pub fn new() -> Self {
		Self {
			handle: unsafe { ffi::epoll_create(1) },
			waker_list: utils::DispatchList::empty(),
		}
	}
}

//
// Internals:
//

const DUMMY_EVENT_POLL: ffi::epoll_event = ffi::epoll_event {
	events: 0,
	u64: 0,
};

const HANG_UP_EVENT_BITS: u32 = (ffi::EPOLLHUP | ffi::EPOLLRDHUP) as u32;

const DEFAULT_EVENT_BITS: u32 = HANG_UP_EVENT_BITS | (ffi::EPOLLONESHOT as u32);

#[inline]
const fn encode_both(index: u16, descriptor: i32) -> u64 {
	(index as u64) | (((descriptor as u32) as u64) << 16)
}

#[inline]
const fn encode_handle(handle: &utils::DispatchHandle) -> u64 {
	encode_both(handle.as_index() as _, handle.as_descriptor())
}

#[inline]
const fn decode_index(data: u64) -> u16 {
	(data & 0xFFFF) as u16
}

#[inline]
const fn decode_descriptor(data: u64) -> i32 {
	((data >> 16) & 0xFFFFFFFF) as u32 as i32
}

#[inline]
const fn decode_handle(data: u64) -> utils::DispatchHandle {
	utils::DispatchHandle::new(decode_index(data) as _, decode_descriptor(data))
}
