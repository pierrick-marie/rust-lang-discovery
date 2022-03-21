extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::{Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use crate::model::{score};

use crate::model::game::{Game, MIN_X_BOUND, MIN_Y_BOUND, MAX_Y_BOUND, MAX_X_BOUND};
use crate::model::coordinate::Coordinate;
use crate::{tetrimino};

const SQUARE_SIZE: u32 = 32;
const SQUARE_BORDER: u32 = 1;
const X_OFF_SET: i32 = 60;
const Y_OFF_SET: i32 = 20;

const MINI_BOARD_X_OFF_SET: i32 = 570;
const MINI_BOARD_Y_OFF_SET: i32 = 120;

const BACKGROUND_COLOR: Color = Color::WHITE;

pub struct View {
	game: Game,
	board: HashMap<Coordinate, Rect>,
	mini_board: HashMap<Coordinate, Rect>,
}

impl View {
	pub fn new(game: Game) -> View {
		let mut board: HashMap<Coordinate, Rect> = HashMap::new();
		let mut coordinate: Coordinate;
		
		for y in MIN_Y_BOUND..MAX_Y_BOUND {
			for x in MIN_X_BOUND..MAX_X_BOUND {
				coordinate = Coordinate { x, y };
				board.insert(coordinate, Rect::new(X_OFF_SET + (x * (SQUARE_SIZE + SQUARE_BORDER)) as i32,
				                                   Y_OFF_SET + (y * (SQUARE_SIZE + SQUARE_BORDER)) as i32,
				                                   SQUARE_SIZE,
				                                   SQUARE_SIZE));
			}
		}
		
		let mut mini_board: HashMap<Coordinate, Rect> = HashMap::new();
		
		for y in 0..tetrimino::SIZE_OF {
			for x in 0..tetrimino::SIZE_OF {
				coordinate = Coordinate { x: x as u32, y: y as u32 };
				mini_board.insert(coordinate, Rect::new(MINI_BOARD_X_OFF_SET + ((x as u32) * (SQUARE_SIZE + SQUARE_BORDER)) as i32,
				                                        MINI_BOARD_Y_OFF_SET + ((y as u32) * (SQUARE_SIZE + SQUARE_BORDER)) as i32,
				                                        SQUARE_SIZE,
				                                        SQUARE_SIZE));
			}
		}
		
		View {
			game,
			board,
			mini_board,
		}
	}
	
	pub fn run(&mut self) {
		let sdl_context = sdl2::init().expect("failed to init SDL");
		let video_subsystem = sdl_context.video().expect("failed to get video context");
		
		let window = video_subsystem.window("sdl2 demo", 800, 600)
			.build()
			.expect("failed to build window");
		
		let mut canvas: Canvas<Window> = window.into_canvas()
			.build()
			.expect("failed to build window's canvas");
		
		let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
		
		self.update_map(&mut canvas);
		
		canvas.present();
		
		let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
		let mut keep_running = true;
		let mut now = SystemTime::now();
		
		let texture_creator = canvas.texture_creator();
		let font = ttf_context.load_font("assets/ubuntu-font.ttf", 512).expect("Failed to create font");
		
		'running: loop {
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running;
					}
					Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
						if keep_running && self.game.move_down() {
							self.update_map(&mut canvas);
						} else {
							keep_running = false;
							self.display_game_over(&mut canvas, &texture_creator, &font);
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
						if keep_running && self.game.move_left() {
							self.update_map(&mut canvas);
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
						if keep_running && self.game.move_right() {
							self.update_map(&mut canvas);
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
						if keep_running && self.game.rotate_left() {
							self.update_map(&mut canvas);
						}
					}
					_ => {}
				}
			}
			
			match now.elapsed() {
				Ok(elapsed) => {
					if 800 <= elapsed.as_millis() && keep_running {
						if self.game.move_down() {
							self.update_map(&mut canvas);
						} else {
							keep_running = false;
							self.display_game_over(&mut canvas, &texture_creator, &font);
						}
						now = SystemTime::now();
					}
				}
				Err(e) => {
					// an error occurred!
					println!("Error: {:?}", e);
				}
			}
			
			self.display_game_information(&mut canvas, &texture_creator, &font);
			canvas.present();
			sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}

		// Window is closed
		score::save(&mut self.game.score);
	}

	fn display_game_over<'a>(&self, canvas: &mut Canvas<Window>,
						  texture_creator: &'a TextureCreator<WindowContext>,
						  font: &sdl2::ttf::Font) {

		let game_over_str = "Game over !".to_string();

		let game_over = create_texture_from_text(&texture_creator, &font,
								&game_over_str).expect("Cannot render text");

		canvas.copy(&game_over, None, get_rect_from_text(&game_over_str,
									   540, 50)).expect("Couldn't copy text");
	}

	fn display_game_information<'a>(&self, canvas: &mut Canvas<Window>,
	                                texture_creator: &'a TextureCreator<WindowContext>,
	                                font: &sdl2::ttf::Font) {
		
		let line_text = format!("Lines: {}", self.game.score.nb_lines);
		let score_text = format!("Score: {}", self.game.score.nb_points);
		
		let line = create_texture_from_text(&texture_creator, &font,
		                                    &line_text).expect("Cannot render text");
		let score = create_texture_from_text(&texture_creator, &font,
		                                     &score_text).expect("Cannot render text");
		
		canvas.copy(&score, None, get_rect_from_text(&score_text,
		                                             550, 250)).expect("Couldn't copy text");
		canvas.copy(&line, None, get_rect_from_text(&line_text,
		                                                  550, 275)).expect("Couldn't copy text");
	}
	
	
	fn update_map(&self, canvas: &mut WindowCanvas) {
		canvas.set_draw_color(BACKGROUND_COLOR);
		canvas.clear();
		
		for coordinate in &self.mini_board {
			if self.game.next_tetrimino.get_state()[coordinate.0.y as usize][coordinate.0.x as usize] {
				canvas.set_draw_color(self.game.next_tetrimino.color);
			} else {
				canvas.set_draw_color(BACKGROUND_COLOR);
			}
			canvas.fill_rect(*coordinate.1).expect("Failed to add a new shape");
		}
		
		for coordinate in &self.board {
			canvas.set_draw_color(self.game.get_cell(coordinate.0).unwrap().color);
			canvas.fill_rect(*coordinate.1).expect("Failed to add a new shape");
		}
	}
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                font: &sdl2::ttf::Font,
                                text: &str) -> Option<Texture<'a>> {
	if let Ok(surface) = font.render(text)
		.blended(Color::BLACK) {
		texture_creator.create_texture_from_surface(&surface).ok()
	} else {
		None
	}
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
	Some(Rect::new(x, y, text.len() as u32 * 15, 25))
}
