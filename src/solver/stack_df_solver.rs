use ::{Grid, Solver, SolverSolution};
use ::utils::Direction;

pub struct StackDFSolver<'a, G: 'a + Grid> {
	grid: &'a G,
	path: SolverSolution
}

impl<'a, G: 'a + Grid> StackDFSolver<'a, G> {
	pub fn new(grid: &'a G) -> Self {
		StackDFSolver {
			grid: grid,
			path: SolverSolution::new()
		}
	}

	fn solve_at(&mut self, x: u64, y: u64) -> bool {
		use ::utils::Direction::*;

		let mut stack = Vec::new();

		stack.push((x as i64, y as i64, (0,)));

		'stack_loop: while !stack.is_empty() {
			let (x, y, (mut i,)) = stack.pop().unwrap();

			if x as u64 == self.grid.width() - 1 && y as u64 == self.grid.height() - 1 {
				return true
			} else if x >= 0 && (x as u64) < self.grid.width()
				&& y >= 0 && (y as u64) < self.grid.height() {
				while i < Direction::enumerate().len() {
					let dir = Direction::enumerate()[i];

					if dir != self.path.last().unwrap_or(&S).opposite()
						&& self.grid.test(x as u64, y as u64, dir as u8) {
						
						let (dx, dy) = match dir {
							S => (0, 1),
							E => (1, 0),
							N => (0, -1),
							W => (-1, 0)
						};

						self.path.push(dir);

						stack.push((x, y, (i + 1,)));
						stack.push((x + dx, y + dy, (0,)));
						continue 'stack_loop;
					}

					i += 1;
				}
			}
			
			self.path.pop();
		}

		false		
	}
}

impl<'a, G: 'a + Grid> Solver for StackDFSolver<'a, G> {
	fn solve(mut self) -> Option<SolverSolution> {
		let has_path = self.solve_at(0, 0);

		if has_path {
			Some(self.path)
		} else {
			None
		}
	}
}
