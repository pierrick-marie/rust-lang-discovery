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

pub enum Action {
	Load(PathBuf),
	Stop,
}

pub struct Player {
	pub app_state: Arc<Mutex<super::State>>,
	pub event_loop: EventLoop,
}

impl Player {
	pub(crate) fn new(app_state: Arc<Mutex<super::State>>) -> Self {
		let event_loop = EventLoop::new();
		{
			let app_state = app_state.clone();
			let event_loop = event_loop.clone();
			thread::spawn(move || {
				let mut buffer = [[0; 2]; BUFFER_SIZE];

				let mut playback: Playback<[i16; 2]> = Playback::new("MP3", "MP3 Playback", None, DEFAULT_RATE);
				let mut source = None;
				loop {
					if let Some(action) = event_loop.queue.pop() {
						match action {
							Load(path) => {
								let file = File::open(path).unwrap();
								source = Some(Mp3Decoder::new(BufReader::new(file)).unwrap());
								let rate = source.as_ref().map(|source|
									source.samples_rate()).unwrap_or(DEFAULT_RATE);
								playback = Playback::new("MP3", "MP3 Playback", None, rate);
								app_state.lock().unwrap().stopped = false;
								*event_loop.playing.lock().unwrap() = true;
							}
							Stop => {}
						}
					} else if *event_loop.playing.lock().unwrap() {
						let mut written = false;
						if let Some(ref mut source) = source {
							let size = iter_to_buffer(source, &mut buffer);
							if size > 0 {
								playback.write(&buffer[..size]);
								written = true;
							}
						}
						if !written {
							app_state.lock().unwrap().stopped = true;
							*event_loop.playing.lock().unwrap() = false;
							source = None;
						}
					}
				}
			});
		}
		Player {
			app_state,
			event_loop,
		}
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

#[derive(Clone)]
pub struct EventLoop {
	pub queue: Arc<SegQueue<Action>>,
	playing: Arc<Mutex<bool>>,
}

impl EventLoop {
	fn new() -> Self {
		EventLoop {
			queue: Arc::new(SegQueue::new()),
			playing: Arc::new(Mutex::new(false)),
		}
	}
}


