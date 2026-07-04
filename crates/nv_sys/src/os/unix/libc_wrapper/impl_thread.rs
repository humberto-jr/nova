use ::core::ops;

use crate::{
	mem, //
	spec,
};

use super::{
	super::utils, //
	ffi,
};

impl<Fn, In, Out> ops::Drop for super::Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	fn drop(&mut self) {
		let task_ptr = self.task.as_mut_ptr();

		let _ = mem::unmap((task_ptr, 1));
	}
}

impl<Fn, In, Out> spec::Thread<Fn, In, Out> for super::Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	fn start(routine: Fn, args: In) -> Self {
		let mut handle = 0;

		let mut task = allocate_thread_task();

		task.set_routine(routine, args);

		let task_ptr = task.as_mut_ptr() as *mut ffi::c_void;

		let info = unsafe { ffi::pthread_create(&mut handle, mem::null(), Self::trampoline, task_ptr) };

		if info != 0 {
			crate::panic!("pthread_create() failed with error code {info}");
		}

		Self {
			handle,
			task,
		}
	}

	fn stop(self) -> Out {
		let info = unsafe { ffi::pthread_join(self.handle, &mut mem::null()) };

		if info != 0 {
			crate::panic!("pthread_join() failed with error code {info}");
		}

		self.task.take_result()
	}
}

impl<Fn, In, Out> super::Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	extern "C" fn trampoline(task_ptr: *mut ffi::c_void) -> *mut ffi::c_void {
		let task_ptr = task_ptr as *mut utils::ThreadTask<Fn, In, Out>;

		let task = unsafe { &mut (*task_ptr) };

		task.execute();
		mem::null()
	}
}

//
// Internals:
//

fn allocate_thread_task<Fn, In, Out>() -> mem::Block<utils::ThreadTask<Fn, In, Out>>
where
	Fn: ops::FnOnce(In) -> Out,
{
	let result = mem::map(1, spec::BlockProtection::ReadAndWrite, spec::BlockSharing::Private);

	if let spec::Result::Ok(uninit_block) = result {
		uninit_block.clear()
	} else {
		let size = mem::size_of::<utils::ThreadTask<Fn, In, Out>>();

		crate::panic!("Unable to allocate {size} bytes during thread creation");
	}
}
