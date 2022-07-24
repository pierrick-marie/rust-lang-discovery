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

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{debug, error, info};
use std::time::Duration;
use async_std::io as async_io;

use tokio::net::tcp::{OwnedWriteHalf, OwnedReadHalf};
use crate::protocol::ServerResponse;

use crate::utils::error::{FtpError, FtpResult};

pub const TIME_OUT: u64 = 300;
const BUFFER_SIZE: usize = 1024;

pub struct Connection {
	buffer_reader: [u8; BUFFER_SIZE],
	rx: OwnedReadHalf,
	tx: OwnedWriteHalf,
}

impl Connection {
	pub fn new(rx: OwnedReadHalf, tx: OwnedWriteHalf) -> Self {
		Connection {
			buffer_reader: [0; BUFFER_SIZE],
			rx,
			tx,
		}
	}

	pub async fn read(&mut self) -> Option<String> {
		debug!("connection::read");

		let mut message: String = String::new();

		loop {
			match async_io::timeout(Duration::from_secs(TIME_OUT), async {
				self.buffer_reader = [0; BUFFER_SIZE];
				self.rx.read(&mut self.buffer_reader).await
			}).await {
				Ok(n) => {
					if n > 0 {
						match String::from_utf8(self.buffer_reader[..n].to_vec()) {
							Ok(msg) => {
								message.push_str(msg.trim());
								info!(" <<<< {}", message);
								if n < BUFFER_SIZE {
									return Some(message);
								}
							}
							Err(e) => {
								error!("UTF_8 error, {:?}", e);
								return None;
							}
						}
					} else {
						error!("Read: Client disconnected");
						return None;
					}
				}
				Err(e) => {
					error!("Read: time out {:?}", e);
					return None;
				}
			}
		}
	}

	pub async fn write(&mut self, mut msg: String) -> FtpResult<()> {
		debug!("connection::write");
		match async_io::timeout(Duration::from_secs(TIME_OUT), async {
			msg.push_str("\r\n");
			self.tx.write(msg.as_bytes()).await
		}).await {
			Ok(_) => {
				info!(" >>>> {}", msg);
				return Ok(());
			}
			Err(e) => {
				error!("Failed to send message: {}, {:?}", msg, e);
				return Err(FtpError::SocketWriteError);
			}
		}
	}

	pub async fn send(&mut self, response: ServerResponse, message: &str) -> FtpResult<()> {
		let message = format!("{} {}", response, message);
		self.write(message).await
	}

	pub async fn close(&mut self) {
		debug!("connection::close");

		if self.tx.shutdown().await.is_ok() {
			info!("Connection closed by server");
		} else {
			error!("Error while closing socket");
		}
	}
}