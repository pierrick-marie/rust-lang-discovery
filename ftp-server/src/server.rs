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
use regex::Regex;

extern crate num;

#[derive(FromPrimitive)]
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
enum FTPCode {
	RestartMarkerReply = 110,
	ServiceReadInXXXMinutes = 120,
	DataConnectionAlreadyOpen = 125,
	FileStatusOk = 150,
	Ok = 200,
	CommandNotImplementedSuperfluousAtThisSite = 202,
	SystemStatus = 211,
	DirectoryStatus = 212,
	FileStatus = 213,
	HelpMessage = 214,
	SystemType = 215,
	ServiceReadyForNewUser = 220,
	ServiceClosingControlConnection = 221,
	DataConnectionOpen = 225,
	ClosingDataConnection = 226,
	EnteringPassiveMode = 227,
	UserLoggedIn = 230,
	RequestedFileActionOkay = 250,
	PATHNAMECreated = 257,
	UserNameOkayNeedPassword = 331,
	NeedAccountForLogin = 332,
	RequestedFileActionPendingFurtherInformation = 350,
	ServiceNotAvailable = 421,
	CantOpenDataConnection = 425,
	ConnectionClosed = 426,
	FileBusy = 450,
	LocalErrorInProcessing = 451,
	InsufficientStorageSpace = 452,
	UnknownCommand = 500,
	InvalidParameterOrArgument = 501,
	CommandNotImplemented = 502,
	BadSequenceOfCommands = 503,
	CommandNotImplementedForThatParameter = 504,
	NotLoggedIn = 530,
	NeedAccountForStoringFiles = 532,
	FileNotFound = 550,
	PageTypeUnknown = 551,
	ExceededStorageAllocation = 552,
	FileNameNotAllowed = 553,
}

pub struct Server {
	listener: TcpListener,
}

struct Command {
	code: i32,
	args: String,
}

impl Command {

	fn from_string(command: &str) -> Self {
		let re = Regex::new(r"(\d{3}) (.+)").unwrap();
		let mut code = -1; // Error code
		let mut args = "";
		if let Some(cap) = re.captures(command) {
			// there are two patterns in the regex + the text itself
			if 3 == cap.len() {
				code = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
				args = cap.get(2).unwrap().as_str();
			} else {
				println!("Impossible to read the code");
			}
		} else {
			println!("Impossible to read the command");
		}
		Command {
			code,
			args: args.to_string(),
		}
	}
	
	fn exec(&self) {
		let element = num::FromPrimitive::from_i32(self.code);
		match element {
			Some(FTPCode::Ok) => {
				println!("OK command with args: {}", self.args)
			},
			Some(FTPCode::UnknownCommand) => {
				println!("Unknown command with args: {}", self.args)
			},
			_ => {
				println!("Empty command")
			},
		}
	}
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
	
	fn read(stream: &TcpStream) -> String {
		let mut message = String::new();
		let mut reader = BufReader::new(stream);
		if let Ok(_) = reader.read_line(&mut message) {
			println!(" <<== {}", message);
			return message;
		}
		println!(" XX== Failed to read response");
		String::new()
	}
	
	fn new_client(stream: &TcpStream) {
		if !Server::write(stream, "Hello") { return; }
		Command::from_string(&Server::read(stream)).exec();
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
