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
use std::io::{BufRead, BufReader, Read, Write};
use std::ptr::replace;
use std::str::from_boxed_utf8_unchecked;
use std::thread;

pub struct Server {
	listener: TcpListener,
}

impl Server {
	
	pub fn new(adress: &str, port: &str) -> Self {
		Server {
			listener: TcpListener::bind(format!("{}:{}", adress, port)).expect("Failed to listen localhost:8080"),
		}
	}
	
	fn write(mut stream: &TcpStream, msg: &str) -> bool {
		if let Err(_) = stream.write_all(format!("{}\n\r", msg).as_bytes()) {
			println!(" ==XX Failed to send {}", msg);
			return false;
		}
		println!(" ==>> {}", msg);
		true
	}
	
	fn read(mut stream: &TcpStream) -> String {
		let mut message = String::new();
		let mut reader = BufReader::new(stream);
		if let Ok(size) = reader.read_line(&mut message) {
			println!(" <<== {}", message);
			return message;
		}
		println!(" XX== Failed to read response");
		"".to_string()
	}
	
	fn new_client(mut stream: &TcpStream) {
		if !Server::write(stream, "Hello") { return }
		Server::read(stream);
		stream.shutdown(Shutdown::Both).expect("Failed to close connection");
	}
	
	pub fn run(&self) {
		
		println!("Waiting for client");
		for client in self.listener.incoming() {
			match client {
				Ok(stream) => {
					println!("New client comming");
					thread::spawn(move || {
						Server::new_client(&stream);
					});
				}
				_ => {
					println!("A client tried to connect");
				}
			}
		}
	}
}
