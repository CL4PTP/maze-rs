pub use self::in_memory_packed_array::InMemoryPackedArray;
pub use self::mmap_packed_array::MMAPPackedArray;

mod in_memory_packed_array;
mod mmap_packed_array;

mod utils {
	#[inline(always)]
	pub fn retrieve_bits(value: u32, nth: u32) -> u32 {
		(value >> (nth * 2)) & 0b11
	}

	#[inline(always)]
	pub fn prepare_bits(value: u32, nth: u32) -> u32 {
		(value & 0b11) << (nth * 2)
	}
}

pub enum PackedOption {
	MMAPFilePath(String)
}

pub trait PackedArray: Send + Sync {
	fn new(len: usize, options: &[PackedOption]) -> Self;

	fn len(&self) -> usize;

	fn fill(&mut self, fill: u8);

	unsafe fn get_unchecked(&self, index: usize) -> u8;

	unsafe fn set_unchecked(&mut self, index: usize, value: u8);

	unsafe fn or_set_unchecked(&mut self, index: usize, value: u8);

	unsafe fn unset_provided_unchecked(&mut self, index: usize, value: u8);
}
