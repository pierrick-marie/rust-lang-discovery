// TODO voir https://crates.io/crates/rustyline

pub fn run() {
	let sdl_context = sdl2::init().expect("failed to init SDL");

	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
	let mut now = SystemTime::now();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } |
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {}
				_ => {}
			}
		}

		match now.elapsed() {
			Ok(elapsed) => {
				if 800 <= elapsed.as_millis() && keep_running {
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

