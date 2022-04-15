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

use std::net::IpAddr;
use bytes::{BytesMut};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{debug, error, info, warn};
use std::time::Duration;
use async_std::io as async_io;

use std::net::SocketAddr;

use crate::ftp_error::{FtpError, FtpResult};

const TIME_OUT: u64 = 120;

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
	
	pub fn addr(&self) -> SocketAddr {
		return self.stream.local_addr().unwrap();
	}
	
	pub async fn read(&mut self) -> Option<String> {
		debug!("connection::read");
		self.buf.clear();
		match async_io::timeout(Duration::from_secs(TIME_OUT), async {
			self.stream.read_buf(&mut self.buf).await
		}).await {
			Ok(n) => {
				if n > 0 {
					match String::from_utf8(self.buf.to_vec()) {
						Ok(msg) => {
							let message = msg.trim();
							info!(" <<<< {}", message);
							return Some(message.to_string());
						}
						Err(e) => {
							error!("UTF_8 error, {:?}", e);
						}
					}
				} else {
					error!("Client disconnected");
				}
			}
			Err(e) => {
				error!("{:?}", e);
			}
		}
		return None;
	}
	
	pub async fn write(&mut self, msg: String) -> FtpResult<()> {
		debug!("connection::write");
		match async_io::timeout(Duration::from_secs(TIME_OUT), async {
			self.stream.write_all(msg.as_bytes()).await
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
	
	pub async fn close(&mut self) {
		debug!("connection::close");
		if let Err(e) = self.stream.shutdown().await {
			error!("Failed to shutdown connection {:?}", e);
		}
		info!("Connection closed by server");
	}
}