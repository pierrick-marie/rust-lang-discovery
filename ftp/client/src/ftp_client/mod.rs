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

use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use std::net::{IpAddr, SocketAddr};
use log::Level::Debug;
use crate::{Connection, DEFAULT_ADDR, protocol, utils};
use crate::utils::error::{FtpError, FtpResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use async_std::{io, task};
use std::{thread::sleep, time::Duration};
use tokio::sync::mpsc::Receiver;
use crate::protocol::{ClientCommand, parse_server_response, ServerResponse, TransfertMode};
use std::io::prelude::*;
use std::path::PathBuf;
use futures::future::err;
use portpicker::pick_unused_port;
use scanpw::scanpw;
use crate::ftp_client::command::{Command, parse_command};

mod command;

pub struct ClientFtp {
	ctrl_connection: Connection,
	mode: TransfertMode,
}

impl ClientFtp {
	pub async fn new(addr: IpAddr, port: u16) -> FtpResult<Self> {
		info!("New ClientFTP {} {}", addr.to_string(), port);
		
		return if let Ok(socket) = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await {
			let (rx, tx) = socket.into_split();
			let mut connection = Connection::new(rx, tx);
			
			Ok(ClientFtp {
				ctrl_connection: connection,
				mode: TransfertMode::Active,
			})
		} else {
			Err(FtpError::ConnectionError)
		};
	}
	
	pub async fn start(&mut self) {
		info!("START !");
		tokio::select! {
			_ = tokio::spawn(wait_ctrlc()) => {
				println!("Wait CTRL-C completed first");
			}
			_ = self.run() => {
				println!("Client run completed first");
			}
		}
		self.close_connection().await;
	}
	
	async fn run(&mut self) {
		if self.connect().await.is_ok() {
			self.syst().await;
			self.handle_commands().await;
		}
	}
	
	async fn connect(&mut self) -> FtpResult<()> {
		self.user().await?;
		self.password().await?;
		
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::UserLoggedIn {
				println!("{}", msg);
				return Ok(());
			}
		}
		error!("Failed to login");
		Err(FtpError::UserConnectionError)
	}
	
	async fn user(&mut self) -> FtpResult<()> {
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::ServiceReadyForNewUser {
				println!("{}", msg);
				let user_name = utils::read_from_cmd_line("Name: ").await?;
				self.ctrl_connection.write(ClientCommand::User(user_name.to_string()).to_string()).await?;
				return Ok(());
			}
		}
		error!("Failed to send USER to the server");
		return Err(FtpError::UserConnectionError);
	}
	
	async fn password(&mut self) -> FtpResult<()> {
		if let Some(msg) = self.ctrl_connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::UserNameOkayNeedPassword {
				let handle = tokio::spawn(async move {
					let password = scanpw!("Password: ");
					return password;
				});
				let password = handle.await.unwrap();
				self.ctrl_connection.write(ClientCommand::Pass(password).to_string()).await;
				return Ok(());
			}
		}
		error!("Failed to send PASS to the server");
		return Err(FtpError::UserConnectionError);
	}
	
	async fn syst(&mut self) -> FtpResult<()> {
		self.ctrl_connection.write(ClientCommand::Syst.to_string()).await?;
		if let Some(msg) = self.ctrl_connection.read().await {
			println!("{}", msg);
		}
		Ok(())
	}
	
	async fn handle_commands(&mut self) -> FtpResult<()> {
		let mut command: String;
		loop {
			command = utils::read_from_cmd_line("ftp>  ").await?;
			let command = parse_command(&command);
			match command {
				Command::Help => { self.help().await; }
				Command::Unknown(arg) => { println!("Unknown command"); }
				Command::Ls(arg) => { self.ls().await; }
				Command::Pass => { self.mode = TransfertMode::Passive; println!("Set up passive mode"); }
			}
		}
		Ok(())
	}
	
	async fn help(&mut self) {
		println!(" Help message");
		println!(" Available commands: help ls");
	}
	
	async fn ls(&mut self) -> FtpResult<()> {
		
		if self.mode == TransfertMode::Active {
			let port: u16 = pick_unused_port().expect("No ports free");
			let listener = TcpListener::bind(format!("{}:{}", DEFAULT_ADDR, port)).await?;
			let socket_addr = listener.local_addr()?;
			info!("Server listening data on {:?}", socket_addr);
			self.ctrl_connection.write(ClientCommand::Port(utils::get_addr_msg(socket_addr)).to_string()).await?;
			
			if let Some(msg) = self.ctrl_connection.read().await {
				if parse_server_response(&msg).0 == ServerResponse::OK {
					println!("{}", msg);
				} else {
					error!("Failed to send LIST command");
					return Err(FtpError::ConnectionError);
				}
			}
			
			self.ctrl_connection.write(ClientCommand::List(PathBuf::from(".")).to_string()).await?;
			
			debug!("Wait new connection");
			let (stream, addr) = listener.accept().await?;
			info!("Data connection open with addr {:?}", addr);
			let (rx, tx) = stream.into_split();
			let mut data_connection = Connection::new(rx, tx);
			
			if let Some(msg) = self.ctrl_connection.read().await {
				if parse_server_response(&msg).0 == ServerResponse::FileStatusOk {
					println!("{}", msg);
				} else {
					error!("Failed to tranfert LIST command");
					return Err(FtpError::ConnectionError);
				}
			}
			
			let mut msg: String = data_connection.read().await.unwrap_or("Failed to read data connection".to_string());
			// while ! msg.starts_with("226") {
			// 	println!("{}", msg);
			// 	msg = data_connection.read().await.unwrap_or("Failed to read data connection".to_string());
			// }
			
			if let Some(msg) = self.ctrl_connection.read().await {
				if parse_server_response(&msg).0 == ServerResponse::ClosingDataConnection {
					println!("{}", msg);
				} else {
					error!("Failed to tranfert LIST command");
					return Err(FtpError::ConnectionError);
				}
			}
			
			data_connection.close();
		} else {
			// Passive mode
			self.ctrl_connection.write(ClientCommand::Pasv.to_string()).await;
		}
		
		
		
		
		
		
		
		Ok(())
	}
	
	async fn close_connection(&mut self) -> FtpResult<()> {
		self.ctrl_connection.close();
		info!("Connection closed");
		Ok(())
	}
}

async fn wait_ctrlc() {
	let keep_running = Arc::new(AtomicBool::new(true));
	let running = keep_running.clone();
	
	ctrlc::set_handler(move || {
		info!("Received CTRL-C");
		running.store(false, Ordering::SeqCst);
	}).expect("Error setting Ctrl-C handler");
	
	while keep_running.load(Ordering::SeqCst) {
		thread::sleep(time::Duration::from_millis(500));
	}
	debug!("End of wait CTRL-C");
}
