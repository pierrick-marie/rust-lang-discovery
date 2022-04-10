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
use std::io::ErrorKind;

use tokio::io;

use self::FTP_Error::*;

pub enum FTP_Error {
	/// Input / Output error
	Io(ErrorKind),
	/// Incomplete or malformed message
	Msg(String),
	/// UTF_8 error
	Utf8(Utf8Error),
	/// Unknown
	Unknown(String),
}

pub type Result<T> = result::Result<T, FTP_Error>;

impl Display for FTP_Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			FTP_Error::Io(error) => {write!(f, "IO error {:?}", error)}
			FTP_Error::Msg(error) => {write!(f, "Msg error {}", error)}
			FTP_Error::Utf8(error) => {write!(f, "UTF_8 error {:?}", error)}
			FTP_Error::Unknown(error) => {write!(f, "Unknown error {:?}", error)}
		}
	}
}

impl error::Error for FTP_Error { }

impl Debug for FTP_Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl From<io::Error> for FTP_Error {
	fn from(error: io::Error) -> Self {
		format!("Input / Output error : {:?}", error).into()
	}
}

impl<'a> From<&'a str> for FTP_Error {
	fn from(error: &'a str) -> Self {
		error.to_string().into()
	}
}

impl From<Utf8Error> for FTP_Error {
	fn from(error: Utf8Error) -> Self {
		format!("UTF_8 error : {:?}", error).into()
	}
}

impl From<String> for FTP_Error {
	fn from(error: String) -> Self {
		error.into()
	}
}