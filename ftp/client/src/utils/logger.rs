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

use log::{Record, Metadata, SetLoggerError, Level};

struct SimpleLogger;

use crate::LEVEL;

impl log::Log for SimpleLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= LEVEL
	}
	
	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			if !record.target().eq("rustyline") {
				if record.level() == Level::Info {
					println!("{}", record.args());
				} else {
					println!("#{}: {}", record.level(), record.args());
				}
			}
		}
	}
	
	fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER)
		.map(|()| log::set_max_level(LEVEL.to_level_filter()))
}