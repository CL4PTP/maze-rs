#![feature(libc)]
#![feature(scoped)]
extern crate num;

pub use self::packed::*;
pub use self::generator::*;

mod packed;
mod generator;

use std::fmt::{Display, Formatter, Error};

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

pub struct MazeBuilder {
	width: i64,
	height: i64,
	seed: Option<[u32; 4]>,
	generator_type: GeneratorType
}

impl MazeBuilder {
	pub fn new() -> Self {
		MazeBuilder {
			width: 0,
			height: 0,
			seed: None,
			generator_type: GeneratorType::RecursiveBacktrack
		}
	}

	pub fn width(mut self, width: i64) -> Self {
		self.width = width; self
	}

	pub fn height(mut self, height: i64) -> Self {
		self.height = height; self
	}

	pub fn generate_using(mut self, generator_type: GeneratorType) -> Self {
		self.generator_type = generator_type; self
	}

	pub fn seed(mut self, seed: u64) ->Self {
		self.seed = Some([(seed >> 32) as u32, seed as u32, 0, 0]); self
	}

	pub fn build<P: PackedArray>(self, backing_options: &[PackedOption]) -> Maze<P> {
		let mut maze = Maze::<P>::new(self.width, self.height, backing_options);

		maze.generator_type = self.generator_type;
		maze.generate(self.seed);

		maze
	}
}

pub struct Maze<P>
	where P: PackedArray {
	width: i64,
	height: i64,
	
	pub generator_type: GeneratorType,
	array: P // row major
}

impl<P: PackedArray> Maze<P> {
	pub fn new(width: i64, height: i64, options: &[PackedOption]) -> Maze<P> {
		let len = (width as usize).checked_mul(height as usize).expect("dimension overflow");

		unsafe { Maze::from_raw_parts(
			width,
			height,
			GeneratorType::RecursiveBacktrack,
			P::new(len, options)
		) }
	}

	pub unsafe fn from_raw_parts(width: i64, height: i64, generator_type: GeneratorType, array: P) -> Maze<P> {
		Maze {
			width: width,
			height: height,
			generator_type: generator_type,
			array: array
		}
	}

	pub fn generate(&mut self, seed: Option<[u32; 4]>) {
		use self::GeneratorType;

		let mut generator: Box<Generator> = match self.generator_type {
			GeneratorType::Sidewinder => 
				Box::new(SidewinderGenerator::new(self)),

			// GeneratorType::RecursiveBacktrack =>
			// 	Box::new(RecursiveBacktrackGenerator::new(self)),

			// GeneratorType::StackBacktrack =>
			// 	Box::new(StackBacktrackGenerator::new(self)),

			// GeneratorType::RecursiveDivision => 
			// 	Box::new(RecursiveDivisionGenerator::new(self)),

			// GeneratorType::StackDivision => 
			// 	Box::new(StackDivisionGenerator::new(self)),

			// GeneratorType::ParallelSidewinder => 
			// 	Box::new(ParallelSidewinderGenerator::new(self)),

			_ => panic!("\"{:?}\" generator algorithm not yet implemented", self.generator_type)
		};

		if let Some(seed) = seed {
			generator.set_seed(seed);
		}
		generator.generate();
	}

	#[inline(always)]
	pub fn width(&self) -> i64 {
		self.width
	}

	#[inline(always)]
	pub fn height(&self) -> i64 {
		self.height
	}

	pub fn fill(&mut self, fill: u8) {
		self.array.fill(fill);
	}

	pub fn get(&self, x: i64, y: i64) -> u8 {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe { self.get_unchecked(x, y) }
		} else {
			S | E
		}
	}

	#[inline(always)]
	pub unsafe fn get_unchecked(&self, x: i64, y: i64) -> u8 {
		self.array.get_unchecked((x + y * self.width) as usize)
	}

	#[allow(dead_code)]
	pub fn set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.set_unchecked(x, y, value);
			}
		}
	}

	#[inline(always)]
	pub unsafe fn set_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.set_unchecked((x + y * self.width) as usize, value);
	}

	pub fn or_set(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.or_set_unchecked(x, y, value)
			}
		}
	}

	#[inline(always)]
	pub unsafe fn or_set_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.or_set_unchecked((x + y * self.width) as usize, value);
	}

	#[allow(dead_code)]
	pub fn unset_provided(&mut self, x: i64, y: i64, value: u8) {
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				self.unset_provided_unchecked(x, y, value);
			}
		}
	}

	#[inline(always)]
	pub unsafe fn unset_provided_unchecked(&mut self, x: i64, y: i64, value: u8) {
		self.array.unset_provided_unchecked((x + y * self.width) as usize, value);
	}
}

impl<P: PackedArray> Display for Maze<P> {
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
