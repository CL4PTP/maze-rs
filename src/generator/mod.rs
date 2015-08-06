pub use self::sidewinder_generator::SidewinderGenerator;

mod sidewinder_generator;

mod utils {
	pub enum Orientation {
		Horizontal,
		Vertical
	}
}

pub const S: u8 = 1;
pub const E: u8 = 2;

pub trait Generator {
	fn set_seed(&mut self, seed: [u32; 4]);

	fn generate(&mut self);
}

// mod naive_sidewinder_generator;
// mod recursive_backtrack_generator;
// mod recursive_division_generator;
// mod stack_backtrack_generator;
// mod stack_division_generator;

// pub use naive_sidewinder_generator::*;
// pub use recursive_backtrack_generator::*;
// pub use recursive_division_generator::*;
// pub use stack_backtrack_generator::*;
// pub use stack_division_generator::*;
