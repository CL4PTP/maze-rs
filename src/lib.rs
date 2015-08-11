#![feature(scoped)]
#![feature(result_expect)]

#![feature(cstr_to_str)]
#![feature(box_raw)]

use generator::*;

pub mod in_memory_packed_grid;
pub mod mmap_packed_grid;
pub mod generator;
pub mod lcg_rng;
pub mod extern_c;

#[derive(Debug)]
#[repr(C)]
pub enum GeneratorType {
	Sidewinder,
	NaiveSidewinder,
	RecursiveBacktrack,
	StackBacktrack,
	RecursiveDivision,
	StackDivision,
	EllersAlgorithm
}

mod utils {
	pub const S: u8 = 1;
	pub const E: u8 = 2;
	pub const N: u8 = 4;
	pub const W: u8 = 8;

	#[inline(always)]
	pub fn retrieve_bits(value: u8, nth: u8) -> u8 {
		(value >> (nth * 2)) & 0b11
	}

	#[inline(always)]
	pub fn prepare_bits(value: u8, nth: u8) -> u8 {
		(value & 0b11) << (nth * 2)
	}
}

pub enum PackedOption {
	MMAPFilePath(String),
	Width(u64),
	Height(u64)
}

pub trait Grid: Send + Sync {
	fn width(&self) -> u64;

	fn height(&self) -> u64;

	fn fill(&mut self, fill: u8);

	unsafe fn get_unchecked(&self, x: u64, y: u64) -> u8;

	unsafe fn set_unchecked(&mut self, x: u64, y: u64, value: u8);

	unsafe fn or_set_unchecked(&mut self, x: u64, y: u64, value: u8);

	unsafe fn unset_provided_unchecked(&mut self, x: u64, y: u64, value: u8);

	fn get(&self, x: u64, y: u64) -> u8 {
		assert!(x < self.width() && y < self.height());

		unsafe { self.get_unchecked(x, y) }
	}

	fn set(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height());

		unsafe { self.set_unchecked(x, y, value); }
	}

	fn or_set(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height());

		unsafe { self.or_set_unchecked(x, y, value) }
	}

	fn unset_provided(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height());
		
		unsafe { self.unset_provided_unchecked(x, y, value); }
	}

	fn to_string(&self) -> String {
		use self::utils::{S, E};

		let mut buf = String::from(" ");

		buf = buf + &String::from_utf8(vec![b'_'; self.width() as usize * 2 - 1]).unwrap();
		buf = buf + "\n";

		for y in 0..self.height() {
			buf = buf + "|";
			
			for x in 0..self.width() {
				buf = buf + if self.get(x, y) & S == 0 { "_" } else { " " };

				buf = buf + if self.get(x, y) & E == 0 { "|" } else { "." };
			}

			buf = buf + "\n";
		}

		buf
	}
}

pub fn generate<G: Grid>(grid: &mut G, generator_type: GeneratorType, options: &[GeneratorOption]) {
	use self::GeneratorType::*;

	let mut generator: Box<Generator + Sized> = match generator_type {
		Sidewinder => 
			Box::new(SidewinderGenerator::new(grid, options)),

		_ => panic!("\"{:?}\" generator algorithm not yet implemented", generator_type)
	};

	generator.generate();
}
