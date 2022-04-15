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
use tokio::net::{TcpListener, TcpStream};
use crate::{protocol_codes, utils};
use crate::connection::Connection;
use crate::ftp_error::{FtpError, FtpResult};
use portpicker::pick_unused_port;
use crate::data_connection::DataConnection;

use users::{get_user_by_name, User};
use users::os::unix::UserExt;
use crate::client::Transfert_Mod::{Active, Passive};

#[derive(PartialEq)]
enum Transfert_Mod {
	Passive,
	Active,
}

pub struct Client {
	ctrl_connection: Connection,
	data_connection: DataConnection,
	transfert_mode: Transfert_Mod,
	user: Option<User>,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Client {
			ctrl_connection: connection,
			data_connection: DataConnection::new(),
			transfert_mode: Active,
			user: None,
		}
	}
	
	pub async fn run(&mut self) -> std::io::Result<()> {
		if let Err(e) = self.ctrl_connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
			return Err(Error::new(ErrorKind::NotConnected, e.to_string()));
		}
		
		self.connect().await;
		info!("Connected {}", self.user.as_ref().unwrap().name().to_str().unwrap());
		
		self.command().await;
		
		self.close_connection().await;
		Ok(())
	}
	
	async fn command(&mut self) {
		debug!("client::command");
		loop {
			let msg = self.ctrl_connection.read().await.unwrap_or(FtpError::SocketReadError.to_string());
			dbg!(msg.clone());
			match self.parse_command(msg.clone()) {
				ClientCommand::Auth => { unimplemented!() }
				ClientCommand::Cwd(arg) => { unimplemented!() }
				ClientCommand::List(arg) => {
					self.transmitData(utils::get_ls(self.user.as_ref().unwrap().home_dir())).await;
				}
				ClientCommand::Mkd(arg) => { unimplemented!() }
				ClientCommand::NoOp => { unimplemented!() }
				ClientCommand::Port(arg) => { unimplemented!() }
				ClientCommand::Pwd => { unimplemented!() }
				ClientCommand::Pasv => {
					self.transfert_mode = Passive;
				}
				ClientCommand::Quit => {
					if let Err(e) = self.ctrl_connection.write(ServerResponse::ServiceClosingControlConnection.to_string()).await {
						error!("{:?}", e);
					}
					break;
				}
				ClientCommand::Retr(arg) => { unimplemented!() }
				ClientCommand::Rmd(arg) => { unimplemented!() }
				ClientCommand::Stor(arg) => { unimplemented!() }
				ClientCommand::Syst => {
					if self.syst().await.is_err() {
						break;
					}
				}
				ClientCommand::Type(arg) => { unimplemented!() }
				ClientCommand::CdUp => { unimplemented!() }
				_ => {
					error!("Unknown command {}", msg);
					break;
				}
			}
		}
	}
	
	async fn transmitData(&mut self, data: Vec<String>) -> FtpResult<()> {
	
		if self.transfert_mode == Passive {
			let msg = self.data_connection.open_passive_connection().await?;
			let message = format!("{} {}\n\r", ServerResponse::EnteringPassiveMode.to_string(), msg);
			self.ctrl_connection.write(message).await?;
			
			self.data_connection.connection_ready().await?;
			self.data_connection.send(data);
			
			dbg!("Connection ready !!");
		}
		
		Ok(())
	}
	
	async fn syst(&mut self) -> FtpResult<()> {
		info!("SYST command");
		self.ctrl_connection.write(ServerResponse::SystemType.to_string()).await
	}
	
	async fn connect(&mut self) {
		loop {
			match self.user().await {
				Some(login) => {
					info!("Login: {}", login);
					if let Err(e) = self.ctrl_connection.write(ServerResponse::UserNameOkayNeedPassword.to_string()).await {
						error!("Not connected {:?}", e);
						break;
					}
					if self.password().await.is_some() {
						info!("Password: \"x\"");
						let user = get_user_by_name(login.as_str());
						if user.is_some() {
							self.user = user;
							if let Err(e) = self.ctrl_connection.write(ServerResponse::UserLoggedIn.to_string()).await {
								error!("Not connected {:?}", e);
							}
							break;
						}
					}
				}
				_ => {}
			}
			if let Err(e) = self.ctrl_connection.write(ServerResponse::NotLoggedIn.to_string()).await {
				error!("Not connected {:?}", e);
				break;
			}
		}
	}
	
	async fn user(&mut self) -> Option<String> {
		debug!("client::user");
		let msg = self.ctrl_connection.read().await?;
		return match self.parse_command(msg.clone()) {
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
		let msg = self.ctrl_connection.read().await?;
		return match self.parse_command(msg.clone()) {
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
	
	fn parse_command(&self, msg: String) -> ClientCommand {
		debug!("client::parse_command {}", msg);
		if let Some(re) = Regex::new(r"([[:upper:]]{3,4})( .+)*").ok() {
			if let Some(cap) = re.captures(msg.as_str()) {
				if let Some(cmd) = cap.get(1) {
					if let Some(args) = cap.get(2) {
						return ClientCommand::new(cmd.as_str(), args.as_str().to_string().trim());
					} else {
						return ClientCommand::new(cmd.as_str(), "");
					}
				}
			}
		}
		error!("failed to parse message: {}", msg);
		ClientCommand::Unknown(msg)
	}
	
	fn check_username(&self, username: &String) -> bool {
		let re = Regex::new(r"^([[:word:]]+)$").unwrap();
		re.captures(username.as_str()).is_some()
	}
	
	pub async fn close_connection(&mut self) {
		info!("Close client connection");
		if let Err(e) = self.ctrl_connection.write(ServerResponse::ConnectionClosed.to_string()).await {
			error!("Failed to close connection with client: {:?}", e);
		}
		self.ctrl_connection.close().await;
	}
}