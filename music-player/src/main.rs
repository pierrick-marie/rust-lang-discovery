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

use std::collections::HashMap;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use gdk_pixbuf::Pixbuf;
use gio::glib;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem, Image, Adjustment, Scale, FileChooserAction, FileChooserDialog, FileFilter, ResponseType};

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

pub struct ProgressBar {
	pub current_time: u64,
	pub durations: HashMap<String, u64>,
}

struct MusicApp {
	toolbar: MusicToolbar,
	cover: Image,
	adjustment: Adjustment,
	playlist: Rc<Playlist>,
	progress_bar: Arc<Mutex<ProgressBar>>,
	window: ApplicationWindow,
}

unsafe impl Sync for MusicApp {}

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

		let current_time = 0;
		let durations = HashMap::new();
		let progress_bar = Arc::new(Mutex::new(ProgressBar {
			current_time,
			durations,
		}));
		let playlist = Rc::new(Playlist::new(progress_bar.clone()));

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
			adjustment,
			playlist,
			progress_bar,
			window,
		}
	}

	fn connect_open(&self) {
		// let playlist = (*self.playlist.lock().unwrap()).clone();

		let playlist = self.playlist.clone();
		let window = self.window.clone();

		self.toolbar.open_button.connect_clicked(move |_| {
			for file in MusicApp::show_open_dialog(&window) {
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
			MusicApp::set_cover(&playlist, &cover);
		});
	}

	fn connect_previous(&self) {
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		self.toolbar.previous_button.connect_clicked(move |_| {
			playlist.previous_song();
			playlist.stop();
			playlist.play();
			MusicApp::set_cover(&playlist, &cover);
		});
	}

	fn connect_remove(&self) {
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		self.toolbar.remove_button.connect_clicked(move |_| {
			playlist.remove_selection();
			MusicApp::set_cover(&playlist, &cover);
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
			MusicApp::set_cover(&playlist, &cover);
			playlist.play();
		});

		let playlist = self.playlist.clone();
		let adjustment = self.adjustment.clone();
		let progress_bar = self.progress_bar.clone();
		glib::timeout_add_local(Duration::new(0, 100_000_000), move || {
			let path = playlist.path();
			if let Some(&duration) = progress_bar.lock().unwrap().durations.get(&path) {
				adjustment.set_upper(duration as f64);
			}

			if playlist.is_playing() {
				adjustment.set_value(progress_bar.lock().unwrap().current_time as f64);
			} else {
				adjustment.set_value(0 as f64);
			}
			Continue(true)
		});
	}

	fn connect_stop(&self) {
		let button = self.toolbar.stop_button.clone();
		let playlist = self.playlist.clone();
		let cover = self.cover.clone();
		let progress_bar = self.progress_bar.clone();
		self.toolbar.stop_button.connect_clicked(move |_| {
			playlist.stop();
			cover.hide();
		});
	}

	fn set_cover(playlist: &Playlist, cover: &Image) {
		let res = playlist.pixbuf();
		match res {
			Ok(pix) => {
				cover.set_from_pixbuf(Some(&pix));
				cover.show();
			}
			Err(_) => {}
		}
	}

	fn show_open_dialog(parent: &ApplicationWindow) -> Vec<PathBuf> {
		let mut files = vec![];
		let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(parent), FileChooserAction::Open);
		let filter = FileFilter::new();
		filter.add_mime_type("audio/mp3");
		filter.set_name(Some("MP3 audio file"));
		dialog.add_filter(&filter);
		dialog.set_select_multiple(true);
		dialog.add_button("Cancel", ResponseType::Cancel);
		dialog.add_button("Accept", ResponseType::Accept);
		let result = dialog.run();
		if result == ResponseType::Accept {
			files = dialog.filenames();
		}
		unsafe { dialog.destroy(); }
		files
	}
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
