#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize; //needed by docopt_macros
extern crate regex;

extern crate maze;

use std::env;
use std::str::FromStr;
use maze::*;
use maze::grid::mmap_packed_grid::MMAPPackedGrid;
use maze::solver::recursive_backtrack_solver::RecursiveDFSolver;

docopt!(Args derive Debug, "
Usage:
  maze generate <width> <height> [<location>] [--print]
  maze (--help | --version)

Options:
  --help  Show this message.
  --version   Show version.
");

fn main() {
	use maze::PackedOption::*;

	let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
	
	if args.cmd_generate {
		let width: u64 = FromStr::from_str(&args.arg_width).unwrap_or(32);
		let height: u64 = FromStr::from_str(&args.arg_height).unwrap_or(32);
		let path = FromStr::from_str(&args.arg_location).unwrap_or(String::from(env::current_dir().unwrap().to_str().unwrap()));

		let mut _maze = MMAPPackedGrid::new(&[
			MMAPFilePath(format!("{}/maze_{}x{}.bin", path, width, height)),
			Width(width),
			Height(height)
		]);

		generate(&mut _maze, GeneratorType::Sidewinder, &[]);

		let solver = RecursiveDFSolver::new(&_maze);
		let directions = solver.solve();

		if args.flag_print {
			println!("{}", _maze.to_string());
			println!("Solution: {:?}", directions);
		}
	} else if args.flag_version {
		println!("{}", env!("CARGO_PKG_VERSION"));
	}
}
