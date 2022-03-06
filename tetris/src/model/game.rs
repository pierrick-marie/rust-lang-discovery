pub mod game {
	use std::fmt::{Display, Formatter};
	use sdl2::init;
	use sdl2::libc::uinput_user_dev;
	use crate::controler::controler::generate_tetrimino;
	use crate::model::tetrimino::tetrimino::*;
	
	type Line = Vec<bool>;
	type Map = Vec<Line>;
	
	const X_BOUNDS: usize = 10;
	const Y_BOUNDS: usize = 16;
	
	#[derive(Debug)]
	pub struct Game {
		map: Map,
		tetriminos: Vec<Tetrimino>,
	}
	
	#[derive(Debug)]
	pub struct Coordinate {
		pub x: usize,
		pub y: usize,
	}
	
	impl Display for Coordinate {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "({}, {})", self.x, self.y)
		}
	}
	
	impl Coordinate {
		fn check_coordinate(&self) -> bool {
			0 <= self.x && self.x < X_BOUNDS && 0 <= self.y && self.y < Y_BOUNDS
		}
	}
	
	pub trait GameFunction {
		fn get_value(&self, coord: &Coordinate) -> Result<bool, String>;
		fn switch_value(&mut self, coord: &Coordinate) -> Result<bool, String>;
	}
	
	impl Game  {
		pub fn new() -> Game {
			let mut game = Game {
				map: Vec::new(),
				tetriminos: Vec::new(),
			};
			
			Game::init(&mut game);
			
			return game;
		}
		
		fn init(game: &mut Game) {
			let mut line: Line = Vec::new();
			for i in 0..X_BOUNDS {
				line.push(false);
			}
			for i in 0..Y_BOUNDS {
				game.map.push(line.clone());
			}
		}
	}
	
	impl GameFunction for Game {
		fn get_value(&self, coord: &Coordinate) -> Result<bool, String> {
			if Coordinate::check_coordinate(coord) {
				return Ok(self.map[coord.y][coord.x]);
			}
			Err("Out of bounds".to_string())
		}
		
		fn switch_value(&mut self, coord: &Coordinate) -> Result<bool, String> {
			if Coordinate::check_coordinate(coord) {
				self.map[coord.y][coord.x] = !self.map[coord.y][coord.x];
				return Ok(self.map[coord.y][coord.x]);
			}
			Err("Out of bounds".to_string())
		}
	}
}