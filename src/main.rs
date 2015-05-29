extern crate maze;

use std::env;
use std::str::FromStr;
use maze::*;

fn main() {
	let mut args = env::args().skip(1);
	
	let width: i64 = args.next().map_or(16, |v| FromStr::from_str(&v).unwrap());
	let height: i64 = args.next().map_or(16, |v| FromStr::from_str(&v).unwrap());
	let seed: u64 = args.next().map_or(0, |v| FromStr::from_str(&v).unwrap());

	let _maze = MazeBuilder::new()
		.width(width)
		.height(height)
		.seed(seed)
		// .backing_type(BackingType::MMAP(format!("maze_{}x{}.bin", width, height)))
		.backing_type(BackingType::InMemory)
		// .generate_using(GeneratorType::StackDivision)
		.generate_using(GeneratorType::Sidewinder)
		.build();
	
	println!("{}x{}", width, height);
	println!("{}", _maze);
}
