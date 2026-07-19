use ::core;

use crate::{
	ffi::unix::wayland::client::protocol as ffi, //
	spec,
};

use super::window;

pub(crate) static CALLBACKS: ffi::wl_pointer_listener = ffi::wl_pointer_listener {
	enter,
	leave,
	motion,
	button,
	axis,
	frame,
	axis_source,
	axis_stop,
	axis_discrete,
	axis_value120,
	axis_relative_direction,
};

unsafe extern "C" fn enter(data: *mut (), pointer: *mut ffi::wl_pointer, serial: u32, surface: *mut ffi::wl_surface, surface_x: ffi::wl_fixed_t, surface_y: ffi::wl_fixed_t) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);
	core::debug_assert!(backend.surface == surface);

	backend.pointer_x = surface_x;
	backend.pointer_y = surface_y;
	backend.pointer_serial = serial;

	backend.event_list.push_pointer_entered_window(surface_x, surface_y);
}

unsafe extern "C" fn leave(data: *mut (), pointer: *mut ffi::wl_pointer, serial: u32, surface: *mut ffi::wl_surface) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);
	core::debug_assert!(backend.surface == surface);

	backend.pointer_serial = serial;

	backend.event_list.push_pointer_left_window(backend.pointer_x, backend.pointer_y);
}

unsafe extern "C" fn motion(data: *mut (), pointer: *mut ffi::wl_pointer, _time: u32, surface_x: ffi::wl_fixed_t, surface_y: ffi::wl_fixed_t) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);

	backend.pointer_x = surface_x;
	backend.pointer_y = surface_y;
	backend.event_list.push_pointer_moved(surface_x, surface_y);
}

unsafe extern "C" fn button(data: *mut (), pointer: *mut ffi::wl_pointer, serial: u32, _time: u32, button: u32, state: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);

	backend.pointer_serial = serial;

	match state {
		ffi::WL_POINTER_BUTTON_STATE_PRESSED => {
			backend.event_list.push_pointer_button_pressed(spec::PointerButton::from_wayland_button(button), backend.pointer_x, backend.pointer_y);
		},

		ffi::WL_POINTER_BUTTON_STATE_RELEASED => {
			backend.event_list.push_pointer_button_released(spec::PointerButton::from_wayland_button(button), backend.pointer_x, backend.pointer_y);
		},

		_ => {},
	}
}

unsafe extern "C" fn axis(data: *mut (), pointer: *mut ffi::wl_pointer, _time: u32, axis: u32, value: ffi::wl_fixed_t) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);

	match axis {
		ffi::WL_POINTER_AXIS_HORIZONTAL_SCROLL => {
			backend.event_list.push_pointer_horizontal_scroll(backend.pointer_x, backend.pointer_y, value as _);
		},

		ffi::WL_POINTER_AXIS_VERTICAL_SCROLL => {
			backend.event_list.push_pointer_vertical_scroll(backend.pointer_x, backend.pointer_y, value as _);
		},

		_ => {},
	}
}

unsafe extern "C" fn frame(data: *mut (), pointer: *mut ffi::wl_pointer) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.pointer == pointer);

	let (x, y, delta_x, delta_y) = {
		let event_list = backend.event_list.as_slice();

		let count = event_list.len();

		if count < 2 {
			return;
		}

		let last_slot = count - 1;

		let prev_slot = count - 2;

		match event_list[last_slot] {
			// NOTE: If this callback is fired, that means the last two inputs have been pointer
			// scrolling along both x and y axes. Thus, we will merge them into a single diagonal
			// scrolling event whose pointer position is the latest of the two.
			crate::WindowEvent::PointerHorizontalScroll {
				x,
				y,
				delta_x,
				..
			} => {
				// horizontal + vertical case
				match event_list[prev_slot] {
					crate::WindowEvent::PointerVerticalScroll {
						delta_y,
						..
					} => (x, y, delta_x, delta_y),

					_ => return,
				}
			},

			crate::WindowEvent::PointerVerticalScroll {
				x,
				y,
				delta_y,
				..
			} => {
				// vertical + horizontal case
				match event_list[prev_slot] {
					crate::WindowEvent::PointerHorizontalScroll {
						delta_x,
						..
					} => (x, y, delta_x, delta_y),

					_ => return,
				}
			},

			_ => return,
		}
	};

	let _ = backend.event_list.pop();

	let _ = backend.event_list.pop();

	backend.event_list.push_pointer_diagonal_scroll(x, y, delta_x, delta_y);
}

unsafe extern "C" fn axis_source(_data: *mut (), _pointer: *mut ffi::wl_pointer, _axis_source: u32) {
	// NOTE: The axis source provides what kind of device made the scrolling event: a mouse wheel,
	// a touchpad, a trackpad, etc. We shall revisit this if we ever need to handle this type of
	// metadata.
}

unsafe extern "C" fn axis_stop(_data: *mut (), _pointer: *mut ffi::wl_pointer, _time: u32, _axis: u32) {
	// NOTE: Indicates that scrolling on a given axis has ended. Will revisit this if we ever need
	// to handle gesture completion, scroll animations, etc.
}

unsafe extern "C" fn axis_discrete(_data: *mut (), _pointer: *mut ffi::wl_pointer, _axis: u32, _discrete: i32) {
	// NOTE: Provides exact wheel-click count for physical mouse wheels. For now, the axis()
	// callback is enough.
}

unsafe extern "C" fn axis_value120(_data: *mut (), _pointer: *mut ffi::wl_pointer, _axis: u32, _value120: i32) {
	// NOTE: This report scrolls deltas in units of 1/120 of a wheel rotation, giving an exact,
	// that is, lossless integer representation. This makes Wayland scroll semantics align with
	// Windows at the lowest level.
}

unsafe extern "C" fn axis_relative_direction(_data: *mut (), _pointer: *mut ffi::wl_pointer, _axis: u32, _direction: u32) {
	// NOTE: Tell us how to interpret the sign of the scroll delta relative to content motion.
	// For instance, if fingers move down, content moves up; that is, an inverted motion. We
	// can handle that already.
}
