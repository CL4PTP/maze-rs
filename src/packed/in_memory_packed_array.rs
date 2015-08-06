use super::{PackedArray, PackedOption};
use super::utils::*;

pub struct InMemoryPackedArray (Vec<u32>);

unsafe impl Sync for InMemoryPackedArray {}
unsafe impl Send for InMemoryPackedArray {}

impl PackedArray for InMemoryPackedArray {
	fn new(len: usize, _: &[PackedOption]) -> Self {
		assert!(len % 16 == 0, "length must be divisible by 16, {} given", len);

		InMemoryPackedArray(vec![0; len / 16])
	}

	#[inline(always)]
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

	#[inline]
	unsafe fn get_unchecked(&self, index: usize) -> u8
	{
		retrieve_bits(*self.0.get_unchecked(index / 16), (index % 16) as u32) as u8
	}

	#[inline]
	unsafe fn set_unchecked(&mut self, index: usize, value: u8)
	{
		let nth = (index % 16) as u32;

		*self.0.get_unchecked_mut(index / 16) &= !(0b11 << (nth * 2));
		*self.0.get_unchecked_mut(index / 16) |= prepare_bits(value as u32, nth);
	}

	#[inline(always)]
	unsafe fn or_set_unchecked(&mut self, index: usize, value: u8)
	{
		*self.0.get_unchecked_mut(index / 16) |= prepare_bits(value as u32, (index % 16) as u32);
	}

	#[inline(always)]
	unsafe fn unset_provided_unchecked(&mut self, index: usize, value: u8)
	{
		*self.0.get_unchecked_mut(index / 16) &= !prepare_bits(value as u32, (index % 16) as u32)
	}
}
