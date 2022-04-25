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
	TimeOut, // Time out to read message from FTP client
	SocketWriteError, // Writ socket error
	Disconnected, // FTP client disconnected
	Utf8, // UTF_8 error during reading message
	FileSystemError,
	ParseMessage, // Error during parsing incoming message from FTP client
	UnknownCommand, // FTP client send an unknown command
	SocketReadError, // Read socket error
	NotLogged, // FTP client not logged
	InternalChannelError, // Error while using tokio channel
	DataConnectionError, // Error with data connection
}

pub type FtpResult<T> = result::Result<T, FtpError>;

impl Display for FtpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			FtpError::TimeOut => { write!(f, "!!Error!! Time out connection") }
			FtpError::NotLogged => { write!(f, "!!Error!! Client not logged") }
			FtpError::SocketWriteError => { write!(f, "!!Error!! Connection closed") }
			FtpError::Utf8 => { write!(f, "!!Error!! UTF_8 error") }
			FtpError::ParseMessage => { write!(f, "!!Error!! Wrong data, impossible to parse message") }
			FtpError::UnknownCommand => { write!(f, "!!Error!! Unknown command") }
			FtpError::Disconnected => { write!(f, "!!Error!! Disconnected") }
			FtpError::SocketReadError => { write!(f, "!!Error!! Client does not answer") }
			FtpError::InternalChannelError => { write!(f, "!!Error!! Internal channel error") }
			FtpError::DataConnectionError => { write!(f, "!!Error!! Data connection error") }
			FtpError::FileSystemError => { write!(f, "!!Error!! File system error") }
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