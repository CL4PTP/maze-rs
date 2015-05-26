extern crate mmap;
extern crate libc;
use self::mmap::*;
use std::fs::File;

#[inline]
fn retrieve_bits(value: u32, nth: u32) -> u32 {
	(value >> (nth * 2)) & 0b11
}

#[inline]
fn prepare_bits(value: u32, nth: u32) -> u32 {
	(value & 0b11) << (nth * 2)
}

pub trait PackedArray {
	fn len(&self) -> usize;

	fn fill(&mut self, fill: u8);

	unsafe fn get(&self, index: usize) -> u8;

	unsafe fn set(&mut self, index: usize, value: u8);

	unsafe fn or_set(&mut self, index: usize, value: u8);

	unsafe fn unset_provided(&mut self, index: usize, value: u8);
}

pub struct InMemoryPackedArray (Vec<u32>);

impl InMemoryPackedArray {
	pub fn new(len: usize) -> Self {
		assert_eq!(len % 16, 0);

		InMemoryPackedArray(vec![0; len / 16])
	}
}

impl PackedArray for InMemoryPackedArray {
	fn len(&self) -> usize {
		self.0.len() * 16
	}

	fn fill(&mut self, fill: u8) {
		let fill = fill as u32;
		let fill = fill | fill << 8 | fill << 16 | fill << 24;

		for val in self.0.iter_mut() {
			*val = fill;
		}
	}

	unsafe fn get(&self, index: usize) -> u8
	{
		retrieve_bits(*self.0.get_unchecked(index / 16), (index % 16) as u32) as u8
	}

	unsafe fn set(&mut self, index: usize, value: u8)
	{
		let nth = (index % 16) as u32;

		*self.0.get_unchecked_mut(index / 16) &= !(0b11 << (nth * 2));
		*self.0.get_unchecked_mut(index / 16) |= prepare_bits(value as u32, nth);
	}

	unsafe fn or_set(&mut self, index: usize, value: u8)
	{
		*self.0.get_unchecked_mut(index / 16) |= prepare_bits(value as u32, (index % 16) as u32);
	}

	unsafe fn unset_provided(&mut self, index: usize, value: u8)
	{
		*self.0.get_unchecked_mut(index / 16) &= !prepare_bits(value as u32, (index % 16) as u32)
	}
}

pub struct MMAPPackedArray {
	len: usize,
	word_len: usize,
	mmap: MemoryMap,
	file: File
}

impl MMAPPackedArray {
	pub fn new(len: usize, file_name: &str) -> Self {
		use std::fs;
		use std::io::Write;
		use std::env;
		use std::mem;

		use std::os::unix::io::AsRawFd;

		assert_eq!(len % 16, 0);

		let word_len = len / 16;
		let mut bin_path = env::current_dir().unwrap();
		bin_path.set_file_name(file_name);

		let mut file = fs::OpenOptions::new()
			.create(true)
			.truncate(true)
			.read(true)
			.write(true)
			.open(&bin_path)
			.unwrap();

		// allocate data in the file for MMAP
		for _ in 0..word_len {
			file.write_all(&[0, 0, 0, 0]).unwrap();
		}

		let mmapped = MemoryMap::new(mem::size_of::<u32>() * word_len, &[
			MapOption::MapReadable,
			MapOption::MapWritable,
			MapOption::MapFd(file.as_raw_fd()),
			MapOption::MapNonStandardFlags(libc::MAP_SHARED)
		]).unwrap();

		MMAPPackedArray {
			len: len,
			word_len: word_len,
			mmap: mmapped,
			file: file
		}
	}

	unsafe fn get_unpacked_unchecked(&self, index: usize) -> *const u32 {
		(self.mmap.data() as *const u32).offset(index as isize)
	}

	unsafe fn get_unpacked_unchecked_mut(&mut self, index: usize) -> *mut u32 {
		(self.mmap.data() as *mut u32).offset(index as isize)
	}
}

impl PackedArray for MMAPPackedArray {
	fn len(&self) -> usize {
		self.len
	}

	fn fill(&mut self, fill: u8) {
		let fill = fill as u32;
		let fill = fill | fill << 8 | fill << 16 | fill << 24;

		for i in 0..self.word_len {
			unsafe { *self.get_unpacked_unchecked_mut(i) = fill; }
		}
	}

	unsafe fn get(&self, index: usize) -> u8
	{
		retrieve_bits(*self.get_unpacked_unchecked(index / 16), (index % 16) as u32) as u8
	}

	unsafe fn set(&mut self, index: usize, value: u8)
	{
		let nth = (index % 16) as u32;

		*self.get_unpacked_unchecked_mut(index / 16) &= !(0b11 << (nth * 2));
		*self.get_unpacked_unchecked_mut(index / 16) |= prepare_bits(value as u32, nth);
	}

	unsafe fn or_set(&mut self, index: usize, value: u8)
	{
		*self.get_unpacked_unchecked_mut(index / 16) |=
			prepare_bits(value as u32, (index % 16) as u32);
	}

	unsafe fn unset_provided(&mut self, index: usize, value: u8)
	{
		*self.get_unpacked_unchecked_mut(index / 16) &=
			!prepare_bits(value as u32, (index % 16) as u32)
	}
}
