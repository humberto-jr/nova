#![allow(non_camel_case_types)]

use ::core;
use core::{
	cmp, //
	convert,
	default,
	fmt,
	ops,
};

use crate::mem;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_backend;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use x86_backend::*;

//
// Implementation macros:
//

macro_rules! impl_simd_index {
	($base:ty, $simd:ty, $index:ty) => {
		impl ops::Index<$index> for $simd {
			type Output = $base;

			#[inline(always)]
			fn index(&self, index: $index) -> &Self::Output {
				unsafe { &self.val[index as usize] }
			}
		}

		impl ops::IndexMut<$index> for $simd {
			#[inline(always)]
			fn index_mut(&mut self, index: $index) -> &mut Self::Output {
				unsafe { &mut self.val[index as usize] }
			}
		}
	};
}

macro_rules! impl_simd_base {
	($base:ty, $simd:ty, $array:ty, $set_fn:ident, $cmp_fn:ident) => {
		impl $simd {
			#[inline(always)]
			pub const fn zeroed() -> Self {
				unsafe { mem::zeroed() }
			}

			#[inline(always)]
			pub fn as_slice(&self) -> &[$base] {
				unsafe { &self.val[..] }
			}

			#[inline(always)]
			pub fn as_mut_slice(&mut self) -> &mut [$base] {
				unsafe { &mut self.val[..] }
			}
		}

		impl default::Default for $simd {
			#[inline(always)]
			fn default() -> Self {
				Self::zeroed()
			}
		}

		impl fmt::Display for $simd {
			#[inline]
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				unsafe { core::write!(f, "{:?}", self.val) }
			}
		}

		impl convert::From<$base> for $simd {
			#[inline(always)]
			fn from(val: $base) -> Self {
				unsafe {
					Self {
						reg: $set_fn(val),
					}
				}
			}
		}

		impl convert::From<$array> for $simd {
			#[inline(always)]
			fn from(val: $array) -> Self {
				Self {
					val,
				}
			}
		}

		impl cmp::PartialEq for $simd {
			#[inline(always)]
			fn eq(&self, other: &Self) -> bool {
				unsafe { $cmp_fn(self.reg, other.reg) }
			}

			#[inline(always)]
			fn ne(&self, other: &Self) -> bool {
				!self.eq(other)
			}
		}

		impl_simd_index!($base, $simd, u8);
		impl_simd_index!($base, $simd, u16);
		impl_simd_index!($base, $simd, u32);
		impl_simd_index!($base, $simd, u64);
		impl_simd_index!($base, $simd, usize);
	};
}

macro_rules! impl_simd_op {
	($trait:ident, $trait_assign:ident, $method:ident, $method_assign:ident, $base:ty, $simd:ty, $op_fn:ident, $set_fn:ident) => {
		impl ops::$trait for $simd {
			type Output = Self;

			#[inline(always)]
			fn $method(self, other: Self) -> Self::Output {
				unsafe {
					Self {
						reg: $op_fn(self.reg, other.reg),
					}
				}
			}
		}

		impl ops::$trait_assign for $simd {
			#[inline(always)]
			fn $method_assign(&mut self, other: Self) {
				unsafe {
					self.reg = $op_fn(self.reg, other.reg);
				}
			}
		}

		impl ops::$trait<$base> for $simd {
			type Output = Self;

			#[inline(always)]
			fn $method(self, other: $base) -> Self::Output {
				unsafe {
					Self {
						reg: $op_fn(self.reg, $set_fn(other)),
					}
				}
			}
		}

		impl ops::$trait_assign<$base> for $simd {
			#[inline(always)]
			fn $method_assign(&mut self, other: $base) {
				unsafe {
					self.reg = $op_fn(self.reg, $set_fn(other));
				}
			}
		}

		impl ops::$trait<$simd> for $base {
			type Output = $simd;

			#[inline(always)]
			fn $method(self, other: $simd) -> Self::Output {
				unsafe {
					Self::Output {
						reg: $op_fn($set_fn(self), other.reg),
					}
				}
			}
		}
	};
}

macro_rules! impl_simd {
	($base:ty, $simd:ty, $array:ty, $set_fn:ident, $cmp_fn:ident, $add_fn:ident, $sub_fn:ident, $mul_fn:ident, $div_fn:ident) => {
		impl_simd_base!($base, $simd, $array, $set_fn, $cmp_fn);
		impl_simd_op!(Add, AddAssign, add, add_assign, $base, $simd, $add_fn, $set_fn);
		impl_simd_op!(Sub, SubAssign, sub, sub_assign, $base, $simd, $sub_fn, $set_fn);
		impl_simd_op!(Mul, MulAssign, mul, mul_assign, $base, $simd, $mul_fn, $set_fn);
		impl_simd_op!(Div, DivAssign, div, div_assign, $base, $simd, $div_fn, $set_fn);
	};

	($base:ty, $simd:ty, $array:ty, $set_fn:ident, $cmp_fn:ident, $add_fn:ident, $sub_fn:ident) => {
		impl_simd_base!($base, $simd, $array, $set_fn, $cmp_fn);
		impl_simd_op!(Add, AddAssign, add, add_assign, $base, $simd, $add_fn, $set_fn);
		impl_simd_op!(Sub, SubAssign, sub, sub_assign, $base, $simd, $sub_fn, $set_fn);
	};
}

//
// i8x16:
//

#[derive(Copy, Clone)]
pub union i8x16 {
	val: [i8; 16],
	reg: int8x16,
}

impl_simd!(i8, i8x16, [i8; 16], int8x16_set, int8x16_eq, int8x16_add, int8x16_sub);

//
// i16x8:
//

#[derive(Copy, Clone)]
pub union i16x8 {
	val: [i16; 8],
	reg: int16x8,
}

impl_simd!(i16, i16x8, [i16; 8], int16x8_set, int16x8_eq, int16x8_add, int16x8_sub);

//
// i32x4:
//

#[derive(Copy, Clone)]
pub union i32x4 {
	val: [i32; 4],
	reg: int32x4,
}

impl_simd!(i32, i32x4, [i32; 4], int32x4_set, int32x4_eq, int32x4_add, int32x4_sub);

//
// i64x2:
//

#[derive(Copy, Clone)]
pub union i64x2 {
	val: [i64; 2],
	reg: int64x2,
}

impl_simd!(i64, i64x2, [i64; 2], int64x2_set, int64x2_eq, int64x2_add, int64x2_sub);

//
// f32x4:
//

#[derive(Copy, Clone)]
pub union f32x4 {
	val: [f32; 4],
	reg: float32x4,
}

impl_simd!(f32, f32x4, [f32; 4], float32x4_set, float32x4_eq, float32x4_add, float32x4_sub, float32x4_mul, float32x4_div);

//
// f32x8:
//

#[derive(Clone, Copy)]
pub union f32x8 {
	val: [f32; 8],
	reg: float32x8,
}

impl_simd!(f32, f32x8, [f32; 8], float32x8_set, float32x8_eq, float32x8_add, float32x8_sub, float32x8_mul, float32x8_div);
