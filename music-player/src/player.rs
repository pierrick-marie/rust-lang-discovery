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

use std::borrow::Borrow;
use std::cell::Cell;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam::queue::SegQueue;
use pulse_simple::{ChannelCount, Playback, Sampleable};

use crate::{mp3, ProgressBar};
use mp3::Mp3Decoder;

use self::Action::*;

const BUFFER_SIZE: usize = 1000;
const DEFAULT_RATE: u32 = 44100;

#[derive(PartialEq, Clone, Debug)]
pub enum Action {
	Play(PathBuf),
	Pause,
	Stop,
}

pub struct Player {
	pub queue: Arc<SegQueue<Action>>,
	pub state: Arc<Mutex<Action>>,
	pub condition_variable: Arc<(Mutex<bool>, Condvar)>,
}

impl Player {
	pub(crate) fn new(progress_bar: Arc<Mutex<ProgressBar>>) -> Self {
		let state = Arc::new(Mutex::new(Action::Stop));
		let queue = Arc::new(SegQueue::new());
		let condition_variable = Arc::new((Mutex::new(false), Condvar::new()));

		let current_state = state.clone();
		{
			let queue = queue.clone();
			let condition_variable = condition_variable.clone();


			thread::spawn(move || {
				let block = || {
					let (ref lock, ref condition_variable) = *condition_variable;
					let mut started = lock.lock().unwrap();
					*started = false;
					while !*started {
						started = condition_variable.wait(started).unwrap();
					}
				};


				let mut buffer = [[0; 2]; BUFFER_SIZE];
				let mut playback: Playback<[i16; 2]> = Playback::new("MP3", "MP3 Playback", None, DEFAULT_RATE);
				let mut source = None;
				let mut written = false;
				let mut play = false;

				loop {
					if let Some(action) = queue.pop() {
						match action {
							Play(path) => {
								let file = File::open(path).unwrap();
								source = Some(Mp3Decoder::new(BufReader::new(file)).unwrap());
								let rate = source.as_ref().map(|source|
									source.samples_rate()).unwrap_or(DEFAULT_RATE);
								playback = Playback::new("MP3", "MP3 Playback", None, rate);
								play = true;
							}
							Pause => {
								play = !play;
							}
							Stop => {
								play = false;
								written = false;
							}
						}
					} else {
						if play {
							written = false;
							if let Some(ref mut source) = source {
								let size = Player::iter_to_buffer(source, &mut buffer);
								if size > 0 {
									playback.write(&buffer[..size]);
									written = true;
									(*progress_bar.lock().unwrap()).current_time = source.current_time();
								} else {
									play = false;
									*current_state.lock().unwrap() = Action::Stop;
								}
							}
						} else {
							if !written {
								play = false;
								source = None;
								(*progress_bar.lock().unwrap()).current_time = 0;
							}
							block();
						}
					}
				}
			});
		}
		Player {
			queue,
			state,
			condition_variable,
		}
	}

	fn iter_to_buffer<I: Iterator<Item=i16>>(iter: &mut I, buffer: &mut [[i16; 2]; BUFFER_SIZE]) -> usize {
		let mut iter = iter.take(BUFFER_SIZE);
		let mut index = 0;
		while let Some(sample1) = iter.next() {
			if let Some(sample2) = iter.next() {
				buffer[index][0] = sample1;
				buffer[index][1] = sample2;
			}
			index += 1;
		}
		index
	}

	pub fn compute_duration<P>(path: P) -> Option<Duration>
		where P: AsRef<Path> {
		let file = File::open(path).unwrap();
		Mp3Decoder::compute_duration(BufReader::new(file))
	}
}

