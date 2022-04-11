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

use std::fmt::{Debug, Display, Formatter, write};
use std::result;
use std::str::Utf8Error;
use std::error;
use std::io::ErrorKind;

use tokio::io;
use crate::protocol_codes::ClientCommand;

pub enum FtpError {
	TimeOut(String, String),
	SocketWriteError(String, String),
	Disconnected(String, String),
	Utf8(String, String),
	ParseMessage(String, String),
	UnknownCommand(String, String),
	UnexpectedCommand(String, String, String),
	SocketReadError(String, String),
	NotLogged(String, String),
}

pub type FtpResult<T> = result::Result<T, FtpError>;

impl Display for FtpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			FtpError::TimeOut(from, msg) => { write!(f, "!!Error!! Time out connection ==> {} --> {}", from, msg) }
			FtpError::NotLogged(from, msg) => { write!(f, "!!Error!! Client not logged ==> {} --> {}", from, msg) }
			FtpError::SocketWriteError(from, msg) => { write!(f, "!!Error!! Connection closed ==> {} --> {}", from, msg) }
			FtpError::Utf8(from, msg) => { write!(f, "!!Error!! UTF_8 error ==> {} --> {}", from, msg) }
			FtpError::ParseMessage(from, msg) => { write!(f, "!!Error!! Wrong data, impossible to parse message ==> {} --> {}", from, msg) }
			FtpError::UnknownCommand(from, msg) => { write!(f, "!!Error!! Unknown command ==> {} --> {}", from, msg) }
			FtpError::Disconnected(from, msg) => { write!(f, "!!Error!! Disconnected ==> {} --> {}", from, msg) }
			FtpError::SocketReadError(from, msg) => { write!(f, "!!Error!! Client does not answer ==> {} --> {}", from, msg) }
			FtpError::UnexpectedCommand(from, expected, found) => { write!(f, "!!Error!! Unexpected command ==> {} --> expected {:?}, found {:?}", from, expected, found) }
		}
	}
}

impl error::Error for FtpError {}

impl Debug for FtpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl From<io::Error> for FtpError {
	fn from(error: io::Error) -> Self {
		format!("Input / Output error : {:?}", error).into()
	}
}

impl<'a> From<&'a str> for FtpError {
	fn from(error: &'a str) -> Self {
		error.to_string().into()
	}
}

impl From<Utf8Error> for FtpError {
	fn from(error: Utf8Error) -> Self {
		format!("UTF_8 error : {:?}", error).into()
	}
}

impl From<String> for FtpError {
	fn from(error: String) -> Self {
		error.into()
	}
}