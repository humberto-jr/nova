use crate::ffi::unix::wayland::client::protocol as ffi;

use super::window;

pub(crate) static CALLBACKS: ffi::wl_buffer_listener = ffi::wl_buffer_listener {
	release,
};

unsafe extern "C" fn release(data: *mut (), buffer: *mut ffi::wl_buffer) {
	let backend = window::Backend::from_opaque_ptr(data);

	for n in 0..super::MAX_BUFFER_COUNT {
		if backend.window_buffer_list[n] == buffer {
			backend.window_buffer_mask |= 1 << n;
			return;
		}
	}
}
