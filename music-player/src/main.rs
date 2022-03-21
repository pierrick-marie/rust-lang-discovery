mod toolbar;

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem};

use crate::toolbar::MusicToolbar;

struct MusicApp {
	window: ApplicationWindow,
	toolbar: MusicToolbar,
}

impl MusicApp {
	fn new(app: &Application) -> Self {

		// We create the main window.
		let window = ApplicationWindow::builder()
			.application(app)
			// .default_width(256)
			// .default_height(120)
			.title("Rust music player")
			.build();

		let toolbar = MusicToolbar::new();

		window.add(toolbar.container());

		// Don't forget to make all widgets visible.
		window.show_all();

		MusicApp {
			window,
			toolbar,
		}
	}
}

fn main() {
	let music_player = Application::builder()
		.application_id("music-player")
		.build();

	music_player.connect_activate(|app| {
		let music_app = MusicApp::new(app);
	});

	music_player.run();
}
