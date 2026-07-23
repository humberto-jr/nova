use crate::{
	mem, //
	spec,
	sync,
	time,
};

const NEW_EVENT_BUFFER: super::EventBuffer = [const { spec::WindowEvent::None }; super::MAX_WINDOW_EVENT_COUNT];

impl super::WindowEventList {
	#[inline]
	pub const fn empty() -> Self {
		Self {
			back_buffer: 0,
			front_count: 0,
			next_event: sync::SpinLock::new(0),
			event_count: sync::SpinLock::new(0),
			event_list: [NEW_EVENT_BUFFER; 2],
		}
	}

	#[inline]
	pub fn is_full(&self) -> bool {
		(*self.event_count.shared_read() as usize) == super::MAX_WINDOW_EVENT_COUNT
	}

	#[inline]
	pub fn clear(&self) {
		let mut lock = self.event_count.exclusive_write();

		*lock = 0;
	}

	#[inline]
	pub fn as_slice(&self) -> &[spec::WindowEvent] {
		let buf_slot = self.read_buffer_index();

		let count = self.front_count as usize;

		// NOTE: Reading cached events is exclusively made from the front
		// buffer while new events are written in the back buffer.
		&self.event_list[buf_slot][..count]
	}

	pub fn swap_back_buffer(&mut self) {
		let mut lock = self.event_count.exclusive_write();

		self.back_buffer = self.read_buffer_index() as _;

		self.front_count = *lock;

		*lock = 0;
	}

	#[inline]
	pub fn push_window_resized(&mut self, x: i32, y: i32, width: u32, height: u32, scale: f32) {
		self.push(spec::WindowEvent::WindowResized {
			x,
			y,
			width,
			height,
			scale,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_window_minimized(&mut self, x: i32, y: i32, width: u32, height: u32, scale: f32) {
		self.push(spec::WindowEvent::WindowMinimized {
			x,
			y,
			width,
			height,
			scale,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_window_maximized(&mut self, x: i32, y: i32, width: u32, height: u32, scale: f32) {
		self.push(spec::WindowEvent::WindowMaximized {
			x,
			y,
			width,
			height,
			scale,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_window_full_screen(&mut self, width: u32, height: u32, scale: f32) {
		self.push(spec::WindowEvent::WindowFullScreen {
			width,
			height,
			scale,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_window_restored(&mut self, x: i32, y: i32, width: u32, height: u32, scale: f32) {
		self.push(spec::WindowEvent::WindowRestored {
			x,
			y,
			width,
			height,
			scale,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_moved(&mut self, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerMoved {
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_left_window(&mut self, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerLeftWindow {
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_entered_window(&mut self, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerEnteredWindow {
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_button_pressed(&mut self, button: spec::PointerButton, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerButtonPressed {
			button,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_button_released(&mut self, button: spec::PointerButton, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerButtonReleased {
			button,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_horizontal_scroll(&mut self, x: i32, y: i32, delta_x: f32) {
		self.push(spec::WindowEvent::PointerHorizontalScroll {
			x,
			y,
			delta_x,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_vertical_scroll(&mut self, x: i32, y: i32, delta_y: f32) {
		self.push(spec::WindowEvent::PointerVerticalScroll {
			x,
			y,
			delta_y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_diagonal_scroll(&mut self, x: i32, y: i32, delta_x: f32, delta_y: f32) {
		self.push(spec::WindowEvent::PointerDiagonalScroll {
			x,
			y,
			delta_x,
			delta_y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_pointer_button_repeated(&mut self, button: spec::PointerButton, x: i32, y: i32) {
		self.push(spec::WindowEvent::PointerButtonRepeated {
			button,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_keyboard_key_pressed(&mut self, key: spec::LogicalKey, x: i32, y: i32) {
		self.push(spec::WindowEvent::KeyboardKeyPressed {
			key,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_keyboard_key_released(&mut self, key: spec::LogicalKey, x: i32, y: i32) {
		self.push(spec::WindowEvent::KeyboardKeyReleased {
			key,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_keyboard_key_repeated(&mut self, key: spec::LogicalKey, x: i32, y: i32) {
		self.push(spec::WindowEvent::KeyboardKeyRepeated {
			key,
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_text_input(&mut self, length: u16, buffer: [u8; spec::WINDOW_TEXT_INPUT_CAPACITY]) {
		self.push(spec::WindowEvent::TextInput {
			length,
			buffer,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	#[inline]
	pub fn push_should_close(&mut self, x: i32, y: i32) {
		self.push(spec::WindowEvent::ShouldClose {
			x,
			y,
			index: self.next_index(),
			timestamp: time::Instant::now(),
		});
	}

	pub fn pop(&mut self) -> spec::WindowEvent {
		let mut lock = self.event_count.exclusive_write();

		let old_slot = *lock as usize;

		if old_slot > 0 {
			let buf_slot = self.write_buffer_index();

			*lock -= 1;
			mem::replace(&mut self.event_list[buf_slot][old_slot], spec::WindowEvent::None)
		} else {
			spec::WindowEvent::None
		}
	}

	//
	// Internals:
	//

	#[inline(always)]
	const fn write_buffer_index(&self) -> usize {
		self.back_buffer as _
	}

	#[inline(always)]
	const fn read_buffer_index(&self) -> usize {
		(1 - self.back_buffer) as _
	}

	fn next_index(&self) -> u64 {
		let mut lock = self.next_event.exclusive_write();

		let index = *lock as u64;

		*lock += 1;
		index
	}

	fn push(&mut self, event: spec::WindowEvent) {
		let mut lock = self.event_count.exclusive_write();

		let new_slot = *lock as usize;

		if new_slot < super::MAX_WINDOW_EVENT_COUNT {
			let buf_slot = self.write_buffer_index();

			self.event_list[buf_slot][new_slot] = event;
			*lock += 1;
		}
	}
}
