use std::borrow::Borrow;
use std::cell::Cell;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use crossbeam::queue::SegQueue;
use pulse_simple::{ChannelCount, Playback, Sampleable};

use crate::mp3;
use mp3::Mp3Decoder;

use self::Action::*;

const BUFFER_SIZE: usize = 1000;
const DEFAULT_RATE: u32 = 44100;

#[derive(PartialEq, Clone)]
pub enum Action {
	Play(PathBuf),
	Pause,
	Stop,
}

pub struct Player {
	pub queue: Arc<SegQueue<Action>>,
	pub state: Arc<Mutex<Action>>,
}

impl Player {
	pub(crate) fn new() -> Self {
		let state = Arc::new(Mutex::new(Action::Stop));
		let queue = Arc::new(SegQueue::new());
		{
			let queue = queue.clone();

			thread::spawn(move || {
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
								let size = iter_to_buffer(source, &mut buffer);
								if size > 0 {
									playback.write(&buffer[..size]);
									written = true;
								}
							}
						}
						if !written {
							play = false;
							source = None;
						}
					}
				}
			});
		}
		Player {
			state,
			queue,
		}
	}

	// pub fn state(&self) -> Action {
	// 	*self.state.lock().unwrap()
	// }
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

