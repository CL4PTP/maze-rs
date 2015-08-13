#![feature(scoped)]
#![feature(result_expect)]
#![feature(cstr_to_str)]
#![feature(box_raw)]

pub mod generator;
pub mod grid;
pub mod solver;
pub mod lcg_rng;
pub mod extern_c;

mod utils {
	use std::convert::From;
	#[repr(u8)]
	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	pub enum Direction {
		S = 1,
		E = 2,
		N = 4,
		W = 8
	}

	impl Direction {
		pub fn enumerate() -> &'static [Direction; 4] {
			use self::Direction::*;

			static DIRECTIONS: [Direction; 4] = [S, E, N, W];

			&DIRECTIONS
		}

		pub fn opposite(&self) -> Direction {
			use self::Direction::*;

			match *self {
				S => N,
				E => W,
				N => S,
				W => E
			}
		}
	}

	impl From<Direction> for u8 {
		fn from(dir: Direction) -> u8 {
			dir as u8
		}
	}

	impl From<u8> for Direction {
		fn from(num: u8) -> Direction {
			use self::Direction::*;

			match num {
				1 => S,
				2 => E,
				4 => N,
				8 => W,
				_ => panic!("Invalid conversion from u8({}) to Direction", num)
			}
		}
	}
	
	pub enum Orientation {
		Horizontal,
		Vertical
	}

	#[inline(always)]
	pub fn retrieve_bits(value: u8, nth: u8) -> u8 {
		(value >> (nth * 2)) & 0b11
	}

	#[inline(always)]
	pub fn prepare_bits(value: u8, nth: u8) -> u8 {
		(value & 0b11) << (nth * 2)
	}
}

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

pub enum PackedOption {
	MMAPFilePath(String),
	Width(u64),
	Height(u64)
}

pub enum GeneratorOption {
	Seed(&'static [u32])
}

pub trait Generator {
	fn generate(&mut self);
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
		assert!(x < self.width() && y < self.height(),
			"{} < {} && {} < {}", x, self.width(), y, self.height());

		unsafe { self.get_unchecked(x, y) }
	}

	fn set(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height(),
			"{} < {} && {} < {}", x, self.width(), y, self.height());

		unsafe { self.set_unchecked(x, y, value); }
	}

	fn or_set(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height(),
			"{} < {} && {} < {}", x, self.width(), y, self.height());

		unsafe { self.or_set_unchecked(x, y, value) }
	}

	fn unset_provided(&mut self, x: u64, y: u64, value: u8) {
		assert!(x < self.width() && y < self.height(),
			"{} < {} && {} < {}", x, self.width(), y, self.height());
		
		unsafe { self.unset_provided_unchecked(x, y, value); }
	}

	fn test(&self, x: u64, y: u64, value: u8) -> bool {
		use self::utils::Direction;
		use self::utils::Direction::*;

		match Into::<Direction>::into(value) {
			S => self.get(x, y) & S as u8 > 0,
			E => self.get(x, y) & E as u8 > 0,
			N => y != 0 && self.get(x, y - 1) & S as u8 > 0,
			W => x != 0 && self.get(x - 1, y) & E as u8 > 0
		}
	}

	fn to_string(&self) -> String {
		use self::utils::Direction::{S, E};

		let mut buf = String::from(" ");

		buf = buf + &String::from_utf8(vec![b'_'; self.width() as usize * 2 - 1]).unwrap();
		buf = buf + "\n";

		for y in 0..self.height() {
			buf = buf + "|";
			
			for x in 0..self.width() {
				buf = buf + if self.get(x, y) & S as u8 == 0 { "_" } else { " " };

				buf = buf + if self.get(x, y) & E as u8 == 0 { "|" } else { "." };
			}

			buf = buf + "\n";
		}

		buf
	}
}

pub type SolverSolution = Vec<utils::Direction>;

pub trait Solver {
	fn solve(mut self) -> Option<SolverSolution>;
}

pub fn generate<G: Grid>(grid: &mut G, generator_type: GeneratorType, options: &[GeneratorOption]) {
	use self::GeneratorType::*;

	let mut generator: Box<Generator + Sized> = match generator_type {
		Sidewinder => 
			Box::new(generator::sidewinder_generator::SidewinderGenerator::new(grid, options)),

		_ => panic!("\"{:?}\" generator algorithm not yet implemented", generator_type)
	};

	generator.generate();
}
