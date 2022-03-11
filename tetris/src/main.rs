mod model;
mod view;

use model::score::*;
use model::game::*;
use model::tetrimino::*;
use crate::model::tetrimino;
use crate::view::*;

fn main() {
	
	let mut game: Game = Game::new();
	let tetrimino: Tetrimino = tetrimino::generate_tetrimino();
	game.add_tetrimino(tetrimino);
	let mut view: View = View::new(game);
	view.run();
}