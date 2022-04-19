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
use std::net::{IpAddr, SocketAddr};
use crate::protocol_codes::*;
use regex::{Regex};

use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use crate::{ADDR, utils};
use crate::connection::Connection;
use crate::ftp_error::{FtpError, FtpResult};
use portpicker::pick_unused_port;

use users::{get_user_by_name, User};
use users::os::unix::UserExt;
use crate::client::TransfertMod::{Active, Passive};

#[derive(PartialEq)]
enum TransfertMod {
	Passive,
	Active,
}

pub struct Client {
	ctrl_connection: Connection,
	data_connection: Option<Connection>,
	transfert_mode: TransfertMod,
	transfert_type: TransferType,
	user: Option<User>,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Client {
			ctrl_connection: connection,
			data_connection: None,
			transfert_mode: Active,
			transfert_type: TransferType::Ascii,
			user: None,
		}
	}
	
	pub async fn run(&mut self) -> std::io::Result<()> {
		if let Err(e) = self.ctrl_connection.write(ServerResponse::ServiceReadyForNewUser.to_string()).await {
			return Err(Error::new(ErrorKind::NotConnected, e.to_string()));
		}
		
		if self.connect().await {
			info!("Connected {}", self.user.as_ref().unwrap().name().to_str().unwrap());
			if let Err(e) = self.command().await {
				error!("{}", e);
			}
		} else {
			error!("Not connected");
		}
		
		self.close_connection().await;
		Ok(())
	}
	
	async fn command(&mut self) -> FtpResult<()> {
		debug!("client::command");
		let mut msg = self.ctrl_connection.read().await;
		while msg.is_some() {
			match self.parse_command(msg.as_ref().unwrap().clone()) {
				ClientCommand::Auth => { unimplemented!() }
				ClientCommand::Cwd(arg) => { unimplemented!() }
				ClientCommand::List(arg) => {
					self.list().await?;
				}
				ClientCommand::Mkd(arg) => { unimplemented!() }
				ClientCommand::NoOp => { unimplemented!() }
				ClientCommand::Port(arg) => {
					self.port(arg).await?;
				}
				ClientCommand::Pwd => { unimplemented!() }
				ClientCommand::Pasv => {
					self.pasv().await?;
				}
				ClientCommand::Quit => {
					self.ctrl_connection.write(ServerResponse::ServiceClosingControlConnection.to_string()).await?;
					self.user = None;
					return Ok(());
				}
				ClientCommand::Retr(arg) => { unimplemented!() }
				ClientCommand::Rmd(arg) => { unimplemented!() }
				ClientCommand::Stor(arg) => { unimplemented!() }
				ClientCommand::Syst => {
					self.syst().await?;
				}
				ClientCommand::Type(arg) => {
					self.transfert_type = arg;
					let message = format!("{} {} {}", ServerResponse::OK.to_string(), "Swith to ", arg.to_string());
					self.ctrl_connection.write(message).await?;
				}
				ClientCommand::CdUp => { unimplemented!() }
				_ => {
					error!("Unknown command {}", msg.unwrap());
					return Err(FtpError::UnknownCommand);
				}
			}
			msg = self.ctrl_connection.read().await;
		}
		Ok(())
	}
	
	async fn port(&mut self, arg: String) -> FtpResult<()> {
		if let Some(addr) = Client::parse_port(arg) {
			let socket = TcpStream::connect(SocketAddr::new(addr.0, addr.1)).await?;
			let (rx, tx) = socket.into_split();
			self.data_connection = Some(Connection::new(rx, tx));
			
			let message = format!("{} PORT command successful. Consider using PASV", ServerResponse::OK.to_string());
			self.ctrl_connection.write(message).await?;
			Ok(())
		} else {
			Err(FtpError::DataConnectionError)
		}
	}
	
	async fn pasv(&mut self) -> FtpResult<()> {
		if self.transfert_mode == Passive {
			self.transfert_mode = Active;
		} else {
			self.transfert_mode = Passive;
			
			let port: u16 = pick_unused_port().expect("No ports free");
			let listener = TcpListener::bind(format!("{}:{}", ADDR, port)).await?;
			let socket_addr = listener.local_addr()?;
			info!("Server listening data on {:?}", socket_addr);
			
			let message = format!("{} {}", ServerResponse::EnteringPassiveMode.to_string(), Client::get_addr_msg(socket_addr));
			self.ctrl_connection.write(message).await?;
			
			let (stream, addr) = listener.accept().await?;
			info!("Data connection open with addr {:?}", addr);
			let (rx, tx) = stream.into_split();
			self.data_connection = Some(Connection::new(rx, tx));
		}
		Ok(())
	}
	
	async fn list(&mut self) -> FtpResult<()> {
		if self.data_connection.is_some() {
			let mut data_connection = self.data_connection.take().unwrap();
			
			let msg = format!("{} {}", ServerResponse::FileStatusOk.to_string(), "Here comes the directory listing.");
			self.ctrl_connection.write(msg).await?;
			
			for msg in utils::get_ls(self.user.as_ref().unwrap().home_dir()) {
				data_connection.write(msg).await?;
			}
			
			data_connection.close().await;
			self.data_connection = None;
			
			let msg = format!("{} {}", ServerResponse::ClosingDataConnection.to_string(), "Directory send OK.");
			self.ctrl_connection.write(msg).await?;
			Ok(())
		} else {
			error!("Data connection not initialized");
			Err(FtpError::DataConnectionError)
		}
	}
	
	fn parse_port(msg: String) -> Option<(IpAddr, u16)> {
		debug!("client::parse_port {}", msg);
		let re = Regex::new(r"^([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3}),([[:digit:]]{1,3})$").ok()?;
		let cap = re.captures(msg.as_str())?;
		
		let mut addr: [u8; 4] = [0; 4];
		for i in 1..5 {
			addr[i - 1] = cap.get(i).unwrap().as_str().to_string().parse::<u8>().ok()?;
		}
		
		let port1 = cap.get(5).unwrap().as_str().to_string().parse::<u16>().ok()?;
		let port2 = cap.get(6).unwrap().as_str().to_string().parse::<u16>().ok()?;
		let port = port1 * 256 + port2;
		
		Some((IpAddr::from(addr), port))
	}
	
	fn get_addr_msg(addr: SocketAddr) -> String {
		let ip = ADDR.replace(".", ",");
		let port = addr.port();
		let port1 = port / 256;
		let port2 = port % 256;
		
		format!("({},{},{})", ip, port1, port2)
	}
	
	async fn syst(&mut self) -> FtpResult<()> {
		info!("SYST command");
		self.ctrl_connection.write(ServerResponse::SystemType.to_string()).await
	}
	
	async fn connect(&mut self) -> bool {
		match self.user().await {
			Some(login) => {
				info!("Login: {}", login);
				if let Err(e) = self.ctrl_connection.write(ServerResponse::UserNameOkayNeedPassword.to_string()).await {
					error!("Not connected {:?}", e);
				}
				if self.password().await.is_some() {
					info!("Password: \"x\"");
					let user = get_user_by_name(login.as_str());
					if user.is_some() {
						self.user = user;
						if let Err(e) = self.ctrl_connection.write(ServerResponse::UserLoggedIn.to_string()).await {
							error!("Not connected {:?}", e);
						}
						return true;
					}
				}
			}
			_ => {}
		}
		if let Err(e) = self.ctrl_connection.write(ServerResponse::NotLoggedIn.to_string()).await {
			error!("Not connected {:?}", e);
		}
		false
	}
	
	async fn user(&mut self) -> Option<String> {
		debug!("client::user");
		let msg = self.ctrl_connection.read().await?;
		
		return match self.parse_command(msg.clone()) {
			ClientCommand::User(args) => {
				if self.check_username(args.clone()) {
					Some(args.clone())
				} else {
					error!("User name error: {}", args);
					None
				}
			}
			err => {
				error!("Unexpected command: {}", err);
				None
			}
		};
	}
	
	async fn password(&mut self) -> Option<String> {
		debug!("client::password");
		let msg = self.ctrl_connection.read().await?;
		return match self.parse_command(msg.clone()) {
			ClientCommand::Pass(args) => {
				if self.check_username(args.clone()) {
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
		debug!("client::parse_command '{}'", msg);
		if let Some(re) = Regex::new(r"^([[:upper:]]{3,4})( .+)*$").ok() {
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
		error!("failed to parse command: {}", msg);
		ClientCommand::Unknown(msg)
	}
	
	fn check_username(&self, username: String) -> bool {
		let re = Regex::new(r"^([[:word:]]+)$").unwrap();
		re.captures(username.as_str()).is_some()
	}
	
	pub async fn close_connection(&mut self) {
		info!("Close client connection");
		if self.user.is_some() {
			if let Err(e) = self.ctrl_connection.write(ServerResponse::ConnectionClosed.to_string()).await {
				error!("Failed to close connection with client: {:?}", e);
			}
			self.user = None;
		}
		self.ctrl_connection.close().await;
	}
}