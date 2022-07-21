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
use crate::ftp_client::command::UserCommand::*;
use crate::protocol::DELE;

pub const HELP: &str = "help";
pub const LS: &str = "ls";
pub const PASS: &str = "pass";
pub const APPEND: &str = "append";
pub const BYE: &str = "bye";
pub const CD: &str = "cd";
pub const CDUP: &str = "cdup";
pub const DELETE: &str = "delete";

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UserCommand {
	Help,
	Ls(String),
	Pass,
	Append(Option<String>),
	Unknown(String),
	Bye,
	Cd(Option<String>),
	CdUp,
	Delete(Option<String>),
}

pub fn parse_user_command(msg: &String) -> UserCommand {
	let msg = msg.trim().to_string();
	debug!("command::parse_command '{}'", msg);
	if let Some(re) = Regex::new(r"^([[:word:]]+)( .+)*$").ok() {
		if let Some(cap) = re.captures(msg.as_str()) {
			if let Some(cmd) = cap.get(1) {
				if let Some(args) = cap.get(2) {
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
			LS => Ls(arg.to_string()),
			CD => Cd(Some(arg.to_string())),
			APPEND => Append(Some(arg.to_string())),
			DELETE => Delete(Some(arg.to_string())),
			_ => {
				Unknown(arg.to_string())
			}
		}
	}

	pub fn new_without_arg(input: &str) -> UserCommand {
		debug!("Command::new {}", &input);

		match input {
			HELP => Help,
			PASS => Pass,
			BYE => Bye,
			APPEND => Append(None),
			CD => Cd(None),
			CDUP => CdUp,
			DELETE => Delete(None),
			_ => {
				Unknown("".to_string())
			},
		}
	}
}

impl Display for UserCommand {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Unknown(arg) => write!(f, "Unknown {}", arg), // doesn't exist in the protocol
			Help => write!(f, "{}", HELP),
			Ls(arg) => write!(f, "{} {}", LS, arg),
			Pass => write!(f, "{}", PASS),
			Append(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", APPEND, args)
				} else {
					write!(f, "{} <empty>", APPEND)
				}
			},
			Bye => write!(f, "{}", BYE),
			Cd(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", CD, args)
				} else {
					write!(f, "{} <empty>", CD)
				}
			}
			CdUp => write!(f, "{}", CDUP),
			Delete(arg) => {
				return if let Some(args) = arg {
					write!(f, "{} {}", DELETE, args)
				} else {
					write!(f, "{} <empty>", DELETE)
				}
			}
		}
	}
}