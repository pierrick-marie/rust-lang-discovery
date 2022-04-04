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

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Button, Box, IconSize, SeparatorToolItem};

#[derive(Clone)]
pub struct MusicToolbar {
	open_button: Button,
	save_button: Button,
	next_button: Button,
	play_button: Button,
	previous_button: Button,
	quit_button: Button,
	remove_button: Button,
	stop_button: Button,
	container: Box,
}

impl MusicToolbar {
	
	pub fn new() -> Self {
		let toolbar = Box::new(gtk::Orientation::Horizontal, 3);

		toolbar.add(&SeparatorToolItem::new());

		let open_button = Button::from_icon_name(Some("gtk-open"), IconSize::LargeToolbar);
		toolbar.add(&open_button);
		
		let save_button = Button::from_icon_name(Some("gtk-save"), IconSize::LargeToolbar);
		toolbar.add(&save_button);
		
		toolbar.add(&SeparatorToolItem::new());

		let previous_button = Button::from_icon_name(Some("gtk-media-previous"), IconSize::LargeToolbar);
		toolbar.add(&previous_button);

		let play_button = Button::from_icon_name(Some("gtk-media-play"), IconSize::LargeToolbar);
		toolbar.add(&play_button);

		let stop_button = Button::from_icon_name(Some("gtk-media-stop"), IconSize::LargeToolbar);
		toolbar.add(&stop_button);

		let next_button = Button::from_icon_name(Some("gtk-media-next"), IconSize::LargeToolbar);
		toolbar.add(&next_button);
		toolbar.add(&SeparatorToolItem::new());

		let remove_button = Button::from_icon_name(Some("gtk-remove"), IconSize::LargeToolbar);
		toolbar.add(&remove_button);
		toolbar.add(&SeparatorToolItem::new());

		let quit_button = Button::from_icon_name(Some("gtk-quit"), IconSize::LargeToolbar);
		toolbar.add(&quit_button);
		
		toolbar.add(&SeparatorToolItem::new());

		MusicToolbar {
			open_button,
			save_button,
			next_button,
			play_button,
			previous_button,
			quit_button,
			remove_button,
			stop_button,
			container: toolbar,
		}
	}
	
	pub fn open_button(&self) -> &Button {
		&self.open_button
	}
	
	pub fn save_button(&self) -> &Button {
		&self.save_button
	}
	
	pub fn next_button(&self) -> &Button {
		&self.next_button
	}
	
	pub fn previous_button(&self) -> &Button {
		&self.previous_button
	}
	
	pub fn stop_button(&self) -> &Button {
		&self.stop_button
	}
	
	pub fn remove_button(&self) -> &Button {
		&self.remove_button
	}
	
	pub fn quit_button(&self) -> &Button {
		&self.quit_button
	}
	
	pub fn play_button(&self) -> &Button {
		&self.play_button
	}
	
	pub fn container(&self) -> &Box {
		&self.container
	}
}