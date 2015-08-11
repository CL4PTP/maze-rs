pub use self::sidewinder_generator::SidewinderGenerator;

mod sidewinder_generator;

mod utils {
	pub enum Orientation {
		Horizontal,
		Vertical
	}
}

pub enum GeneratorOption {
	Seed(&'static [u32])
}

pub trait Generator {
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
