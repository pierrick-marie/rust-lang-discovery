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

#[feature(proc_macro)]
extern crate gtk;
extern crate gtk_sys;

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate gio;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
use std::collections::HashMap;
use gst::ClockTime;

use std::rc::Rc;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use gio::ffi::g_socket_set_keepalive;
use gio::glib;
use gst::StreamStatusType::Stop;

use gtk::{
	Adjustment,
	ApplicationWindow,
	Button,
	FileChooserAction,
	FileChooserDialog,
	FileFilter,
	IconSize,
	Image,
	Inhibit,
	Label,
	ResponseType,
	Scale,
	SeparatorToolItem,
	Window,
	WindowType,
};
use gtk::prelude::*;
use gtk::prelude::{
	ButtonExt,
	ContainerExt,
	LabelExt,
	WidgetExt,
};
use gtk::Orientation::Vertical;
use relm_derive::Msg;
use relm::{connect, Relm, Update, Widget, WidgetTest, interval};

mod model;
mod view;

use crate::view::main_window;
use crate::view::toolbar;
use crate::view::playlist;

use playlist::Playlist;
use main_window::MainWindow;
use toolbar::MusicToolbar;
use crate::model::Music;
use self::State::*;

enum State {
	Playing,
	Paused,
	Stopped,
}

struct CurrentSong {
	song: Option<Music>,
	state: State,
}

struct Model {
	songs: HashMap<String, Music>,
	current_song: CurrentSong,
	player: gst_player::Player,
}

struct MusicApp {
	model: Model,
	view: MainWindow,
}

#[derive(Msg)]
enum Msg {
	Open,
	Play,
	Quit,
	Stop,
	Remove,
	UpView,
}

impl Model {
	pub fn play(&mut self, uri: &String) {
		match self.current_song.state {
			Playing => {
				self.player.pause();
				self.current_song.state = Paused;
			}
			Paused => {
				self.player.play();
				self.current_song.state = Playing
			}
			Stopped => {
				self.player.set_uri(Some(uri.as_str()));
				self.player.play();
				self.current_song = CurrentSong {
					song: Some(self.songs.get(uri.as_str()).unwrap().clone()),
					state: Playing,
				}
			}
		}
	}
	
	pub fn stop(&mut self) {
		self.player.stop();
		self.current_song.state = Stopped;
	}
	
	pub fn remove(&mut self, uri: String) {
		if let Some(song) = self.songs.remove(uri.as_str()) {
			if let Some(player_uri) = self.player.uri() {
				if song.uri() == player_uri.as_str() {
					self.player.stop();
					self.current_song.song = None;
					self.current_song.state = Stopped;
				}
			}
		}
	}
}

impl Update for MusicApp {
	// Specify the model used for this widget.
	type Model = Model;
	// Specify the model parameter used to init the model.
	type ModelParam = ();
	// Specify the type of the messages sent to the update function.
	type Msg = Msg;
	
	fn model(_: &Relm<Self>, _: ()) -> Model {
		let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
		let player = gst_player::Player::new(None, Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()));
		Model {
			songs: HashMap::new(),
			current_song: CurrentSong {
				song: None,
				state: Stopped,
			},
			player,
		}
	}
	
	fn subscriptions(&mut self, relm: &Relm<Self>) {
		interval(relm.stream(), 100, || Msg::UpView);
	}
	
	fn update(&mut self, event: Msg) {
		match event {
			Msg::Open => {
				for file in self.view.show_open_dialog() {
					let music = Music::new(file.as_path());
					self.view.add_music(&music);
					self.model.songs.insert(music.uri(), music);
				}
				self.view.treeview.queue_resize();
				self.view.window.queue_resize();
			}
			Msg::Play => {
				if let Ok(uri) = self.view.get_selected_music() {
					self.model.play(&uri);
					match self.model.current_song.state {
						Playing => {
							self.view.play(self.model.current_song.song.as_ref().unwrap());
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
					self.model.remove(uri);
					self.view.remove_selected_music();
				}
			}
			Msg::Stop => {
				self.model.stop();
				self.view.stop();
			}
			Msg::UpView => {
				match self.model.current_song.state {
					Playing => {
						if let Some(durration) = self.model.player.duration() {
							if let Some(current_time) = self.model.player.position() {
								self.view.update_duration(current_time.mseconds(), durration.mseconds());
							}
						}
					}
					Stopped => {
						self.view.update_duration(0, 0);
						self.view.stop();
					}
					_ => { }
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
		self.view.window.clone()
	}
	
	fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
		let view = MainWindow::new();
		
		connect!(relm, view.toolbar.remove_button, connect_clicked(_), Msg::Remove);
		connect!(relm, view.toolbar.stop_button, connect_clicked(_), Msg::Stop);
		connect!(relm, view.toolbar.play_button, connect_clicked(_), Msg::Play);
		connect!(relm, view.toolbar.open_button, connect_clicked(_), Msg::Open);
		connect!(relm, view.toolbar.quit_button, connect_clicked(_), Msg::Quit);
		connect!(relm, view.window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		
		MusicApp {
			model,
			view,
		}
	}
}

fn main() {
	MusicApp::run(()).expect("Win::run failed");
}
