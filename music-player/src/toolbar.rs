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
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem};

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
	open_button: Button,
	next_button: Button,
	play_button: Button,
	previous_button: Button,
	quit_button: Button,
	remove_button: Button,
	stop_button: Button,
	toolbar: Box,
}

impl MusicToolbar {

	pub fn new() -> Self {

		let toolbar = Box::new(gtk::Orientation::Horizontal, 3);

		let open_button = Button::from_icon_name(Some("gtk-open"), IconSize::LargeToolbar);
		toolbar.add(&open_button);

		toolbar.add(&SeparatorToolItem::new());

		let previous_button = Button::from_icon_name(Some("gtk-media-previous"), IconSize::LargeToolbar);
		toolbar.add(&previous_button);

		let play_button = Button::from_icon_name(Some(PLAY_STOCK), IconSize::LargeToolbar);
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

		MusicToolbar {
			open_button,
			next_button,
			play_button,
			previous_button,
			quit_button,
			remove_button,
			stop_button,
			toolbar,
		}
	}

	pub fn connect_toolbar_events(&self, win: &ApplicationWindow) {
		let window = win.clone();
		self.quit_button.connect_clicked(move |_| {
			unsafe { window.destroy(); }
		});

		let play_button = self.play_button.clone();
		self.play_button.connect_clicked(move |_| {
			// if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
			// 	play_button.set_stock_id(PAUSE_STOCK);
			// } else {
			// 	play_button.set_stock_id(PLAY_STOCK);
			// }
		});
	}

	pub fn container(&self) -> &Box {
		&self.toolbar
	}
}
