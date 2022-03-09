extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::keyboard::Keycode::Hash;
use sdl2::keyboard::Scancode::H;
use sdl2::libc::__u32;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};
use crate::model::coordinate;

use crate::model::game;
use crate::model::game::{Game, MIN_X_BOUND, MIN_Y_BOUND, MAX_Y_BOUND, MAX_X_BOUND};
use crate::model::coordinate::Coordinate;
use crate::tetrimino;

const SQUARE_SIZE: u32 = 32;
const SQUARE_BORDER: u32 = 1;
const X_OFF_SET: i32 = 60;
const Y_OFF_SET: i32 = 20;

const MINI_BOARD_X_OFF_SET: i32 = 600;
const MINI_BOARD_Y_OFF_SET: i32 = 120;

const BACKGROUND_COLOR: Color = Color::WHITE;

pub struct View {
	game: Game,
	board: HashMap<Coordinate, Rect>,
	mini_board: HashMap<Coordinate, Rect>,
	canvas: WindowCanvas,
	sdl_context: Sdl,
}

impl View {
	pub fn new(game: Game) -> View {
		let sdl_context = sdl2::init().expect("failed to init SDL");
		let video_subsystem = sdl_context.video().expect("failed to get video context");
		
		let window = video_subsystem.window("sdl2 demo", 800, 600)
			.build()
			.expect("failed to build window");
		
		let canvas: Canvas<Window> = window.into_canvas()
			.build()
			.expect("failed to build window's canvas");
		
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
			canvas,
			sdl_context,
		}
	}
	
	pub fn run(&mut self) {
		self.update_map();
		
		self.canvas.present();
		
		let mut event_pump = self.sdl_context.event_pump().expect("Failed to get SDL event pump");
		let mut keep_running = true;
		let mut now = SystemTime::now();
		
		'running: loop {
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running;
					}
					Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
						if self.game.move_down() {
							self.update_map();
							self.canvas.present();
						} else {
							println!("Fin du game");
							keep_running = false;
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
						if self.game.move_left() {
							self.update_map();
							self.canvas.present();
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
						if self.game.move_right() {
							self.update_map();
							self.canvas.present();
						}
					}
					Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
						if self.game.rotate_left() {
							self.update_map();
							self.canvas.present();
						}
					}
					_ => {}
				}
			}
			
			match now.elapsed() {
				Ok(elapsed) => {
					if 800 <= elapsed.as_millis() && keep_running {
						if self.game.move_down() {
							self.update_map();
							self.canvas.present();
						} else {
							println!("Fin du game");
							keep_running = false;
						}
						now = SystemTime::now();
					}
				}
				Err(e) => {
					// an error occurred!
					println!("Error: {:?}", e);
				}
			}
			
			sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
	
	fn update_map(&mut self) {
		self.canvas.set_draw_color(BACKGROUND_COLOR);
		self.canvas.clear();
		
		
		for coordinate in &self.mini_board {
			if self.game.next_tetrimino.get_state()[coordinate.0.y as usize][coordinate.0.x as usize] {
				self.canvas.set_draw_color(self.game.next_tetrimino.color);
			} else {
				self.canvas.set_draw_color(BACKGROUND_COLOR);
			}
			self.canvas.fill_rect(*coordinate.1).expect("Failed to add a new shape");
		}
		
		for coordinate in &self.board {
			self.canvas.set_draw_color(self.game.get_cell(coordinate.0).unwrap().color);
			self.canvas.fill_rect(*coordinate.1).expect("Failed to add a new shape");
		}
	}
}
