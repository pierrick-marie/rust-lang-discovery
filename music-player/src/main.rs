/* Copyright 2022 Pierrick MARIE

This file is part of rust-discovery

LCS is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Rust-discovery is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with rust-discovery.  If not, see <http://www.gnu.org/licenses/>. */

mod toolbar;
mod playlist;

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem, Image, Adjustment, Scale};
use crate::playlist::Playlist;

use crate::toolbar::MusicToolbar;

struct MusicApp {
	toolbar: MusicToolbar,
	cover: Image,
	scale: Scale,
	playlist: Playlist,
	window: ApplicationWindow,
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

		let main_container = Box::new(gtk::Orientation::Vertical, 3);
		let toolbar = MusicToolbar::new();

		let cover = Image::new();
		cover.set_from_file(Some("assets/cover.png"));

		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);

		let playlist = Playlist::new();

		main_container.add(toolbar.container());
		main_container.add(&cover);
		main_container.add(&scale);
		main_container.add(playlist.view());

		window.add(&main_container);

		// Don't forget to make all widgets visible.
		window.show_all();

		toolbar.connect_toolbar_events(&window);

		MusicApp {
			toolbar,
			cover,
			scale,
			playlist,
			window,
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
