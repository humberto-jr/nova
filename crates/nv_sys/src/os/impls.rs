use ::core;

use crate::{
	mem, //
	spec,
	time,
};

//
// Error:
//

impl spec::Error {
	#[inline]
	pub const fn take(&mut self) -> Self {
		mem::replace(self, Self::None)
	}

	#[inline]
	pub const fn is_none(self) -> bool {
		core::matches!(self, Self::None)
	}
}

//
// WindowEvent:
//

impl spec::WindowEvent {
	#[inline]
	pub const fn take(&mut self) -> Self {
		mem::replace(self, Self::None)
	}

	#[inline]
	pub const fn is_none(&self) -> bool {
		core::matches!(self, Self::None)
	}

	#[inline]
	pub const fn is_valid(&self) -> bool {
		!self.is_none()
	}

	pub const fn as_usize(&self) -> usize {
		match self {
			Self::None => 0,

			Self::WindowResized {
				..
			} => 1,

			Self::WindowMinimized {
				..
			} => 2,

			Self::WindowMaximized {
				..
			} => 3,

			Self::WindowFullScreen {
				..
			} => 4,

			Self::WindowRestored {
				..
			} => 5,

			Self::PointerMoved {
				..
			} => 6,

			Self::PointerLeftWindow {
				..
			} => 7,

			Self::PointerEnteredWindow {
				..
			} => 8,

			Self::PointerButtonPressed {
				..
			} => 9,

			Self::PointerButtonReleased {
				..
			} => 10,

			Self::PointerHorizontalScroll {
				..
			} => 11,

			Self::PointerVerticalScroll {
				..
			} => 12,

			Self::PointerDiagonalScroll {
				..
			} => 13,

			Self::PointerButtonRepeated {
				..
			} => 14,

			Self::KeyboardKeyPressed {
				..
			} => 15,

			Self::KeyboardKeyReleased {
				..
			} => 16,

			Self::KeyboardKeyRepeated {
				..
			} => 17,

			Self::TextInput {
				..
			} => 18,

			Self::ShouldClose {
				..
			} => 19,
			// NOTE: If new variants are included, please increment
			// the count() method below accordingly.
		}
	}

	#[inline]
	pub const fn count() -> usize {
		20
	}

	pub const fn index(&self) -> u64 {
		match self {
			Self::None => 0,

			Self::WindowResized {
				index,
				..
			} => *index,

			Self::WindowMinimized {
				index,
				..
			} => *index,

			Self::WindowMaximized {
				index,
				..
			} => *index,

			Self::WindowFullScreen {
				index,
				..
			} => *index,

			Self::WindowRestored {
				index,
				..
			} => *index,

			Self::PointerMoved {
				index,
				..
			} => *index,

			Self::PointerLeftWindow {
				index,
				..
			} => *index,

			Self::PointerEnteredWindow {
				index,
				..
			} => *index,

			Self::PointerButtonPressed {
				index,
				..
			} => *index,

			Self::PointerButtonReleased {
				index,
				..
			} => *index,

			Self::PointerHorizontalScroll {
				index,
				..
			} => *index,

			Self::PointerVerticalScroll {
				index,
				..
			} => *index,

			Self::PointerDiagonalScroll {
				index,
				..
			} => *index,

			Self::PointerButtonRepeated {
				index,
				..
			} => *index,

			Self::KeyboardKeyPressed {
				index,
				..
			} => *index,

			Self::KeyboardKeyReleased {
				index,
				..
			} => *index,

			Self::KeyboardKeyRepeated {
				index,
				..
			} => *index,

			Self::TextInput {
				index,
				..
			} => *index,

			Self::ShouldClose {
				index,
				..
			} => *index,
		}
	}

	pub fn timestamp(&self) -> time::Instant {
		match self {
			Self::None => {
				crate::panic!("Attempt to use a WindowEvent::None variant");
			},

			Self::WindowResized {
				timestamp,
				..
			} => *timestamp,

			Self::WindowMinimized {
				timestamp,
				..
			} => *timestamp,

			Self::WindowMaximized {
				timestamp,
				..
			} => *timestamp,

			Self::WindowFullScreen {
				timestamp,
				..
			} => *timestamp,

			Self::WindowRestored {
				timestamp,
				..
			} => *timestamp,

			Self::PointerMoved {
				timestamp,
				..
			} => *timestamp,

			Self::PointerLeftWindow {
				timestamp,
				..
			} => *timestamp,

			Self::PointerEnteredWindow {
				timestamp,
				..
			} => *timestamp,

			Self::PointerButtonPressed {
				timestamp,
				..
			} => *timestamp,

			Self::PointerButtonReleased {
				timestamp,
				..
			} => *timestamp,

			Self::PointerHorizontalScroll {
				timestamp,
				..
			} => *timestamp,

			Self::PointerVerticalScroll {
				timestamp,
				..
			} => *timestamp,

			Self::PointerDiagonalScroll {
				timestamp,
				..
			} => *timestamp,

			Self::PointerButtonRepeated {
				timestamp,
				..
			} => *timestamp,

			Self::KeyboardKeyPressed {
				timestamp,
				..
			} => *timestamp,

			Self::KeyboardKeyReleased {
				timestamp,
				..
			} => *timestamp,

			Self::KeyboardKeyRepeated {
				timestamp,
				..
			} => *timestamp,

			Self::TextInput {
				timestamp,
				..
			} => *timestamp,

			Self::ShouldClose {
				timestamp,
				..
			} => *timestamp,
		}
	}
}
