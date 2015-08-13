extern crate rand;

use ::lcg_rng::LCGRng;
use ::{Grid, Generator, GeneratorOption};
use self::rand::{Rng, SeedableRng};

pub struct RecursiveBacktrackGenerator<'a, G: 'a + Grid> {
	grid: &'a mut G,
	rng: LCGRng
}

impl<'a, G: Grid> RecursiveBacktrackGenerator<'a, G> {
	pub fn new(grid: &'a mut G, options: &[GeneratorOption]) -> Self {
		let mut seed = None;

		for o in options {
			match o {
				&GeneratorOption::Seed(in_seed) => seed = Some(in_seed)
			}
		}

		RecursiveBacktrackGenerator {
			grid: grid,
			rng: if let Some(s) = seed {
//TODO: revise this hackish thing
					LCGRng::from_seed((s[0] as u64) + (s[1] as u64) << 32)
				} else {
					LCGRng::new_unseeded()
				}
		}
	}

	pub fn recursive_carve(&mut self, x: i64, y: i64) {
		use ::utils::Direction;
		use ::utils::Direction::*;

		let directions = Direction::enumerate();
		let direction_offset = (self.rng.next_f64() * directions.len() as f64) as usize;

		for i in 0..4 {
			let dir = directions[(direction_offset + i * 3) % directions.len()];
			let (nx, ny) = match dir {
				S => (x, y + 1),
				E => (x + 1, y),
				N => (x, y - 1),
				W => (x - 1, y)
			};

			if nx >= 0 && (nx as u64) < self.grid.width()
				&& ny >= 0 && (ny as u64) < self.grid.height() {
				let mut val = self.grid.get(nx as u64, ny as u64);
				if nx > 0 { val |= self.grid.get((nx - 1) as u64, ny as u64) & E as u8 };
				if ny > 0 { val |= self.grid.get(nx as u64, (ny - 1) as u64) & S as u8 };
				
				if val == 0 {
					if dir == S || dir == E {
						self.grid.or_set(x as u64, y as u64, dir as u8);
					} else {
						self.grid.or_set(nx as u64, ny as u64, dir.opposite() as u8);
					}

					self.recursive_carve(nx, ny);
				}
			}
		}
	}
}

impl<'a, G: Grid> Generator for RecursiveBacktrackGenerator<'a, G> {
	fn generate(&mut self) {
		self.recursive_carve(0, 0);
	}
}
