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

use std::fmt::{Display, Formatter};
use std::net::Shutdown::Write;
use std::path::PathBuf;
use log::{debug, error};
use regex::Regex;
use crate::protocol::ClientCommand::Rmd;
use crate::user::command::UserCommand::*;
use crate::protocol::DELE;
use crate::protocol::TransferType::Binary;

pub const HELP: &str = "help";
pub const QUESTION: &str = "?";
pub const LS: &str = "ls";
pub const PASS: &str = "pass";
pub const APPEND: &str = "append";
pub const BYE: &str = "bye";
pub const CD: &str = "cd";
pub const CDUP: &str = "cdup";
pub const DELETE: &str = "delete";
pub const DIR: &str = "dir";
pub const EXIT: &str = "exit";
pub const GET: &str = "get";
pub const ASCII: &str = "ascii";
pub const IMAGE: &str = "binary";
pub const LCD: &str = "lcd";
pub const NLIST: &str = "nlist";
pub const PUT: &str = "put";
pub const PWD: &str = "pwd";
pub const QUIT: &str = "quit";
pub const RECV: &str = "recv";
pub const RENAME: &str = "rename";
pub const RMDIR: &str = "rmdir";
pub const SEND: &str = "send";
pub const SYSTEM: &str = "system";

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UserCommand {
	Help(Option<String>),
	Ls(Option<String>),
	Pass,
	Append(Option<String>),
	Unknown(String),
	Bye,
	Cd(Option<String>),
	CdUp,
	Delete(Option<String>),
	Dir,
	Exit,
	Get(Option<String>),
	Ascii,
	Image,
	Lcd,
	Nlist(Option<String>),
	Put(Option<String>),
	Pwd,
	Quit,
	Recv(Option<String>),
	Rename(Option<String>),
	Rmdir(Option<String>),
	Send(Option<String>),
	System,
}

pub fn parse_user_command(msg: &String) -> UserCommand {
	let msg = msg.trim().to_string();
	debug!("command::parse_command '{}'", msg);
	if let Some(re) = Regex::new(r"^(([[:word:]]+)|\?)( .+)*$").ok() {
		if let Some(cap) = re.captures(msg.as_str()) {
			dbg!(&cap);
			if let Some(cmd) = cap.get(1) {
				if let Some(args) = cap.get(cap.len() - 1) {
					return UserCommand::new_with_args(cmd.as_str(), args.as_str().to_string().trim());
				} else {
					return UserCommand::new_without_arg(cmd.as_str());
				}
			}
		}
	}
	error!("failed to parse command: {}", msg);
	UserCommand::Unknown(msg.clone())
}

impl UserCommand {
	pub fn new_with_args(input: &str, arg: &str) -> UserCommand {
		debug!("Command::new {} {}", &input, &arg);

		match input {
			LS => Ls(Some(arg.to_string())),
			HELP => Help(Some(arg.to_string())),
			QUESTION => Help(Some(arg.to_string())),
			CD => Cd(Some(arg.to_string())),
			APPEND => Append(Some(arg.to_string())),
			DELETE => Delete(Some(arg.to_string())),
			GET => Get(Some(arg.to_string())),
			NLIST => Nlist(Some(arg.to_string())),
			PUT => Put(Some(arg.to_string())),
			RECV => Recv(Some(arg.to_string())),
			RENAME => Rename(Some(arg.to_string())),
			RMDIR => Rmdir(Some(arg.to_string())),
			SEND => Send(Some(arg.to_string())),
			_ => {
				Unknown(arg.to_string())
			}
		}
	}

	pub fn new_without_arg(input: &str) -> UserCommand {
		debug!("Command::new {}", &input);

		match input {
			HELP => Help(None),
			QUESTION => Help(None),
			PASS => Pass,
			BYE => Bye,
			LS => Ls(None),
			APPEND => Append(None),
			CD => Cd(None),
			CDUP => CdUp,
			DELETE => Delete(None),
			DIR => Dir,
			EXIT => Exit,
			GET => Get(None),
			ASCII => Ascii,
			IMAGE => Image,
			LCD => Lcd,
			NLIST => Nlist(None),
			PUT => Put(None),
			PWD => Pwd,
			QUIT => Quit,
			RECV => Recv(None),
			RENAME => Rename(None),
			RMDIR => Rmdir(None),
			SEND => Send(None),
			SYSTEM => System,
			_ => {
				Unknown("".to_string())
			}
		}
	}
}

impl Display for UserCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Unknown(arg) => write!(f, "Unknown {}", arg), // doesn't exist in the protocol
			Help(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", HELP, args)
				} else {
					write!(f, "{} <empty>", HELP)
				};
			}
			Ls(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", LS, args)
				} else {
					write!(f, "{} <empty>", LS)
				};
			}
			Pass => write!(f, "{}", PASS),
			Append(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", APPEND, args)
				} else {
					write!(f, "{} <empty>", APPEND)
				};
			}
			Bye => write!(f, "{}", BYE),
			Cd(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", CD, args)
				} else {
					write!(f, "{} <empty>", CD)
				};
			}
			CdUp => write!(f, "{}", CDUP),
			Delete(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", DELETE, args)
				} else {
					write!(f, "{} <empty>", DELETE)
				};
			}
			Dir => write!(f, "{}", DIR),
			Exit => write!(f, "{}", EXIT),
			Get(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", GET, args)
				} else {
					write!(f, "{} <empty>", GET)
				};
			}
			Ascii => write!(f, "{}", ASCII),
			Image => write!(f, "{}", Image),
			Lcd => write!(f, "{}", Lcd),
			Nlist(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", NLIST, args)
				} else {
					write!(f, "{} <empty>", NLIST)
				};
			}
			Put(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", PUT, args)
				} else {
					write!(f, "{} <empty>", PUT)
				};
			}
			Pwd => write!(f, "{}", PWD),
			Quit => write!(f, "{}", QUIT),
			Recv(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", RECV, args)
				} else {
					write!(f, "{} <empty>", RECV)
				};
			}
			Rename(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", RENAME, args)
				} else {
					write!(f, "{} <empty>", RENAME)
				};
			}
			Rmdir(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", RMDIR, args)
				} else {
					write!(f, "{} <empty>", RMDIR)
				};
			}
			Send(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", SEND, args)
				} else {
					write!(f, "{} <empty>", SEND)
				};
			}
			System => write!(f, "{}", SYSTEM),
		}
	}
}