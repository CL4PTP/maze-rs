extern crate rand;

use super::*;
use self::rand::{Isaac64Rng, Rng, SeedableRng};

const S: u8 = 1;
const E: u8 = 2;
const N: u8 = 4;
const W: u8 = 8;

enum Orientation {
	Horizontal,
	Vertical
}

pub trait Generator {
	fn set_seed(&mut self, seed: [u32; 4]);

	fn generate(&mut self);
}


pub struct RecursiveBacktrackGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> RecursiveBacktrackGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		RecursiveBacktrackGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}

	pub fn recursive_carve(&mut self, cx: i64, cy: i64) {
		let direction_offset: i64 = self.rng.gen_range(0, NUM_DIRECTIONS);

		static DIRECTIONS: [u8; 4] = [S, E, W, N];
		const NUM_DIRECTIONS: i64 = 4;
		static OPPOSITE: [u8; 9] = [0,  N, W, 0, S,  0, 0, 0, E];

		for i in 0..4 {
			let direction = DIRECTIONS[((direction_offset + i * 3) % NUM_DIRECTIONS) as usize];
			let nx = cx + if direction == E { 1 } else if direction == W { -1 } else { 0 };
			let ny = cy + if direction == S { 1 } else if direction == N { -1 } else { 0 };
			let mut val = self.maze.get(nx, ny);
			if nx - 1 >= 0 { val |= self.maze.get(nx - 1, ny) & E; }
			if ny - 1 >= 0 { val |= self.maze.get(nx, ny - 1) & S; }
			
			if val == 0 {
				if direction == S || direction == E {
					self.maze.or_set(cx, cy, direction);
				} else {
					self.maze.or_set(nx, ny, OPPOSITE[direction as usize]);
				}

				self.recursive_carve(nx, ny);
			}
		}
	}
}

impl<'a> Generator for RecursiveBacktrackGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}

	fn generate(&mut self) {
		self.recursive_carve(0, 0);
	}
}


pub struct StackBacktrackGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> StackBacktrackGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		StackBacktrackGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}
}

impl<'a> Generator for StackBacktrackGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}
	
	fn generate(&mut self) {
		const NUM_DIRECTIONS: i64 = 4;
		static DIRECTIONS: [u8; 4] = [S, E, W, N];
		static OPPOSITE: [u8; 9] = [0,  N, W, 0, S,  0, 0, 0, E];

		let mut stack: Vec<(i64, i64)> = Vec::with_capacity(self.maze.width() as usize);
		stack.push((0, 0));

		while stack.len() > 0 {
			let (cx, cy) = stack.pop().unwrap();
			let direction_offset: i64 = self.rng.gen_range(0, NUM_DIRECTIONS);

			for i in 0..4 {
				let direction = DIRECTIONS[((direction_offset + i * 3) % NUM_DIRECTIONS) as usize];
				let nx = cx + if direction == E { 1 } else if direction == W { -1 } else { 0 };
				let ny = cy + if direction == S { 1 } else if direction == N { -1 } else { 0 };
				let mut val = self.maze.get(nx, ny);
				if nx - 1 >= 0 { val |= self.maze.get(nx - 1, ny) & E; }
				if ny - 1 >= 0 { val |= self.maze.get(nx, ny - 1) & S; }
				
				if val == 0 {
					if direction == S || direction == E {
						self.maze.or_set(cx, cy, direction);
					} else {
						self.maze.or_set(nx, ny, OPPOSITE[direction as usize]);
					}

					stack.push((nx, ny));
				}
			}
		}
	}
}


pub struct RecursiveDivisionGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> RecursiveDivisionGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		RecursiveDivisionGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}

	fn recursive_divide(&mut self, x: i64, y: i64, width: i64, height: i64) {
		let orientation =
			if self.rng.next_f64() + if height > width { 0.25_f64 } else { -0.25_f64 } > 0.5 {
				Orientation::Horizontal
			} else {
				Orientation::Vertical
			};

		if width < 2 || height < 2 {
			return;
		}

		// where the line will start
		let (mx, my) = match orientation {
			Orientation::Horizontal => (x, y + self.rng.gen_range(0, height - 1)),
			Orientation::Vertical => (x + self.rng.gen_range(0, width - 1), y)
		};
		// in which direction we should offset each iteration
		let (dx, dy) = match orientation {
			Orientation::Horizontal => (1, 0),
			Orientation::Vertical => (0, 1)
		};
		// how long the bisector line will be
		let length = match orientation {
			Orientation::Horizontal => width,
			Orientation::Vertical => height
		};
		// where the line will have its one opening
		let opening = self.rng.gen_range(0, length);
		// in which direction we should set the bisector line
		let direction = match orientation {
			Orientation::Horizontal => S,
			Orientation::Vertical => E
		};

		// draw the dividing line 
		for i in 0..length {
			// except for where we've chosen the hole to be
			if i != opening {
				self.maze.unset_provided(mx + dx * i, my + dy * i, direction);
			}
		}

		// add areas recursively to the stack
		match orientation {
			Orientation::Horizontal => {
				self.recursive_divide(x, y, width, my - y + 1);
				self.recursive_divide(x, my + 1, width, y + height - my - 1);
			},
			Orientation::Vertical => {
				self.recursive_divide(x, y, mx - x + 1, height);
				self.recursive_divide(mx + 1, y, x + width - mx - 1, height);
			}
		}
	}
}

impl<'a> Generator for RecursiveDivisionGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}
	
	fn generate(&mut self) {
		self.maze.fill(0xFF);

		let (maze_width, maze_height) = (self.maze.width(), self.maze.height());

		// add walls on the south side of the maze
		for x in 0..self.maze.width() {
			self.maze.unset_provided(x, maze_height - 1, S);
		}

		// add walls on the east side of the maze
		for y in 0..self.maze.width() {
			self.maze.unset_provided(maze_width - 1, y, E);
		}

		self.recursive_divide(0, 0, maze_width, maze_height);
	}
}


pub struct StackDivisionGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> StackDivisionGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		StackDivisionGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}
}

impl<'a> Generator for StackDivisionGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}
	
	fn generate(&mut self) {
		self.maze.fill(0xFF);

		let (maze_width, maze_height) = (self.maze.width(), self.maze.height());

		// add walls on the south side of the maze
		for x in 0..self.maze.width() {
			self.maze.unset_provided(x, maze_height - 1, S);
		}

		// add walls on the east side of the maze
		for y in 0..self.maze.width() {
			self.maze.unset_provided(maze_width - 1, y, E);
		}

		let mut stack = Vec::with_capacity(self.maze.width() as usize);

		stack.push((0, 0, self.maze.width(), self.maze.height()));

		while stack.len() > 0 {
			let (x, y, width, height) = stack.pop().unwrap();
			let orientation =
				if self.rng.next_f64() + if height > width { 0.25_f64 } else { -0.25_f64 } > 0.5 {
					Orientation::Horizontal
				} else {
					Orientation::Vertical
				};

			if width < 2 || height < 2 {
				continue;
			}

			// where the line will start
			let (mx, my) = match orientation {
				Orientation::Horizontal => (x, y + self.rng.gen_range(0, height - 1)),
				Orientation::Vertical => (x + self.rng.gen_range(0, width - 1), y)
			};
			// in which direction we should offset each iteration
			let (dx, dy) = match orientation {
				Orientation::Horizontal => (1, 0),
				Orientation::Vertical => (0, 1)
			};
			// how long the bisector line will be
			let length = match orientation {
				Orientation::Horizontal => width,
				Orientation::Vertical => height
			};
			// where the line will have its one opening
			let opening = self.rng.gen_range(0, length);
			// in which direction we should set the bisector line
			let direction = match orientation {
				Orientation::Horizontal => S,
				Orientation::Vertical => E
			};

			// draw the dividing line 
			for i in 0..length {
				// except for where we've chosen the hole to be
				if i != opening {
					unsafe { self.maze.unset_provided_unchecked(mx + dx * i, my + dy * i, direction); }
				}
			}

			// add areas recursively to the stack
			match orientation {
				Orientation::Horizontal => {
					stack.push((x, y, width, my - y + 1));
					stack.push((x, my + 1, width, y + height - my - 1));
				},
				Orientation::Vertical => {
					stack.push((x, y, mx - x + 1, height));
					stack.push((mx + 1, y, x + width - mx - 1, height));
				}
			}
		}
	}
}


pub struct SidewinderGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> SidewinderGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		SidewinderGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}
}

impl<'a> Generator for SidewinderGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}
	
	fn generate(&mut self) {
		for y in 0..self.maze.height() {
			let mut run_start = 0;

			for x in 0..self.maze.width() {
				if y > 0 && (x + 1 == self.maze.width() || self.rng.next_f64() > 0.50) {
					let carve_point = run_start + self.rng.gen_range(0, x - run_start + 1);

					unsafe { self.maze.or_set_unchecked(carve_point, y - 1, S); }
					run_start = x + 1;
				} else if x + 1 < self.maze.width() {
					unsafe { self.maze.or_set_unchecked(x, y, E); }
				}
			}
		}
	}
}


pub struct ParallelSidewinderGenerator<'a> {
	maze: &'a mut Maze,
	rng: Isaac64Rng
}

impl<'a> ParallelSidewinderGenerator<'a> {
	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
		ParallelSidewinderGenerator {
			maze: maze,
			rng: SeedableRng::from_seed(seed)
		}
	}
}

impl<'a> Generator for ParallelSidewinderGenerator<'a> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}

	fn generate(&mut self) {
		use threadpool::ScopedPool;
		use std::sync::Arc;

		let pool = ScopedPool::new(8);

		for y in 0..self.maze.height() {

			pool.execute(|| {
				let mut y = y;
				unsafe { Maze::or_set_unchecked(self.maze, 0, y - 1, S); }
				// unsafe { (*child_maze).or_set_unchecked(0, y - 1, S); }
			});
		}
	}
}

// struct EllersAlgorithmSetState {
// 	has_connection: bool // store whether this set has made a connection to the next row

// 	// could be used to store other things to fine tune the algorithm,
// 	// like vertical/horizontal biases, an array of cells in set, total number of cells in set, etc.
// }

// pub struct EllersAlgorithmGenerator<'a> {
// 	maze: &'a mut Maze,
// 	rng: Isaac64Rng,

// 	row_set_ids: Vec<Vec<u32>>,
// 	set_info: HashMap<u32, EllersAlgorithmSetState>
// }

// impl<'a> EllersAlgorithmGenerator<'a> {
// 	pub fn new(maze: &'a mut Maze, seed: &[u64]) -> Self {
// 		EllersAlgorithmGenerator {
// 			maze: maze,
// 			rng: SeedableRng::from_seed(seed),
// 			row_set_ids: vec![(0..maze.width).collect(), Vec::with_capacity(maze.width as usize)],
// 			set_info: HashMap::with_capacity(maze.width as usize)
// 		}
// 	}
// }

// impl<'a> Generator for EllersAlgorithmGenerator<'a> {
// 	fn generate(&mut self) {

// 		for y in 0 .. self.height-1 {
// 			for x in 0 .. self.width-1 {
// 				if current_row_sets[x] != current_row_sets[x + 1] && self.rng.next_f64() {

// 				}
// 			}
// 		}


// 	}
// }
