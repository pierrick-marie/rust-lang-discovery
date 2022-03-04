extern crate sdl2;
use crate::model::score::{DataScore, read_score, save_score};

mod model;

fn main() {
	let result_to_save = DataScore { name: String::new(), nb_points: 56, nb_lines: 12 };
	save_score(&result_to_save).expect("Failed to save results");
	
	let saved_result = read_score();
	assert_eq!(result_to_save, saved_result);
}