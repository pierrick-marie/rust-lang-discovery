extern crate gtk;
extern crate gio;

use std::env::args;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, IconSize, SeparatorToolItem};

use std::sync::atomic::{AtomicIsize, Ordering};


const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
	let app = Application::builder()
		.application_id("org.example.HelloWorld")
		.build();

	app.connect_activate(|app| {
		// We create the main window.
		let window = ApplicationWindow::builder()
			.application(app)
			// .default_width(256)
			// .default_height(120)
			.title("Rust music player")
			.build();

		// Container
		let container = Box::new(gtk::Orientation::Horizontal, 3);

		let open = Button::from_icon_name(Some("gtk-open"), IconSize::LargeToolbar);
		container.add(&open);

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

		window.connect_delete_event(|_, _| {
			Inhibit(false)
		});

		window.add(&container);

		// Don't forget to make all widgets visible.
		window.show_all();
	});

	app.run();
}
