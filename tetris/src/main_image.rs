extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture, InitFlag};

pub fn main() {
	let sdl_context = sdl2::init().expect("SDL initialization failed");
	let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
	
	sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("Failed initialize image context");
	
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
	
	let texture_creator: TextureCreator<_> = canvas.texture_creator();
	
	let image_texture = texture_creator.load_texture("assets/background.jpg").expect("Failed to load background image");
	
	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
	
	let mut now = SystemTime::now();
	
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
		
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		
		canvas.copy(&image_texture, None, None)
			.expect("Could not copy texture into window");
		canvas.present();
		
		sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
