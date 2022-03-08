extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::keyboard::Keycode::Hash;
use sdl2::libc::__u32;
use sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use crate::model::coordinate;

use crate::model::game;
use crate::model::game::Game;
use crate::model::coordinate::Coordinate;

const SQUARE_SIZE: u32 = 32;
const SQUARE_BORDER: i32 = 1;
const X_OFF_SET: i32 = 60;
const Y_OFF_SET: i32 = 20;

fn crate_texture_rect<'a>(canvas: &mut Canvas<Window>,
                          texture_creator: &'a TextureCreator<WindowContext>,
                          color: Color) -> Texture<'a> {
	
	let mut square_texture = texture_creator.create_texture_static(
		None,
		SQUARE_SIZE,
		SQUARE_SIZE).expect("plop");
	
		canvas.with_texture_canvas(&mut square_texture, |texture| {
			texture.set_draw_color(color);
			texture.clear();
		}).expect("Failed to color texture");

		square_texture
}

fn init() -> WindowCanvas {
	
	let sdl_context = sdl2::init().expect("SDL initialization failed");
	let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
	let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
		.position_centered()
		.opengl()
		.build()
		.expect("Failed to create window");
	
	let mut canvas = window.into_canvas()
		.target_texture()
		.present_vsync()
		.build()
		.expect("Failed to convert window into canvas");
	
	canvas
	
}

pub fn run() {
	
	let mut colors: HashMap<&Color, &Texture> = HashMap::new();
	let mut canvas  =  init();
	
	
	let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
	
	let mut texture = crate_texture_rect(
		&mut canvas,
		&texture_creator,
		Color::GREEN);
	colors.insert(&Color::GREEN, &texture);
	let mut texture = crate_texture_rect(
		&mut canvas,
		&texture_creator,
		Color::BLUE);
	colors.insert(&Color::BLUE, &texture);
	
	
	
	
	
	let mut now = SystemTime::now();
	
	
	
	
	
	
	// 'running: loop {
	// 	for event in event_pump.poll_iter() {
	// 		match event {
	// 			Event::Quit { .. } |
	// 			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
	// 				break 'running;
	// 			}
	// 			_ => {}
	// 		}
	// 	}
	//
	// 	canvas.set_draw_color(Color::RGB(255, 0, 0));
	// 	canvas.clear();
	//
	// 	canvas.copy(colors.get(&Color::BLUE).unwrap(), None, Rect::new(100, 100, SQUARE_SIZE, SQUARE_SIZE))
	// 		.expect("Could not copy texture into window");
	//
	// 	match now.elapsed() {
	// 		Ok(elapsed) => {
	// 			// if 1 < elapsed.as_secs() {
	// 			// 	blue_square_color = !blue_square_color;
	// 			// 	now = SystemTime::now();
	// 			// }
	// 		}
	// 		Err(e) => {
	// 			// an error occurred!
	// 			println!("Error: {:?}", e);
	// 		}
	// 	}
	//
	// 	canvas.present();
	//
	// 	sleep(Duration::new(0, 1_000_000_000u32 / 60));
	// }
}







