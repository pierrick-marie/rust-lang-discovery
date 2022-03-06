extern crate sdl2;

mod model;
mod controler;

use crate::model::score::score_module::*;
use crate::controler::*;
use crate::controler::controler::generate_tetrimino;
use crate::game::{Coordinate, Game, GameFunction};
use crate::model::game::*;

fn main() {
	let mut result_to_save = vec![Score { name: String::from("Toto"), nb_points: 32, nb_lines: 12 },
	                              Score { name: String::from("Titi"), nb_points: 42, nb_lines: 6 },
	                              Score { name: String::from("Tata"), nb_points: 52, nb_lines: 3 }, ];
	
	save_score(&mut result_to_save);
	
	let saved_result = read_score();
	
	let mut game: Game = Game::new();
	
	let coord = Coordinate{
		x: 0,
		y: 0,
	};
	
	game.switch_value(&coord);
	println!("Value {}: {}", coord, game.get_value(&coord).unwrap());
	game.switch_value(&coord);
	println!("Value {}: {}", coord, game.get_value(&coord).unwrap());
}