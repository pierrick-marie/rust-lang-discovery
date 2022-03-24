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

use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;
use simplemad;

pub struct Mp3Decoder<R> where R: Read {
	reader: simplemad::Decoder<R>,
	current_frame: simplemad::Frame,
	current_frame_channel: usize,
	current_frame_sample_pos: usize,
	current_time: u64,
}

pub fn to_millis(duration: Duration) -> u64 {
	duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
}

fn is_mp3<R>(mut data: R) -> bool
	where R: Read + Seek {
	let stream_pos = data.seek(SeekFrom::Current(0)).unwrap();
	let is_mp3 = simplemad::Decoder::decode(data.by_ref()).is_ok();
	data.seek(SeekFrom::Start(stream_pos)).unwrap();
	is_mp3
}

fn next_frame<R: Read>(decoder: &mut simplemad::Decoder<R>) -> simplemad::Frame {
	decoder.filter_map(|f| f.ok()).next()
		.unwrap_or_else(|| {
			simplemad::Frame {
				bit_rate: 128,
				layer: Default::default(),
				mode: Default::default(),
				sample_rate: 44100,
				samples: vec![Vec::new()],
				position: Duration::from_secs(0),
				duration: Duration::from_secs(0),
			}
		})
}


impl<R> Mp3Decoder<R> where R: Read + Seek {
	pub fn new(mut data: R) -> Result<Mp3Decoder<R>, R> {
		if !is_mp3(data.by_ref()) {
			return Err(data);
		}

		let mut reader = simplemad::Decoder::decode(data).unwrap();
		let current_frame = next_frame(&mut reader);
		let current_time = to_millis(current_frame.duration);
		Ok(Mp3Decoder {
			reader,
			current_frame,
			current_frame_channel: 0,
			current_frame_sample_pos: 0,
			current_time,
		})
	}

	pub fn current_time(&self) -> u64 {
		self.current_time
	}
	pub fn samples_rate(&self) -> u32 {
		self.current_frame.sample_rate
	}


	pub fn compute_duration(mut data: R) -> Option<Duration> {
		if !is_mp3(data.by_ref()) {
			return None;
		}
		let decoder = simplemad::Decoder::decode_headers(data).unwrap();
		Some(decoder.filter_map(|frame| {
			match frame {
				Ok(frame) => Some(frame.duration),
				Err(_) => None,
			}
		})
			.sum())
	}
}

fn next_sample<R: Read>(decoder: &mut Mp3Decoder<R>) -> Option<i16> {
	if decoder.current_frame.samples[0].len() == 0 {
		return None;
	}

	// getting the sample and converting it from fixed step to i16
	let sample = decoder.current_frame.samples[decoder.current_frame_channel][decoder.current_frame_sample_pos];
	let sample = sample.to_i16();

	decoder.current_frame_channel += 1;

	if decoder.current_frame_channel < decoder.current_frame.samples.len() {
		return Some(sample);
	}

	decoder.current_frame_channel = 0;
	decoder.current_frame_sample_pos += 1;

	if decoder.current_frame_sample_pos < decoder.current_frame.samples[0].len() {
		return Some(sample);
	}

	decoder.current_frame = next_frame(&mut decoder.reader);
	decoder.current_frame_channel = 0;
	decoder.current_frame_sample_pos = 0;
	decoder.current_time += to_millis(decoder.current_frame.duration);

	return Some(sample);
}

impl<R> Iterator for Mp3Decoder<R> where R: Read {

	type Item = i16;

	fn next(&mut self) -> Option<i16> {
		next_sample(self)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.current_frame.samples[0].len(), None)
	}
}
