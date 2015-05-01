mod maze;

use std::env;
use std::str::FromStr;
use maze::*;

fn main() {
	let mut args = env::args().skip(1);
	
	let width: i64 = args.next().map_or(20, |v| FromStr::from_str(&v).unwrap());
	let height: i64 = args.next().map_or(20, |v| FromStr::from_str(&v).unwrap());
	
	let mut maze = Maze2D::new(width, height);

	maze.carve(MazeGenerationType::RecursiveBacktrack);
	
	println!("{}x{}", width, height);
	println!("{}", maze);
}
