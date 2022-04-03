extern crate gdk_pixbuf;
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};

extern crate id3;
use id3::{Tag, TagLike};

use std::fs::File;
use std::path::Path;
use std::{fs};

use gtk::Image;

use crate::gtk::prelude::*;

const THUMBNAIL_SIZE: i32 = 64;
const IMAGE_SIZE: i32 = 256;
const URI: &str = "file://";
const INTERP_HYPER: InterpType = InterpType::Hyper;

#[derive(Debug)]
pub struct Song {
	title: String,
	artist: String,
	album: String,
	genre: String,
	year: String,
	track: String,
	uri: String,
	duration: u64,
	thumbnail: Image,
	cover: Image,
}

impl Song {
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
			let covers = Song::get_pixbuf(&tag);
			
			return Song {
				album,
				artist,
				cover: Image::from_pixbuf(Some(&covers.clone().unwrap().0)),
				duration: 0,
				genre,
				thumbnail: Image::from_pixbuf(Some(&covers.clone().unwrap().1)),
				title,
				track: track_value,
				uri,
				year,
			};
		}
		// else
		return Song {
			album: "".to_string(),
			artist: "".to_string(),
			cover: Image::new(),
			duration: 0,
			genre: "".to_string(),
			thumbnail: Image::new(),
			title: path.to_str().unwrap_or("(no title)").to_string(),
			track: "".to_string(),
			uri,
			year: "".to_string(),
		};
	}
	
	/*
	 * returns Option<(Cover, Thumbnail)>
	 */
	fn get_pixbuf(tag: &Tag) -> Option<(Pixbuf, Pixbuf)> {
		if let Some(picture) = tag.pictures().next() {
			let pixbuf_loader = PixbufLoader::new();
			pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
			pixbuf_loader.write(&picture.data).unwrap();
			
			if let Some(pixbuf) = pixbuf_loader.pixbuf() {
				let thumbnail = pixbuf.scale_simple(THUMBNAIL_SIZE, THUMBNAIL_SIZE, INTERP_HYPER).unwrap();
				pixbuf_loader.close().unwrap();
				return Some((pixbuf, thumbnail));
			}
			pixbuf_loader.close().unwrap();
		}
		None
	}
}