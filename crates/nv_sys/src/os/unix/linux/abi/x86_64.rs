use ::core::arch;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"syscall",
			in("rax") nr,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			in("rsi") a1,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			in("rsi") a1,
			in("rdx") a2,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			in("rsi") a1,
			in("rdx") a2,
			in("r10") a3,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			in("rsi") a1,
			in("rdx") a2,
			in("r10") a3,
			in("r8") a4,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
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
			"syscall",
			in("rax") nr,
			in("rdi") a0,
			in("rsi") a1,
			in("rdx") a2,
			in("r10") a3,
			in("r8") a4,
			in("r9") a5,
			lateout("rax") ret,
			lateout("rcx") _,
			lateout("r11") _,
			options(nostack),
		);

		ret
	}
}
