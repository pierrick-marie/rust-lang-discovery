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

use std::net::{Shutdown, TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

use crate::protocol_codes::*;
use crate::client::*;

pub struct Server {
	listener: TcpListener,
}

impl Server {
	pub fn new(listener: TcpListener) -> Self {
		Server {
			listener
		}
	}
	
	pub fn run(&self) {
		println!("Waiting for client");
		for client in self.listener.incoming() {
			match client {
				Ok(stream) => {
					println!("New client comming");
					// thread::spawn(move || {
					// let client = Io::new(stream);
					// Client::new(client).run();
					// });
				}
				_ => {
					println!("A client tried to connect");
				}
			}
		}
	}
}
