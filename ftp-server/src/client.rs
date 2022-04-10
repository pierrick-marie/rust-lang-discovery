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

use std::future::Future;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::protocol_codes::*;
use crate::connection::*;
use std::io::{Error, ErrorKind, Result};
use std::str::Utf8Error;
use bytes::{Buf, BytesMut};
use futures::future::err;
use regex::{Captures, Regex};

use std::time::Duration;
use async_std::io as async_io;
use log::{debug, info, warn};
use crate::{ftp_error};
use crate::ftp_error::FTP_Error::{Io, Msg};

pub struct Client {
	stream: TcpStream,
	buf: BytesMut,
}

impl Client {
	pub fn new(connection: TcpStream) -> Self {
		Client {
			stream: connection,
			buf: BytesMut::new(),
		}
	}
	
	async fn write(&mut self, response: ServerResponse) {
		println!(" -->> {}", response);
		match self.stream.write_all(response.to_string().as_bytes()).await {
			Ok(_) => {}
			Err(e) => { eprintln!("Failed to write message {:?}", e) }
		}
	}
	
	// async fn sync_read(&mut self) -> Result<()> {
	//
	//
	// 	if let Ok(n) =  {
	// 		if 0 < n {
	// 			return self.parse_command(buf);
	// 		}
	// 	}
	// 	// return Err(error::Error::Io(ErrorKind::ConnectionAborted));
	// }
	
	async fn read(&mut self) -> Result<()> {
		let mut attempt = 2;
		loop {
			info!("Waiting for client command, attempt: {}", attempt);
			match async_io::timeout(Duration::from_secs(5), async {
				self.stream.read_buf(&mut self.buf).await
			}).await {
				Ok(_) => { return Ok(()); }
				Err(e) => {
					if attempt > 0 {
						attempt -= 1;
					} else {
						eprintln!("Time out -> exit");
						return Err(e.into());
					}
				}
			}
		}
	}
	
	fn parse_command(&self) -> ftp_error::Result<ClientCommand> {
		let re = Regex::new(
			r"([[:upper:]]+) (.*)"
		).unwrap();
		match &String::from_utf8(self.buf.to_vec()) {
			Ok(command) => {
				println!(" <<-- {}", command);
				match re.captures(command) {
					Some(cap) => {
						// there are 2 patterns in the regex + the text itself
						if 3 == cap.len() {
							if let Some(cmd) = cap.get(1) {
								if let Some(args) = cap.get(2) {
									return self.get_command(cmd.as_str(), args.as_str());
								}
							}
						}
						return Err(ftp_error::FTP_Error::Io(ErrorKind::InvalidData));
					}
					None => return Err(ftp_error::FTP_Error::Io(ErrorKind::InvalidInput))
				}
			}
			Err(e) => return Err(ftp_error::FTP_Error::Utf8(e.utf8_error()))
		}
	}
	
	fn get_command(&self, command: &str, args: &str) -> ftp_error::Result<ClientCommand> {
		match command {
			USER => Ok(ClientCommand::User(args.to_string())),
			_ => Err(ftp_error::FTP_Error::Msg("Unknown command".to_string()))
		}
	}
	
	
	pub async fn run(&mut self) {
		println!("Lest's start a new client !");
		
		// let res = async_io::timeout(Duration::from_secs(10), async {
		// 	self.user().await
		// }).await;
		// match res {
		// 	Ok(_) => { println!("Connected") }
		// 	Err(e) => { eprintln!("Not connected {:?}", e) }
		// }
		
		if let Ok(_) = self.user().await {
			println!("Logged !");
		} else {
			eprintln!("Pas logged :(");
		}
	}
	
	async fn user(&mut self) -> ftp_error::Result<()> {
		let mut attempt = 3;
		
		loop {
			self.write(ServerResponse::ServiceReadyForNewUser).await;
			match self.read().await {
				Ok(_) => {
					match self.parse_command() {
						Ok(command) => {
							println!(" <<-- {}", command);
							match command {
								ClientCommand::User(ref args) => {
									println!("OK !!! {:?}", command);
									return Ok(());
								}
								_ => {
									if attempt > 0 {
										eprintln!("ask user again, atemp: {}", attempt);
										attempt -= 1;
									} else {
										eprintln!("not logged in, connection aborted");
										self.write(ServerResponse::NotLoggedIn).await;
										return Err(Io(ErrorKind::ConnectionAborted.into()));
									}
								}
							}
						}
						Err(e) => {
							if attempt > 0 {
								eprintln!("ask user again, atemp: {}", attempt);
								attempt -= 1;
							} else {
								eprintln!("{:?} not logged in, connection aborted", e);
								self.write(ServerResponse::NotLoggedIn).await;
								return Err(e);
							}
						}
					}
				}
				Err(e) => {
					eprintln!("Read Error");
					return Err(Io(ErrorKind::TimedOut.into()));
				}
			}
		}
	}

// fn user_cmd(&mut self, user_name: String) {
// 	let user = user_name;
// 	self.io.write(ServerCommand::new(ServerResponse::UserNameOkayNeedPassword, "Password required"));
// }
}