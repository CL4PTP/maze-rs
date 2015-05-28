#![feature(libc)]
extern crate num;

mod packed;
mod generator;

// use self::num::{ToPrimitive, FromPrimitive, Integer};
use packed::*;
use std::fmt::{Display, Formatter, Error};

const S: u8 = 1;
const E: u8 = 2;

#[derive(Debug)]
pub enum GeneratorType {
	RecursiveBacktrack,
	StackBacktrack,
	RecursiveDivision,
	StackDivision,
	EllersAlgorithm
}

pub enum BackingType {
	InMemory,
	MMAP(String)
}

pub struct MazeBuilder {
	width: i64,
	height: i64,
	seed: u64 ,
	backing_type: BackingType,
	generator_type: GeneratorType
}

impl<'a> MazeBuilder {
	pub fn new() -> Self {
		MazeBuilder {
			width: 0,
			height: 0,
			seed: 0,
			backing_type: BackingType::InMemory,
			generator_type: GeneratorType::RecursiveBacktrack
		}
	}

	pub fn width(mut self, width: i64) -> Self {
		self.width = width; self
	}

	pub fn height(mut self, height: i64) -> Self {
		self.height = height; self
	}

	pub fn backing_type(mut self, backing_type: BackingType) -> Self {
		self.backing_type = backing_type; self
	}

	pub fn generate_using(mut self, generator_type: GeneratorType) -> Self {
		self.generator_type = generator_type; self
	}

	pub fn seed(mut self, seed: u64) ->Self {
		self.seed = seed; self
	}

	pub fn build(self) -> Maze {
		let mut maze = Maze::new(self.width, self.height, self.backing_type);

		maze.seed = self.seed;
		maze.generator_type = self.generator_type;
		maze.generate();

		maze
	}
}

pub struct Maze {
	width: i64,
	height: i64,
	pub seed: u64,
	
	pub generator_type: GeneratorType,
	pub backing_type: BackingType,

	array: Box<PackedArray> // row major
}

impl Maze {
	pub fn new(width: i64, height: i64, backing_type: BackingType) -> Self {
		use self::BackingType;

		let len = (width as usize).checked_mul(height as usize).expect("dimension overflow");
		Maze {
			width: width,
			height: height,

			array: match backing_type {
				BackingType::InMemory => Box::new(InMemoryPackedArray::new(len)) as Box<PackedArray>,
				BackingType::MMAP(ref path) => Box::new(MMAPPackedArray::new(len, path)) as Box<PackedArray>
			},
			seed: 0,

			generator_type: GeneratorType::RecursiveBacktrack,
			backing_type: backing_type
		}
	}

	pub fn generate(&mut self) {
		use self::GeneratorType;
		use self::generator::*;

		let seed = self.seed; //good job borrow checker

		// this is so ugly
		match self.generator_type {
			GeneratorType::RecursiveBacktrack =>
				RecursiveBacktrackGenerator::new(self, seed).generate(),

			GeneratorType::StackBacktrack =>
				StackBacktrackGenerator::new(self, seed).generate(),

			GeneratorType::RecursiveDivision => 
				RecursiveDivisionGenerator::new(self, seed).generate(),

			GeneratorType::StackDivision => 
				StackDivisionGenerator::new(self, seed).generate(),
				
			_ => panic!(concat!("\"", stringify!(self.generator_type), "\" not implemented"))
		};
	}

	#[inline(always)]
	fn width(&self) -> i64 {
		self.width
	}

	#[inline(always)]
	fn height(&self) -> i64 {
		self.height
	}

	fn fill(&mut self, fill: u8) {
		self.array.fill(fill);
	}

	fn get(&self, x: i64, y: i64) -> u8 {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe { self.array.get((x + y * self.width) as usize) }
		} else {
			S | E
		}
	}

	fn set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.array.set((x + y * self.width) as usize, value);
			}
		}
	}

	fn or_set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.array.or_set((x + y * self.width) as usize, value);
			}
		}
	}

	fn unset_provided(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.array.unset_provided((x + y * self.width) as usize, value);
			}
		}
	}
}

impl Display for Maze {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let mut buf = String::with_capacity((self.width * (self.height + 1)) as usize);

		buf = buf + &format!(" {}\n", vec!["_"; self.width as usize * 2 - 1].iter().fold(String::new(), |a, b| a + b));

		for y in 0..self.height {
			buf = buf + "|";
			
			for x in 0..self.width {
				buf = buf + if self.get(x, y) & S == 0 { "_" } else { " " };

				buf = buf + if self.get(x, y) & E == 0 { "|" } else { "." };
			}

			buf = buf + "\n";
		}

		f.write_str(&buf)
	}
}
