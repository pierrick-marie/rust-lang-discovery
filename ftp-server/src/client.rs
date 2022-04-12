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

use crate::protocol_codes::*;
use regex::{Regex};

use log::{error, info, warn};
use crate::protocol_codes;
use crate::connection::Connection;
use crate::ftp_error::{FtpError, FtpResult};

pub struct Client {
	connection: Connection,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Client {
			connection,
		}
	}
	
	fn parse_command(&self, msg: String) -> FtpResult<ClientCommand> {
		let re = Regex::new(r"([[:upper:]]{3,4})( .+)*").unwrap();
		if let Some(cap) = re.captures(msg.as_str()) {
			if let Some(cmd) = cap.get(1) {
				if let Some(args) = cap.get(2) {
					return self.get_command(cmd.as_str(), args.as_str());
				} else {
					return self.get_command(cmd.as_str(), "");
				}
			}
		}
		return Err(FtpError::ParseMessage("client::parse_command".to_string(), format!("failed to parse message: {:?}", msg)));
	}
	
	fn get_command(&self, command: &str, args: &str) -> FtpResult<ClientCommand> {
		match command {
			protocol_codes::USER => Ok(ClientCommand::User(args.to_string())),
			protocol_codes::PWD => Ok(ClientCommand::Pwd),
			_ => Err(FtpError::UnknownCommand("client::get_command".to_string(), command.to_string()))
		}
	}
	
	pub async fn stop(&mut self) {
		info!("Stop client");
		if let Err(e) = self.connection.write(ServerResponse::ConnectionClosed.to_string()).await {
			error!("Failed to close connection with client: {:?}", e);
		}
		self.connection.close().await;
	}
	
	pub async fn run(&mut self) -> std::io::Result<()>{
		match self.user().await {
			Ok(_) => {
				info!("Logged !");
			}
			Err(e) => {
				error!("{}", e);
			}
		}
		
		Ok(())
	}
	
	async fn user(&mut self) -> FtpResult<()> {
		let mut attempt = 3;
		loop {
			if attempt <= 0 {
				self.connection.close().await;
				return Err(FtpError::NotLogged("client::user".to_string(), "3 attempts left".to_string()));
			}
			if let Err(e) = self.connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
				return Err(FtpError::Disconnected("client::user".to_string(), e.to_string()));
			}
			match self.connection.read().await {
				Ok(msg) => {
					match self.parse_command(msg) {
						Ok(command) => {
							match command {
								ClientCommand::User(ref args) => {
									info!("OK !!! {:?}", command);
									return Ok(());
								}
								_ => {
									warn!("Unexpected command: {}", command);
									attempt -= 1;
								}
							}
						}
						Err(e) => {
							attempt -= 1;
							warn!("{}", e);
						}
					}
				}
				Err(e) => {
					match e {
						FtpError::Utf8(_, _) => {
							attempt -= 1;
							warn!("{}", e);
						}
						_ => {
							self.connection.close().await;
							return Err(FtpError::Disconnected("client::user".to_string(), e.to_string()));
						}
					}
				}
			}
		}
	}
}