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

mod model;

use crate::model::*;
use crate::model::music;
use model::State::*;
use model::MusicModel;
use music::Music;

mod view;
use view::MainWindow;

use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

extern crate gtk;
extern crate gtk_sys;
use gtk::{
	Window,
};
use gtk::gdk::keys::constants::Music;
use gtk::prelude::*;
use gtk::prelude::{
	ButtonExt,
	WidgetExt,
};
use m3u::Entry::Path;

extern crate relm;
extern crate relm_derive;
use relm_derive::Msg;
use relm::{connect, interval, Relm, Update, Widget};

extern crate gio;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

pub struct MusicApp {
	model: MusicModel,
	view: MainWindow,
}

#[derive(Msg)]
pub enum Msg {
	Open,
	Play,
	Quit,
	Stop,
	Remove,
	UpView,
	Next,
	Prev,
	Save,
}

impl MusicApp {
	
	fn add_mp3(&mut self, music: &Music) {
		self.view.add_music(&music);
		self.model.add_music(&music);
	}
	
	fn add_m3u(&mut self, path: &PathBuf) {
		let filename = path.to_string_lossy().to_string();
		if let Ok(mut reader) = m3u::Reader::open(filename) {
			let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
			for song in read_playlist {
				match song {
					m3u::Entry::Path(path) => {
						if let Some(music) = Music::parse_file(path.as_path()) {
							self.add_mp3(&music);
						}
					}
					_ => {}
				}
			}
		} else {
			// It's not a m3u file
		}
	}
	
	pub fn save_playlist(&self, path: &PathBuf) {
		let mut file = File::create(path.to_string_lossy().to_string()).unwrap();
		file.write_all(b"").expect("Failed to clean content of the playlist file");
		let mut writer = m3u::Writer::new(&mut file);
		let mut entries = vec![];
		
		for song in self.model.songs() {
			let path = &song.0.replace(music::URI, "");
			entries.push(m3u::path_entry(fs::canonicalize(path).unwrap()));
		}
		
		for entry in &entries {
			writer.write_entry(entry).unwrap();
		}
	}
}

impl Update for MusicApp {
	// Specify the song used for this widget.
	type Model = MusicModel;
	// Specify the song parameter used to init the song.
	type ModelParam = ();
	// Specify the type of the messages sent to the update function.
	type Msg = Msg;
	
	fn model(_: &Relm<Self>, _: ()) -> MusicModel {
		MusicModel::new()
	}
	
	fn subscriptions(&mut self, relm: &Relm<Self>) {
		interval(relm.stream(), 100, || Msg::UpView);
	}
	
	fn update(&mut self, event: Msg) {
		match event {
			Msg::Open => {
				for file in self.view.show_open_dialog() {
					if file.to_string_lossy().to_string().ends_with(".mp3") {
						if let Some(music) = Music::parse_file(file.as_path()) {
							self.add_mp3(&music);
						}
					} else {
						if file.to_string_lossy().to_string().ends_with(".m3u") {
							self.add_m3u(&file);
						}
					}
				}
			}
			Msg::Save => {
				if let Some(path) = self.view.show_save_dialog() {
					self.save_playlist(&path);
					self.view.show_msg_dialog(&"Playlist saved".to_string());
				} else {
					self.view.show_error_dialog(&"Impossible to save playlist".to_string());
				}
			}
			Msg::Play => {
				if let Ok(uri) = self.view.get_selected_music() {
					self.model.play(&uri);
					match self.model.current_song().state() {
						Playing => {
							self.view.play(self.model.current_song().song().unwrap());
						}
						Paused => {
							self.view.pause();
						}
						_ => {}
					}
				}
			}
			Msg::Remove => {
				if let Ok(uri) = self.view.get_selected_music() {
					self.model.remove_music(uri);
					self.view.remove_selected_music();
				}
			}
			Msg::Stop => {
				self.model.stop();
				self.view.stop();
			}
			Msg::Next => {
				self.view.next_selected_music();
				self.model.reset_current_song();
				self.update(Msg::Play);
			}
			Msg::Prev => {
				self.view.prev_selected_music();
				self.model.reset_current_song();
				self.update(Msg::Play);
			}
			Msg::UpView => {
				match self.model.current_song().state() {
					Playing => {
						self.view.update_duration(self.model.position(), self.model.duration());
					}
					Stopped => {
						self.view.update_duration(0, 0);
						self.view.stop();
					}
					_ => {}
				}
			}
			Msg::Quit => {
				gtk::main_quit();
			}
		}
	}
}

impl Widget for MusicApp {
	// Specify the type of the root widget.
	type Root = Window;
	
	// Return the root widget.
	fn root(&self) -> Self::Root {
		self.view.window().clone()
	}
	
	fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
		let view = MainWindow::new();
		
		connect!(relm, view.toolbar().save_button(), connect_clicked(_), Msg::Save);
		connect!(relm, view.toolbar().previous_button(), connect_clicked(_), Msg::Prev);
		connect!(relm, view.toolbar().next_button(), connect_clicked(_), Msg::Next);
		connect!(relm, view.toolbar().remove_button(), connect_clicked(_), Msg::Remove);
		connect!(relm, view.toolbar().stop_button(), connect_clicked(_), Msg::Stop);
		connect!(relm, view.toolbar().play_button(), connect_clicked(_), Msg::Play);
		connect!(relm, view.toolbar().open_button(), connect_clicked(_), Msg::Open);
		connect!(relm, view.toolbar().quit_button(), connect_clicked(_), Msg::Quit);
		connect!(relm, view.window(), connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		
		MusicApp {
			model,
			view,
		}
	}
}

fn main() {
	MusicApp::run(()).expect("Win::run failed");
}
