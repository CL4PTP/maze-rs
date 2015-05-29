#![feature(libc)]
extern crate num;
extern crate threadpool;

mod packed;
mod generator;

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
	Sidewinder,
	ParallelSidewinder,
	EllersAlgorithm
}

#[derive(Debug)]
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

		maze.generator_type = self.generator_type;
		maze.generate(&[self.seed]);

		maze
	}
}

pub struct Maze {
	width: i64,
	height: i64,
	
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
				BackingType::InMemory =>
					Box::new(InMemoryPackedArray::new(len)) as Box<PackedArray>,

				BackingType::MMAP(ref path) =>
					Box::new(MMAPPackedArray::new(len, path)) as Box<PackedArray>
			},

			generator_type: GeneratorType::RecursiveBacktrack,
			backing_type: backing_type
		}
	}

	pub fn generate(&mut self, seed: &[u64]) {
		use self::GeneratorType;
		use self::generator::*;

		let mut generator: Box<Generator> = match self.generator_type {
			GeneratorType::RecursiveBacktrack =>
				Box::new(RecursiveBacktrackGenerator::new(self, seed)),

			GeneratorType::StackBacktrack =>
				Box::new(StackBacktrackGenerator::new(self, seed)),

			GeneratorType::RecursiveDivision => 
				Box::new(RecursiveDivisionGenerator::new(self, seed)),

			GeneratorType::StackDivision => 
				Box::new(StackDivisionGenerator::new(self, seed)),

			GeneratorType::Sidewinder => 
				Box::new(SidewinderGenerator::new(self, seed)),

			_ => panic!("\"{:?}\" generator algorithm not yet implemented", self.generator_type)
		};

		generator.generate();
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
			unsafe { self.get_unchecked(x, y) }
		} else {
			S | E
		}
	}

	#[inline(always)]
	unsafe fn get_unchecked(&self, x: i64, y: i64) -> u8 {
		self.array.get_unchecked((x + y * self.width) as usize)
	}

	fn set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.set_unchecked(x, y, value);
			}
		}
	}

	#[inline(always)]
	unsafe fn set_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.set_unchecked((x + y * self.width) as usize, value);
	}

	fn or_set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.or_set_unchecked(x, y, value)
			}
		}
	}

	#[inline(always)]
	unsafe fn or_set_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.or_set_unchecked((x + y * self.width) as usize, value);
	}

	fn unset_provided(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.unset_provided_unchecked(x, y, value);
			}
		}
	}

	#[inline(always)]
	unsafe fn unset_provided_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.unset_provided_unchecked((x + y * self.width) as usize, value);
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
