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
use log::{debug, error};
use regex::Regex;
use crate::ftp_client::command::Command::*;

pub const HELP: &str = "help";
pub const LS: &str = "ls";
pub const PASS: &str = "pass";

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Command {
	Help,
	Ls(String),
	Pass,
	Unknown(String),
}

pub fn parse_command(msg: &String) -> Command {
	let msg = msg.trim().to_string();
	debug!("command::parse_command '{}'", msg);
	if let Some(re) = Regex::new(r"^([[:word:]]+)( .+)*$").ok() {
		if let Some(cap) = re.captures(msg.as_str()) {
			if let Some(cmd) = cap.get(1) {
				if let Some(args) = cap.get(2) {
					return Command::new(cmd.as_str(), args.as_str().to_string().trim());
				} else {
					return Command::new(cmd.as_str(), "");
				}
			}
		}
	}
	error!("failed to parse command: {}", msg);
	Command::Unknown(msg.clone())
}

impl Command {
	pub fn new(input: &str, arg: &str) -> Command {
		debug!("Command::new {} {}", &input, &arg);
		
		match input {
			HELP => Help,
			LS => Ls(arg.to_string()),
			PASS => Pass,
			_ => {
				Unknown(arg.to_string())
			}
		}
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Unknown(arg) => write!(f, "Unknown {}", arg), // doesn't exist in the protocol
			Help => write!(f, "{}", HELP),
			Ls(arg) => write!(f, "{} {}", LS, arg),
			Pass => write!(f, "{}", PASS),
		}
	}
}