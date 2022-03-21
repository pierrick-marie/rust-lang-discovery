extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem};

const PLAY_STOCK: &str = "gtk-media-play";

pub struct MusicToolbar {
	open_button: Button,
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

		let container = Box::new(gtk::Orientation::Horizontal, 3);

		let open_button = Button::from_icon_name(Some("gtk-open"), IconSize::LargeToolbar);
		container.add(&open_button);

		container.add(&SeparatorToolItem::new());

		let previous_button = Button::from_icon_name(Some("gtk-media-previous"), IconSize::LargeToolbar);
		container.add(&previous_button);

		let play_button = Button::from_icon_name(Some(PLAY_STOCK), IconSize::LargeToolbar);
		container.add(&play_button);

		let stop_button = Button::from_icon_name(Some("gtk-media-stop"), IconSize::LargeToolbar);
		container.add(&stop_button);

		let next_button = Button::from_icon_name(Some("gtk-media-next"), IconSize::LargeToolbar);
		container.add(&next_button);
		container.add(&SeparatorToolItem::new());

		let remove_button = Button::from_icon_name(Some("gtk-remove"), IconSize::LargeToolbar);
		container.add(&remove_button);
		container.add(&SeparatorToolItem::new());

		let quit_button = Button::from_icon_name(Some("gtk-quit"), IconSize::LargeToolbar);
		container.add(&quit_button);

		MusicToolbar {
			open_button,
			next_button,
			play_button,
			previous_button,
			quit_button,
			remove_button,
			stop_button,
			container,
		}
	}

	pub fn container(&self) -> &Box {
		&self.container
	}
}
