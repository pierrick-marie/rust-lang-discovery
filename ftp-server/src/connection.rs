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

use bytes::{BytesMut};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{error, info, warn};
use std::time::Duration;
use async_std::io as async_io;

use crate::ftp_error::{FtpError, FtpResult};

pub struct Connection {
	buf: BytesMut,
	stream: TcpStream,
}

impl Connection {
	pub fn new(stream: TcpStream) -> Self {
		Connection {
			buf: BytesMut::new(),
			stream,
		}
	}
	
	pub async fn read(&mut self) -> FtpResult<String> {
		self.buf.clear();
		match async_io::timeout(Duration::from_secs(50), async {
			self.stream.read_buf(&mut self.buf).await
		}).await {
			Ok(n) => {
				if n > 0 {
					if let Ok(msg) = String::from_utf8(self.buf.to_vec()) {
						let message = msg.trim();
						info!(" <<<< {}", message);
						return Ok(message.to_string());
					}
				}
				return Err(FtpError::Utf8("connection::read".to_string(), format!("UTF_8 error, buffer={:?}", self.buf)));
			}
			Err(e) => {
				return Err(FtpError::SocketReadError("connection::read".to_string(), e.to_string()));
			}
		}
	}
	
	pub async fn write(&mut self, msg: String) -> FtpResult<String> {
		let mut attempt = 2;
		loop {
			match async_io::timeout(Duration::from_secs(5), async {
				self.stream.write_all(msg.as_bytes()).await
			}).await {
				Ok(_) => {
					info!(" >>>> {}", msg);
					return Ok(msg);
				}
				Err(e) => {
					if attempt > 0 {
						attempt -= 1;
						warn!("Failed to send message: {}", msg);
					} else {
						return Err(FtpError::SocketWriteError("connection::write".to_string(), e.to_string()));
					}
				}
			}
		}
	}
	
	pub async fn close(&mut self) {
		if let Err(e) = self.stream.shutdown().await {
			error!("Failed to shutdown connection {:?}", e);
		}
		info!("Connection closed");
	}
}