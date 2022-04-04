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

extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
extern crate gio;
extern crate crossbeam;
extern crate pulse_simple;
extern crate simplemad;

use std::collections::HashMap;
use gio::glib::SeekType::Cur;
use gtk::prelude::*;

pub mod music;
use music::Music;

use self::State::*;

#[derive(Clone)]
pub enum State {
	Playing,
	Paused,
	Stopped,
}

#[derive(Clone)]
pub struct CurrentSong {
	song: Option<Music>,
	state: State,
}

pub struct MusicModel {
	songs: HashMap<String, Music>,
	current_song: CurrentSong,
	player: gst_player::Player,
}

impl CurrentSong {
	
	pub fn new() -> Self {
		CurrentSong {
			song: None,
			state: Stopped,
		}
	}
	
	pub fn song(&self) -> Option<&Music> {
		self.song.as_ref()
	}
	
	pub fn state(&self) -> State {
		self.state.clone()
	}
}

impl MusicModel {
	
	pub fn new() -> Self {
		let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
		let player = gst_player::Player::new(None, Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()));

		MusicModel {
			songs: HashMap::new(),
			current_song: CurrentSong::new(),
			player,
		}
	}
	
	pub fn songs(&self) -> &HashMap<String, Music> {
		&self.songs
	}
	
	pub fn current_song(&self) -> CurrentSong {
		self.current_song.clone()
	}
	
	pub fn reset_current_song(&mut self)  {
		self.current_song = CurrentSong::new();
	}
	
	pub fn add_music(&mut self, music: &Music) {
		self.songs.insert(music.uri().to_string(), music.clone());
	}
	
	pub fn duration(&self) -> u64 {
		if let Some(duration) = self.player.duration() {
			return duration.mseconds();
		}
		// else
		return 0 as u64;
	}
	
	pub fn position(&self) -> u64 {
		if let Some(position) = self.player.position() {
			return position.mseconds();
		}
		// else
		return 0 as u64;
	}
	
	pub fn remove_music(&mut self, uri: String) {
		if let Some(song) = self.songs.remove(uri.as_str()) {
			if let Some(player_uri) = self.player.uri() {
				if song.uri() == player_uri.as_str() {
					self.player.stop();
					self.current_song = CurrentSong::new();
				}
			}
		}
	}
	
	pub fn play(&mut self, uri: &String) {
		match self.current_song.state {
			Playing => {
				self.player.pause();
				self.current_song.state = Paused;
			}
			Paused => {
				if uri != self.current_song.song.as_ref().unwrap().uri() {
					self.player.set_uri(Some(uri.as_str()));
					self.current_song.song = Some(self.songs.get(uri.as_str()).unwrap().clone());
				}
				self.player.play();
				self.current_song.state = Playing
			}
			Stopped => {
				self.player.set_uri(Some(uri.as_str()));
				self.player.play();
				self.current_song = CurrentSong {
					song: Some(self.songs.get(uri.as_str()).unwrap().clone()),
					state: Playing,
				}
			}
		}
	}
	
	pub fn stop(&mut self) {
		self.player.stop();
		self.current_song.state = Stopped;
	}
}