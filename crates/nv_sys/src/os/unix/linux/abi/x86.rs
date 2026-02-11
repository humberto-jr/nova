use ::core::arch;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> isize {
	unsafe {
		let ret: isize;

		arch::asm!(
			"int $0x80",
			in("eax") nr,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
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
			"int $0x80",
			in("eax") nr,
			in("ebx") a0,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
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
			"int $0x80",
			in("eax") nr,
			in("ebx") a0,
			in("ecx") a1,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
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
			"int $0x80",
			in("eax") nr,
			in("ebx") a0,
			in("ecx") a1,
			in("edx") a2,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
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
			"int $0x80",
			in("eax") nr,
			in("ebx") a0,
			in("ecx") a1,
			in("edx") a2,
			in("esi") a3,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
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
			"int $0x80",
			in("eax") nr,
			in("ebx") a0,
			in("ecx") a1,
			in("edx") a2,
			in("esi") a3,
			in("edi") a4,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
			options(nostack),
		);

		ret
	}
}

#[inline(always)]
pub unsafe fn syscall6(nr: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
	unsafe {
		let ret: isize;

		// NOTE: On 32-bit x86 Linux, the 6-th syscall argument must be passed in the ebp register.
		// Since it may also be reserved by the compiler as a frame pointer, its initial value is
		// temporarily saved on the stack while ebp is reused to pass the argument 6 to the kernel.
		// Its original value is restored after the syscall completes.

		arch::asm!(
			"push ebp",
			"mov ebp, {arg5}",
			"int $0x80",
			"pop ebp",
			arg5 = in(reg) a5,
			in("eax") nr,
			in("ebx") a0,
			in("ecx") a1,
			in("edx") a2,
			in("esi") a3,
			in("edi") a4,
			lateout("eax") ret,
			lateout("ebx") _,
			lateout("ecx") _,
			lateout("edx") _,
			lateout("esi") _,
			lateout("edi") _,
		);

		ret
	}
}
