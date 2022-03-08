use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::thread::current;
use sdl2::libc::{termios, uinput_user_dev};
use sdl2::pixels::Color;

use crate::model::coordinate;
use crate::model::coordinate::{Coordinate};
use crate::model::tetrimino::*;

#[derive(Debug, Clone)]
pub struct Cell {
	is_occupied: bool,
	pub color: Color,
}

type Line = Vec<Cell>;
type Map = Vec<Line>;

const DEFAULT_COLOR: Color = Color::BLACK;

pub const MIN_X_BOUND: usize = 0;
pub const MAX_X_BOUND: usize = 10;
pub const MIN_Y_BOUND: usize = 0;
pub const MAX_Y_BOUND: usize = 16;

#[derive(Debug)]
pub struct Game {
	map: Map,
	active_tetrimino: Tetrimino,
	active_coordinate: Coordinate,
}

impl Game {
	pub fn new() -> Game {
		let mut game = Game {
			map: Vec::new(),
			active_tetrimino: Tetrimino::new(),
			active_coordinate: Coordinate { x: 3, y: 0 },
		};
		
		Game::init(&mut game);
		
		return game;
	}
	
	fn init(game: &mut Game) {
		let mut line: Line = Vec::new();
		for i in MIN_X_BOUND..MAX_X_BOUND {
			line.push(Cell { is_occupied: false, color: DEFAULT_COLOR });
		}
		for i in MIN_Y_BOUND..MAX_Y_BOUND {
			game.map.push(line.clone());
		}
	}
	
	pub fn get_cell(&self, coord: &Coordinate) -> Result<&Cell, String> {
		if self.check_coordinate(coord) {
			return Ok(&self.map[coord.y][coord.x]);
		}
		Err("Out of bounds".to_string())
	}
	
	fn set_cell(&mut self, coord: &Coordinate, val: &Cell) {
		self.map[coord.y][coord.x] = val.clone();
	}
	
	pub fn add_tetrimino(&mut self, tetrimino: &Tetrimino) -> bool {
		if self.check_free_place(&tetrimino) {
			for x in 0..3 {
				for y in 0..3 {
					if tetrimino.get_state()[y][x] {
						self.set_cell(&( self.active_coordinate + Coordinate{x, y}),
						              &Cell { is_occupied: true, color: tetrimino.color });
					}
				}
			}
			println!("Tetrimino added");
			return true;
		}
		println!("Tetrimino NOT added");
		false
	}
	
	pub fn can_move(&self, tetrimino: &mut Tetrimino, f: fn(&mut Tetrimino)) -> bool {
		false
	}
	
	fn check_free_place(&self, tetrimino: &Tetrimino) -> bool {
		for x in 0..3 {
			for y in 0..3 {
				if tetrimino.get_state()[y][x] {
					println!("tetrimino ({},{}) is true", y, x);
					match self.get_cell(&(self.active_coordinate + Coordinate { x, y })) {
						Ok(cell) => if cell.is_occupied {
							println!("game is true");
							return false;
						},
						Err(message) => return false,
					}
				}
			}
		}
		true
	}
	
	fn check_coordinate(&self, coordinate: &Coordinate) -> bool {
		0 <= coordinate.x && coordinate.x < MAX_X_BOUND && 0 <= coordinate.y && coordinate.y < MAX_Y_BOUND
	}
}

#[cfg(test)]
mod tests {
	use crate::model::game::*;
	
	#[test]
	fn run_tests() {
		test_bounds();
		// test_generate_random();
	}
	
	fn test_bounds() {
		let mut coordinate = Coordinate { x: 0, y: 0 };
		assert!(check_coordinate(&coordinate));
		
		coordinate.x = MAX_X_BOUND - 1;
		coordinate.y = MAX_Y_BOUND - 1;
		assert!(check_coordinate(&coordinate));
		
		coordinate = coordinate + Coordinate { x: 1, y: 0 };
		assert_eq!(false, check_coordinate(&coordinate));
		
		coordinate.x = MAX_X_BOUND - 1;
		coordinate.y = MAX_Y_BOUND - 1;
		assert!(check_coordinate(&coordinate));
		
		coordinate = coordinate + Coordinate { x: 0, y: 1 };
		assert_eq!(false, check_coordinate(&coordinate));
		
		println!("Test bounds: OK");
	}
}