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

extern crate gdk_pixbuf;
extern crate id3;

use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gdk_pixbuf::glib::value::ValueTypeChecker;

use gio::glib::value::{ValueTypeMismatchError, ValueTypeMismatchOrNoneError};

use crate::gtk::prelude::*;
use gtk::{CellLayout, CellRendererPixbuf, CellRendererText, ListStore, TreeIter, TreeView, TreeViewColumn, FileChooserAction, FileChooserDialog, FileFilter, ApplicationWindow};

use id3::{Tag, TagLike};

use crate::player::{Player, Action};

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const PATH_COLUMN: u32 = 7;
const PIXBUF_COLUMN: u32 = 8;
const IMAGE_SIZE: i32 = 256;
const THUMBNAIL_SIZE: i32 = 64;

const INTERP_HYPER: InterpType = InterpType::Hyper;

pub struct Playlist {
	model: ListStore,
	player: Player,
	treeview: TreeView,
}

use self::Visibility::*;

#[derive(PartialEq)]
enum Visibility {
	Invisible,
	Visible,
}

impl Playlist {
	pub(crate) fn new() -> Self {
		let model = ListStore::new(&[
			Pixbuf::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			String::static_type(),
			Pixbuf::static_type(),
		]);
		let treeview = TreeView::with_model(&model);
		treeview.set_hexpand(true);
		treeview.set_vexpand(true);

		Self::create_columns(&treeview);

		Playlist {
			model,
			player: Player::new(),
			treeview,
		}
	}

	pub fn view(&self) -> &TreeView {
		&self.treeview
	}

	fn create_columns(treeview: &TreeView) {
		Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32, Visible);
		Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
		Self::add_text_column(treeview, "Artist", ARTIST_COLUMN as i32);
		Self::add_text_column(treeview, "Album", ALBUM_COLUMN as i32);
		Self::add_text_column(treeview, "Genre", GENRE_COLUMN as i32);
		Self::add_text_column(treeview, "Year", YEAR_COLUMN as i32);
		Self::add_text_column(treeview, "Track", TRACK_COLUMN as i32);
		Self::add_pixbuf_column(treeview, PIXBUF_COLUMN as i32, Invisible);
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

	fn add_pixbuf_column(treeview: &TreeView, column: i32, visibility: Visibility) {
		let view_column = TreeViewColumn::new();
		if visibility == Visible {
			let cell = CellRendererPixbuf::new();
			view_column.pack_start(&cell, true);
			view_column.add_attribute(&cell, "pixbuf", column);
		}
		treeview.append_column(&view_column);
	}

	fn set_pixbuf(&self, row: &TreeIter, tag: &Tag) {
		if let Some(picture) = tag.pictures().next() {
			let pixbuf_loader = PixbufLoader::new();
			pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
			pixbuf_loader.write(&picture.data).unwrap();

			if let Some(pixbuf) = pixbuf_loader.pixbuf() {
				let thumbnail = pixbuf.scale_simple(THUMBNAIL_SIZE, THUMBNAIL_SIZE, INTERP_HYPER).unwrap();
				self.model.set_value(row, THUMBNAIL_COLUMN, &thumbnail.to_value());
				self.model.set_value(row, PIXBUF_COLUMN, &pixbuf.to_value());
			}
			pixbuf_loader.close().unwrap();
		}
	}

	pub fn selected_path(&self) -> Result<String, ValueTypeMismatchOrNoneError> {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			let value = self.model.value(&iter, PATH_COLUMN as i32);
			return value.get::<String>();
		}
		Err(ValueTypeMismatchOrNoneError::UnexpectedNone)
	}

	pub fn play(&self) {
		let state = (*self.player.state.lock().unwrap()).clone();
		match state {
			Action::Stop => {
				let res = self.selected_path();

				match res {
					Ok(path) => {
						let action = Action::Play(Path::new(&path).to_path_buf());
						self.player.queue.push(action.clone());
						*self.player.state.lock().unwrap() = action.clone();
					}
					Err(msg) => {
					}
				}
			}
			_ => {
				let action = Action::Pause;
				self.player.queue.push(action.clone());
				*self.player.state.lock().unwrap() = action.clone();
			}
		}
	}

	pub fn stop(&self) {

		self.player.queue.push(Action::Stop);
		*self.player.state.lock().unwrap() = Action::Stop;
	}

	pub fn remove_selection(&self) {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			self.model.remove(&iter);
		}
	}

	pub fn pixbuf(&self) -> Result<Pixbuf, ValueTypeMismatchOrNoneError> {
		let selection = self.treeview.selection();
		if let Some((_, iter)) = selection.selected() {
			let value = self.model.value(&iter, PIXBUF_COLUMN as i32);
			return value.get::<Pixbuf>();
		}
		Err(ValueTypeMismatchOrNoneError::UnexpectedNone)
	}

	pub fn add(&self, path: &Path) {
		let filename = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
		let row = self.model.append();

		if let Ok(tag) = Tag::read_from_path(path) {
			let title = tag.title().unwrap_or(filename);
			let artist = tag.artist().unwrap_or("(no artist)");
			let album = tag.album().unwrap_or("(no album)");
			let genre = tag.genre().unwrap_or("(no genre)");
			let year = tag.year().map(|year|
				year.to_string()).unwrap_or("(no year)".to_string());
			let track = tag.track().map(|track|
				track.to_string()).unwrap_or("??".to_string());
			let total_tracks = tag.total_tracks().map(|total_tracks|
				total_tracks.to_string()).unwrap_or("??".to_string());
			let track_value = format!("{} / {}", track, total_tracks);

			self.set_pixbuf(&row, &tag);
			self.model.set_value(&row, TITLE_COLUMN, &title.to_value());
			self.model.set_value(&row, ARTIST_COLUMN, &artist.to_value());
			self.model.set_value(&row, ALBUM_COLUMN, &album.to_value());
			self.model.set_value(&row, GENRE_COLUMN, &genre.to_value());
			self.model.set_value(&row, YEAR_COLUMN, &year.to_value());
			self.model.set_value(&row, TRACK_COLUMN, &track_value.to_value());
		} else {
			self.model.set_value(&row, TITLE_COLUMN, &filename.to_value());
		}
		let path = path.to_str().unwrap_or_default();
		self.model.set_value(&row, PATH_COLUMN, &path.to_value());
	}
}
