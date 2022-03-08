extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::keyboard::Keycode::Hash;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
	Green,
	Blue,
}

fn crate_texture_rect<'a>(canvas: &mut Canvas<Window>,
                          texture_creator: &'a TextureCreator<WindowContext>,
                          color: TextureColor,
                          size: u32) -> Option<Texture<'a>> {
	if let Ok(mut square_texture) = texture_creator.create_texture_target(
		None,
		size,
		size) {
		canvas.with_texture_canvas(&mut square_texture, |texture| {
			match color {
				TextureColor::Green =>
					texture.set_draw_color(Color::RGB(0, 255, 0)),
				TextureColor::Blue =>
					texture.set_draw_color(Color::RGB(0, 0, 255)),
			}
			texture.clear();
		}).expect("Failed to color texture");
		Some(square_texture)
	} else {
		None
	}
}

pub fn main() {
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
	
	// ==== <<< BEGIN personal texture >>> ====
	let now = SystemTime::now();
	
	let texture_creator: TextureCreator<_> = canvas.texture_creator();
	// let blue_square_texture: Texture = crate_texture_rect(
	// 	&mut canvas,
	// 	&texture_creator,
	// 	TextureColor::Blue,
	// 	TEXTURE_SIZE).expect("Failed to create blue square");
	// let mut green_square_texture: Texture = crate_texture_rect(
	// 	&mut canvas,
	// 	&texture_creator,
	// 	TextureColor::Green,
	// 	TEXTURE_SIZE).expect("Failed to create blue square");
	// ==== <<< END personal texture >>> ====

	let mut colors: HashMap<Color, Texture> = HashMap::new();
	colors.insert(Color::GREEN, crate_texture_rect(
		&mut canvas,
		&texture_creator,
		TextureColor::Green,
		TEXTURE_SIZE).expect("Failed to create blue square"));
	colors.insert(Color::BLUE, crate_texture_rect(
		&mut canvas,
		&texture_creator,
		TextureColor::Blue,
		TEXTURE_SIZE).expect("Failed to create blue square"));
	
	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
	
	let mut now = SystemTime::now();
	let mut blue_square_color = true;
	
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
		
		canvas.set_draw_color(Color::RGB(255, 0, 0));
		canvas.clear();
		
		canvas.copy(colors.get(&Color::BLUE).unwrap(), None, Rect::new(100, 100, TEXTURE_SIZE, TEXTURE_SIZE))
			.expect("Could not copy texture into window");
		
		match now.elapsed() {
			Ok(elapsed) => {
				if 1 < elapsed.as_secs() {
					blue_square_color = !blue_square_color;
					now = SystemTime::now();
				}
			}
			Err(e) => {
				// an error occurred!
				println!("Error: {:?}", e);
			}
		}
		
		if blue_square_color {
			canvas.copy(colors.get(&Color::BLUE).unwrap(), None, Rect::new(10, 10, TEXTURE_SIZE, TEXTURE_SIZE))
				.expect("Could not copy texture into window");
		} else {
			canvas.copy(colors.get(&Color::GREEN).unwrap(), None, Rect::new(10, 10, TEXTURE_SIZE, TEXTURE_SIZE))
				.expect("Could not copy texture into window");
		}
		canvas.present();
		
		sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
