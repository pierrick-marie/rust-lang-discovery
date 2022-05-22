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

use std::fmt::{Debug, Display, Formatter};
use std::result;
use std::str::Utf8Error;
use std::error;
use log::debug;
use tokio::io;

pub enum FtpError {
	FileSystemError(String),
	ConnectionError(String),
	Abord(String), // Stop current task
	UserConnectionError(String), // Failed to connect to server
	InternalError(String),
}

pub type FtpResult<T> = result::Result<T, FtpError>;

impl Display for FtpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			FtpError::FileSystemError(msg) => { write!(f, "!!Error!! File system error: {}", msg) }
			FtpError::Abord(msg) => { write!(f, "!!Error!! Stop current data transfer: {}", msg) }
			FtpError::InternalError(msg) => { write!(f, "!!Error!! Internal error: {}", msg) }
			FtpError::UserConnectionError(msg) => { write!(f, "!!Error!! Failed to connect to server: {}", msg) }
			FtpError::ConnectionError(msg) => { write!(f, "!!Error!! Connection error: {}", msg) }
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