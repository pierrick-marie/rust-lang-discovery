pub mod tetrimino {
	use std::fmt::{Display, Formatter, write};
	use sdl2::pixels::Color;
	
	type Orientation = Vec<Vec<bool>>;
	type States = Vec<Orientation>;
	
	#[derive(Debug)]
	pub struct Tetrimino {
		states: States,
		x: u32,
		y: u32,
		current_state: u8,
		name: String,
		color: Color,
	}
	
	impl Display for Tetrimino {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "Tetrimino name: {} ; \
			color: (R:{:0>3}, G:{:0>3}, B:{:0>3}) ; \
			current state: {} ; \
			position: ({} {})",
			       self.name,
			       self.color.r, self.color.g, self.color.b,
			       self.current_state,
			       self.x, self.y)
		}
	}
	
	impl Tetrimino {
		pub fn rotate(&mut self) {
			self.current_state += 1;
			if self.current_state as usize >= self.states.len() {
				self.current_state = 0;
			}
		}
	}
	
	pub trait TetriminoGenerator {
		fn new() -> Tetrimino;
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
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("I"),
				color: Color::CYAN,
			}
		}
	}
	
	pub struct TetriminoL;
	impl TetriminoGenerator for TetriminoL {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![false, true, false, false],
				                  vec![false, true, false, false],
				                  vec![false, true, true, false]],
				             vec![vec![false, false, false, false],
				                  vec![false, false, false, false],
				                  vec![false, false, false, true],
				                  vec![false, true, true, true]],
				             vec![vec![false, false, false, false],
				                  vec![false, true, true, false],
				                  vec![false, false, true, false],
				                  vec![false, false, true, false]],
				             vec![vec![false, false, false, false],
				                  vec![true, true, true, false],
				                  vec![false, false, true, false],
				                  vec![false, false, false, false]],
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("L"),
				color: Color::RGB(255, 165, 0), // Orange
			}
		}
	}
	
	pub struct TetriminoJ;
	impl TetriminoGenerator for TetriminoJ {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![false, true, false, false],
				                  vec![false, true, false, false],
				                  vec![true, true, false, false]],
				             vec![vec![false, false, false, false],
				                  vec![false, false, false, false],
				                  vec![false, true, true, true],
				                  vec![false, false, false, true]],
				             vec![vec![false, false, false, false],
				                  vec![false, true, true, false],
				                  vec![false, true, false, false],
				                  vec![false, true, false, false]],
				             vec![vec![false, false, false, false],
				                  vec![true, true, true, false],
				                  vec![true, false, false, false],
				                  vec![false, false, false, false]],
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("J"),
				color: Color::RED,
			}
		}
	}
	
	pub struct TetriminoO;
	impl TetriminoGenerator for TetriminoO {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![false, true, true, false],
				                  vec![false, true, true, false],
				                  vec![false, false, false, false]]
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("O"),
				color: Color::YELLOW,
			}
		}
	}
	
	pub struct TetriminoS;
	impl TetriminoGenerator for TetriminoS {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![false, false, true, true],
				                  vec![false, true, true, false],
				                  vec![false, false, false, false]],
				             vec![vec![false, true, false, false],
				                  vec![false, true, true, false],
				                  vec![false, false, true, false],
				                  vec![false, false, false, false]],
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("S"),
				color: Color::RGB(255, 192, 203), // Pink
			}
		}
	}
	
	pub struct TetriminoZ;
	impl TetriminoGenerator for TetriminoZ {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![true, true, false, false],
				                  vec![false, true, true, false],
				                  vec![false, false, false, false]],
				             vec![vec![false, false, true, false],
				                  vec![false, true, true, false],
				                  vec![false, true, false, false],
				                  vec![false, false, false, false]],
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("Z"),
				color: Color::GREEN,
			}
		}
	}
	
	pub struct TetriminoT;
	impl TetriminoGenerator for TetriminoT {
		fn new() -> Tetrimino {
			Tetrimino {
				states: vec![vec![vec![false, false, false, false],
				                  vec![false, true, false, false],
				                  vec![true, true, true, false],
				                  vec![false, false, false, false]],
				             vec![vec![false, false, true, false],
				                  vec![false, true, true, false],
				                  vec![false, false, true, false],
				                  vec![false, false, false, false]],
				             vec![vec![false, false, false, false],
				                  vec![true, true, true, false],
				                  vec![false, true, false, false],
				                  vec![false, false, false, false]],
				             vec![vec![true, false, false, false],
				                  vec![true, true, false, false],
				                  vec![true, false, false, false],
				                  vec![false, false, false, false]],
				],
				x: 0,
				y: 0,
				current_state: 0,
				name: String::from("T"),
				color: Color::RGB(128, 0, 128), // Purple
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::model::tetrimino::tetrimino::*;
	
	#[test]
	fn tests() {
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