pub mod controler {
	use crate::model;
	use model::tetrimino::tetrimino::*;
	
	pub fn generate_tetrimino() -> Tetrimino {
		static mut PREV: u8 = 7;
		let mut rand_nb = rand::random::<u8>() % 7;
		
		while unsafe { PREV } == rand_nb {
			rand_nb = rand::random::<u8>() % 7;
		}
		unsafe {PREV = rand_nb}
		
		match rand_nb {
			0 => TetriminoI::new(),
			1 => TetriminoJ::new(),
			2 => TetriminoL::new(),
			3 => TetriminoO::new(),
			4 => TetriminoS::new(),
			5 => TetriminoZ::new(),
			6 => TetriminoT::new(),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	
	use crate::controler::controler::generate_tetrimino;
	
	#[test]
	fn tests() {
		for i in 0..10 {
			println!("{}", generate_tetrimino());
		}
	}
}