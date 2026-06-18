use ::core::ops;

use crate::{
	mem, //
	spec,
	spec::AllocatedBlock,
};

use super::syscall;

impl ops::Drop for super::File {
	#[inline]
	fn drop(&mut self) {
		if self.0 != super::INVALID_DESCRIPTOR {
			let _ = syscall::close(self.0);
		}
	}
}

impl spec::File for super::File {
	#[inline]
	fn open(&mut self, filename: &str, access: spec::FileAccess) -> spec::Result<()> {
		self.0 = syscall::open(filename, access)?;

		spec::Result::Ok(())
	}

	#[inline]
	fn is_open(&self) -> bool {
		self.0 != super::INVALID_DESCRIPTOR
	}

	#[inline]
	fn size(&self) -> spec::Result<usize> {
		let status = syscall::file_status(self.0)?;

		spec::Result::Ok(status.stx_size as _)
	}

	#[inline]
	fn write(&mut self, buf: &[super::Byte]) -> spec::Result<usize> {
		let count = syscall::write(self.0, buf)?;

		spec::Result::Ok(count)
	}

	#[inline]
	fn read(&mut self, buf: &mut [super::Byte]) -> spec::Result<usize> {
		let count = syscall::read(self.0, buf)?;

		spec::Result::Ok(count)
	}

	#[inline]
	fn seek(&mut self, pos: spec::SeekFrom) -> spec::Result<usize> {
		let offset = syscall::seek(self.0, pos)?;

		spec::Result::Ok(offset)
	}

	#[inline]
	fn flush(&mut self) -> spec::Result<()> {
		let _ = syscall::flush(self.0)?;

		spec::Result::Ok(())
	}

	#[inline]
	fn map(&self, offset: usize, size: usize, prot: spec::BlockProtection, vis: spec::BlockSharing) -> spec::Result<mem::Block<super::Byte>> {
		let raw = syscall::memory_map(0, size, prot, vis, self.0, offset)?;

		unsafe { spec::Result::Ok(mem::Block::from_raw(raw, size)) }
	}

	#[inline]
	fn unmap(&self, block: mem::Block<super::Byte>) -> spec::Result<()> {
		let (raw, size) = unsafe { block.into_raw() };

		syscall::memory_unmap(raw, size)
	}

	#[inline]
	fn close(&mut self) -> spec::Result<()> {
		let result = syscall::close(self.0);

		self.0 = super::INVALID_DESCRIPTOR;
		result
	}
}

impl super::File {
	#[inline]
	pub const fn new() -> Self {
		Self(super::INVALID_DESCRIPTOR)
	}
}
