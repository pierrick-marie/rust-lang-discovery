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

use std::io::{Error, ErrorKind};
use crate::protocol_codes::*;
use regex::{Regex};

use log::{debug, error, info, warn};
use crate::protocol_codes;
use crate::connection::Connection;

pub struct Client {
	connection: Connection,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Client {
			connection,
		}
	}
	
	pub async fn run(&mut self) -> std::io::Result<()> {
		match self.user().await {
			Some(name) => {
				info!("Logged: {}", name);
			}
			_ => {
				self.close_connection().await;
				return Err(Error::new(ErrorKind::NotConnected, "client::run ==> Failed to log in"));
			}
		}
		self.close_connection().await;
		Ok(())
	}
	
	async fn user(&mut self) -> Option<String> {
		debug!("client::user");
		if let Err(e) = self.connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
			error!("{:?}", e);
			return None;
		}
		let msg = self.connection.read().await?;
		return match self.parse_command(msg.clone())? {
			ClientCommand::User(ref args) => {
				if self.check_username(args) {
					info!("USER: {}", args);
					Some(args.clone())
				} else {
					error!("User name error: {}", args);
					None
				}
			}
			_ => {
				error!("Unexpected command: {}", msg);
				None
			}
		};
	}
	
	fn parse_command(&self, msg: String) -> Option<ClientCommand> {
		debug!("client::parse_command {}", msg);
		let re = Regex::new(r"([[:upper:]]{3,4})( .+)*").ok()?;
		let cap = re.captures(msg.as_str())?;
		return if let Some(cmd) = cap.get(1) {
			if let Some(args) = cap.get(2) {
				self.get_command(cmd.as_str(), args.as_str().to_string().trim())
			} else {
				self.get_command(cmd.as_str(), "")
			}
		} else {
			error!("failed to parse message: {}", msg);
			None
		};
	}
	
	fn get_command(&self, command: &str, args: &str) -> Option<ClientCommand> {
		debug!("client::get_command");
		match command {
			protocol_codes::USER => Some(ClientCommand::User(args.to_string())),
			protocol_codes::PWD => Some(ClientCommand::Pwd),
			_ => {
				error!("Command not found: {:?}", command);
				return None;
			}
		}
	}
	
	fn check_username(&self, username: &String) -> bool {
		let re = Regex::new(r"^([[:word:]]+)$").unwrap();
		re.captures(username.as_str()).is_some()
	}
	
	pub async fn close_connection(&mut self) {
		info!("Close client connection");
		if let Err(e) = self.connection.write(ServerResponse::ConnectionClosed.to_string()).await {
			error!("Failed to close connection with client: {:?}", e);
		}
		self.connection.close().await;
	}
}