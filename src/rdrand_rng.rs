extern crate rand;

use self::rand::{Rng, Rand};

#[derive(Clone, Copy)]
pub struct RDRandRng;

unsafe impl Send for RDRandRng {}
unsafe impl Sync for RDRandRng {}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn rdrand_u32() -> u32 {
	let out: u32;

	asm!("rdrand $0;" : "=r"(out) ::: "volatile");

	out
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn rdrand_u64() -> u64 {
	let out: u64;

	asm!("rdrand $0;" : "=r"(out) ::: "volatile");

	out
}



#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
unsafe fn rdrand_u32() -> u32 {
	panic!("RDRAND not supported on non x86/x86_64 architectures")
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
unsafe fn rdrand_u64() -> u64 {
	panic!("RDRAND not supported on non x86/x86_64 architectures")
}

impl RDRandRng {
	pub fn new_unseeded() -> RDRandRng {
		RDRandRng
	}
}

impl Rng for RDRandRng {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		unsafe { rdrand_u32() }
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		unsafe { rdrand_u64() }
	}
}

impl Rand for RDRandRng {
	fn rand<R: Rng>(_: &mut R) -> RDRandRng {
		RDRandRng
	}
}
