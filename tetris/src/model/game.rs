use sdl2::pixels::Color;

use crate::model::coordinate::{Coordinate};
use crate::model::tetrimino::*;
use crate::{Score, tetrimino};

#[derive(Debug, Clone)]
pub struct Cell {
	is_occupied: bool,
	pub color: Color,
}

type Line = Vec<Cell>;
type Map = Vec<Line>;

const DEFAULT_COLOR: Color = Color::BLACK;
const DEFAULT_COORD: Coordinate = Coordinate { x: 4, y: 0 };
pub const MIN_X_BOUND: u32 = 0;
pub const MAX_X_BOUND: u32 = 10;
pub const MIN_Y_BOUND: u32 = 0;
pub const MAX_Y_BOUND: u32 = 16;

#[derive(Debug)]
pub struct Game {
	map: Map,
	pub score: Score,
	active_tetrimino: Tetrimino,
	active_coordinate: Coordinate,
	pub next_tetrimino: Tetrimino,
}

impl Game {
	pub fn new() -> Game {
		let mut game = Game {
			map: Vec::new(),
			score: Score::new(),
			active_tetrimino: Tetrimino::new(),
			active_coordinate: DEFAULT_COORD,
			next_tetrimino: tetrimino::generate_tetrimino(),
		};

		Game::init(&mut game);

		return game;
	}

	fn new_line() -> Line {
		let mut line: Line = Vec::new();
		for _ in MIN_X_BOUND..MAX_X_BOUND {
			line.push(Cell { is_occupied: false, color: DEFAULT_COLOR });
		}
		return line;
	}

	fn init(&mut self) {
		for _ in MIN_Y_BOUND..MAX_Y_BOUND {
			self.map.push(Game::new_line());
		}
	}

	pub fn get_cell(&self, coord: &Coordinate) -> Result<&Cell, String> {
		if self.check_coordinate(coord) {
			return Ok(&self.map[coord.y as usize][coord.x as usize]);
		}
		Err("Out of bounds".to_string())
	}

	fn set_cell(&mut self, coord: &Coordinate, val: &Cell) {
		self.map[coord.y as usize][coord.x as usize] = val.clone();
	}

	pub fn add_tetrimino(&mut self, tetrimino: Tetrimino) -> bool {
		self.active_tetrimino = tetrimino.clone();
		self.active_coordinate = DEFAULT_COORD;
		if self.check_free_place(&self.active_coordinate, &self.active_tetrimino) {
			self.save_tetrimino(&DEFAULT_COORD, &tetrimino);
			return true;
		}
		false
	}

	fn remove_tetrimino(&mut self, coordinate: &Coordinate, tetrimino: &Tetrimino) {
		for x in 0..tetrimino::SIZE_OF {
			for y in 0..tetrimino::SIZE_OF {
				if tetrimino.get_state()[y][x] {
					self.set_cell(&(*coordinate + Coordinate { x: x as u32, y: y as u32 }),
							  &Cell { is_occupied: false, color: DEFAULT_COLOR });
				}
			}
		}
	}

	fn save_tetrimino(&mut self, coordinate: &Coordinate, tetrimino: &Tetrimino) {
		for x in 0..tetrimino::SIZE_OF {
			for y in 0..tetrimino::SIZE_OF {
				if tetrimino.get_state()[y][x] {
					self.set_cell(&(*coordinate + Coordinate { x: x as u32, y: y as u32 }),
							  &Cell { is_occupied: true, color: self.active_tetrimino.color });
				}
			}
		}
	}

	pub fn remove_full_lines(&mut self) {
		let mut index_line = 0;
		let mut line_to_remove = vec![];

		for line in &self.map {
			let res = line.windows(1).all(|it| true == it[0].is_occupied);

			if res {
				line_to_remove.push(index_line);
			}
			index_line += 1;
		}

		let nb_lines = line_to_remove.len();

		for line in line_to_remove {
			self.map.remove(line as usize);
			self.map.insert(0, Game::new_line());
		}

		self.score.add_line(nb_lines as u32);
	}

	pub fn move_down(&mut self) -> bool {
		let new_coordinate = Coordinate { x: self.active_coordinate.x, y: self.active_coordinate.y + 1 };

		return self.move_tetrimino(&new_coordinate);
	}

	pub fn new_tetrimino(&mut self) -> bool {
		let next = self.next_tetrimino.clone();
		self.next_tetrimino = tetrimino::generate_tetrimino();
		return self.add_tetrimino(next);
	}

	pub fn move_left(&mut self) -> bool {
		if 0 >= self.active_coordinate.x {
			return false;
		}
		let new_coordinate = Coordinate { x: self.active_coordinate.x - 1, y: self.active_coordinate.y };

		self.move_tetrimino(&new_coordinate)
	}

	pub fn move_right(&mut self) -> bool {
		let new_coordinate = Coordinate { x: self.active_coordinate.x + 1, y: self.active_coordinate.y };

		self.move_tetrimino(&new_coordinate)
	}

	pub fn rotate_left(&mut self) -> bool {
		let mut new_tetrimino = self.active_tetrimino.clone();
		new_tetrimino.rotate_left();

		self.remove_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());

		if self.check_free_place(&self.active_coordinate, &new_tetrimino) {
			self.save_tetrimino(&self.active_coordinate.clone(), &new_tetrimino);
			self.active_tetrimino = new_tetrimino;
			true
		} else {
			self.save_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());
			false
		}
	}

	/*
	 * never used
	 *
	pub fn rotate_right(&mut self) -> bool {
		let mut new_tetrimino = self.active_tetrimino.clone();
		new_tetrimino.rotate_right();
		
		self.remove_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());
		
		if self.check_free_place(&self.active_coordinate, &new_tetrimino) {
			self.active_tetrimino = new_tetrimino;
			self.save_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());
			true
		} else {
			self.save_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());
			false
		}
	}
	*/

	fn move_tetrimino(&mut self, new_coordinate: &Coordinate) -> bool {
		self.remove_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());

		if self.check_free_place(&new_coordinate, &self.active_tetrimino) {
			self.save_tetrimino(new_coordinate, &self.active_tetrimino.clone());
			self.active_coordinate = *new_coordinate;
			true
		} else {
			self.save_tetrimino(&self.active_coordinate.clone(), &self.active_tetrimino.clone());
			false
		}
	}

	fn check_free_place(&self, coordinate: &Coordinate, tetrimino: &Tetrimino) -> bool {
		for x in 0..4 {
			for y in 0..4 {
				if tetrimino.get_state()[y][x] {
					match self.get_cell(&(*coordinate + Coordinate { x: x as u32, y: y as u32 })) {
						Ok(cell) => if cell.is_occupied {
							return false;
						},
						Err(_) => return false,
					}
				}
			}
		}
		true
	}

	fn check_coordinate(&self, coordinate: &Coordinate) -> bool {
		coordinate.x < MAX_X_BOUND && coordinate.y < MAX_Y_BOUND
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn run_tests() {}
}