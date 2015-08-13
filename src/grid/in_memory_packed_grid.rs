use ::{Grid, PackedOption};
use ::utils::*;

pub struct InMemoryPackedGrid {
	arr: Vec<u8>,
	width: u64,
	height: u64
}

unsafe impl Sync for InMemoryPackedGrid {}
unsafe impl Send for InMemoryPackedGrid {}

impl InMemoryPackedGrid {
	pub fn new(options: &[PackedOption]) -> Self {

		let mut width = 0;
		let mut height = 0;

		for o in options {
			match *o {
				PackedOption::Width(in_width) => width = in_width,
				PackedOption::Height(in_height) => height = in_height,

				_ => {}
			}
		}

		let len = (width * height) as usize;
		assert!(len % 4 == 0, "area must be divisible by 4, {} given", len);

		InMemoryPackedGrid {
			arr: vec![0; len / 4],
			width: width,
			height: height
		}
	}

	unsafe fn get_unpacked_unchecked(&self, x: u64, y: u64) -> &u8 {
		self.arr.get_unchecked(((y * self.width() + x) / 4) as usize)
	}

	unsafe fn get_unpacked_unchecked_mut(&mut self, x: u64, y: u64) -> &mut u8 {
		let width = self.width();
		self.arr.get_unchecked_mut(((y * width + x) / 4) as usize)
	}
}

impl Grid for InMemoryPackedGrid {

	#[inline(always)]
	fn width(&self) -> u64 {
		self.width
	}

	#[inline(always)]
	fn height(&self) -> u64 {
		self.height
	}

	fn fill(&mut self, fill: u8) {
		for val in self.arr.iter_mut() {
			*val = fill;
		}
	}

	#[inline(always)]
	unsafe fn get_unchecked(&self, x: u64, y: u64) -> u8
	{
		retrieve_bits(*self.get_unpacked_unchecked(x, y), ((y * self.width() + x) % 4) as u8)
	}

	#[inline(always)]
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
