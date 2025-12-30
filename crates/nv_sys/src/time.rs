use ::core::ops;

use super::backend;

//
// Instant:
//

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instant(backend::Time);

impl Instant {
	#[inline]
	pub fn now() -> Self {
		Self(backend::ns_time())
	}

	#[inline(always)]
	pub const fn as_u64(self) -> u64 {
		self.0 as u64
	}

	#[inline(always)]
	pub const fn as_f64(self) -> f64 {
		self.0 as f64
	}
}

//
// Duration:
//

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Duration(backend::Time);

impl Duration {
	#[inline(always)]
	pub const fn zeroed() -> Self {
		Self(0 as backend::Time)
	}

	#[inline(always)]
	pub const fn as_nanosecs_u64(self) -> u64 {
		self.0 as u64
	}

	#[inline(always)]
	pub const fn as_microsecs_u64(self) -> u64 {
		(self.0 as u64) / 1000
	}

	#[inline(always)]
	pub const fn as_millisecs_u64(self) -> u64 {
		(self.0 as u64) / (1000 * 1000)
	}

	#[inline(always)]
	pub const fn as_secs_u64(self) -> u64 {
		(self.0 as u64) / (1000 * 1000 * 1000)
	}

	#[inline(always)]
	pub const fn as_mins_u64(self) -> u64 {
		(self.0 as u64) / (60 * 1000 * 1000 * 1000)
	}

	#[inline(always)]
	pub const fn as_hours_u64(self) -> u64 {
		(self.0 as u64) / (60 * 60 * 1000 * 1000 * 1000)
	}

	#[inline(always)]
	pub const fn as_days_u64(self) -> u64 {
		(self.0 as u64) / (24 * 60 * 60 * 1000 * 1000 * 1000)
	}

	#[inline(always)]
	pub const fn as_nanosecs_f64(self) -> f64 {
		self.0 as f64
	}

	#[inline(always)]
	pub const fn as_microsecs_f64(self) -> f64 {
		(self.0 as f64) * 1.0e-3
	}

	#[inline(always)]
	pub const fn as_millisecs_f64(self) -> f64 {
		(self.0 as f64) * 1.0e-6
	}

	#[inline(always)]
	pub const fn as_secs_f64(self) -> f64 {
		(self.0 as f64) * 1.0e-9
	}

	#[inline(always)]
	pub const fn as_mins_f64(self) -> f64 {
		(self.0 as f64) / (60.0 * 1.0e9)
	}

	#[inline(always)]
	pub const fn as_hours_f64(self) -> f64 {
		(self.0 as f64) / (60.0 * 60.0 * 1.0e9)
	}

	#[inline(always)]
	pub const fn as_days_f64(self) -> f64 {
		(self.0 as f64) / (24.0 * 60.0 * 60.0 * 1.0e9)
	}
}

//
// Instant and Duration arithmetic relations:
//

impl ops::Add<Duration> for Instant {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Duration) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl ops::AddAssign<Duration> for Instant {
	#[inline(always)]
	fn add_assign(&mut self, rhs: Duration) {
		self.0 += rhs.0;
	}
}

impl ops::Sub for Instant {
	type Output = Duration;

	#[inline(always)]
	fn sub(self, other: Self) -> Self::Output {
		Duration(self.0 - other.0)
	}
}

impl ops::Add for Duration {
	type Output = Self;

	#[inline(always)]
	fn add(self, other: Self) -> Self::Output {
		Self(self.0 + other.0)
	}
}

impl ops::AddAssign for Duration {
	#[inline(always)]
	fn add_assign(&mut self, other: Self) {
		self.0 += other.0;
	}
}
