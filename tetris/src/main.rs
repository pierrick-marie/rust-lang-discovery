extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;

const TEXTURE_SIZE: u32 = 32;

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
	let texture_creator: TextureCreator<_> = canvas.texture_creator();
	
	let mut square_texture: Texture = texture_creator
		.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
		.expect("Failed to create a square");
	
	canvas.with_texture_canvas(&mut square_texture, |texture| {
		texture.set_draw_color(Color::RGB(0, 255, 0));
		texture.clear();
	});
	// ==== <<< END personal texture >>> ====
	
	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
					{
						break 'running;
					}
				_ => {}
			}
		}
		
		canvas.set_draw_color(Color::RGB(255, 0, 0));
		canvas.clear();
		
		canvas.copy(&square_texture, None, Rect::new(10, 10, TEXTURE_SIZE, TEXTURE_SIZE))
			.expect("Could not copy texture into window");
		canvas.present();
		
		sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
