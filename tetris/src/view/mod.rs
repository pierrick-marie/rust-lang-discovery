extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::libc::__u32;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use crate::model::coordinate;

use crate::model::game;
use crate::model::game::Game;
use crate::model::coordinate::Coordinate;

const SQUARE_SIZE: i32 = 32;
const SQUARE_BORDER: i32 = 1;
const X_OFF_SET: i32 = 60;
const Y_OFF_SET: i32 = 20;

pub struct View<'a> {
	sdl_context: sdl2::Sdl,
	canvas: WindowCanvas,
	texture_creator: &'a TextureCreator<WindowContext>,
	colors: HashMap<&'a Color, &'a Texture<'a>>,
}

impl View<'_> {
	
	pub fn new<'a>() -> &'a View<'a> {
		let sdl_context = sdl2::init().expect("SDL initialization failed");
		let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
		let window = video_subsystem.window("Tetris", 800, 600)
			.position_centered()
			.opengl()
			.build()
			.expect("Failed to create window");
		let canvas: WindowCanvas = window.into_canvas()
			.target_texture()
			.present_vsync()
			.build()
			.expect("Failed to convert window into canvas");
		let texture_creator: &TextureCreator<WindowContext> = &canvas.texture_creator();
		
		let mut colors = HashMap::new();
		// colors.insert(
		// 	Color::BLACK,
		// 	View::create_texture_rect(&mut canvas, &mut texture_creator, Color::BLACK).expect("Failed to create blue square"));
		// colors.insert(
		// 	Color::YELLOW,
		// 	View::create_texture_rect(&mut canvas, &mut texture_creator, Color::YELLOW).expect("Failed to create blue square"));
		
		& View {
			sdl_context,
			canvas,
			texture_creator,
			colors,
		}
	}
	
	pub fn display(&mut self, game: &Game) {
		
		// ==== <<< BEGIN personal texture >>> ====
		// let now = SystemTime::now();
		//
		// let texture_creator: TextureCreator<_> = canvas.texture_creator();
		// let blue_square_texture: Texture = crate_texture_rect(
		// 	&mut canvas,
		// 	&texture_creator,
		// 	TextureColor::Blue,
		// 	TEXTURE_SIZE).expect("Failed to create blue square");
		// let green_square_texture: Texture = crate_texture_rect(
		// 	&mut canvas,
		// 	&texture_creator,
		// 	TextureColor::Green,
		// 	TEXTURE_SIZE).expect("Failed to create blue square");
		// ==== <<< END personal texture >>> ====
		
		let mut event_pump = self.sdl_context.event_pump().expect("Failed to get SDL event pump");
		
		let mut now = SystemTime::now();
		
		self.canvas.set_draw_color(Color::WHITE);
		self.canvas.clear();
		
		// self.init_colors();
		
		// let black_square_texture: Texture = crate_texture_rect(
		// 	&mut canvas,
		// 	&texture_creator,
		// 	Color::BLACK).expect("Failed to create blue square");
		
		for y in 0..game::MAX_Y_BOUND {
			for x in 0..game::MAX_X_BOUND {
				// match colors.get(&game.get_cell(&Coordinate{x, y}).unwrap().color) {
				// 	Some(texture) => self.display_square(x as i32, y as i32, texture),
				// 	_ => println!("Unknowed color"),
				// }
			}
		}
		
		self.canvas.present();
		
		'running: loop {
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'running;
					}
					_ => {}
				}
			}
			
			// match now.elapsed() {
			// 	Ok(elapsed) => {
			// 		if 1 < elapsed.as_secs() {
			// 			blue_square_color = !blue_square_color;
			// 			now = SystemTime::now();
			// 		}
			// 	}
			// 	Err(e) => {
			// 		// an error occurred!
			// 		println!("Error: {:?}", e);
			// 	}
			// }
			//
			// if blue_square_color {
			// 	canvas.copy(&blue_square_texture, None, Rect::new(10, 10, TEXTURE_SIZE, TEXTURE_SIZE))
			// 		.expect("Could not copy texture into window");
			// } else {
			// 	canvas.copy(&green_square_texture, None, Rect::new(10, 10, TEXTURE_SIZE, TEXTURE_SIZE))
			// 		.expect("Could not copy texture into window");
			// }
			// canvas.present();
			
			sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}
	
	fn display_square(&mut self, x: i32, y: i32, texture: &Texture) {
		self.canvas.copy(texture,
		                 None,
		                 Rect::new(
			                 X_OFF_SET + x * (SQUARE_SIZE + SQUARE_BORDER),
			                 Y_OFF_SET + y * (SQUARE_SIZE + SQUARE_BORDER),
			                 SQUARE_SIZE as u32,
			                 SQUARE_SIZE as u32))
			.expect("Could not copy texture into window");
	}
	
	pub fn add_texture_rect(&mut self, color: &'static Color)  {
		if let Ok(mut square_texture) = self.texture_creator.create_texture_target(
			None,
			SQUARE_SIZE as u32,
			SQUARE_SIZE as u32) {
			self.canvas.with_texture_canvas(&mut square_texture, |texture| {
				texture.set_draw_color(*color);
				texture.clear();
			}).expect("Failed to color texture");
			self.colors.insert(
				color, &square_texture);
		}
	}
}







