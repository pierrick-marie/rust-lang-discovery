use std::fmt::{Display, Formatter, write};
use sdl2::pixels::Color;
use crate::model::coordinate;
use crate::model::coordinate::Coordinate;

type Orientation = Vec<Vec<bool>>;
type States = Vec<Orientation>;

#[derive(Debug, Clone)]
pub struct Tetrimino {
	states: States,
	// pub coordinate: Coordinate,
	pub current_state: u8,
	pub color: Color,
	pub name: String,
}

impl Display for Tetrimino {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "name: {} ; \
			color: (R:{:0>3}, G:{:0>3}, B:{:0>3}) ; \
			current state: {}", // ; \
		       // position: {}",
		       self.name,
		       self.color.r, self.color.g, self.color.b,
		       self.current_state) //,
		// self.coordinate)
	}
}

impl Tetrimino {
	pub fn rotate_right(&mut self) {
		if self.current_state as usize >= (self.states.len() - 1) {
			self.current_state = 0;
		} else {
			// current_state < states.len()
			self.current_state += 1;
		}
		// self.update_coordinate();
	}
	
	pub fn rotate_left(&mut self) {
		if self.current_state > 0 {
			self.current_state -= 1;
		} else {
			// current_state == 0
			self.current_state = (self.states.len() - 1) as u8;
		}
		// self.update_coordinate();
	}
	
	// fn update_coordinate(&mut self) {
	// 	for y in 0..3 {
	// 		for x in 0..3 {
	// 			if self.states[self.current_state as usize][y][x] {
	// 				self.coordinate.x = x;
	// 				self.coordinate.y = y;
	// 				return;
	// 			}
	// 		}
	// 	}
	// }
	
	pub fn move_left(&mut self) {
		// if self.coordinate.x > 0 {
		// 	self.coordinate.x -= 1;
		// }
	}
	
	pub fn move_right(&mut self) {
		// match self.coordinate.x  {
		// 	coordinate::X_BOUNDS => self.coordinate.x += 1,
		// 	_ => println!("Out of bounds"),
		// }
	}
	
	pub fn move_down(&mut self) {
		// if self.coordinate.y < coordinate::Y_BOUNDS {
		// 	self.coordinate.y += 1;
		// }
	}
	
	pub fn get_state(&self) -> &Orientation {
		&self.states[self.current_state as usize]
	}
}

pub fn generate_tetrimino() -> Tetrimino {
	static mut PREV: u8 = 7;
	let mut rand_nb = rand::random::<u8>() % 7;
	
	while unsafe { PREV } == rand_nb {
		rand_nb = rand::random::<u8>() % 7;
	}
	unsafe { PREV = rand_nb }
	
	let mut tetrimino = match rand_nb {
		0 => TetriminoI::new(),
		1 => TetriminoJ::new(),
		2 => TetriminoL::new(),
		3 => TetriminoO::new(),
		4 => TetriminoS::new(),
		5 => TetriminoZ::new(),
		6 => TetriminoT::new(),
		_ => unreachable!(),
	};
	
	while unsafe { PREV } == rand_nb {
		rand_nb = rand::random::<u8>() % tetrimino.states.len() as u8;
	}
	unsafe { PREV = rand_nb };
	for i in 0..rand_nb {
		tetrimino.rotate_left();
	};
	
	tetrimino
}

pub trait TetriminoGenerator {
	fn new() -> Tetrimino;
}

impl TetriminoGenerator for Tetrimino {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![]]],
			name: "Tetrimino".to_string(),
			// coordinate: Coordinate { x: 0, y: 0 },
			current_state: 0,
			color: Color::WHITE,
		}
	}
}

pub struct TetriminoI;
impl TetriminoGenerator for TetriminoI {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![true, true, true, true],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, true, false, false]],
			],
			name: "TetriminoI".to_string(),
			// coordinate: Coordinate { x: 0, y: 0 },
			current_state: 0,
			color: Color::CYAN,
		}
	}
}

pub struct TetriminoL;
impl TetriminoGenerator for TetriminoL {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, true, true, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, false, false, true],
			                  vec![false, true, true, true],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, true, false],
			                  vec![false, false, true, false],
			                  vec![false, false, true, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, true, false],
			                  vec![true, false, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoL".to_string(),
			// coordinate: Coordinate { x: 1, y: 0 },
			current_state: 0,
			color: Color::RGB(255, 165, 0), // Orange
		}
	}
}

pub struct TetriminoJ;
impl TetriminoGenerator for TetriminoJ {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![true, true, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, true, true],
			                  vec![false, false, false, true],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, true, false],
			                  vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, false, false, false],
			                  vec![true, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoJ".to_string(),
			// coordinate: Coordinate { x: 1, y: 0 },
			current_state: 0,
			color: Color::RED,
		}
	}
}

pub struct TetriminoO;
impl TetriminoGenerator for TetriminoO {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, true, true, false],
			                  vec![false, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]]
			],
			name: "TetriminoO".to_string(),
			// coordinate: Coordinate { x: 1, y: 0 },
			current_state: 0,
			color: Color::YELLOW,
		}
	}
}

pub struct TetriminoS;
impl TetriminoGenerator for TetriminoS {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, false, true, true],
			                  vec![false, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, false, false],
			                  vec![false, true, true, false],
			                  vec![false, false, true, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoS".to_string(),
			// coordinate: Coordinate { x: 2, y: 0 },
			current_state: 0,
			color: Color::RGB(255, 192, 203), // Pink
		}
	}
}

pub struct TetriminoZ;
impl TetriminoGenerator for TetriminoZ {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![true, true, false, false],
			                  vec![false, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, false, true, false],
			                  vec![false, true, true, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoZ".to_string(),
			// coordinate: Coordinate { x: 1, y: 0 },
			current_state: 0,
			color: Color::GREEN,
		}
	}
}

pub struct TetriminoT;
impl TetriminoGenerator for TetriminoT {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, true, false, false],
			                  vec![true, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, false, true, false],
			                  vec![false, true, true, false],
			                  vec![false, false, true, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, true, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, true, false, false],
			                  vec![false, true, true, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoT".to_string(),
			// coordinate: Coordinate { x: 1, y: 0 },
			current_state: 0,
			color: Color::RGB(128, 0, 128), // Purple
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::model::tetrimino::*;
	
	#[test]
	fn run_tests() {
		test_constructors();
		test_generate_random();
	}
	
	fn test_generate_random() {
		for i in 0..10 {
			println!("{}", generate_tetrimino());
		}
	}
	
	fn test_constructors() {
		let i: Tetrimino = TetriminoI::new();
		// println!("{:#?}", i);
		
		let j: Tetrimino = TetriminoJ::new();
		// println!("{:#?}", j);
		
		let l: Tetrimino = TetriminoL::new();
		// println!("{:#?}", l);
		
		let o: Tetrimino = TetriminoO::new();
		// println!("{:#?}", o);
		
		let s: Tetrimino = TetriminoS::new();
		// println!("{:#?}", s);
		
		let z: Tetrimino = TetriminoZ::new();
		// println!("{:#?}", z);
		
		let t: Tetrimino = TetriminoT::new();
		// println!("{:#?}", t);
	}
}