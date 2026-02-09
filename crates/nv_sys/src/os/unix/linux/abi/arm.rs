use ::core::arch;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"svc #0",
			in("r7") nr,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			in("r1") a1,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			in("r1") a1,
			in("r2") a2,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			in("r1") a1,
			in("r2") a2,
			in("r3") a3,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			in("r1") a1,
			in("r2") a2,
			in("r3") a3,
			in("r4") a4,
			lateout("r0") ret,
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
			in("r7") nr,
			in("r0") a0,
			in("r1") a1,
			in("r2") a2,
			in("r3") a3,
			in("r4") a4,
			in("r5") a5,
			lateout("r0") ret,
			clobber_abi("C"),
			options(nostack),
		);

		ret
	}
}
