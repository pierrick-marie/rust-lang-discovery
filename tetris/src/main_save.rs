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

fn test(tetrimino: &mut Tetrimino, f: fn(&mut Tetrimino) ) {
	println!("{}", tetrimino);
	f(tetrimino);
	println!("{}", tetrimino);
}

fn main() {
	
	let mut game: Game = Game::new();
	
	let mut tetrimino: Tetrimino = tetrimino::generate_tetrimino();
	println!("{}", tetrimino);
	tetrimino.rotate_left();
	println!("{}", tetrimino);
	
	// println!("{}", tetrimino.coordinate.add((2, 2)));
	
	// tetrimino.move_right();
	// tetrimino.move_left();
	// tetrimino.move_right();
	
	game.add_tetrimino(&tetrimino);
	// game.add(&tetrimino);
	
	let mut view: &View = View::new();
	view.add_texture_rect(&Color::WHITE);
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