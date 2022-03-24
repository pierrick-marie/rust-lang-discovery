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
mod player;

extern crate gtk;
extern crate gio;
extern crate gtk_sys;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use gdk_pixbuf::Pixbuf;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem, Image, Adjustment, Scale, FileChooserAction, FileChooserDialog, FileFilter, ResponseType};

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

struct MusicApp {
	toolbar: MusicToolbar,
	cover: Image,
	scale: Scale,
	playlist: Rc<Playlist>,
	window: ApplicationWindow,
}

impl MusicApp {
	fn new(app: &Application) -> Self {

		// We create the main window.
		let window = ApplicationWindow::builder()
			.application(app)
			.title("Rust music player")
			.build();

		let main_container = Box::new(gtk::Orientation::Vertical, 3);
		let toolbar = MusicToolbar::new();

		let cover = Image::new();

		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);

		let playlist = Rc::new(Playlist::new());

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
			window,
		}
	}

	fn connect_open(&self) {
		let playlist = self.playlist.clone();
		let window = self.window.clone();

		// DEBUG
		playlist.add(Path::new("./assets/songs/naps-la-kiffance-clip-officiel.mp3"));
		playlist.add(Path::new("./assets/songs/timal-gazo-filtre-clip-officiel.mp3"));
		playlist.add(Path::new("./assets/songs/bande-organisee-remix-version-feminine-clip-officiel.mp3"));

		self.toolbar.open_button.connect_clicked(move |_| {
			let file = show_open_dialog(&window);
			if let Some(file) = file {
				playlist.add(&file);
			}
		});
	}

	fn connect_next(&self) {
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		self.toolbar.next_button.connect_clicked(move |_| {
			playlist.next_song();
			playlist.stop();
			playlist.play();
			let res = playlist.pixbuf();
			match res {
				Ok(pixbuf) => {
					cover.set_from_pixbuf(Some(&pixbuf));
					cover.show();
				}
				_ => {}
			}
		});
	}

	fn connect_previous(&self) {
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		self.toolbar.previous_button.connect_clicked(move |_| {
			playlist.previous_song();
			playlist.stop();
			playlist.play();
			let res = playlist.pixbuf();
			match res {
				Ok(pixbuf) => {
					cover.set_from_pixbuf(Some(&pixbuf));
					cover.show();
				}
				_ => {}
			}
		});
	}

	fn connect_remove(&self) {
		let playlist = self.playlist.clone();
		self.toolbar.remove_button.connect_clicked(move |_| {
			playlist.remove_selection();
		});
	}

	fn connect_quit(&self) {
		let window = self.window.clone();
		self.toolbar.quit_button.connect_clicked(move |_| {
			unsafe { window.destroy(); }
		});
	}

	fn connect_play(&self) {
		let button = self.toolbar.play_button.clone();
		let cover = self.cover.clone();
		let playlist = self.playlist.clone();
		self.toolbar.play_button.connect_clicked(move |_| {
			playlist.play();
			let res = playlist.pixbuf();
			match res {
				Ok(pixbuf) => {
					cover.set_from_pixbuf(Some(&pixbuf));
					cover.show();
				}
				_ => {}
			}
		});
	}

	fn connect_stop(&self) {
		let button = self.toolbar.stop_button.clone();
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		self.toolbar.stop_button.connect_clicked(move |_| {
			playlist.stop();
			cover.hide();
		});
	}

	fn set_cover(&self) {
		let res = self.playlist.pixbuf();
		match res {
			Ok(pix) => {
				self.cover.set_from_pixbuf(Some(&pix));
				self.cover.show();
			}
			Err(msg) => {}
		}
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
		music_app.connect_stop();
		music_app.connect_next();
		music_app.connect_previous();
	});

	music_player.run();
}
