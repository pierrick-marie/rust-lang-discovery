use std::fmt::{Display, Formatter};
use sdl2::pixels::Color;

type Orientation = Vec<Vec<bool>>;
type States = Vec<Orientation>;

pub const SIZE_OF: usize = 4;

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
	}
}

impl Tetrimino {
	/*
	 * never used
	 *
	pub fn rotate_right(&mut self) {
		if self.current_state as usize >= (self.states.len() - 1) {
			self.current_state = 0;
		} else {
			self.current_state += 1;
		}
	}
	 */
	
	pub fn rotate_left(&mut self) {
		if self.current_state > 0 {
			self.current_state -= 1;
		} else {
			self.current_state = (self.states.len() - 1) as u8;
		}
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
	for _ in 0..rand_nb {
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
			             vec![vec![true, false, false, false],
			                  vec![true, false, false, false],
			                  vec![true, false, false, false],
			                  vec![true, false, false, false]],
			],
			name: "TetriminoI".to_string(),
			current_state: 0,
			color: Color::CYAN,
		}
	}
}

pub struct TetriminoL;
impl TetriminoGenerator for TetriminoL {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![true, false, false, false],
			                  vec![true, false, false, false],
			                  vec![true, true, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![false, false, true, false],
			                  vec![true, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, true, false],
			                  vec![true, false, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoL".to_string(),
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
			             vec![vec![true, true, true, false],
			                  vec![false, false, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, false, false],
			                  vec![true, false, false, false],
			                  vec![true, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, false, false, false],
			                  vec![true, true, true, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoJ".to_string(),
			current_state: 0,
			color: Color::RED,
		}
	}
}

pub struct TetriminoO;
impl TetriminoGenerator for TetriminoO {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![true, true, false, false],
			                  vec![true, true, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]]
			],
			name: "TetriminoO".to_string(),
			current_state: 0,
			color: Color::YELLOW,
		}
	}
}

pub struct TetriminoS;
impl TetriminoGenerator for TetriminoS {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![vec![vec![false, true, true, false],
			                  vec![true, true, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, false, false, false],
			                  vec![true, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoS".to_string(),
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
			             vec![vec![false, true, false, false],
			                  vec![true, true, false, false],
			                  vec![true, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoZ".to_string(),
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
			             vec![vec![false, true, false, false],
			                  vec![true, true, false, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, true, true, false],
			                  vec![false, true, false, false],
			                  vec![false, false, false, false],
			                  vec![false, false, false, false]],
			             vec![vec![true, false, false, false],
			                  vec![true, true, false, false],
			                  vec![true, false, false, false],
			                  vec![false, false, false, false]],
			],
			name: "TetriminoT".to_string(),
			current_state: 0,
			color: Color::RGB(128, 0, 128), // Purple
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn run_tests() {}
}