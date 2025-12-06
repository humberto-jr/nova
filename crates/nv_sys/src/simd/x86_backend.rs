#![allow(non_camel_case_types)]

#[cfg(target_arch = "x86")]
pub use ::core::arch::x86;

#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64 as x86;

//
// int8x16:
//

pub type int8x16 = x86::__m128i;

pub use x86::_mm_add_epi8 as int8x16_add;
pub use x86::_mm_set1_epi8 as int8x16_set;
pub use x86::_mm_sub_epi8 as int8x16_sub;

#[inline(always)]
pub fn int8x16_eq(rhs: int8x16, lhs: int8x16) -> bool {
	unsafe { x86::_mm_movemask_epi8(x86::_mm_cmpeq_epi8(rhs, lhs)) == 0xFFFF }
}

//
// int16x8:
//

pub type int16x8 = x86::__m128i;

pub use x86::_mm_add_epi16 as int16x8_add;
pub use x86::_mm_set1_epi16 as int16x8_set;
pub use x86::_mm_sub_epi16 as int16x8_sub;

#[inline(always)]
pub fn int16x8_eq(rhs: int16x8, lhs: int16x8) -> bool {
	unsafe { x86::_mm_movemask_epi8(x86::_mm_cmpeq_epi16(rhs, lhs)) == 0xFFFF }
}

//
// int32x4:
//

pub type int32x4 = x86::__m128i;

pub use x86::_mm_add_epi32 as int32x4_add;
pub use x86::_mm_set1_epi32 as int32x4_set;
pub use x86::_mm_sub_epi32 as int32x4_sub;

#[inline(always)]
pub fn int32x4_eq(rhs: int32x4, lhs: int32x4) -> bool {
	unsafe { x86::_mm_movemask_epi8(x86::_mm_cmpeq_epi32(rhs, lhs)) == 0xFFFF }
}

//
// int64x2:
//

pub type int64x2 = x86::__m128i;

pub use x86::_mm_add_epi64 as int64x2_add;
pub use x86::_mm_set1_epi64x as int64x2_set;
pub use x86::_mm_sub_epi64 as int64x2_sub;

#[inline(always)]
pub fn int64x2_eq(rhs: int64x2, lhs: int64x2) -> bool {
	unsafe { x86::_mm_movemask_epi8(x86::_mm_cmpeq_epi64(rhs, lhs)) == 0xFFFF }
}

//
// float32x4:
//

pub type float32x4 = x86::__m128;

pub use x86::_mm_add_ps as float32x4_add;
pub use x86::_mm_div_ps as float32x4_div;
pub use x86::_mm_mul_ps as float32x4_mul;
pub use x86::_mm_set1_ps as float32x4_set;
pub use x86::_mm_sub_ps as float32x4_sub;

#[inline(always)]
pub fn float32x4_eq(rhs: float32x4, lhs: float32x4) -> bool {
	unsafe { x86::_mm_movemask_ps(x86::_mm_cmpeq_ps(rhs, lhs)) == 0xF }
}

//
// float32x8:
//

pub type float32x8 = x86::__m256;

pub use x86::_mm256_add_ps as float32x8_add;
pub use x86::_mm256_div_ps as float32x8_div;
pub use x86::_mm256_mul_ps as float32x8_mul;
pub use x86::_mm256_set1_ps as float32x8_set;
pub use x86::_mm256_sub_ps as float32x8_sub;

#[inline(always)]
pub fn float32x8_eq(rhs: float32x8, lhs: float32x8) -> bool {
	unsafe { x86::_mm256_movemask_ps(x86::_mm256_cmp_ps(rhs, lhs, x86::_CMP_EQ_OQ)) == 0xFF }
}

//
// float32x16:
// TODO: At the time of writing, __m512 is only available on nightly Rust.
//

#[allow(unused)]
pub type float32x16 = x86::__m512;
