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
use crate::{Connection, protocol, utils};
use crate::utils::error::{FtpError, FtpResult};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use async_std::{io, task};
use std::{thread::sleep, time::Duration};
use tokio::sync::mpsc::Receiver;
use crate::protocol::{ClientCommand, ServerResponse};
use std::io::prelude::*;

pub struct ClientFtp {
	pub connection: Connection,
}

impl ClientFtp {
	pub async fn new(addr: IpAddr, port: u16) -> FtpResult<Self> {
		info!("New ClientFTP {} {}", addr.to_string(), port);
		
		return if let Ok(socket) = TcpStream::connect(SocketAddr::new(addr, port.to_string().parse::<u16>().unwrap())).await {
			let (rx, tx) = socket.into_split();
			let mut connection = Connection::new(rx, tx);
			
			Ok(ClientFtp {
				connection,
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
		self.connect().await;
		self.handle_commands().await;
	}
	
	async fn connect(&mut self) -> FtpResult<()> {
		self.user().await?;
		self.password().await?;
		
		if let Some(msg) = self.connection.read().await {
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
		if let Some(msg) = self.connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::ServiceReadyForNewUser {
				println!("{}", msg);
				let user_name = utils::read_from_cmd_line("Name: ").await?;
				self.connection.write(ClientCommand::User(user_name.to_string()).to_string()).await?;
				return Ok(());
			}
		}
		error!("Failed to send USER to the server");
		return Err(FtpError::UserConnectionError);
	}
	
	async fn password(&mut self) -> FtpResult<()> {
		if let Some(msg) = self.connection.read().await {
			let response = protocol::parse_server_response(&msg);
			if response.0 == ServerResponse::UserNameOkayNeedPassword {
				println!("{}", msg);
				let password = utils::read_from_cmd_line("Password: ").await?;
				self.connection.write(ClientCommand::Pass(password.to_string()).to_string()).await?;
				return Ok(());
			}
		}
		error!("Failed to send PASS to the server");
		return Err(FtpError::UserConnectionError);
	}
	
	async fn handle_commands(&mut self) -> FtpResult<()> {
		loop {
			thread::sleep(time::Duration::from_millis(1000));
		}
		Ok(())
	}
	
	async fn close_connection(&mut self) -> FtpResult<()> {
		self.connection.close();
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
