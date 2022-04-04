use crate::{Music, MusicToolbar};
use crate::Playlist;

use gdk_pixbuf::{
	InterpType,
	Pixbuf,
	PixbufLoader,
};

use gtk::prelude::*;
use gtk::{Adjustment, FileChooserAction, FileChooserDialog, FileFilter, IconSize, Image, Label, ResponseType, ScrollablePolicy, SeparatorToolItem, Window, WindowType};
use gtk::{
	CellRendererPixbuf,
	CellRendererText,
	ListStore,
	TreeIter,
	TreeView,
	TreeViewColumn
};

use std::path::{Path, PathBuf};

use gio::glib::value::{ValueTypeMismatchOrNoneError};

use self::Visibility::*;

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const URI_COLUMN: u32 = 7;
// const PIXBUF_COLUMN: u32 = 8;

#[derive(PartialEq)]
enum Visibility {
	Invisible,
	Visible,
}

#[derive(Clone)]
pub struct MainWindow {
	pub toolbar: MusicToolbar,
	pub cover: Image,
	adjustment: Adjustment,
	duration_label: Label,
	play: Image,
	pause: Image,
	pub treeview: TreeView,
	model: ListStore,
	pub window: Window,
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
		
		main_container.add(&toolbar.container);
		main_container.add(&cover);
		main_container.add(&duration_box);
		main_container.add(&treeview);
		
		let window = Window::new(WindowType::Toplevel);
		window.add(&main_container);
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
			treeview,
			window,
		}
	}
	
	pub fn add_music(&self, music: &Music) {
		let row = self.model.append();
		
		self.model.set_value(&row, TITLE_COLUMN, &music.title.to_value());
		self.model.set_value(&row, ARTIST_COLUMN, &music.artist.to_value());
		self.model.set_value(&row, ALBUM_COLUMN, &music.album.to_value());
		self.model.set_value(&row, GENRE_COLUMN, &music.genre.to_value());
		self.model.set_value(&row, YEAR_COLUMN, &music.year.to_value());
		self.model.set_value(&row, TRACK_COLUMN, &music.track.to_value());
		self.model.set_value(&row, URI_COLUMN, &music.uri().to_value());
		self.model.set_value(&row, THUMBNAIL_COLUMN, &music.thumbnail.as_ref().unwrap().to_value());
	}
	
	pub fn remove_selected_music(&self) {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			self.model.remove(&iter);
		}
	}
	
	fn create_columns(treeview: &TreeView) {
		Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32, Visible);
		Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
		Self::add_text_column(treeview, "Artist", ARTIST_COLUMN as i32);
		Self::add_text_column(treeview, "Album", ALBUM_COLUMN as i32);
		Self::add_text_column(treeview, "Genre", GENRE_COLUMN as i32);
		Self::add_text_column(treeview, "Year", YEAR_COLUMN as i32);
		Self::add_text_column(treeview, "Track", TRACK_COLUMN as i32);
	}
	
	pub fn play(&self, music: &Music) {
		self.toolbar.play_button.set_image(Some(&self.pause));
		self.cover.set_from_pixbuf(music.cover.as_ref());
		self.cover.show();
	}
	
	pub fn pause(&self) {
		self.toolbar.play_button.set_image(Some(&self.play));
	}
	
	pub fn stop(&self) {
		self.toolbar.play_button.set_image(Some(&self.play));
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
	
	fn add_pixbuf_column(treeview: &TreeView, column: i32, visibility: Visibility) {
		let view_column = TreeViewColumn::new();
		if visibility == Visible {
			let cell = CellRendererPixbuf::new();
			view_column.pack_start(&cell, true);
			view_column.add_attribute(&cell, "pixbuf", column);
		}
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
}