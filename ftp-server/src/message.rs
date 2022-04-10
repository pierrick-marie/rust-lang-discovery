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
along with rust-discovery.  If not, see <http://www.gnu.org/licenses/>.
*/

/*
This file comes from the project tokio-rs/mini-redis (Licence MIT) : https://github.com/tokio-rs/mini-redis/blob/master/src/frame.rs
I modified some parts of it.
*/

//! Provides a type representing a FTP protocol message as well as utilities for
//! parsing message from a byte array.

use bytes::{Buf, Bytes};
use std::convert::TryInto;
use std::{fmt, result};
use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use crate::protocol_codes::*;
use crate::ftp_error::*;

/// A frame in the Redis protocol.
#[derive(Debug)]
pub enum Message {
	Command(ClientCommand),
	Response(ServerResponse),
	Null,
	Error(FTP_Error),
	Array(Vec<Message>),
}

impl Message {
	
	/// Converts the frame to an "unexpected frame" error
	pub fn to_error(&self) -> FTP_Error {
		format!("unexpected frame: {}", self).into()
	}
}

impl fmt::Display for Message {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		
		match self {
			Message::Error(msg) => write!(fmt, "error: {}", msg),
			Message::Null => "(nil)".fmt(fmt),
			Message::Array(parts) => {
				for part in parts {
					write!(fmt, "{}", part);
				}
				Ok(())
			}
			Message::Command(cc) => { write!(fmt, "{}", cc) }
			Message::Response(sr) => { write!(fmt, "{}", sr) }
		}
	}
}

