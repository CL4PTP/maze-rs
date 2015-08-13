use ::{Grid, Generator, GeneratorOption};
use ::lcg_rng::LCGRng;
use super::utils::init_rng;
use super::utils::rand::Rng;

pub struct StackBacktrackGenerator<'a, G: 'a + Grid> {
	grid: &'a mut G,
	rng: LCGRng
}

impl<'a, G: Grid> StackBacktrackGenerator<'a, G> {
	pub fn new(grid: &'a mut G, options: &[GeneratorOption]) -> Self {
		let mut seed = None;

		for o in options {
			match o {
				&GeneratorOption::Seed(in_seed) => seed = Some(in_seed)
			}
		}

		StackBacktrackGenerator {
			grid: grid,
			rng: init_rng(seed)
		}
	}

	fn stack_carve(&mut self, sx: i64, sy: i64) {
		use ::utils::Direction;
		use ::utils::Direction::*;

		let directions = Direction::enumerate();

		let mut stack = Vec::new();
		stack.push((sx, sy, (0,)));

		'stack_loop: while !stack.is_empty() {
			let (x, y, (i,)) = stack.pop().unwrap();

			let direction_offset = (self.rng.next_f64() * directions.len() as f64) as usize;

			for i in i..4 {
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

						stack.push((x, y, (i + 1,)));
						stack.push((nx, ny, (0,)));

						continue 'stack_loop;
					}
				}
			}
		}
	}
}

impl<'a, G: Grid> Generator for StackBacktrackGenerator<'a, G> {
	fn generate(&mut self) {
		self.stack_carve(0, 0);
	}
}
