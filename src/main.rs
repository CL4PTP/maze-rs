extern crate maze;

use std::env;
use std::str::FromStr;
use maze::*;

fn main() {
	let mut args = env::args().skip(1);
	
	let width: i64 = args.next().map_or(32, |v| FromStr::from_str(&v).unwrap());
	let height: i64 = args.next().map_or(32, |v| FromStr::from_str(&v).unwrap());

	let _maze = MazeBuilder::new()
		.width(width)
		.height(height)
		.generate_using(GeneratorType::Sidewinder)
		.build::<MMAPPackedArray>(&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", width, height))]);
		// .build::<InMemoryPackedArray>(&[]);
	
	// println!("{}x{}", width, height);
	// println!("{}", _maze);
}
