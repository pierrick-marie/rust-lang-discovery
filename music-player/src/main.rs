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

use std::rc::Rc;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use gio::glib;

use gtk::{Adjustment, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, FileFilter, IconSize, Image, Inhibit, Label, ResponseType, Scale, SeparatorToolItem, Window, WindowType};
use gtk::prelude::*;
use gtk::prelude::{
	ButtonExt,
	ContainerExt,
	LabelExt,
	WidgetExt,
};
use gtk::Orientation::Vertical;
use relm_derive::Msg;
use relm::{connect, Relm, Update, Widget, WidgetTest};

pub mod playlist;
use playlist::Playlist;
use crate::toolbar::MusicToolbar;
mod toolbar;
mod song;
mod view;
use view::View;

struct Model {
	playlist: Playlist,
}

struct MusicApp {
	model: Model,
	view: View,
}

#[derive(Msg)]
enum Msg {
	Open,
	Play,
	Quit,
}

impl Update for MusicApp {
	// Specify the model used for this widget.
	type Model = Model;
	// Specify the model parameter used to init the model.
	type ModelParam = ();
	// Specify the type of the messages sent to the update function.
	type Msg = Msg;
	
	fn model(_: &Relm<Self>, _: ()) -> Model {
		Model {
			playlist: Playlist::new(),
			// counter: 0,
		}
	}
	
	fn update(&mut self, event: Msg) {
		// let label = &self.widgets.counter_label;
		
		match event {
			Msg::Open => {
				for file in self.show_open_dialog() {
					self.model.playlist.add(&file);
				}
			}
			Msg::Play => {
				self.view.is_playing = self.model.playlist.play(&self.view.is_playing);
				if self.view.is_playing {
					MusicApp::update_cover(&self.view.cover, &self.model.playlist);
				}
				
				let playlist = &self.model.playlist;
				let widgets = &self.view;
				let is_playing = &self.view.is_playing;
				let cover = &self.view.cover;
				
				// glib::timeout_add_local(Duration::new(0, 100_000_000), move || {
				// 	if *is_playing {
				// 		let m_duration = playlist.duration();
				// 		let m_current_time = playlist.current_time();
				//
				// 		widgets.adjustment.set_upper(m_duration as f64);
				// 		widgets.adjustment.set_value(m_current_time as f64);
				// 		widgets.duration_label.set_label(&format!("{} / {}", Win::milli_to_string(m_current_time), Win::milli_to_string(m_duration)));
				// 		widgets.toolbar.play_button.set_image(Some(&widgets.pause));
				// 	} else {
				// 		widgets.toolbar.play_button.set_image(Some(&widgets.play));
				// 		Win::update_cover(&cover, &playlist);
				// 	}
				// 	Continue(true)
				// });
			}
			Msg::Quit => gtk::main_quit(),
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
		// Create the view using the normal GTK+ method calls.
		let main_container = gtk::Box::new(Vertical, 3);
		
		let toolbar = MusicToolbar::new();
		
		let cover = Image::new();
		
		let duration_label = Label::new(Some("0 / 0"));
		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);
		scale.set_hexpand(true);
		
		let pause = Image::from_icon_name(Some("gtk-media-pause"), IconSize::LargeToolbar);
		let play = Image::from_icon_name(Some("gtk-media-play"), IconSize::LargeToolbar);
		
		let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 3);
		hbox.add(&SeparatorToolItem::new());
		hbox.add(&scale);
		hbox.add(&duration_label);
		hbox.add(&SeparatorToolItem::new());
		
		main_container.add(&toolbar.container);
		main_container.add(&cover);
		main_container.add(&hbox);
		main_container.add(model.playlist.view());
		
		main_container.add(&toolbar.container);
		
		let window = Window::new(WindowType::Toplevel);
		
		window.add(&main_container);
		
		window.show_all();
		
		// Send the message Increment when the button is clicked.
		// connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
		// connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
		connect!(relm, toolbar.play_button, connect_clicked(_), Msg::Play);
		connect!(relm, toolbar.open_button, connect_clicked(_), Msg::Open);
		connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		
		MusicApp {
			model,
			view: View {
				toolbar,
				cover,
				adjustment,
				duration_label,
				play,
				pause,
				window,
				is_playing: false,
			},
		}
	}
}

impl MusicApp {
	fn show_open_dialog(&self) -> Vec<PathBuf> {
		let mut files = vec![];
		let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(&self.view.window), FileChooserAction::Open);
		
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
	
	fn update_cover(cover: &Image, playlist: &Playlist) {
		let res = playlist.get_pixbuf();
		match res {
			Ok(pix) => {
				cover.set_from_pixbuf(Some(&pix));
			}
			Err(_) => {
				cover.clear();
			}
		}
		cover.show();
	}
	
	fn milli_to_string(milli: u64) -> String {
		let mut seconds = milli / 1000;
		let minutes = seconds / 60;
		seconds = seconds - minutes * 60;
		format!("{}:{}", minutes, seconds)
	}
}

fn main() {
	MusicApp::run(()).expect("Win::run failed");
}
