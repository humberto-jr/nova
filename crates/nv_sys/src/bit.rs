use ::core;
use core::{
	fmt, //
	ops,
};

use crate::mem;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mask8(u8);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mask16(u16);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mask32(u32);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mask64(u64);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mask128(u128);

macro_rules! impl_Mask {
	($mask_type:ty, $base_type:ty, $format:literal) => {
		impl $mask_type {
			#[inline(always)]
			pub const fn zeroed() -> Self {
				Self(0 as $base_type)
			}

			#[inline(always)]
			pub const fn from(mask: $base_type) -> Self {
				Self(mask)
			}

			#[inline(always)]
			pub const fn from_range(start: u8, end: u8) -> Self {
				if start >= end {
					Self::zeroed()
				} else {
					let len = end - start;

					if len >= Self::capacity() {
						Self::from(!0)
					} else {
						Self((((1 as $base_type) << len) - 1) << start)
					}
				}
			}

			#[inline(always)]
			pub const fn is_null(self) -> bool {
				self.0 == (0 as $base_type)
			}

			#[inline(always)]
			pub const fn bits(self) -> $base_type {
				self.0
			}

			#[inline(always)]
			pub const fn capacity() -> u8 {
				// NOTE: Assuming 8 bits per byte.
				(mem::size_of::<$base_type>() * 8) as u8
			}

			#[inline(always)]
			pub const fn set_bit(&mut self, bit: u8) {
				self.0 |= 1 << (bit as $base_type);
			}

			#[inline(always)]
			pub const fn set_bits(&mut self, mask: $base_type) {
				self.0 |= mask;
			}

			#[inline(always)]
			pub const fn set_all(&mut self) {
				self.0 = !(0 as $base_type);
			}

			#[inline(always)]
			pub const fn unset_bit(&mut self, bit: u8) {
				self.0 &= !(1 << (bit as $base_type));
			}

			#[inline(always)]
			pub const fn unset_bits(&mut self, mask: $base_type) {
				self.0 &= !mask;
			}

			#[inline(always)]
			pub const fn unset_all(&mut self) {
				self.0 = (0 as $base_type);
			}

			#[inline(always)]
			pub const fn toggle_bit(&mut self, bit: u8) {
				self.0 ^= 1 << (bit as $base_type);
			}

			#[inline(always)]
			pub const fn toggle_bits(&mut self, mask: $base_type) {
				self.0 ^= mask;
			}

			#[inline(always)]
			pub const fn toggle_all(&mut self) {
				self.0 = !self.0;
			}

			#[inline(always)]
			pub const fn read_bit(self, bit: u8) -> u8 {
				((self.0 >> (bit as $base_type)) & 1) as u8
			}

			#[inline(always)]
			pub const fn check_bit(self, bit: u8) -> bool {
				(self.0 & (1 << (bit as $base_type))) > 0
			}

			#[inline(always)]
			pub const fn count_ones(self) -> u8 {
				self.0.count_ones() as u8
			}

			#[inline(always)]
			pub const fn count_zeros(self) -> u8 {
				self.0.count_zeros() as u8
			}
		}

		impl ops::BitOr for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn bitor(self, other: Self) -> Self::Output {
				Self(self.0 | other.0)
			}
		}

		impl ops::BitOrAssign for $mask_type {
			#[inline(always)]
			fn bitor_assign(&mut self, other: Self) {
				self.0 |= other.0;
			}
		}

		impl ops::BitAnd for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn bitand(self, other: Self) -> Self::Output {
				Self(self.0 & other.0)
			}
		}

		impl ops::BitAndAssign for $mask_type {
			#[inline(always)]
			fn bitand_assign(&mut self, other: Self) {
				self.0 &= other.0;
			}
		}

		impl ops::BitXor for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn bitxor(self, other: Self) -> Self::Output {
				Self(self.0 ^ other.0)
			}
		}

		impl ops::BitXorAssign for $mask_type {
			#[inline(always)]
			fn bitxor_assign(&mut self, other: Self) {
				self.0 ^= other.0;
			}
		}

		impl ops::Not for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}

		impl ops::Shl<u8> for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn shl(self, rhs: u8) -> Self::Output {
				Self(self.0 << (rhs as $base_type))
			}
		}

		impl ops::ShlAssign<u8> for $mask_type {
			#[inline(always)]
			fn shl_assign(&mut self, rhs: u8) {
				self.0 <<= (rhs as $base_type);
			}
		}

		impl ops::Shr<u8> for $mask_type {
			type Output = Self;

			#[inline(always)]
			fn shr(self, rhs: u8) -> Self::Output {
				Self(self.0 >> (rhs as $base_type))
			}
		}

		impl ops::ShrAssign<u8> for $mask_type {
			#[inline(always)]
			fn shr_assign(&mut self, rhs: u8) {
				self.0 >>= (rhs as $base_type);
			}
		}

		impl fmt::Display for $mask_type {
			#[inline(always)]
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				core::write!(f, $format, self.0)
			}
		}
	};
}

impl_Mask!(Mask8, u8, "{:#010b}");
impl_Mask!(Mask16, u16, "{:#018b}");
impl_Mask!(Mask32, u32, "{:#034b}");
impl_Mask!(Mask64, u64, "{:#066b}");
impl_Mask!(Mask128, u128, "{:#0130b}");
