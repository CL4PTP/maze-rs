extern crate rand;

use ::lcg_rng::LCGRng;
use ::{Grid, Generator, GeneratorOption};
use ::utils::Direction::{S, E};
use self::rand::{Rng, SeedableRng};

pub struct NaiveSidewinderGenerator<'a, G: 'a + Grid> {
	grid: &'a mut G,
	rng: LCGRng
}

impl<'a, G: Grid> NaiveSidewinderGenerator<'a, G> {
	pub fn new(grid: &'a mut G, options: &[GeneratorOption]) -> Self {

		let mut seed = None;

		for o in options {
			match o {
				&GeneratorOption::Seed(in_seed) => seed = Some(in_seed)
			}
		}

		NaiveSidewinderGenerator {
			grid: grid,
			rng: if let Some(s) = seed {
//TODO: revise this hackish thing
					LCGRng::from_seed((s[0] as u64) + (s[1] as u64) << 32)
				} else {
					LCGRng::new_unseeded()
				}
		}
	}
}

impl<'a, G: Grid> Generator for NaiveSidewinderGenerator<'a, G> {
	
	fn generate(&mut self) {
		let grid = &mut *self.grid;

		for y in 0..grid.height() {
			let mut run_start = 0;

			for x in 0..grid.width() {
				if y > 0 && (x + 1 == grid.width() || self.rng.next_f64() > 0.50) {
					let carve_point =
						run_start + (self.rng.next_f64() * (x - run_start + 1) as f64) as u64;

					unsafe { grid.or_set_unchecked(carve_point, y - 1, S as u8); }
					run_start = x + 1;
				} else if x + 1 < grid.width() {
					unsafe { grid.or_set_unchecked(x, y, E as u8); }
				}
			}
		}
	}
}
