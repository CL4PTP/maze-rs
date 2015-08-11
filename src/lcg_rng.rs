extern crate rand;

use std::num::Wrapping as w;
use self::rand::{Rng, SeedableRng, Rand};

#[derive(Clone)]
pub struct LCGRng (w<u64>);

impl LCGRng {
	pub fn new_unseeded() -> LCGRng {
		LCGRng(w(0))
	}
}

impl Rng for LCGRng {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		self.next_u64() as u32
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		self.0 = self.0 * w(6364136223846793005) + w(1442695040888963407);
		(self.0).0
	}
}

impl SeedableRng<u64> for LCGRng {
	fn reseed(&mut self, seed: u64) {
		self.0 = w(seed);
	}

	fn from_seed(seed: u64) -> LCGRng {
		LCGRng(w(seed))
	}
}

impl Rand for LCGRng {
	fn rand<R: Rng>(rng: &mut R) -> LCGRng {
		LCGRng(w(rng.next_u64()))
	}
}
