mod sidewinder_generator;
mod naive_sidewinder_generator;
mod recursive_backtrack_generator;
mod stack_backtrack_generator;
mod recursive_division_generator;
mod stack_division_generator;

pub use self::sidewinder_generator::SidewinderGenerator;
pub use self::naive_sidewinder_generator::NaiveSidewinderGenerator;
pub use self::recursive_backtrack_generator::RecursiveBacktrackGenerator;
pub use self::stack_backtrack_generator::StackBacktrackGenerator;
pub use self::recursive_division_generator::RecursiveDivisionGenerator;
pub use self::stack_division_generator::StackDivisionGenerator;

mod utils {
	extern crate rand;

	use ::lcg_rng::LCGRng;
	use self::rand::SeedableRng;

	pub fn init_rng(seed: Option<&[u32]>) -> LCGRng {
		if let Some(s) = seed {
			LCGRng::from_seed((s[0] as u64) + (s[1] as u64) << 32)
		} else {
			LCGRng::new_unseeded()
		}
	}
}
