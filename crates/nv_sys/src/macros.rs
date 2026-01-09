use ::core;
use core::{
	fmt, //
	panic::Location,
};

pub const ANSI_COLOR_BLACK: &str = "\x1b[30m";
pub const ANSI_COLOR_RED: &str = "\x1b[31m";
pub const ANSI_COLOR_GREEN: &str = "\x1b[32m";
pub const ANSI_COLOR_YELLOW: &str = "\x1b[33m";
pub const ANSI_COLOR_BLUE: &str = "\x1b[34m";
pub const ANSI_COLOR_MAGENTA: &str = "\x1b[35m";
pub const ANSI_COLOR_CYAN: &str = "\x1b[36m";
pub const ANSI_COLOR_WHITE: &str = "\x1b[37m";
pub const ANSI_COLOR_RESET: &str = "\x1b[0m";

#[track_caller]
pub fn panic_wrapper(prefix: &str, header_ansi_color: &str, args: fmt::Arguments) -> ! {
	let caller = Location::caller();

	core::panic!("\n{}{}{}, line {}:{}\n  {}\n", header_ansi_color, prefix, caller.file(), caller.line(), ANSI_COLOR_RESET, args);
}

#[macro_export]
macro_rules! panic {
	($($args:tt)*) => {
		$crate::macros::panic_wrapper("panic at ", $crate::macros::ANSI_COLOR_RED, ::core::format_args!($($args)*));
	};
}

#[macro_export]
macro_rules! panic_if {
	($cond:expr $(,)?) => {
		if $cond {
			$crate::panic!("{}", ::core::stringify!($cond));
		}
	};

	($cond:expr, $($args:tt)+) => {
		if $cond {
			$crate::panic!($($args:tt)+);
		}
	};
}
