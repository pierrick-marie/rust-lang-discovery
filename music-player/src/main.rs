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
use gtk::prelude::*;

use gtk::{
	Button,
	Inhibit,
	Label,
	Window,
	WindowType,
	prelude::ButtonExt,
	prelude::ContainerExt,
	prelude::LabelExt,
	prelude::WidgetExt,
};
use gtk::Orientation::Vertical;
use relm_derive::Msg;
use relm::{connect, Relm, Update, Widget, WidgetTest};

pub mod playlist;
use playlist::Playlist;
mod toolbar;
use crate::toolbar::MusicToolbar;

struct Model {
	counter: i32,
}

#[derive(Msg)]
enum Msg {
	Decrement,
	Increment,
	Quit,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
	counter_label: Label,
	minus_button: Button,
	plus_button: Button,
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
			counter: 0,
		}
	}
	
	fn update(&mut self, event: Msg) {
		let label = &self.widgets.counter_label;
		
		match event {
			Msg::Decrement => {
				self.model.counter -= 1;
				// Manually update the view.
				label.set_text(&self.model.counter.to_string());
			},
			Msg::Increment => {
				self.model.counter += 1;
				label.set_text(&self.model.counter.to_string());
			},
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
		let vbox = gtk::Box::new(Vertical, 0);
		
		let plus_button = Button::with_label("+");
		vbox.add(&plus_button);
		
		let counter_label = Label::new(Some("0"));
		vbox.add(&counter_label);
		
		let minus_button = Button::with_label("-");
		vbox.add(&minus_button);
		
		let window = Window::new(WindowType::Toplevel);
		
		window.add(&vbox);
		
		window.show_all();
		
		// Send the message Increment when the button is clicked.
		connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
		connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
		connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
		
		Win {
			model,
			widgets: Widgets {
				counter_label,
				minus_button,
				plus_button,
				window,
			},
		}
	}
}

fn main() {
	Win::run(()).expect("Win::run failed");
}
