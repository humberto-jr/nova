use ::core::arch;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall1(nr: usize, a0: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall2(nr: usize, a0: usize, a1: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			in("x1") a1,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			in("x1") a1,
			in("x2") a2,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall4(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			in("x1") a1,
			in("x2") a2,
			in("x3") a3,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall5(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			in("x1") a1,
			in("x2") a2,
			in("x3") a3,
			in("x4") a4,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall6(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("x8") nr,
			in("x0") a0,
			in("x1") a1,
			in("x2") a2,
			in("x3") a3,
			in("x4") a4,
			in("x5") a5,
			lateout("x0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}
