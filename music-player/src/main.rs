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



mod view;
mod model;


use crate::model::music;
use model::State::*;
use model::MusicModel;
use model::CurrentSong;
use music::Music;

use crate::view::{main_window, playlist, toolbar};
use view::main_window::MainWindow;


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

use std::path::PathBuf;
use gio::glib;
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
use relm::{connect, interval, Relm, Update, Widget, WidgetTest};

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
					let music = Music::new(file.as_path());
					self.view.add_music(&music);
					self.model.add_music(&music);
				}
				self.view.treeview.queue_resize();
				self.view.window.queue_resize();
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
				self.model.current_song().set_song(None);
				self.model.current_song().set_state(Stopped);
				self.update(Msg::Play);
			}
			Msg::Prev => {
				self.view.prev_selected_music();
				self.model.current_song().set_song(None);
				self.model.current_song().set_state(Stopped);
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
		self.view.window.clone()
	}
	
	fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
		let view = MainWindow::new();
		
		connect!(relm, view.toolbar.previous_button, connect_clicked(_), Msg::Prev);
		connect!(relm, view.toolbar.next_button, connect_clicked(_), Msg::Next);
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
