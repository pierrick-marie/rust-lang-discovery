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

use crate::gtk::prelude::*;
use std::path::Path;
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gtk::{
	CellLayout,
	CellRendererPixbuf,
	CellRendererText,
	ListStore,
	TreeIter,
	TreeView,
	TreeViewColumn,
};
use id3::Tag;

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

pub struct Playlist {
	model: ListStore,
	treeview: TreeView,
}

use self::Visibility::*;

#[derive(PartialEq)]
enum Visibility {
	Invisible,
	Visible,
}

impl Playlist {
	pub fn new() -> Self {
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
}
