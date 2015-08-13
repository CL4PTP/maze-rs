extern crate libc;

use ::{GeneratorType, generate, Grid};
use ::grid::mmap_packed_grid::MMAPPackedGrid;
use self::libc::*;
use std::mem::transmute;
use std::ffi::CStr;
use std::fs::OpenOptions;

#[no_mangle]
pub unsafe extern "C"
fn maze_create(filepath: *const c_char, width: uint64_t, height: uint64_t) -> *mut c_void {
	use PackedOption::*;

	let maze = Box::new(MMAPPackedGrid::new(&[
		MMAPFilePath(String::from(CStr::from_ptr(filepath).to_str().unwrap())),
		Width(width),
		Height(height)
	]));

	transmute(Box::into_raw(maze))
}

#[no_mangle]
pub unsafe extern "C"
fn maze_open(filepath: *const c_char) -> *mut c_void {
	let filepath_str = CStr::from_ptr(filepath).to_str().unwrap();

	let maze = Box::new(
			MMAPPackedGrid::from_file(
				OpenOptions::new().read(true).write(true).open(&filepath_str).unwrap()
			)
		);

	transmute(Box::into_raw(maze))
}

#[no_mangle]
pub unsafe extern "C"
fn maze_free(maze: *mut c_void) {
	let _: Box<MMAPPackedGrid> = Box::from_raw(transmute(maze));

	// NOTE: the box will drop here
}

#[no_mangle]
pub unsafe extern "C"
fn maze_get(maze: *mut c_void, x: uint64_t, y: uint64_t) -> uint8_t {
	let maze: &mut MMAPPackedGrid = transmute(maze);

	maze.get(x, y)
}

#[no_mangle]
pub unsafe extern "C"
fn maze_generate(maze: *mut c_void, generator_type: GeneratorType) {
	let maze: &mut MMAPPackedGrid = transmute(maze);

	generate(maze, generator_type, &[]);
}

#[no_mangle]
pub unsafe extern "C"
fn maze_width(maze: *mut c_void) -> uint64_t {
	let maze: &mut MMAPPackedGrid = transmute(maze);

	maze.width()
}

#[no_mangle]
pub unsafe extern "C"
fn maze_height(maze: *mut c_void) -> uint64_t {
	let maze: &mut MMAPPackedGrid = transmute(maze);

	maze.height()
}
