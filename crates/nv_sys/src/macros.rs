use ::core;
use core::{
	fmt, //
	panic::Location,
};

#[track_caller]
pub fn panic_wrapper(args: fmt::Arguments) -> ! {
	let caller = Location::caller();

	core::panic!("\n\x1b[33mpanic at {}, line {}:\x1b[0m\n  {}\n", caller.file(), caller.line(), args);
}

#[macro_export]
macro_rules! panic {
	($($args:tt)*) => {
		$crate::macros::panic_wrapper(::core::format_args!($($args)*));
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
