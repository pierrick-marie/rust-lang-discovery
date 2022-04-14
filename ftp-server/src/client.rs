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

use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use crate::protocol_codes::*;
use regex::{Match, Regex};

use log::{debug, error, info, warn};
use crate::protocol_codes;
use crate::connection::Connection;
use crate::ftp_error::FtpResult;

struct User {
	login: String,
	// password: String, // Do not save the passwaord
}

impl Display for User {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "FTP User: {}", self.login)
	}
}

pub struct Client {
	connection: Connection,
	user: Option<User>,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Client {
			connection,
			user: None,
		}
	}
	
	pub async fn run(&mut self) -> std::io::Result<()> {
		
		if let Err(e) = self.connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
			return Err(Error::new(ErrorKind::NotConnected, e.to_string()));
		}
		
		if let Some(user) = self.connect().await {
			self.user = Some(user);
			if let Err(e) = self.connection.write(ServerResponse::UserLoggedIn.to_string()).await {
				return Err(Error::new(ErrorKind::NotConnected, e.to_string()));
			}
			info!("Connected {}", self.user.as_ref().unwrap());
		}
		
		self.close_connection().await;
		Ok(())
	}
	
	async fn connect(&mut self) -> Option<User> {
		loop {
			match self.user().await {
				Some(login) => {
					info!("Login: {}", login);
					if let Err(e) = self.connection.write(ServerResponse::UserNameOkayNeedPassword.to_string()).await {
						error!("Not connected {:?}", e);
						return None;
					}
					match self.password().await {
						Some(pass) => {
							info!("Password: {}", pass);
							return Some(User {
								login,
							});
						}
						_ => {
							if let Err(e) = self.connection.write(ServerResponse::NotLoggedIn.to_string()).await {
								error!("Not connected {:?}", e);
								return None;
							}
						}
					}
				}
				_ => {
					if let Err(e) = self.connection.write(ServerResponse::NotLoggedIn.to_string()).await {
						error!("Not connected {:?}", e);
						return None;
					}
				}
			}
		}
	}
	
	async fn user(&mut self) -> Option<String> {
		debug!("client::user");
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
	
	async fn password(&mut self) -> Option<String> {
		debug!("client::password");
		let msg = self.connection.read().await?;
		return match self.parse_command(msg.clone())? {
			ClientCommand::Pass(ref args) => {
				if self.check_username(args) {
					info!("PASSWORD xxx");
					Some(args.clone())
				} else {
					error!("Password error: {}", args);
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
				Some(ClientCommand::new(cmd.as_str(), args.as_str().to_string().trim()))
			} else {
				Some(ClientCommand::new(cmd.as_str(), ""))
			}
		} else {
			error!("failed to parse message: {}", msg);
			None
		};
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