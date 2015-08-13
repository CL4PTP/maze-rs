use ::{Grid, Generator, GeneratorOption};
use ::utils::Direction::*;
use ::lcg_rng::LCGRng;
use super::utils::init_rng;
use super::utils::rand::Rng;

pub struct StackDivisionGenerator<'a, G: 'a + Grid> {
	grid: &'a mut G,
	rng: LCGRng
}

impl<'a, G: Grid> StackDivisionGenerator<'a, G> {
	pub fn new(grid: &'a mut G, options: &[GeneratorOption]) -> Self {
		let mut seed = None;

		for o in options {
			match o {
				&GeneratorOption::Seed(in_seed) => seed = Some(in_seed)
			}
		}

		StackDivisionGenerator {
			grid: grid,
			rng: init_rng(seed)
		}
	}

	fn stack_divide(&mut self, x: u64, y: u64, width: u64, height: u64) {
		use ::utils::Orientation::*;

		let mut stack = Vec::new();

		stack.push((x, y, width, height));

		while stack.len() > 0 {
			let (x, y, width, height) = stack.pop().unwrap();

			// randomly choose an orientation biasing shorter corridors
			let orientation =
				if self.rng.next_f64() + if height > width { 0.35_f64 } else { -0.35_f64 } > 0.5 {
					Horizontal
				} else {
					Vertical
				};

			// we can't divide something that is too narrow
			if width < 2 || height < 2 {
				continue;
			}

			// where the line will start
			let (mx, my) = match orientation {
				Horizontal => (x, y + self.rng.gen_range(0, height - 1)),
				Vertical => (x + self.rng.gen_range(0, width - 1), y)
			};
			// in which direction we should offset each iteration
			let (dx, dy) = match orientation {
				Horizontal => (1, 0),
				Vertical => (0, 1)
			};
			// how long the bisector line will be
			let length = match orientation {
				Horizontal => width,
				Vertical => height
			};
			// where the line will have its one opening
			let opening = self.rng.gen_range(0, length);
			// in which direction we should set the bisector line
			let direction = match orientation {
				Horizontal => S,
				Vertical => E
			};

			// draw the dividing line 
			for i in 0..length {
				// except for where we've chosen the hole to be
				if i != opening {
					unsafe {
						self.grid.unset_provided_unchecked(
							mx + dx * i,
							my + dy * i,
							direction as u8
						);
					}
				}
			}

			// add areas recursively to the stack
			match orientation {
				Horizontal => {
					stack.push((x, y, width, my - y + 1));
					stack.push((x, my + 1, width, y + height - my - 1));
				},
				Vertical => {
					stack.push((x, y, mx - x + 1, height));
					stack.push((mx + 1, y, x + width - mx - 1, height));
				}
			}
		}
	}
}

impl<'a, G: Grid> Generator for StackDivisionGenerator<'a, G> {
	fn generate(&mut self) {
		self.grid.fill(0xFF);

		let (grid_width, grid_height) = (self.grid.width(), self.grid.height());

		// add walls on the south side of the grid
		for x in 0..grid_width {
			unsafe { self.grid.unset_provided_unchecked(x, grid_height - 1, S as u8); }
		}

		// add walls on the east side of the grid
		for y in 0..grid_width {
			unsafe { self.grid.unset_provided_unchecked(grid_width - 1, y, E as u8); }
		}

		self.stack_divide(0, 0, grid_width, grid_height);
	}
}
