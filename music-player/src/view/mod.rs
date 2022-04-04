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

pub mod toolbar;

use gdk_pixbuf::{
	Pixbuf,
};
use gtk::prelude::*;
use gtk::{Adjustment, FileChooserAction, FileChooserDialog, FileFilter, IconSize, Image, Label, ResponseType, SeparatorToolItem, Window, WindowType, CellRendererPixbuf, CellRendererText, ListStore, TreeView, TreeViewColumn, ScrolledWindow, MessageDialog, DialogFlags, MessageType, ButtonsType};
use std::path::{PathBuf};
use gio::glib::value::{ValueTypeMismatchOrNoneError};

use crate::view::toolbar::*;
use crate::model::music::*;

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const URI_COLUMN: u32 = 7;

#[derive(Clone)]
pub struct MainWindow {
	toolbar: MusicToolbar,
	cover: Image,
	adjustment: Adjustment,
	duration_label: Label,
	play: Image,
	pause: Image,
	treeview: TreeView,
	scrolled_window: ScrolledWindow,
	model: ListStore,
	window: Window,
}

impl MainWindow {
	
	pub fn new() -> Self {
		// Create the music_window using the normal GTK+ method calls.
		let main_container = gtk::Box::new(gtk::Orientation::Vertical, 3);
		
		let toolbar = MusicToolbar::new();
		
		let cover = Image::new();
		
		let duration_label = Label::new(Some("0:00 / 0:00"));
		let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
		adjustment.set_upper(0 as f64);
		adjustment.set_value(0 as f64);
		let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
		scale.set_draw_value(false);
		scale.set_hexpand(true);
		
		let pause = Image::from_icon_name(Some("gtk-media-pause"), IconSize::LargeToolbar);
		let play = Image::from_icon_name(Some("gtk-media-play"), IconSize::LargeToolbar);
		
		let duration_box = gtk::Box::new(gtk::Orientation::Horizontal, 3);
		duration_box.add(&SeparatorToolItem::new());
		duration_box.add(&scale);
		duration_box.add(&duration_label);
		duration_box.add(&SeparatorToolItem::new());
		
		let model = ListStore::new(&[
			Pixbuf::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
		]);
		let treeview = TreeView::with_model(&model);
		treeview.set_hexpand(true);
		treeview.set_vexpand(true);
		Self::create_columns(&treeview);
		
		let scrolled_window_builder = gtk::ScrolledWindow::builder();
		let scrolled_window = scrolled_window_builder.build();
		scrolled_window.set_hexpand(true);
		scrolled_window.set_vexpand(true);
		scrolled_window.add(&treeview);
		
		main_container.add(toolbar.container());
		main_container.add(&cover);
		main_container.add(&duration_box);
		main_container.add(&scrolled_window);
		
		let window = Window::new(WindowType::Toplevel);
		window.add(&main_container);
		window.resize(800, 600);
		window.show_all();
		cover.hide();
		
		MainWindow {
			adjustment,
			cover,
			duration_label,
			pause,
			play,
			toolbar,
			model,
			scrolled_window,
			treeview,
			window,
		}
	}
	
	pub fn add_music(&self, music: &Music) {
		let row = self.model.append();
		
		self.model.set_value(&row, TITLE_COLUMN, &music.title().to_value());
		self.model.set_value(&row, ARTIST_COLUMN, &music.artist().to_value());
		self.model.set_value(&row, ALBUM_COLUMN, &music.album().to_value());
		self.model.set_value(&row, GENRE_COLUMN, &music.genre().to_value());
		self.model.set_value(&row, YEAR_COLUMN, &music.year().to_value());
		self.model.set_value(&row, TRACK_COLUMN, &music.track().to_value());
		self.model.set_value(&row, URI_COLUMN, &music.uri().to_value());
		self.model.set_value(&row, THUMBNAIL_COLUMN, &music.thumbnail().as_ref().unwrap().to_value());
	}
	
	pub fn remove_selected_music(&self) {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			self.model.remove(&iter);
		}
	}
	
	pub fn next_selected_music(&self) {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			if self.model.iter_next(&iter) {
				selection.select_iter(&iter);
			}
		}
	}
	
	pub fn prev_selected_music(&self) {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			if self.model.iter_previous(&iter) {
				selection.select_iter(&iter);
			}
		}
	}
	
	fn create_columns(treeview: &TreeView) {
		Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32);
		Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
		Self::add_text_column(treeview, "Artist", ARTIST_COLUMN as i32);
		Self::add_text_column(treeview, "Album", ALBUM_COLUMN as i32);
		Self::add_text_column(treeview, "Genre", GENRE_COLUMN as i32);
		Self::add_text_column(treeview, "Year", YEAR_COLUMN as i32);
		Self::add_text_column(treeview, "Track", TRACK_COLUMN as i32);
	}
	
	pub fn play(&self, music: &Music) {
		self.toolbar.play_button().set_image(Some(&self.pause));
		self.cover.set_from_pixbuf(music.cover().as_ref());
		self.cover.show();
	}
	
	pub fn pause(&self) {
		self.toolbar.play_button().set_image(Some(&self.play));
	}
	
	pub fn stop(&self) {
		self.toolbar.play_button().set_image(Some(&self.play));
		self.cover.clear();
	}
	
	pub fn get_selected_music(&self) -> Result<String, ValueTypeMismatchOrNoneError> {
		
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			let value = self.model.value(&iter, URI_COLUMN as i32);
			return value.get::<String>();
		}
		Err(ValueTypeMismatchOrNoneError::UnexpectedNone)
	}
	
	pub fn update_duration(&self, current_time: u64, duration: u64) {
		self.adjustment.set_upper(duration as f64);
		self.adjustment.set_value(current_time as f64);
		self.duration_label.set_label(&format!("{} / {}", MainWindow::milli_to_string(current_time), MainWindow::milli_to_string(duration)));
	}
	
	fn add_pixbuf_column(treeview: &TreeView, column: i32) {
		let view_column = TreeViewColumn::new();
			let cell = CellRendererPixbuf::new();
			view_column.pack_start(&cell, true);
			view_column.add_attribute(&cell, "pixbuf", column);
		treeview.append_column(&view_column);
	}
	
	fn add_text_column(treeview: &TreeView, title: &str, column: i32) {
		let view_column = TreeViewColumn::new();
		view_column.set_title(title);
		let cell = CellRendererText::new();
		view_column.set_expand(true);
		view_column.pack_start(&cell, true);
		view_column.add_attribute(&cell, "text", column);
		treeview.append_column(&view_column);
	}
	
	pub fn show_save_dialog(&self) -> Option<PathBuf> {
		let mut file = None;
		let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(&self.window), FileChooserAction::Open);
		
		dialog.set_action(FileChooserAction::Save);
		dialog.add_button("Cancel", ResponseType::Cancel);
		dialog.add_button("Accept", ResponseType::Accept);
		let result = dialog.run();
		
		if result == ResponseType::Accept {
			file = dialog.filename();
		}
		unsafe { dialog.destroy(); }
		file
	}
	
	pub fn show_msg_dialog(&self, msg: &String) {
		let dialog = MessageDialog::new(Some(&self.window), DialogFlags::MODAL, MessageType::Info, ButtonsType::Ok, msg);
		
		dialog.run();
		
		unsafe { dialog.destroy(); }
	}
	
	pub fn show_error_dialog(&self, msg: &String) {
		let dialog = MessageDialog::new(Some(&self.window), DialogFlags::MODAL, MessageType::Error, ButtonsType::Ok, msg);
		
		dialog.run();
		
		unsafe { dialog.destroy(); }
	}
	
	pub fn show_open_dialog(&self) -> Vec<PathBuf> {
		let mut files = vec![];
		let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(&self.window), FileChooserAction::Open);
		
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
	
	pub fn milli_to_string(milli: u64) -> String {
		let mut seconds = milli / 1000;
		let minutes = seconds / 60;
		seconds = seconds - minutes * 60;
		format!("{}:{:02}", minutes, seconds)
	}
	
	pub fn treeview(&self) -> &TreeView {
		&self.treeview
	}
	
	pub fn window(&self) -> &Window {
		&self.window
	}
	
	pub fn toolbar(&self) -> &MusicToolbar {
		&self.toolbar
	}
}