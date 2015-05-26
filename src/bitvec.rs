extern crate alloc;
extern crate num;

use self::num::{ToPrimitive, FromPrimitive, Integer};
use std::fmt::Display;

pub struct RawBitVec2 {
	packed: *mut u32
}

impl RawBitVec2 {
	pub fn from_length(size: usize) -> Self {
		use self::alloc::heap;
		use std::mem;

		assert!(size >= 16);
		assert!(size % 16 == 0);

		let size = size
			.checked_div(16).expect("bitpacking div underflow")
			.checked_mul(mem::size_of::<u32>()).expect("size overflow");
		let ptr = unsafe { heap::allocate(size, mem::min_align_of::<u32>()) as *mut u32};
		if ptr.is_null() { alloc::oom() }

		for i in 0..size { unsafe { *ptr.offset(i as isize) = mem::zeroed(); } }

		unsafe { Self::from_raw_parts(ptr) }
	}

	pub unsafe fn from_raw_parts(ptr: *mut u32) -> Self {
		RawBitVec2 { packed: ptr }
	}

	fn retrieve_bits(value: u32, nth: u32) -> u32 {
		(value >> (nth * 2)) & 0b11
	}

	fn prepare_bits(value: u32, nth: u32) -> u32 {
		(value & 0b11) << (nth * 2)
	}

	pub unsafe fn get_raw<T>(&self, index: T) -> u32 
		where T: ToPrimitive + FromPrimitive + Integer + Copy
	{
		*self.packed.offset(index.to_isize().unwrap())
	}

	pub unsafe fn get<T>(&self, index: T) -> u8
		where T: ToPrimitive + FromPrimitive + Integer + Copy
	{
		let c_16: T = FromPrimitive::from_u32(16).unwrap();
		Self::retrieve_bits(
			*self.packed.offset((index / c_16).to_isize().unwrap()),
			(index % c_16).to_u32().unwrap()
		) as u8
	}

	pub unsafe fn set<T>(&mut self, index: T, value: u8)
		where T: ToPrimitive + FromPrimitive + Integer + Copy
	{
		let c_16: T = FromPrimitive::from_u32(16).unwrap();
		let nth = (index % c_16).to_u32().unwrap();

		*self.packed.offset((index / c_16).to_isize().unwrap()) &= !(0b11 << (nth * 2));
		*self.packed.offset((index / c_16).to_isize().unwrap()) |= Self::prepare_bits(value as u32, nth);
	}

	pub unsafe fn or_set<T>(&mut self, index: T, value: u8)
		where T: ToPrimitive + FromPrimitive + Integer + Copy
	{
		let c_16: T = FromPrimitive::from_u32(16).unwrap();
		*self.packed.offset((index / c_16).to_isize().unwrap()) |=
			Self::prepare_bits(value as u32, (index % c_16).to_u32().unwrap());
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_8x8() {
		let mut bv = RawBitVec2::from_length(8 * 8);

		unsafe { bv.set(0 + 0 * 16, 1); }
		unsafe { bv.set(0 + 1 * 16, 1); }
		unsafe { bv.set(0 + 2 * 16, 1); }
		unsafe { bv.set(0 + 3 * 16, 1); }
		unsafe { bv.set(1 + 0 * 16, 1); }
		unsafe { bv.set(1 + 1 * 16, 1); }
		unsafe { bv.set(1 + 2 * 16, 1); }
		unsafe { bv.set(1 + 3 * 16, 1); }

		assert_eq!(unsafe { bv.get(0 + 0 * 16) }, 1);
		assert_eq!(unsafe { bv.get(0 + 1 * 16) }, 1);
		assert_eq!(unsafe { bv.get(0 + 2 * 16) }, 1);
		assert_eq!(unsafe { bv.get(0 + 3 * 16) }, 1);
		assert_eq!(unsafe { bv.get(1 + 0 * 16) }, 1);
		assert_eq!(unsafe { bv.get(1 + 1 * 16) }, 1);
		assert_eq!(unsafe { bv.get(1 + 2 * 16) }, 1);
		assert_eq!(unsafe { bv.get(1 + 3 * 16) }, 1);
	}

	#[test]
	#[should_panic]
	fn check_less_than_16() {
		let mut bv = RawBitVec2::from_length(1);
	}
}
