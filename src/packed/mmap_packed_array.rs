extern crate mmap;
extern crate libc;

use super::{PackedArray, PackedOption};
use super::utils::*;
use self::mmap::*;
use std::fs::{OpenOptions, File};

pub struct MMAPPackedArray {
	len: usize,
	_file: File,
	mmap: MemoryMap
}

unsafe impl Sync for MMAPPackedArray {}
unsafe impl Send for MMAPPackedArray {}

impl MMAPPackedArray {
	pub fn from_file(file: File) -> Self {
		use std::os::unix::io::AsRawFd;

		let byte_len = file.metadata().unwrap().len() as usize;
		
		let mmapped = MemoryMap::new(byte_len, &[
			MapOption::MapReadable,
			MapOption::MapWritable,
			MapOption::MapFd(file.as_raw_fd()),
			MapOption::MapNonStandardFlags(libc::MAP_SHARED)
		]).unwrap();

		MMAPPackedArray {
			len: byte_len * 4,
			_file: file,
			mmap: mmapped
		}
	}

	#[inline]
	unsafe fn get_unpacked_unchecked(&self, index: usize) -> *const u32 {
		(self.mmap.data() as *const u32).offset(index as isize)
	}

	#[inline]
	unsafe fn get_unpacked_unchecked_mut(&mut self, index: usize) -> *mut u32 {
		(self.mmap.data() as *mut u32).offset(index as isize)
	}
}

impl PackedArray for MMAPPackedArray {
	fn new(len: usize, options: &[PackedOption]) -> Self {
		use std::env;
		use std::path::Path;

		assert!(len % 16 == 0, "length must be divisible by 16, {} given", len);

		let cwd = env::current_dir().unwrap();
		let mut bin_path = cwd.as_path();

		for o in options {
			match *o {
				PackedOption::MMAPFilePath(ref in_bin_path) => bin_path = Path::new(in_bin_path)
			}
		}

		let byte_len = len / 4;

		let file = OpenOptions::new()
			.create(true)
			.truncate(true)
			.read(true)
			.write(true)
			.open(&bin_path)
			.unwrap();

		file.set_len(byte_len as u64).unwrap();

		MMAPPackedArray::from_file(file)
	}

	#[inline(always)]
	fn len(&self) -> usize {
		self.len
	}

	fn fill(&mut self, fill: u8) {
		let fill = fill as u32;
		let fill = fill | fill << 8 | fill << 16 | fill << 24;

		for i in 0..(self.len() / 16) {
			unsafe { *self.get_unpacked_unchecked_mut(i) = fill; }
		}
	}

	#[inline]
	unsafe fn get_unchecked(&self, index: usize) -> u8
	{
		retrieve_bits(*self.get_unpacked_unchecked(index / 16), (index % 16) as u32) as u8
	}

	#[inline]
	unsafe fn set_unchecked(&mut self, index: usize, value: u8)
	{
		let nth = (index % 16) as u32;

		*self.get_unpacked_unchecked_mut(index / 16) &= !(0b11 << (nth * 2));
		*self.get_unpacked_unchecked_mut(index / 16) |= prepare_bits(value as u32, nth);
	}

	#[inline(always)]
	unsafe fn or_set_unchecked(&mut self, index: usize, value: u8)
	{
		*self.get_unpacked_unchecked_mut(index / 16) |=
			prepare_bits(value as u32, (index % 16) as u32);
	}

	#[inline(always)]
	unsafe fn unset_provided_unchecked(&mut self, index: usize, value: u8)
	{
		*self.get_unpacked_unchecked_mut(index / 16) &=
			!prepare_bits(value as u32, (index % 16) as u32)
	}
}
