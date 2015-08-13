pub mod recursive_backtrack_solver {
	use ::{Grid, Solver, SolverSolution};
	use ::utils::Direction;
	use std::cell::RefCell;

	pub struct RecursiveDFSolver<'a, G: 'a + Grid> {
		grid: &'a G,
		path: SolverSolution
	}

	impl<'a, G: 'a + Grid> RecursiveDFSolver<'a, G> {
		pub fn new(grid: &'a G) -> Self {
			RecursiveDFSolver {
				grid: grid,
				path: SolverSolution::new()
			}
		}

		fn solve_at(&mut self, x: i64, y: i64) -> bool {
			use ::utils::Direction::*;

			if x as u64 == self.grid.width() - 1 && y as u64 == self.grid.height() - 1 {
				true
			} else if x < 0 || x as u64 >= self.grid.width()
				|| y < 0 || y as u64 >= self.grid.height() {
				false
			} else {
				for &dir in Direction::enumerate() {
					if dir != self.path.last().unwrap_or(&S).opposite()
						&& self.grid.test(x as u64, y as u64, dir as u8) {

						let (dx, dy) = match dir {
							S => (0, 1),
							E => (1, 0),
							N => (0, -1),
							W => (-1, 0)
						};

						self.path.push(dir);
						
						let has_path = self.solve_at(x + dx, y + dy);
						if has_path {
							return true;
						}
						
						self.path.pop();
					}
				}

				false
			}
		}
	}

	impl<'a, G: 'a + Grid> Solver for RecursiveDFSolver<'a, G> {
		fn solve(mut self) -> Option<SolverSolution> {
			let has_path = self.solve_at(0, 0);

			if has_path {
				Some(self.path)
			} else {
				None
			}
		}
	}
}