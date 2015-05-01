extern crate rand;

use self::rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter, Error};

type CellType = u8;

#[derive(Debug)]
pub enum MazeGenerationType {
	RecursiveBacktrack
}

const S: CellType = 1;
const E: CellType = 2;
const N: CellType = 4;
const W: CellType = 8;

#[derive(Debug)]
pub struct Maze2D {
	grid: Vec<Vec<CellType>>, // column major
	pub width: i64,
	pub height: i64
}

impl Maze2D {
	pub fn new(width: i64, height: i64) -> Self {
		Maze2D { grid: vec![vec![0; height as usize]; width as usize], width: width, height: height }
	}

	pub fn carve(&mut self, generator_type: MazeGenerationType) {
		use self::MazeGenerationType;

		match generator_type {
			MazeGenerationType::RecursiveBacktrack => self.carve_using_recursive_backtrack(0, 0)
		};
	}

	fn carve_using_recursive_backtrack(&mut self, cx: i64, cy: i64) {
		static DIRECTIONS: [CellType; 4] = [S, E, W, N];
		const NUM_DIRECTIONS: i64 = 4;
		static OPPOSITE: [CellType; 9] = [0,  N, W, 0, S,  0, 0, 0, E];
		
		let direction_offset : i64 = thread_rng().gen_range(0, NUM_DIRECTIONS);

		for i in 0..4 {
			let direction = DIRECTIONS[((direction_offset + i * 3) % NUM_DIRECTIONS) as usize];
			let nx = cx + if direction == E { 1 } else if direction == W { -1 } else { 0 };
			let ny = cy + if direction == S { 1 } else if direction == N { -1 } else { 0 };
			let mut val = self.get(nx, ny);
			if nx - 1 >= 0 { val |= self.get(nx-1, ny) & E; }
			if ny - 1 >= 0 { val |= self.get(nx, ny-1) & S; }
			
			if val == 0 {
				if direction == S || direction == E {
					let borrowchecker_plz = self.get(cx, cy);

					self.set(cx, cy, borrowchecker_plz | direction);
				} else {
					let borrowchecker_plz = self.get(nx, ny);

					self.set(nx, ny, borrowchecker_plz | OPPOSITE[direction as usize]);
				}

				self.carve_using_recursive_backtrack(nx, ny);
			}
		}
	}

	fn get(&self, x: i64, y: i64) -> CellType {
		//the unsafe block is fine, because we check the bounds ourselves
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe { *self.grid.get_unchecked(x as usize).get_unchecked(y as usize) }
		} else {
			S | E
		}
	}

	fn set(&mut self, x: i64, y: i64, value: CellType) {
		//the unsafe block is fine, because we check the bounds ourselves
		if 0 <= x && x < self.width && 0 <= y && y < self.height {
			unsafe {
				*self.grid.get_unchecked_mut(x as usize).get_unchecked_mut(y as usize) = value;
			}
		}
	}
}

impl Display for Maze2D {
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
