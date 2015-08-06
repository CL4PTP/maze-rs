extern crate num_cpus;
extern crate rand;

use self::rand::{XorShiftRng, Rng, SeedableRng};

use super::{S, E};
use super::Generator;
use ::packed::*;
use ::Maze;

pub struct SidewinderGenerator<'a, P: 'a + PackedArray> {
	maze: &'a mut Maze<P>,
	rng: XorShiftRng
}

impl<'a, P: PackedArray> SidewinderGenerator<'a, P> {
	pub fn new(maze: &'a mut Maze<P>) -> Self {
		SidewinderGenerator {
			maze: maze,
			rng: XorShiftRng::new_unseeded()
		}
	}
}

impl<'a, P: PackedArray> Generator for SidewinderGenerator<'a, P> {
	fn set_seed(&mut self, seed: [u32; 4]) {
		self.rng.reseed(seed);
	}

	fn generate(&mut self) {
		#![allow(mutable_transmutes)]
		use std::thread;
		use std::sync::Arc;
		use std::mem::transmute;
		use self::num_cpus;

		let num_cores = num_cpus::get() as i64;
		let rows_per_thread = self.maze.height() / num_cores;

		assert!(self.maze.height() % num_cores == 0,
			"Maze height must be divisible by {}, but {} given", num_cores, self.maze.height());

		let maze = Arc::new(&mut *self.maze);
		let mut rng = self.rng.clone();

        let mut threads = Vec::with_capacity(num_cores as usize);

		for tn in 0..num_cores {
			let (x,y,z,w) = (rng.next_u32(), rng.next_u32(), rng.next_u32(), rng.next_u32());
			rng.reseed([w,z,x,y]);

			let maze = maze.clone();
			let mut rng = rng.clone();

			threads.push(thread::scoped(move || {
				let maze: &mut Maze<P> = unsafe { transmute(&**maze) };

				for y in (tn * rows_per_thread)..((tn + 1) * rows_per_thread) {
					let mut run_start = 0;

					for x in 0..maze.width() {
						if y > 0 && (x + 1 == maze.width() || rng.next_f64() > 0.50) {
							let carve_point =
								run_start + (rng.next_f64() * (x - run_start + 1) as f64) as i64;

							unsafe { maze.or_set_unchecked(carve_point, y - 1, S); }
							run_start = x + 1;
						} else if x + 1 < maze.width() {
							unsafe { maze.or_set_unchecked(x, y, E); }
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
