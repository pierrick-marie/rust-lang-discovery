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
This file comes from the project tokio-rs/mini-redis (Licence MIT) : https://raw.githubusercontent.com/tokio-rs/mini-redis/master/src/connection.rs
I modified some parts of it.
*/

use std::borrow::Borrow;
use crate::message::*;

use bytes::{Buf, BytesMut};
use std::io::{self, Cursor};
use std::time::Duration;
use futures::task::SpawnExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, AsyncBufRead, BufStream, BufReader, BufWriter, ReadHalf, WriteHalf};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use crate::{Client, ftp_error};

use crate::ftp_error::*;
use crate::message::Message::Response;
use crate::protocol_codes::ServerResponse;

/// Send and receive `Message` values from a remote peer.
///
/// When sending message, the frame is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
///
#[derive(Debug)]
pub struct Connection {
	listener: TcpListener,
}

impl Connection {
	/// Create a new `Connection`, backed by `socket`. Read and write buffers are initialized.
	pub fn new(socket: TcpListener) -> Connection {
		Connection {
			listener: socket
		}
	}
	
	pub async fn run(&mut self) {
		loop {
			match self.accept().await {
				Ok(stream) => {
					Client::new(stream).run().await;
				}
				Err(e) => {
					eprintln!("Failed to accept new connection {:?}", e);
				}
			}
		}
	}
	
	/// Accept an inbound connection.
	///
	/// Errors are handled by backing off and retrying. An exponential backoff
	/// strategy is used. After the first failure, the task waits for 1 second.
	/// After the second failure, the task waits for 2 seconds. Each subsequent
	/// failure doubles the wait time. If accepting fails on the 6th try after
	/// waiting for 64 seconds, then this function returns with an error.
	async fn accept(&mut self) -> ftp_error::Result<TcpStream> {
		let mut backoff = 1;
		
		// Try to accept a few times
		loop {
			// Perform the accept operation. If a socket is successfully
			// accepted, return it. Otherwise, save the error.
			match self.listener.accept().await {
				Ok((socket, _)) => {
					println!("New connection established");
					return Ok(socket);
				}
				Err(err) => {
					if backoff > 64 {
						// Accept has failed too many times. Return the error.
						return Err(err.into());
					}
				}
			}
			
			// Pause execution until the back off period elapses.
			time::sleep(Duration::from_secs(backoff)).await;
			
			// Double the back off
			backoff *= 2;
		}
	}
}
