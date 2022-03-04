extern crate sdl2;
use crate::model::score::{Score, read_score, save_score};

mod model;

fn main() {
	
	let mut result_to_save = vec![Score { name: String::from("Toto"), nb_points: 32, nb_lines: 12 },
	                              Score { name: String::from("Titi"), nb_points: 42, nb_lines: 6 },
	                              Score { name: String::from("Tata"), nb_points: 52, nb_lines: 3 },];


	save_score(&mut result_to_save);
	
	let saved_result = read_score();
	println!("{:#?}", saved_result);
}