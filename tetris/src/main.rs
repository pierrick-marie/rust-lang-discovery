extern crate sdl2;

mod model;
mod view;

use sdl2::libc::termios;
use sdl2::pixels::Color;
use model::score::*;
use model::game::*;
use model::tetrimino::*;
use crate::model::tetrimino;
use crate::view::*;

fn main() {
	
	
	
	let mut game: Game = Game::new();
	
	let mut tetrimino: Tetrimino = tetrimino::generate_tetrimino();
	// let mut tetrimino = TetriminoI::new();
	// tetrimino.rotate_left();
	
	game.add_tetrimino(tetrimino);
	
	let mut view: View = View::new(game);
	view.run();
	
	
	// view.add_texture_rect(&Color::WHITE);
	// view.add_texture_rect(Color:Black);
	
	// view.display(&game);
	
	// test(&mut tetrimino, Tetrimino::rotate_left);
	
	// tetrimino.rotate_left();
	
	// println!("{}", tetrimino);
	
	// let coord = Coordinate{
	// 	x: 0,
	// 	y: 0,
	// };
	
	// game.switch_value(&coord);
	// println!("Value {}: {}", coord, game.get_value(&coord).unwrap());
	// game.switch_value(&coord);
	// println!("Value {}: {}", coord, game.get_value(&coord).unwrap());
}