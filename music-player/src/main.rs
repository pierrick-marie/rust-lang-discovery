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

use gtk::{Button, Inhibit, Image, Adjustment, Label, Window, WindowType, ApplicationWindow, FileChooserDialog, FileChooserAction, FileFilter, ResponseType, Scale, IconSize, SeparatorToolItem};
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
mod toolbar;
use crate::toolbar::MusicToolbar;

struct Model {
	playlist: Playlist,
	// counter: i32,
}

#[derive(Msg)]
enum Msg {
	// Decrement,
	// Increment,
	Open,
	Quit,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
	toolbar: MusicToolbar,
	cover: Image,
	adjustment: Adjustment,
	duration_label: Label,
	play: Image,
	pause: Image,
	// counter_label: Label,
	// minus_button: Button,
	// plus_button: Button,
	window: Window,
}

struct Win {
	model: Model,
	widgets: Widgets,
}

impl Update for Win {
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
				for file in Widgets::show_open_dialog(&self.widgets.window) {
					self.model.playlist.add(&file);
				}
			}
			// Msg::Decrement => {
			// 	self.model.counter -= 1;
			// 	// Manually update the view.
			// 	label.set_text(&self.model.counter.to_string());
			// },
			// Msg::Increment => {
			// 	self.model.counter += 1;
			// 	label.set_text(&self.model.counter.to_string());
			// },
			Msg::Quit => gtk::main_quit(),
		}
	}
}

impl Widget for Win {
	// Specify the type of the root widget.
	type Root = Window;
	
	// Return the root widget.
	fn root(&self) -> Self::Root {
		self.widgets.window.clone()
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
		connect!(relm, toolbar.open_button, connect_clicked(_), Msg::Open);
		connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		
		Win {
			model,
			widgets: Widgets {
				toolbar,
				cover,
				adjustment,
				duration_label,
				play,
				pause,
				window,
			},
		}
	}
}

impl Widgets {
	fn show_open_dialog(parent: &Window) -> Vec<PathBuf> {
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
	Win::run(()).expect("Win::run failed");
}