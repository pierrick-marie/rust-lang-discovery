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
mod mp3;
mod utils;
mod player;

extern crate gtk;
extern crate gio;
extern crate gtk_sys;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

use std::rc::Rc;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem, Image, Adjustment, Scale, FileChooserAction, FileChooserDialog, FileFilter, ResponseType};

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

pub struct State {
	stopped: bool,
}

struct MusicApp {
	toolbar: MusicToolbar,
	cover: Image,
	scale: Scale,
	playlist: Rc<Playlist>,
	state: Arc<Mutex<State>>,
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
		// cover.set_from_file(Some("assets/cover.png"));

		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);

		let state = Arc::new(Mutex::new(State {
			stopped: true,
		}));

		let playlist = Rc::new(Playlist::new(state.clone()));

		main_container.add(&toolbar.container);
		main_container.add(&cover);
		main_container.add(&scale);
		main_container.add(playlist.view());

		window.add(&main_container);

		// Don't forget to make all widgets visible.
		window.show_all();

		MusicApp {
			toolbar,
			cover,
			scale,
			playlist,
			state,
			window,
		}
	}

	fn connect_open(&self) {
		let playlist_quit = self.playlist.clone();
		let win_diag = self.window.clone();
		self.toolbar.open_button.connect_clicked(move |_| {
			let file = show_open_dialog(&win_diag);
			if let Some(file) = file {
				playlist_quit.add(&file);
			}
		});
	}

	fn connect_remove(&self) {
		let playlist_remove = self.playlist.clone();
		self.toolbar.remove_button.connect_clicked(move |_| {
			playlist_remove.remove_selection();
		});
	}

	fn connect_quit(&self) {
		let win_quit = self.window.clone();
		self.toolbar.quit_button.connect_clicked(move |_| {
			unsafe { win_quit.destroy(); }
		});
	}

	fn connect_play(&self) {
		let play_button = self.toolbar.play_button.clone();
		let cover_play = self.cover.clone();
		let play = self.playlist.clone();
		self.toolbar.play_button.connect_clicked(move |_| {

			cover_play.set_from_pixbuf(Some(&play.pixbuf().unwrap()));
			cover_play.show();
			// if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
			// 	play_button.set_stock_id(PAUSE_STOCK);
			// } else {
			// 	play_button.set_stock_id(PLAY_STOCK);
			// }
		});
	}

	fn set_cover(&self) {
		self.cover.set_from_pixbuf(Some(&self.playlist.pixbuf().unwrap()));
		self.cover.show();
	}
}



fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
	let mut file = None;
	let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(parent), FileChooserAction::Open);
	let filter = FileFilter::new();
	filter.add_mime_type("audio/mp3");
	filter.set_name(Some("MP3 audio file"));
	dialog.add_filter(&filter);
	dialog.add_button("Cancel", ResponseType::Cancel);
	dialog.add_button("Accept", ResponseType::Accept);
	let result = dialog.run();
	if result == ResponseType::Accept {
		file = dialog.filename();
	}
	unsafe { dialog.destroy(); }
	file
}

fn main() {
	let music_player = Application::builder()
		.application_id("music-player")
		.build();

	music_player.connect_activate(|app| {
		let music_app = MusicApp::new(app);
		music_app.connect_open();
		music_app.connect_quit();
		music_app.connect_play();
		music_app.connect_remove();
	});

	music_player.run();
}
