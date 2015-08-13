extern crate num_cpus;
extern crate rand;

use ::lcg_rng::LCGRng;
use ::{Grid, Generator, GeneratorOption};
use ::utils::Direction::{S, E};
use self::rand::{Rng, SeedableRng};

pub struct SidewinderGenerator<'a, G: 'a + Grid + Send + Sync> {
	grid: &'a mut G,
	rng: LCGRng
}

impl<'a, G: 'a + Grid + Send + Sync> SidewinderGenerator<'a, G> {
	pub fn new(grid: &'a mut G, options: &[GeneratorOption]) -> Self {
		use GeneratorOption::*;

		let mut seed = None;

		for o in options {
			match o {
				&GeneratorOption::Seed(in_seed) => seed = Some(in_seed)
			}
		}

		SidewinderGenerator {
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

impl<'a, G: 'a + Grid + Send + Sync> Generator for SidewinderGenerator<'a, G> {
	fn generate(&mut self) {
		#![allow(mutable_transmutes)]
		use std::thread;
		use std::sync::Arc;
		use std::mem::transmute;
		use self::num_cpus;

		let num_cores = num_cpus::get() as u64;
		let rows_per_thread = self.grid.height() / num_cores;

		assert!(self.grid.height() % num_cores == 0,
			"Maze height must be divisible by {}, but {} given", num_cores, self.grid.height());

		let grid = Arc::new(&mut *self.grid);
		let mut rng = self.rng.clone();

        let mut threads = Vec::with_capacity(num_cores as usize);

		for tn in 0..num_cores {
			let seed = !rng.next_u64();
			rng.reseed(seed);

			let grid = grid.clone();
			let mut rng = rng.clone();

			threads.push(thread::scoped(move || {
				let grid: &mut G = unsafe { transmute(&**grid) };

				for y in ((tn * rows_per_thread)..((tn + 1) * rows_per_thread)).rev() {
					let mut run_start = 0;

					for x in 0..grid.width() {
						if y < grid.height() - 1
							&& (x + 1 == grid.width() || rng.next_f64() > 0.50) {
							let carve_point =
								run_start + (rng.next_f64() * (x - run_start + 1) as f64) as u64;

							unsafe { grid.or_set_unchecked(carve_point, y, S as u8); }
							run_start = x + 1;
						} else if x + 1 < grid.width() {
							unsafe { grid.or_set_unchecked(x, y, E as u8); }
						}
					}
				}
			}));
		}

		for thread in threads {
            thread.join();
        }
	}
}
