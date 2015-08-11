#![feature(test)]
extern crate test;
extern crate maze;

macro_rules! bench_build_maze {
	($b:expr, $width:expr, $height:expr, $gen:expr) => {
		$b.iter(|| {
			let _ = MazeBuilder::new()
					.width($width)
					.height($height)
					.generate_using($gen)
					.build::<InMemoryPackedGrid>(&[]);
		})
	};
	($b:expr, $width:expr, $height:expr, $gen:expr, $mem:ty, $options:expr) => {
		$b.iter(|| {
			let _ = MazeBuilder::new()
					.width($width)
					.height($height)
					.generate_using($gen)
					.build::<$mem>($options);
		})
	}
}

mod benches {
	use maze::*;
	use test::Bencher;

	#[bench]
	fn memory_32_32_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::Sidewinder);
	}

	#[bench]
	fn memory_32_32_parallel_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::ParallelSidewinder);
	}

	#[bench]
	fn memory_32_32_recursive_division(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::RecursiveDivision);
	}

	#[bench]
	fn memory_32_32_stack_division(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::StackDivision);
	}

	#[bench]
	fn memory_32_32_stack_backtrack(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::StackBacktrack);
	}



	#[bench]
	fn memory_1024_1024_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::Sidewinder);
	}

	#[bench]
	fn memory_1024_1024_parallel_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::ParallelSidewinder);
	}

	#[bench]
	fn memory_1024_1024_recursive_division(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::RecursiveDivision);
	}

	#[bench]
	fn memory_1024_1024_stack_division(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::StackDivision);
	}

	#[bench]
	fn memory_1024_1024_stack_backtrack(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::StackBacktrack);
	}



	#[bench]
	fn mmap_32_32_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::Sidewinder, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 32, 32))]);
	}

	#[bench]
	fn mmap_32_32_parallel_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::ParallelSidewinder, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 32, 32))]);
	}

	#[bench]
	fn mmap_32_32_recursive_division(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::RecursiveDivision, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 32, 32))]);
	}

	#[bench]
	fn mmap_32_32_stack_division(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::StackDivision, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 32, 32))]);
	}

	#[bench]
	fn mmap_32_32_stack_backtrack(b: &mut Bencher) {
		bench_build_maze!(b, 32, 32, GeneratorType::StackBacktrack, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 32, 32))]);
	}



	#[bench]
	fn mmap_1024_1024_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::Sidewinder, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 1024, 1024))]);
	}

	#[bench]
	fn mmap_1024_1024_parallel_sidewinder(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::ParallelSidewinder, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 1024, 1024))]);
	}

	#[bench]
	fn mmap_1024_1024_recursive_division(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::RecursiveDivision, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 1024, 1024))]);
	}

	#[bench]
	fn mmap_1024_1024_stack_division(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::StackDivision, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 1024, 1024))]);
	}

	#[bench]
	fn mmap_1024_1024_stack_backtrack(b: &mut Bencher) {
		bench_build_maze!(b, 1024, 1024, GeneratorType::StackBacktrack, MMAPPackedGrid,
			&[PackedOption::MMAPFilePath(format!("maze_{}x{}.bin", 1024, 1024))]);
	}
}
