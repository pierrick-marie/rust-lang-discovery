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
use gio::glib;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, IconSize, SeparatorToolItem, Image, Adjustment, Scale, FileChooserAction, FileChooserDialog, FileFilter, ResponseType};
use crate::player::Action;

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

pub struct State {
	pub action: Action,
	pub current_time: u64,
	pub durations: HashMap<String, u64>,
}

#[derive(Clone)]
struct UI {
	toolbar: MusicToolbar,
	cover: Image,
	adjustment: Adjustment,
	duration_label: Label,
	window: ApplicationWindow,
	play: Image,
	pause: Image,
}

struct MusicApp {
	ui: UI,
	playlist: Rc<Playlist>,
	state: Arc<Mutex<State>>,
}

unsafe impl Sync for MusicApp {}

impl MusicApp {
	fn new(app: &Application) -> Self {
		let current_time = 0;
		let durations = HashMap::new();
		let state = Arc::new(Mutex::new(State {
			action: Action::Stop,
			current_time,
			durations,
		}));

		let playlist = Rc::new(Playlist::new(state.clone()));

		// We create the main window.
		let window = ApplicationWindow::builder()
			.application(app)
			.title("Rust music player")
			.build();

		let main_container = Box::new(gtk::Orientation::Vertical, 3);
		let toolbar = MusicToolbar::new();

		let cover = Image::new();

		let duration_label = Label::new(Some("0 / 0"));
		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);
		scale.set_hexpand(true);

		let pause = Image::from_icon_name(Some("gtk-media-pause"), IconSize::LargeToolbar);
		let play = Image::from_icon_name(Some("gtk-media-play"), IconSize::LargeToolbar);

		let hbox = Box::new(gtk::Orientation::Horizontal, 3);
		hbox.add(&SeparatorToolItem::new());
		hbox.add(&scale);
		hbox.add(&duration_label);
		hbox.add(&SeparatorToolItem::new());

		main_container.add(&toolbar.container);
		main_container.add(&cover);
		main_container.add(&hbox);
		main_container.add(playlist.view());

		window.add(&main_container);
		window.show_all();

		let ui = UI {
			toolbar,
			cover,
			adjustment,
			duration_label,
			window,
			play,
			pause,
		};

		MusicApp {
			ui,
			playlist,
			state,
		}
	}

	fn connect_open(&self) {
		let playlist = self.playlist.clone();
		let window = self.ui.window.clone();

		playlist.add(Path::new("./assets/songs/timal-gazo-filtre-clip-officiel.mp3"));

		self.ui.toolbar.open_button.connect_clicked(move |_| {
			for file in MusicApp::show_open_dialog(&window) {
				playlist.add(&file);
			}
		});
	}

	fn connect_next(&self) {
		let playlist = self.playlist.clone();
		let cover = self.ui.cover.clone();
		self.ui.toolbar.next_button.connect_clicked(move |_| {
			playlist.next_song();
			playlist.stop();
			playlist.play();
			MusicApp::set_cover(&playlist, &cover);
		});
	}

	fn connect_previous(&self) {
		let playlist = self.playlist.clone();
		let cover = self.ui.cover.clone();
		self.ui.toolbar.previous_button.connect_clicked(move |_| {
			playlist.previous_song();
			playlist.stop();
			playlist.play();
			MusicApp::set_cover(&playlist, &cover);
		});
	}

	fn connect_remove(&self) {
		let playlist = self.playlist.clone();
		let cover = self.ui.cover.clone();
		self.ui.toolbar.remove_button.connect_clicked(move |_| {
			playlist.remove_selection();
			MusicApp::set_cover(&playlist, &cover);
		});
	}

	fn connect_quit(&self) {
		let window = self.ui.window.clone();
		self.ui.toolbar.quit_button.connect_clicked(move |_| {
			unsafe { window.destroy(); }
		});
	}

	fn connect_play(&self) {
		let cover = self.ui.cover.clone();
		let playlist = self.playlist.clone();
		self.ui.toolbar.play_button.connect_clicked(move |_| {
			MusicApp::set_cover(&playlist, &cover);
			playlist.play();
		});

		let playlist = self.playlist.clone();
		let state = self.state.clone();
		let ui = self.ui.clone();
		glib::timeout_add_local(Duration::new(0, 100_000_000), move || {
			let path = playlist.path();
			let current_time = state.lock().unwrap().current_time;
			if let Some(&duration) = state.lock().unwrap().durations.get(&path) {
				ui.adjustment.set_upper(duration as f64);
				ui.duration_label.set_label(&format!("{} / {}", MusicApp::convert_milli_to_min(&current_time), MusicApp::convert_milli_to_min(&duration)));
			}
			ui.adjustment.set_value(current_time as f64);
			match state.lock().unwrap().action {
				Action::Play(_) => {
					ui.toolbar.play_button.set_image(Some(&ui.pause));
				}
				_ => {
					ui.toolbar.play_button.set_image(Some(&ui.play));
				}
			}
			Continue(true)
		});
	}

	fn convert_milli_to_min(milli: &u64) -> String {
		let mut nb_seconds = milli / 1000;
		let nb_minutes = nb_seconds / 60;
		nb_seconds = nb_seconds - (nb_minutes * 60);

		format!("{}m {}s", nb_minutes, nb_seconds)
	}

	fn connect_stop(&self) {
		let playlist = self.playlist.clone();
		let cover = self.ui.cover.clone();
		self.ui.toolbar.stop_button.connect_clicked(move |_| {
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

		let mp3_filter = FileFilter::new();
		mp3_filter.add_mime_type("audio/mp3");
		mp3_filter.set_name(Some("MP3 audio file"));
		dialog.add_filter(&mp3_filter);

		let m3u_filter = FileFilter::new();
		m3u_filter.add_mime_type("audio/m3u");
		m3u_filter.set_name(Some("M3U audio playlist"));
		dialog.add_filter(&m3u_filter);

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
