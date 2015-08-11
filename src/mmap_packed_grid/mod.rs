extern crate mmap;
extern crate libc;
extern crate byteorder;

use ::{Grid, PackedOption};
use ::utils::*;
use self::mmap::*;
use self::carray::*;
use self::byteorder::{NativeEndian, WriteBytesExt};
use std::fs::{OpenOptions, File};

mod carray;

#[repr(C)]
struct RawPackedGrid {
	width: u64,
	height: u64,

	arr: CArray<u8>
}

pub struct MMAPPackedGrid {
	raw: *mut RawPackedGrid,

	_file: File,
	_mmap: MemoryMap
}

unsafe impl Sync for MMAPPackedGrid {}
unsafe impl Send for MMAPPackedGrid {}

impl MMAPPackedGrid {
	pub fn new(options: &[PackedOption]) -> Self {
		use std::env;
		use std::path::Path;
		use std::mem::size_of;

		let cwd = env::current_dir().unwrap();
		let mut bin_path = cwd.as_path();
		let mut width = 0;
		let mut height = 0;
		let len;

		for o in options {
			match *o {
				PackedOption::MMAPFilePath(ref in_bin_path) => bin_path = Path::new(in_bin_path),
				PackedOption::Width(in_width) => width = in_width,
				PackedOption::Height(in_height) => height = in_height
			}
		}

		len = width * height;

		assert!(len % 4 == 0, "area must be divisible by 4, {} given", len);

		let mut file = OpenOptions::new()
			.create(true)
			.truncate(true)
			.read(true)
			.write(true)
			.open(&bin_path)
			.unwrap();

		file.set_len(len / 4 + (size_of::<u64>() * 2) as u64).unwrap();

		file.write_u64::<NativeEndian>(width).expect("error writing to file");
		file.write_u64::<NativeEndian>(height).expect("error writing to file");

		MMAPPackedGrid::from_file(file)
	}

	pub fn from_file(file: File) -> Self {
		use std::os::unix::io::AsRawFd;
		use std::mem::transmute;

		let len = file.metadata().unwrap().len() as usize;
		
		let mmapped = MemoryMap::new(len, &[
			MapOption::MapReadable,
			MapOption::MapWritable,
			MapOption::MapFd(file.as_raw_fd()),
			MapOption::MapNonStandardFlags(libc::MAP_SHARED)
		]).unwrap();

		MMAPPackedGrid {
			raw: unsafe { transmute::<*mut u8, *mut RawPackedGrid>(mmapped.data()) },

			_file: file,
			_mmap: mmapped
		}
	}

	#[inline]
	unsafe fn get_unpacked_unchecked(&self, x: u64, y: u64) -> &u8 {
		(*self.raw).arr.get_unchecked(((y * self.width() + x) / 4) as usize)
	}

	#[inline]
	unsafe fn get_unpacked_unchecked_mut(&mut self, x: u64, y: u64) -> &mut u8 {
		(*self.raw).arr.get_unchecked_mut(((y * self.width() + x) / 4) as usize)
	}

}

impl Grid for MMAPPackedGrid {
	#[inline(always)]
	fn width(&self) -> u64 {
		unsafe { (*self.raw).width }
	}

	#[inline(always)]
	fn height(&self) -> u64 {
		unsafe { (*self.raw).height }
	}

	fn fill(&mut self, fill: u8) {
		for i in 0..(self.width() * self.height() / 4) {
			unsafe { *(*self.raw).arr.get_unchecked_mut(i as usize) = fill; }
		}
	}

	#[inline]
	unsafe fn get_unchecked(&self, x: u64, y: u64) -> u8
	{
		retrieve_bits(*self.get_unpacked_unchecked(x, y), ((y * self.width() + x) % 4) as u8)
	}

	#[inline]
	unsafe fn set_unchecked(&mut self, x: u64, y: u64, value: u8)
	{
		let nth = ((y * self.width() + x) % 4) as u8;

		*self.get_unpacked_unchecked_mut(x, y) &= !(0b11 << (nth * 2));
		*self.get_unpacked_unchecked_mut(x, y) |= prepare_bits(value, nth);
	}

	#[inline(always)]
	unsafe fn or_set_unchecked(&mut self, x: u64, y: u64, value: u8)
	{
		*self.get_unpacked_unchecked_mut(x, y) |=
			prepare_bits(value, ((y * self.width() + x) & 0b11) as u8);
	}

	#[inline(always)]
	unsafe fn unset_provided_unchecked(&mut self, x: u64, y: u64, value: u8)
	{
		*self.get_unpacked_unchecked_mut(x, y) &=
			!prepare_bits(value, ((y * self.width() + x) & 0b11) as u8)
	}
}
