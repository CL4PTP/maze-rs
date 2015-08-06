#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize; //needed by docopt_macros
extern crate regex;

extern crate maze;

use std::env;
use std::str::FromStr;
use maze::*;

docopt!(Args derive Debug, "
Usage:
  maze generate <width> <height> [<location>]
  maze (--help | --version)

Options:
  --help  Show this message.
  --version   Show version.
");

fn main() {
	let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
	
	if args.cmd_generate {
		let width: i64 = FromStr::from_str(&args.arg_width).unwrap_or(32);
		let height: i64 = FromStr::from_str(&args.arg_height).unwrap_or(32);
		let path = FromStr::from_str(&args.arg_location).unwrap_or(String::from(env::current_dir().unwrap().to_str().unwrap()));

		let _maze = MazeBuilder::new()
			.width(width)
			.height(height)
			.generate_using(GeneratorType::ParallelSidewinder)
			.build::<MMAPPackedArray>(&[PackedOption::MMAPFilePath(format!("{}/maze_{}x{}.bin", path, width, height))]);
	} else if args.flag_version {
		println!("{}", env!("CARGO_PKG_VERSION"));
	}
}
