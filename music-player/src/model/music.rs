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
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};

extern crate id3;
use id3::{Tag, TagLike};

use std::fs::File;
use std::path::Path;
use std::{fs};
use std::os::unix::raw::time_t;

use gtk::Image;

use gtk::prelude::*;

const THUMBNAIL_SIZE: i32 = 64;
const IMAGE_SIZE: i32 = 256;
const URI: &str = "file://";
const INTERP_HYPER: InterpType = InterpType::Hyper;

#[derive(Debug, Clone)]
pub struct Music {
	title: String,
	artist: String,
	album: String,
	genre: String,
	year: String,
	track: String,
	uri: String,
	thumbnail: Option<Pixbuf>,
	cover: Option<Pixbuf>,
}

impl Music {
	
	pub fn new(path: &Path) -> Self {
		let uri = format!("{}{}", URI, path.clone().to_string_lossy().to_string());
		
		if let Ok(tag) = Tag::read_from_path(path.clone()) {
			let title = tag.title().unwrap_or(&path.to_str().unwrap_or("(no title)")).to_string();
			let artist = tag.artist().unwrap_or("(no artist)").to_string();
			let album = tag.album().unwrap_or("(no album)").to_string();
			let genre = tag.genre().unwrap_or("(no genre)").to_string();
			let year = tag.year().map(|year| year.to_string()).unwrap_or("(no year)".to_string()).to_string();
			let track = tag.track().map(|track| track.to_string()).unwrap_or("??".to_string());
			let total_tracks = tag.total_tracks().map(|total_tracks| total_tracks.to_string()).unwrap_or("??".to_string());
			let track_value = format!("{} / {}", track, total_tracks);
			let covers = Music::get_pixbuf(&tag);
			
			return Music {
				album,
				artist,
				cover: covers.clone().0,
				genre,
				thumbnail: covers.clone().1,
				title,
				track: track_value,
				uri,
				year,
			};
		}
		// else
		return Music {
			album: "".to_string(),
			artist: "".to_string(),
			cover: None,
			genre: "".to_string(),
			thumbnail: None,
			title: path.to_str().unwrap_or("(no title)").to_string(),
			track: "".to_string(),
			uri,
			year: "".to_string(),
		};
	}
	
	/*
	 * returns Option<(Cover, Thumbnail)>
	 */
	fn get_pixbuf(tag: &Tag) -> (Option<Pixbuf>, Option<Pixbuf>) {
		if let Some(picture) = tag.pictures().next() {
			let pixbuf_loader = PixbufLoader::new();
			pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
			pixbuf_loader.write(&picture.data).unwrap();
			
			if let Some(pixbuf) = pixbuf_loader.pixbuf() {
				let thumbnail = pixbuf.scale_simple(THUMBNAIL_SIZE, THUMBNAIL_SIZE, INTERP_HYPER);
				pixbuf_loader.close().unwrap();
				return (Some(pixbuf), thumbnail);
			}
			pixbuf_loader.close().unwrap();
		}
		(None, None)
	}
	
	pub fn uri(&self) -> &String {
		&self.uri
	}
	
	pub fn title(&self) -> &String {
		&self.title
	}
	
	pub fn artist(&self) -> &String {
		&self.artist
	}
	
	pub fn album(&self) -> &String {
		&self.album
	}
	
	pub fn genre(&self) -> &String {
		&self.genre
	}
	
	pub fn year(&self) -> &String {
		&self.year
	}
	
	pub fn track(&self) -> &String {
		&self.track
	}
	
	pub fn thumbnail(&self) -> &Option<Pixbuf> {
		&self.thumbnail
	}
	
	pub fn cover(&self) -> &Option<Pixbuf> {
		&self.cover
	}
}