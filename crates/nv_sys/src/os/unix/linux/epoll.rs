use ::core::{
	ops, //
	task,
};

use crate::{
	ffi::unix::linux::uapi::eventpoll as ffi, //
	os,
	os::unix,
	os::unix::utils,
	spec,
};

use super::syscall;

pub struct Dispatcher {
	handle: unix::Descriptor,
	waker_list: utils::DispatchList,
}

impl ops::Drop for Dispatcher {
	#[inline]
	fn drop(&mut self) {
		let _ = syscall::close(self.handle);
	}
}

impl spec::Dispatcher for Dispatcher {
	type Event = spec::DispatchEvent;
	type Handle = utils::DispatchHandle;
	type Descriptor = unix::Descriptor;

	fn register(&mut self, list: &[Self::Event], source: &Self::Descriptor, waker: task::Waker) -> spec::Result<Self::Handle> {
		let slot = self.waker_list.find_free_slot_and_lock();

		if slot == usize::MAX {
			return spec::Result::Err(spec::Error::DispatcherNotReady);
		}

		let handle = Self::Handle::new(slot, *source);

		let mut poll = ffi::epoll_event {
			events: DEFAULT_EVENT_BITS,
			data: encode_handle(&handle),
		};

		for event in list {
			match event {
				Self::Event::None => {},

				Self::Event::Readable => {
					poll.events |= ffi::EPOLLIN;
				},

				Self::Event::Writable => {
					poll.events |= ffi::EPOLLOUT;
				},

				Self::Event::Error => {
					poll.events |= ffi::EPOLLERR;
				},

				Self::Event::Closed => {
					// NOTE: EPOLLHUP and EPOLLRDHUP are included by default.
				},

				Self::Event::TimerExpired => {
					poll.events |= ffi::EPOLLIN;
				},

				Self::Event::WindowInput => {
					poll.events |= ffi::EPOLLIN;
				},

				Self::Event::UnixSignal(_) => {
					// NOTE: Unix signals cannot be directly waited on using epoll.
					// But you can use the signalfd API to turn signals into file
					// descriptors.
					poll.events |= ffi::EPOLLIN;
				},

				Self::Event::WindowsCtrlSignal(_) => {},
			}
		}

		let result = syscall::epoll_control(self.handle, ffi::EPOLL_CTL_ADD, handle.as_descriptor(), &poll);

		if let spec::Result::Err(error) = result {
			self.waker_list.unlock(slot);

			spec::Result::Err(error)
		} else {
			self.waker_list.insert(&handle, poll.events, waker);

			spec::Result::Ok(handle)
		}
	}

	fn update_waker(&mut self, handle: &Self::Handle, waker: task::Waker) -> spec::Result<()> {
		if self.waker_list.has(handle) {
			let events = self.waker_list.get_events(handle);

			let data = encode_handle(handle);

			let poll = ffi::epoll_event {
				events,
				data,
			};

			let result = syscall::epoll_control(self.handle, ffi::EPOLL_CTL_MOD, handle.as_descriptor(), &poll);

			if let spec::Result::Ok(_) = result {
				self.waker_list.update_waker(handle, waker);
			}

			result
		} else {
			spec::Result::Err(spec::Error::DispatcherHandleUnknown)
		}
	}

	fn unregister(&mut self, handle: &Self::Handle) -> spec::Result<()> {
		if self.waker_list.has(handle) {
			let _ = self.waker_list.remove(handle);

			syscall::epoll_control(self.handle, ffi::EPOLL_CTL_DEL, handle.as_descriptor(), &DUMMY_EVENT_POLL)
		} else {
			spec::Result::Err(spec::Error::DispatcherHandleUnknown)
		}
	}

	fn wait_and_dispatch(&mut self, ms_timeout: unix::Time) -> u32 {
		let mut event_list = [DUMMY_EVENT_POLL; os::MAX_DISPATCHER_WAKER_COUNT];

		if let spec::Result::Ok(count) = syscall::epoll_wait(self.handle, &mut event_list[..], ms_timeout) {
			for slot in 0..(count as usize) {
				// NOTE: Since we reconstruct the handle from the data provided by the kernel,
				// which in turn came from the registration step, we assume it is always valid.
				let handle = decode_handle(event_list[slot].data);

				self.waker_list.wake(&handle);

				if (event_list[slot].events & (ffi::EPOLLHUP | ffi::EPOLLRDHUP)) > 0 {
					let _ = self.unregister(&handle);
				}
			}

			count
		} else {
			0
		}
	}
}

impl Dispatcher {
	#[inline]
	pub fn new() -> Self {
		Self {
			handle: syscall::epoll_create().unwrap(),
			waker_list: utils::DispatchList::empty(),
		}
	}
}

//
// Internals:
//

const DUMMY_EVENT_POLL: ffi::epoll_event = ffi::epoll_event {
	events: 0,
	data: 0,
};

const DEFAULT_EVENT_BITS: u32 = ffi::EPOLLONESHOT | ffi::EPOLLHUP | ffi::EPOLLRDHUP;

#[inline]
pub const fn encode_both(index: u16, descriptor: i32) -> u64 {
	(index as u64) | (((descriptor as u32) as u64) << 16)
}

#[inline]
pub const fn encode_handle(handle: &utils::DispatchHandle) -> u64 {
	encode_both(handle.as_index() as _, handle.as_descriptor())
}

#[inline]
pub const fn decode_index(data: u64) -> u16 {
	(data & 0xFFFF) as u16
}

#[inline]
pub const fn decode_descriptor(data: u64) -> i32 {
	((data >> 16) & 0xFFFFFFFF) as u32 as i32
}

#[inline]
pub const fn decode_handle(data: u64) -> utils::DispatchHandle {
	utils::DispatchHandle::new(decode_index(data) as _, decode_descriptor(data))
}
